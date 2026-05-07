#!/usr/bin/env bash
# scripts/desktop/aim_local_launch.sh
# Launch LOCAL AIM Phoenix dev server (force local, no prod fallback).
#
# Difference from aim_ui_launch.sh: this skips the prod check entirely.
# Always boots `mix phx.server` on 127.0.0.1:4000 and opens browser.
#
# Use case: developing or demoing AIM offline / on a machine with no
# network access to aim.longevity.ge.
set -uo pipefail
REPO_ROOT="/home/oem/Desktop/LongevityCommon/AIM"
PHOENIX="$REPO_ROOT/phoenix-umbrella"
PORT="${AIM_LOCAL_PORT:-4000}"
LOCAL_URL="http://127.0.0.1:${PORT}/"

reachable() {
    /usr/bin/curl -m 2 -sS -o /dev/null -w "%{http_code}" "$1" 2>/dev/null
}

# 1) Already running?
if [ "$(reachable "$LOCAL_URL")" = "200" ]; then
    /usr/bin/xdg-open "$LOCAL_URL" >/dev/null 2>&1 &
    exit 0
fi

# 2) Boot in visible terminal so user can read errors / reload notices.
/usr/bin/gnome-terminal --title="AIM local Phoenix dev server (port ${PORT})" -- \
    /usr/bin/bash -lc "
        cd '${PHOENIX}' || exit 1;
        echo '── Starting local AIM Phoenix LiveView (port ${PORT}) ──';
        echo '   Mode: LOCAL ONLY (prod aim.longevity.ge bypassed)';
        echo '   Set AIM_ADMIN_ENABLE=1 to unlock /admin action buttons.';
        echo;
        export AIM_ADMIN_ENABLE=1;
        PORT=${PORT} /usr/bin/mix phx.server;
        echo;
        echo 'phx.server exited; press Enter to close.';
        read
    "

# 3) Wait for bind, then open browser
for i in $(/usr/bin/seq 1 20); do
    /usr/bin/sleep 0.5
    if [ "$(reachable "$LOCAL_URL")" = "200" ]; then
        /usr/bin/xdg-open "${LOCAL_URL}admin" >/dev/null 2>&1 &
        exit 0
    fi
done

# Fallback — open anyway (user will see "can't connect" + terminal explains)
/usr/bin/xdg-open "${LOCAL_URL}admin" >/dev/null 2>&1 &
exit 0
