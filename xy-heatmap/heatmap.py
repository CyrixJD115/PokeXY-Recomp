#!/usr/bin/env python3
"""CLI entry for the xy-heatmap-py toolkit (thin shim -> xhmap.cli).

Run from anywhere:
    python3 xy-heatmap-py/heatmap.py --help
    uv run heatmap.py serve --watch        # from inside xy-heatmap-py/
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from xhmap.cli import main

if __name__ == "__main__":
    sys.exit(main())
