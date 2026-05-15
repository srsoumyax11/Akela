"""
simple_stt.py — Simplest Mistral Realtime STT demo.

Captures microphone audio via sounddevice and streams it to
Mistral's Voxtral model for live transcription.

Usage:
    python simple_stt.py
    python simple_stt.py --sample-rate 44100

Press Ctrl+C to stop.
"""

import asyncio
import os
import sys
from typing import AsyncIterator

import numpy as np
import sounddevice as sd
from dotenv import load_dotenv

from mistralai.client import Mistral
from mistralai.extra.realtime import UnknownRealtimeEvent
from mistralai.client.models import (
    AudioFormat,
    RealtimeTranscriptionError,
    RealtimeTranscriptionSessionCreated,
    TranscriptionStreamDone,
    TranscriptionStreamTextDelta,
)

# ── Load API key from .env ───────────────────────────────────────────
load_dotenv()
api_key = os.environ.get("MISTRAL_API_KEY")
if not api_key:
    print("ERROR: MISTRAL_API_KEY not set. Add it to .env or environment.")
    sys.exit(1)

client = Mistral(api_key=api_key)

# ── Audio config ─────────────────────────────────────────────────────
SAMPLE_RATE = 16000
CHUNK_DURATION_MS = 480  # 480ms chunks
audio_format = AudioFormat(encoding="pcm_s16le", sample_rate=SAMPLE_RATE)


async def iter_microphone(
    *,
    sample_rate: int,
    chunk_duration_ms: int,
) -> AsyncIterator[bytes]:
    """
    Yield microphone PCM chunks using sounddevice (16-bit mono).
    """
    chunk_samples = int(sample_rate * chunk_duration_ms / 1000)
    loop = asyncio.get_running_loop()

    # Open an InputStream via sounddevice
    stream = sd.InputStream(
        samplerate=sample_rate,
        channels=1,
        dtype="int16",
        blocksize=chunk_samples,
    )
    stream.start()

    try:
        while True:
            # stream.read() is blocking; run it off-thread
            data, _ = await loop.run_in_executor(
                None, stream.read, chunk_samples
            )
            yield data.tobytes()
    finally:
        stream.stop()
        stream.close()


async def main():
    print("🎤 Starting Mistral Realtime STT...")
    print("   Model: voxtral-mini-transcribe-realtime-2602")
    print("   Press Ctrl+C to stop.\n")

    audio_stream = iter_microphone(
        sample_rate=SAMPLE_RATE,
        chunk_duration_ms=CHUNK_DURATION_MS,
    )

    try:
        async for event in client.audio.realtime.transcribe_stream(
            audio_stream=audio_stream,
            model="voxtral-mini-transcribe-realtime-2602",
            audio_format=audio_format,
        ):
            if isinstance(event, RealtimeTranscriptionSessionCreated):
                print("Session created. Listening...\n")
            elif isinstance(event, TranscriptionStreamTextDelta):
                print(event.text, end="", flush=True)
            elif isinstance(event, TranscriptionStreamDone):
                print("\n\nTranscription done.")
            elif isinstance(event, RealtimeTranscriptionError):
                print(f"\nError: {event}")
            elif isinstance(event, UnknownRealtimeEvent):
                continue
    except KeyboardInterrupt:
        print("\n\nStopped.")


if __name__ == "__main__":
    sys.exit(asyncio.run(main()))
