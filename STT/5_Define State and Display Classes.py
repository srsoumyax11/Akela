@dataclass
class DualTranscriptState:
    """Tracks transcript state for dual-delay transcription."""

    fast_full_text: str = ""
    slow_full_text: str = ""
    fast_status: str = "🔌 Connecting..."
    slow_status: str = "🔌 Connecting..."
    error: str | None = None
    fast_done: bool = False
    slow_done: bool = False

    def set_error(self, message: str) -> None:
        self.error = message
        self.fast_status = "❌ Error"
        self.slow_status = "❌ Error"


class DualTranscriptDisplay:
    """Renders a live dual-delay transcription UI."""

    def __init__(
        self,
        *,
        model: str,
        fast_delay_ms: int,
        slow_delay_ms: int,
        state: DualTranscriptState,
    ) -> None:
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
            partial_text = f" {self.state.fast_full_text}".rstrip()
            return "", partial_text

        slow_norm = [self._normalize_word(word) for word in slow_words]
        fast_norm = [self._normalize_word(word) for word in fast_words]

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
            ahead_words = fast_words[last_fast_index:]
            partial_text = " " + " ".join(ahead_words) if ahead_words else ""
        else:
            partial_text = ""

        return self.state.slow_full_text, partial_text

    @staticmethod
    def _status_style(status: str) -> str:
        if "Listening" in status:
            return "green"
        if "Connecting" in status:
            return "yellow dim"
        if "Done" in status or "Stopped" in status:
            return "dim"
        return "red"

    def render(self) -> Layout:
        layout = Layout()

        header_text = Text()
        header_text.append("│ ", style="dim")
        header_text.append(self.model, style="dim")
        header_text.append(" │ ", style="dim")
        header_text.append(
            f"fast {self.fast_delay_ms}ms", style="bright_yellow"
        )
        header_text.append(
            f" {self.state.fast_status}",
            style=self._status_style(self.state.fast_status),
        )
        header_text.append(" │ ", style="dim")
        header_text.append(f"slow {self.slow_delay_ms}ms", style="white")
        header_text.append(
            f" {self.state.slow_status}",
            style=self._status_style(self.state.slow_status),
        )

        header = Align.left(header_text, vertical="middle", pad=False)

        final_text, partial_text = self._compute_display_texts()
        transcript_text = Text()
        if final_text or partial_text:
            transcript_text.append(final_text, style="white")
            transcript_text.append(partial_text, style="bright_yellow")
        else:
            transcript_text.append("...", style="dim")

        transcript = Panel(
            Align.left(transcript_text, vertical="top"),
            border_style="dim",
            padding=(1, 2),
        )

        footer_text = Text()
        footer_text.append("ctrl+c", style="dim")
        footer_text.append(" quit", style="dim italic")
        footer = Align.left(footer_text, vertical="middle", pad=False)

        if self.state.error:
            layout.split_column(
                Layout(header, name="header", size=1),
                Layout(transcript, name="body"),
                Layout(
                    Panel(Text(self.state.error, style="red"), border_style="red"),
                    name="error",
                    size=4,
                ),
                Layout(footer, name="footer", size=1),
            )
        else:
            layout.split_column(
                Layout(header, name="header", size=1),
                Layout(transcript, name="body"),
                Layout(footer, name="footer", size=1),
            )

        return layout