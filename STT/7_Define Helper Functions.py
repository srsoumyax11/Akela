def _status_for_event(event: object) -> str:
    if isinstance(event, RealtimeTranscriptionSessionCreated):
        return "🎤 Listening..."
    return "✅ Done"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Dual-delay real-time microphone transcription."
    )
    parser.add_argument(
        "--model",
        default="voxtral-mini-transcribe-realtime-2602",
        help="Model ID",
    )
    parser.add_argument(
        "--fast-delay-ms",
        type=int,
        default=240,
        help="Fast target streaming delay in ms",
    )
    parser.add_argument(
        "--slow-delay-ms",
        type=int,
        default=2400,
        help="Slow target streaming delay in ms",
    )
    parser.add_argument(
        "--sample-rate",
        type=int,
        default=16000,
        choices=[8000, 16000, 22050, 44100, 48000],
        help="Sample rate in Hz",
    )
    parser.add_argument(
        "--chunk-duration",
        type=int,
        default=10,
        help="Chunk duration in ms",
    )
    parser.add_argument(
        "--api-key",
        default=os.environ.get("MISTRAL_API_KEY"),
        help="Mistral API key",
    )
    parser.add_argument(
        "--base-url",
        default=os.environ.get("MISTRAL_BASE_URL", "wss://api.mistral.ai"),
    )
    return parser.parse_args()