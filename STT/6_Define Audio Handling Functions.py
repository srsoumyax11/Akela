async def iter_microphone(
    *,
    sample_rate: int,
    chunk_duration_ms: int,
) -> AsyncIterator[bytes]:
    """
    Yield microphone PCM chunks using PyAudio (16-bit mono).
    Encoding is always pcm_s16le.
    """
    pyaudio = load_pyaudio()

    p = pyaudio.PyAudio()
    chunk_samples = int(sample_rate * chunk_duration_ms / 1000)

    stream = p.open(
        format=pyaudio.paInt16,
        channels=1,
        rate=sample_rate,
        input=True,
        frames_per_buffer=chunk_samples,
    )

    loop = asyncio.get_running_loop()
    try:
        while True:
            data = await loop.run_in_executor(None, stream.read, chunk_samples, False)
            yield data
    finally:
        stream.stop_stream()
        stream.close()
        p.terminate()


async def queue_audio_iter(
    queue: asyncio.Queue[bytes | None],
) -> AsyncIterator[bytes]:
    """Yield audio chunks from a queue until a None sentinel is received."""
    while True:
        chunk = await queue.get()
        if chunk is None:
            break
        yield chunk


async def broadcast_microphone(
    *,
    sample_rate: int,
    chunk_duration_ms: int,
    queues: Sequence[asyncio.Queue[bytes | None]],
) -> None:
    """Read from the microphone once and broadcast to multiple queues."""
    try:
        async for chunk in iter_microphone(
            sample_rate=sample_rate, chunk_duration_ms=chunk_duration_ms
        ):
            for queue in queues:
                await queue.put(chunk)
    finally:
        for queue in queues:
            while True:
                try:
                    queue.put_nowait(None)
                    break
                except asyncio.QueueFull:
                    try:
                        queue.get_nowait()
                    except asyncio.QueueEmpty:
                        break