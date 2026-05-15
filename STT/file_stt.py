"""
file_stt.py — Dual-delay file transcription with Rich UI.

Transcribes a WAV file using two parallel Mistral STT streams:
  - Fast (240ms delay): quick feedback
  - Slow (2400ms delay): high accuracy
Merges results in a beautiful terminal UI, simulating real-time playback.

Usage:
    python file_stt.py
    python file_stt.py --file audio.wav
"""

import argparse
import asyncio
import difflib
import os
import struct
import sys
import wave
from dataclasses import dataclass
from typing import AsyncIterator, Sequence

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

# ── State & Display (Ported from Fragments 5) ────────────────────────

@dataclass
class DualTranscriptState:
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
            if block.size == 0: continue
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
        hdr.append(f" {self.state.fast_status}", style=self._status_style(self.state.fast_status))
        hdr.append(" │ ", style="dim")
        hdr.append(f"slow {self.slow_delay_ms}ms", style="white")
        hdr.append(f" {self.state.slow_status}", style=self._status_style(self.state.slow_status))
        header = Align.left(hdr, vertical="middle", pad=False)

        final_text, partial_text = self._compute_display_texts()
        body = Text()
        if final_text or partial_text:
            body.append(final_text, style="white")
            body.append(partial_text, style="bright_yellow")
        else:
            body.append("...", style="dim")
        transcript = Panel(Align.left(body, vertical="top"), border_style="dim", padding=(1, 2))

        ftr = Text()
        ftr.append("ctrl+c", style="dim")
        ftr.append(" quit", style="dim italic")
        footer = Align.left(ftr, vertical="middle", pad=False)

        layout.split_column(
            Layout(header, name="header", size=1),
            Layout(transcript, name="body"),
            Layout(footer, name="footer", size=1),
        )
        return layout

# ── Audio Processing (Ported from Fragments 6) ───────────────────────

async def iter_wav(path: str, sr: int = 16000, chunk_ms: int = 480) -> AsyncIterator[bytes]:
    with wave.open(path, "rb") as wf:
        nch, sw, fsr, nf = wf.getnchannels(), wf.getsampwidth(), wf.getframerate(), wf.getnframes()
        raw = wf.readframes(nf)
    samples = list(struct.unpack(f"<{len(raw)//2}h", raw)) if sw == 2 else [(b-128)*256 for b in raw]
    if nch == 2:
        samples = [(samples[i]+samples[i+1])//2 for i in range(0, len(samples), 2)]
    if fsr != sr:
        ratio = sr / fsr
        samples = [samples[min(int(i/ratio), len(samples)-1)] for i in range(int(len(samples)*ratio))]
    cs = int(sr * chunk_ms / 1000)
    for s in range(0, len(samples), cs):
        chunk = samples[s:s+cs]
        if len(chunk) < cs: chunk.extend([0]*(cs-len(chunk)))
        await asyncio.sleep(chunk_ms / 1000.0) # Simulate real-time
        yield struct.pack(f"<{len(chunk)}h", *chunk)

async def queue_audio_iter(queue: asyncio.Queue[bytes | None]) -> AsyncIterator[bytes]:
    while True:
        chunk = await queue.get()
        if chunk is None: break
        yield chunk

async def broadcast_file(path: str, sr: int, chunk_ms: int, queues: Sequence[asyncio.Queue[bytes | None]]) -> None:
    try:
        async for chunk in iter_wav(path, sr, chunk_ms):
            for q in queues: await q.put(chunk)
    finally:
        for q in queues: await q.put(None)

# ── Main Implementation ──────────────────────────────────────────────

async def run_stream(client, model, delay, audio_stream, audio_format, state, update_queue, is_fast):
    try:
        async for ev in client.audio.realtime.transcribe_stream(
            audio_stream=audio_stream, model=model, audio_format=audio_format, target_streaming_delay_ms=delay
        ):
            if isinstance(ev, RealtimeTranscriptionSessionCreated):
                if is_fast: state.fast_status = "Listening..."
                else: state.slow_status = "Listening..."
            elif isinstance(ev, TranscriptionStreamTextDelta):
                if is_fast: state.fast_full_text += ev.text
                else: state.slow_full_text += ev.text
            elif isinstance(ev, TranscriptionStreamDone):
                if is_fast: state.fast_status = "Done"; state.fast_done = True
                else: state.slow_status = "Done"; state.slow_done = True
                break
            if update_queue.empty(): update_queue.put_nowait(None)
    except Exception as exc:
        state.set_error(str(exc))
        if update_queue.empty(): update_queue.put_nowait(None)

async def ui_loop(display, update_queue, stop_event):
    with Live(display.render(), console=console, refresh_per_second=12, screen=True) as live:
        while not stop_event.is_set():
            try: await asyncio.wait_for(update_queue.get(), timeout=0.25)
            except asyncio.TimeoutError: pass
            live.update(display.render())

async def main():
    p = argparse.ArgumentParser()
    p.add_argument("--file", default="audio.wav")
    args = p.parse_args()
    if not os.path.exists(args.file):
        print(f"ERROR: {args.file} not found"); return 1

    api_key = os.environ.get("MISTRAL_API_KEY")
    client = Mistral(api_key=api_key)
    state = DualTranscriptState()
    display = DualTranscriptDisplay(model="voxtral-mini-transcribe-realtime-2602", fast_delay_ms=240, slow_delay_ms=2400, state=state)
    af = AudioFormat(encoding="pcm_s16le", sample_rate=16000)

    fast_q, slow_q = asyncio.Queue(maxsize=50), asyncio.Queue(maxsize=50)
    stop_event, update_queue = asyncio.Event(), asyncio.Queue(maxsize=1)

    broadcaster = asyncio.create_task(broadcast_file(args.file, 16000, 10, (fast_q, slow_q)))
    fast_task = asyncio.create_task(run_stream(client, display.model, 240, queue_audio_iter(fast_q), af, state, update_queue, True))
    slow_task = asyncio.create_task(run_stream(client, display.model, 2400, queue_audio_iter(slow_q), af, state, update_queue, False))
    ui_task = asyncio.create_task(ui_loop(display, update_queue, stop_event))

    try:
        while not (state.fast_done and state.slow_done) and not state.error:
            await asyncio.sleep(0.1)
    except KeyboardInterrupt:
        pass
    finally:
        stop_event.set()
        broadcaster.cancel()
        await asyncio.gather(broadcaster, fast_task, slow_task, return_exceptions=True)
        await ui_task
    return 0

if __name__ == "__main__":
    sys.exit(asyncio.run(main()))
