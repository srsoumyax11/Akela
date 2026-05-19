"""
dual_delay_stt.py — Dual-delay realtime transcription with Rich UI.

Runs two parallel Mistral STT streams:
  - Fast (240ms delay): quick but less accurate
  - Slow (2400ms delay): slower but more accurate
Merges results in a beautiful terminal UI.

Usage:
    python dual_delay_stt.py
    python dual_delay_stt.py --fast-delay-ms 500 --slow-delay-ms 3000

Press Ctrl+C to stop.
"""

import argparse
import asyncio
import difflib
import os
import sys
from dataclasses import dataclass
from typing import AsyncIterator, Sequence

import numpy as np
import sounddevice as sd
from dotenv import load_dotenv

from rich.align import Align
from rich.console import Console
from rich.layout import Layout
from rich.live import Live
from rich.panel import Panel
from rich.text import Text

from mistralai.client import Mistral
from mistralai.extra.realtime import UnknownRealtimeEvent
from mistralai.client.models import (
    AudioFormat,
    RealtimeTranscriptionError,
    RealtimeTranscriptionSessionCreated,
    TranscriptionStreamDone,
    TranscriptionStreamTextDelta,
)

load_dotenv()
console = Console()


# ── State & Display ──────────────────────────────────────────────────

@dataclass
class DualTranscriptState:
    """Tracks transcript state for dual-delay transcription."""
    fast_full_text: str = ""
    slow_full_text: str = ""
    fast_status: str = "Connecting..."
    slow_status: str = "Connecting..."
    error: str | None = None
    fast_done: bool = False
    slow_done: bool = False

    def set_error(self, message: str) -> None:
        self.error = message
        self.fast_status = "Error"
        self.slow_status = "Error"


class DualTranscriptDisplay:
    """Renders a live dual-delay transcription UI."""

    def __init__(self, *, model: str, fast_delay_ms: int,
                 slow_delay_ms: int, state: DualTranscriptState) -> None:
        self.model = model
        self.fast_delay_ms = fast_delay_ms
        self.slow_delay_ms = slow_delay_ms
        self.state = state

    @staticmethod
    def _normalize_word(word: str) -> str:
        return word.strip(".,!?;:\"'()[]{}").lower()

    def _compute_display_texts(self) -> tuple[str, str]:
        slow_words = self.state.slow_full_text.split()
        fast_words = self.state.fast_full_text.split()
        if not slow_words:
            return "", f" {self.state.fast_full_text}".rstrip()
        slow_norm = [self._normalize_word(w) for w in slow_words]
        fast_norm = [self._normalize_word(w) for w in fast_words]
        matcher = difflib.SequenceMatcher(None, slow_norm, fast_norm)
        last_fast_index = 0
        slow_progress = 0
        for block in matcher.get_matching_blocks():
            if block.size == 0:
                continue
            slow_end = block.a + block.size
            if slow_end > slow_progress:
                slow_progress = slow_end
                last_fast_index = block.b + block.size
        if last_fast_index < len(fast_words):
            partial = " " + " ".join(fast_words[last_fast_index:])
        else:
            partial = ""
        return self.state.slow_full_text, partial

    @staticmethod
    def _status_style(status: str) -> str:
        if "Listening" in status: return "green"
        if "Connecting" in status: return "yellow dim"
        if "Done" in status or "Stopped" in status: return "dim"
        return "red"

    def render(self) -> Layout:
        layout = Layout()
        hdr = Text()
        hdr.append("│ ", style="dim")
        hdr.append(self.model, style="dim")
        hdr.append(" │ ", style="dim")
        hdr.append(f"fast {self.fast_delay_ms}ms", style="bright_yellow")
        hdr.append(f" {self.state.fast_status}",
                   style=self._status_style(self.state.fast_status))
        hdr.append(" │ ", style="dim")
        hdr.append(f"slow {self.slow_delay_ms}ms", style="white")
        hdr.append(f" {self.state.slow_status}",
                   style=self._status_style(self.state.slow_status))
        header = Align.left(hdr, vertical="middle", pad=False)

        final_text, partial_text = self._compute_display_texts()
        body = Text()
        if final_text or partial_text:
            body.append(final_text, style="white")
            body.append(partial_text, style="bright_yellow")
        else:
            body.append("...", style="dim")
        transcript = Panel(Align.left(body, vertical="top"),
                          border_style="dim", padding=(1, 2))

        ftr = Text()
        ftr.append("ctrl+c", style="dim")
        ftr.append(" quit", style="dim italic")
        footer = Align.left(ftr, vertical="middle", pad=False)

        if self.state.error:
            layout.split_column(
                Layout(header, name="header", size=1),
                Layout(transcript, name="body"),
                Layout(Panel(Text(self.state.error, style="red"),
                            border_style="red"), name="error", size=4),
                Layout(footer, name="footer", size=1),
            )
        else:
            layout.split_column(
                Layout(header, name="header", size=1),
                Layout(transcript, name="body"),
                Layout(footer, name="footer", size=1),
            )
        return layout


# ── Audio capture (sounddevice) ──────────────────────────────────────

async def iter_microphone(*, sample_rate: int,
                          chunk_duration_ms: int) -> AsyncIterator[bytes]:
    chunk_samples = int(sample_rate * chunk_duration_ms / 1000)
    loop = asyncio.get_running_loop()
    stream = sd.InputStream(samplerate=sample_rate, channels=1,
                            dtype="int16", blocksize=chunk_samples)
    stream.start()
    try:
        while True:
            data, _ = await loop.run_in_executor(
                None, stream.read, chunk_samples)
            yield data.tobytes()
    finally:
        stream.stop()
        stream.close()


async def queue_audio_iter(
    queue: asyncio.Queue[bytes | None],
) -> AsyncIterator[bytes]:
    while True:
        chunk = await queue.get()
        if chunk is None:
            break
        yield chunk


async def broadcast_microphone(*, sample_rate: int, chunk_duration_ms: int,
                               queues: Sequence[asyncio.Queue[bytes | None]]) -> None:
    try:
        async for chunk in iter_microphone(
            sample_rate=sample_rate, chunk_duration_ms=chunk_duration_ms
        ):
            for q in queues:
                await q.put(chunk)
    finally:
        for q in queues:
            while True:
                try:
                    q.put_nowait(None); break
                except asyncio.QueueFull:
                    try: q.get_nowait()
                    except asyncio.QueueEmpty: break


# ── Helpers ──────────────────────────────────────────────────────────

def _status_for_event(event: object) -> str:
    if isinstance(event, RealtimeTranscriptionSessionCreated):
        return "Listening..."
    return "Done"


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(
        description="Dual-delay real-time microphone transcription.")
    p.add_argument("--model", default="voxtral-mini-transcribe-realtime-2602")
    p.add_argument("--fast-delay-ms", type=int, default=240)
    p.add_argument("--slow-delay-ms", type=int, default=2400)
    p.add_argument("--sample-rate", type=int, default=16000,
                   choices=[8000, 16000, 22050, 44100, 48000])
    p.add_argument("--chunk-duration", type=int, default=10)
    p.add_argument("--api-key", default=os.environ.get("MISTRAL_API_KEY"))
    p.add_argument("--base-url",
                   default=os.environ.get("MISTRAL_BASE_URL",
                                          "wss://api.mistral.ai"))
    return p.parse_args()


# ── Stream & UI ──────────────────────────────────────────────────────

async def run_stream(*, client: Mistral, model: str, delay_ms: int,
                     audio_stream: AsyncIterator[bytes],
                     audio_format: AudioFormat,
                     state: DualTranscriptState,
                     update_queue: asyncio.Queue[None],
                     is_fast: bool) -> None:
    try:
        async for event in client.audio.realtime.transcribe_stream(
            audio_stream=audio_stream, model=model,
            audio_format=audio_format,
            target_streaming_delay_ms=delay_ms,
        ):
            if isinstance(event, RealtimeTranscriptionSessionCreated):
                if is_fast: state.fast_status = _status_for_event(event)
                else: state.slow_status = _status_for_event(event)
            elif isinstance(event, TranscriptionStreamTextDelta):
                if is_fast: state.fast_full_text += event.text
                else: state.slow_full_text += event.text
            elif isinstance(event, TranscriptionStreamDone):
                if is_fast:
                    state.fast_status = _status_for_event(event)
                    state.fast_done = True
                else:
                    state.slow_status = _status_for_event(event)
                    state.slow_done = True
                break
            elif isinstance(event, RealtimeTranscriptionError):
                state.set_error(str(event.error)); break
            elif isinstance(event, UnknownRealtimeEvent):
                continue
            if update_queue.empty():
                update_queue.put_nowait(None)
    except Exception as exc:
        state.set_error(str(exc))
        if update_queue.empty():
            update_queue.put_nowait(None)


async def ui_loop(display: DualTranscriptDisplay,
                  update_queue: asyncio.Queue[None],
                  stop_event: asyncio.Event, *,
                  refresh_hz: float = 12.0) -> None:
    with Live(display.render(), console=console,
              refresh_per_second=refresh_hz, screen=True) as live:
        while not stop_event.is_set():
            try:
                await asyncio.wait_for(update_queue.get(), timeout=0.25)
            except asyncio.TimeoutError:
                pass
            live.update(display.render())


# ── Main ─────────────────────────────────────────────────────────────

async def main() -> int:
    args = parse_args()
    api_key = args.api_key
    if not api_key:
        console.print("ERROR: MISTRAL_API_KEY not set.", style="red")
        return 1

    state = DualTranscriptState()
    display = DualTranscriptDisplay(
        model=args.model, fast_delay_ms=args.fast_delay_ms,
        slow_delay_ms=args.slow_delay_ms, state=state)

    client = Mistral(api_key=api_key, server_url=args.base_url)
    audio_format = AudioFormat(encoding="pcm_s16le",
                               sample_rate=args.sample_rate)

    fast_q: asyncio.Queue[bytes | None] = asyncio.Queue(maxsize=50)
    slow_q: asyncio.Queue[bytes | None] = asyncio.Queue(maxsize=50)
    stop_event = asyncio.Event()
    update_queue: asyncio.Queue[None] = asyncio.Queue(maxsize=1)

    broadcaster = asyncio.create_task(broadcast_microphone(
        sample_rate=args.sample_rate,
        chunk_duration_ms=args.chunk_duration,
        queues=(fast_q, slow_q)))
    fast_task = asyncio.create_task(run_stream(
        client=client, model=args.model, delay_ms=args.fast_delay_ms,
        audio_stream=queue_audio_iter(fast_q),
        audio_format=audio_format, state=state,
        update_queue=update_queue, is_fast=True))
    slow_task = asyncio.create_task(run_stream(
        client=client, model=args.model, delay_ms=args.slow_delay_ms,
        audio_stream=queue_audio_iter(slow_q),
        audio_format=audio_format, state=state,
        update_queue=update_queue, is_fast=False))
    ui_task = asyncio.create_task(
        ui_loop(display, update_queue, stop_event, refresh_hz=12.0))

    try:
        while True:
            await asyncio.sleep(0.1)
            for task in (broadcaster, fast_task, slow_task):
                if not task.done(): continue
                exc = task.exception()
                if exc:
                    state.set_error(str(exc))
                    if update_queue.empty():
                        update_queue.put_nowait(None)
                    stop_event.set(); break
            if state.error: stop_event.set(); break
            if state.fast_done and state.slow_done:
                stop_event.set(); break
    except KeyboardInterrupt:
        state.fast_status = "Stopped"
        state.slow_status = "Stopped"
        stop_event.set()
    finally:
        broadcaster.cancel(); fast_task.cancel(); slow_task.cancel()
        await asyncio.gather(broadcaster, fast_task, slow_task,
                            return_exceptions=True)
        await ui_task
    return 0 if not state.error else 1


if __name__ == "__main__":
    sys.exit(asyncio.run(main()))
