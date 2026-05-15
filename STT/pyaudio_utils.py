"""
pyaudio_utils.py — Audio utility module for Mistral Realtime STT.

Uses `sounddevice` instead of PyAudio for zero-hassle installation
on modern Python (no C compiler / Visual Studio Build Tools needed).
"""

import numpy as np


def load_pyaudio():
    """
    Legacy compatibility shim.
    The dual-delay demo references `from pyaudio_utils import load_pyaudio`.
    We satisfy that contract but our actual mic capture uses sounddevice.
    """
    try:
        import sounddevice  # noqa: F401
        return sounddevice
    except ImportError:
        raise RuntimeError(
            "sounddevice is required but not installed.\n"
            "Install it with:  pip install sounddevice"
        )
