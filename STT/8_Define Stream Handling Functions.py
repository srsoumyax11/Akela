async def run_stream(
    *,
    client: Mistral,
    model: str,
    delay_ms: int,
    audio_stream: AsyncIterator[bytes],
    audio_format: AudioFormat,
    state: DualTranscriptState,
    update_queue: asyncio.Queue[None],
    is_fast: bool,
) -> None:
    try:
        async for event in client.audio.realtime.transcribe_stream(
            audio_stream=audio_stream,
            model=model,
            audio_format=audio_format,
            target_streaming_delay_ms=delay_ms,
        ):
            if isinstance(event, RealtimeTranscriptionSessionCreated):
                if is_fast:
                    state.fast_status = _status_for_event(event)
                else:
                    state.slow_status = _status_for_event(event)
            elif isinstance(event, TranscriptionStreamTextDelta):
                if is_fast:
                    state.fast_full_text += event.text
                else:
                    state.slow_full_text += event.text
            elif isinstance(event, TranscriptionStreamDone):
                if is_fast:
                    state.fast_status = _status_for_event(event)
                    state.fast_done = True
                else:
                    state.slow_status = _status_for_event(event)
                    state.slow_done = True
                break
            elif isinstance(event, RealtimeTranscriptionError):
                state.set_error(str(event.error))
                break
            elif isinstance(event, UnknownRealtimeEvent):
                continue

            if update_queue.empty():
                update_queue.put_nowait(None)
    except Exception as exc:  # pragma: no cover - safety net for UI demo
        state.set_error(str(exc))
        if update_queue.empty():
            update_queue.put_nowait(None)


async def ui_loop(
    display: DualTranscriptDisplay,
    update_queue: asyncio.Queue[None],
    stop_event: asyncio.Event,
    *,
    refresh_hz: float = 12.0,
) -> None:
    with Live(
        display.render(), console=console, refresh_per_second=refresh_hz, screen=True
    ) as live:
        while not stop_event.is_set():
            try:
                await asyncio.wait_for(update_queue.get(), timeout=0.25)
            except asyncio.TimeoutError:
                pass
            live.update(display.render())