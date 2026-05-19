from mistralai.client import Mistral
from mistralai.extra.realtime import UnknownRealtimeEvent
from mistralai.client.models import AudioFormat, RealtimeTranscriptionError, RealtimeTranscriptionSessionCreated, TranscriptionStreamDone, TranscriptionStreamTextDelta

import asyncio
import sys
from typing import AsyncIterator

api_key = "nZvyCo17oScjkNgJPIaw5b3YC5mu3ElV"
client = Mistral(api_key=api_key)

audio_format = AudioFormat(encoding="pcm_s16le", sample_rate=16000)
audio_stream = ...

async def main():
    try:
        async for event in client.audio.realtime.transcribe_stream(
            audio_stream=audio_stream, # audio stream corresponds to any iterable of bytes
            model="voxtral-mini-transcribe-realtime-2602",
            audio_format=audio_format,
        ):
            if isinstance(event, RealtimeTranscriptionSessionCreated):
                print(f"Session created.")
            elif isinstance(event, TranscriptionStreamTextDelta):
                print(event.text, end="", flush=True)
            elif isinstance(event, TranscriptionStreamDone):
                print("Transcription done.")
            elif isinstance(event, RealtimeTranscriptionError):
                print(f"Error: {event}")
            elif isinstance(event, UnknownRealtimeEvent):
                print(f"Unknown event: {event}")
                continue
    except KeyboardInterrupt:
        print("Stopping...")

sys.exit(asyncio.run(main()))