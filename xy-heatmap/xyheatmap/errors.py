"""Package-internal error type; every failure surfaces as one of these."""

from __future__ import annotations


class HeatmapError(Exception):
    """A user-facing error: bad input, missing data, unknown view/module.

    The CLI prints these cleanly to stderr; nothing anywhere raises bare
    exceptions for expected failure modes.
    """
