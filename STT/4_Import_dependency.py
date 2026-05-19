import argparse
import asyncio
import difflib
import os
import sys
from dataclasses import dataclass
from typing import AsyncIterator, Sequence

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

from pyaudio_utils import load_pyaudio

console = Console()