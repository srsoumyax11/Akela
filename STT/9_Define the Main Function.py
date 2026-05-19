async def main() -> int:
    args = parse_args()
    api_key = args.api_key or os.environ["MISTRAL_API_KEY"]

    try:
        load_pyaudio()
    except RuntimeError as exc:
        console.print(str(exc), style="red")
        return 1

    state = DualTranscriptState()
    display = DualTranscriptDisplay(
        model=args.model,
        fast_delay_ms=args.fast_delay_ms,
        slow_delay_ms=args.slow_delay_ms,
        state=state,
    )

    client = Mistral(api_key=api_key, server_url=args.base_url)
    audio_format = AudioFormat(encoding="pcm_s16le", sample_rate=args.sample_rate)

    fast_queue: asyncio.Queue[bytes | None] = asyncio.Queue(maxsize=50)
    slow_queue: asyncio.Queue[bytes | None] = asyncio.Queue(maxsize=50)

    stop_event = asyncio.Event()
    update_queue: asyncio.Queue[None] = asyncio.Queue(maxsize=1)

    broadcaster = asyncio.create_task(
        broadcast_microphone(
            sample_rate=args.sample_rate,
            chunk_duration_ms=args.chunk_duration,
            queues=(fast_queue, slow_queue),
        )
    )

    fast_task = asyncio.create_task(
        run_stream(
            client=client,
            model=args.model,
            delay_ms=args.fast_delay_ms,
            audio_stream=queue_audio_iter(fast_queue),
            audio_format=audio_format,
            state=state,
            update_queue=update_queue,
            is_fast=True,
        )
    )

    slow_task = asyncio.create_task(
        run_stream(
            client=client,
            model=args.model,
            delay_ms=args.slow_delay_ms,
            audio_stream=queue_audio_iter(slow_queue),
            audio_format=audio_format,
            state=state,
            update_queue=update_queue,
            is_fast=False,
        )
    )

    ui_task = asyncio.create_task(
        ui_loop(display, update_queue, stop_event, refresh_hz=12.0)
    )

    try:
        while True:
            await asyncio.sleep(0.1)
            for task in (broadcaster, fast_task, slow_task):
                if not task.done():
                    continue
                exc = task.exception()
                if exc:
                    state.set_error(str(exc))
                    if update_queue.empty():
                        update_queue.put_nowait(None)
                    stop_event.set()
                    break
            if state.error:
                stop_event.set()
                break
            if state.fast_done and state.slow_done:
                stop_event.set()
                break
    except KeyboardInterrupt:
        state.fast_status = "⏹️ Stopped"
        state.slow_status = "⏹️ Stopped"
        stop_event.set()
    finally:
        broadcaster.cancel()
        fast_task.cancel()
        slow_task.cancel()
        await asyncio.gather(broadcaster, fast_task, slow_task, return_exceptions=True)
        await ui_task

    return 0 if not state.error else 1


if __name__ == "__main__":
    sys.exit(asyncio.run(main()))