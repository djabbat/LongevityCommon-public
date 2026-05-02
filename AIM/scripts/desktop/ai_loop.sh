#!/usr/bin/env bash
# Thin launcher → ai_loop.py (canonical). The .py file owns the input-sanitizer
# (shell-prompt strip), banner, and streaming loop. Don't fork logic into the
# heredoc form — it desyncs from .py and breaks input() on stdin-attached TTY.
cd "/home/oem/Desktop/LongevityCommon/AIM"
[ -d venv ] && source venv/bin/activate
exec python3 "/home/oem/Desktop/LongevityCommon/AIM/scripts/desktop/ai_loop.py"
