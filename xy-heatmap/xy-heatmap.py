#!/usr/bin/env python3
"""xy-heatmap launcher — run the tool straight from the working tree.

    ./xy-heatmap/xy-heatmap.py serve --watch --port 8765
    ./xy-heatmap/xy-heatmap.py export --view treemap --format png --out t.png --scale 3

No install step: the package code lives in xyheatmap/ beside this script.
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from xyheatmap.cli import main  # noqa: E402

if __name__ == "__main__":
    sys.exit(main())
