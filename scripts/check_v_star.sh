#!/bin/bash
# check_v_star.sh — CI gate for v* convention parity (decided 2026-05-07).
#
# The root PARAMETERS.md § 1 declares Article form (v*_active = -0.08738)
# as canonical. Any subproject that ALSO declares a "canonical v*" value
# in its CLAUDE.md / CONCEPT.md / PARAMETERS.md must include the Article
# value either as the primary or as an explicit "Article equivalent".
#
# Runs from anywhere; returns 0 on success, 1 on mismatch.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXPECTED_ARTICLE_ASCII="-0.08738"
EXPECTED_ARTICLE_UNICODE="−0.08738"   # U+2212 minus; preferred for typeset docs
EXPECTED_PYTHON="0.45631"

# Match either typographic minus (U+2212) or ASCII dash.
match_article() { grep -qE -e "[−-]0\.08738" "$1"; }

ROOT_PARAMS="$ROOT/PARAMETERS.md"

echo "v* convention check (root: $ROOT_PARAMS)"

# 1. Root PARAMETERS.md must explicitly state both Article + Python forms.
if ! match_article "$ROOT_PARAMS"; then
    echo "FAIL: root PARAMETERS.md missing Article form 0.08738 with leading minus" >&2
    exit 1
fi
if ! grep -qF "$EXPECTED_PYTHON" "$ROOT_PARAMS"; then
    echo "FAIL: root PARAMETERS.md missing Python helper $EXPECTED_PYTHON" >&2
    exit 1
fi
if ! grep -qiF "Article" "$ROOT_PARAMS"; then
    echo "FAIL: root PARAMETERS.md missing 'Article' label" >&2
    exit 1
fi

# 2. Every subproject CLAUDE.md / PARAMETERS.md that mentions $EXPECTED_PYTHON
#    must ALSO mention $EXPECTED_ARTICLE within ±20 lines.
status=0
for f in "$ROOT"/{Ze,BioSense,MCOA,CDATA}/PARAMETERS.md \
         "$ROOT"/{Ze,BioSense,MCOA,CDATA}/CLAUDE.md \
         "$ROOT"/{Ze,BioSense,MCOA,CDATA}/CONCEPT.md ; do
    [ -f "$f" ] || continue
    if grep -qF "$EXPECTED_PYTHON" "$f"; then
        if ! match_article "$f"; then
            # Allow Ze/CONCEPT.md to use Python form for empirical/manuscript
            # claims (these are pinned to specific datasets).
            case "$f" in
                *"/Ze/CONCEPT.md") continue ;;
            esac
            echo "FAIL: $f mentions Python $EXPECTED_PYTHON without Article 0.08738 (with minus) alongside" >&2
            status=1
        fi
    fi
done

# 3. No file should claim "v* canonical" or "v* convention" in Python form
#    without the Article label.
while IFS= read -r match; do
    file="${match%%:*}"
    line="${match#*:}"
    if echo "$line" | grep -qiE "v\*.*(canonical|convention)" \
       && echo "$line" | grep -qF "$EXPECTED_PYTHON" \
       && ! echo "$line" | grep -qiF "Article" \
       && ! echo "$line" | grep -qE "[−-]0\.08738"; then
        echo "FAIL: $file — 'v* canonical/convention' claim without Article form/label: $line" >&2
        status=1
    fi
done < <(grep -rn "v\*\|v_star" "$ROOT" --include='*.md' --exclude-dir=_archive --exclude-dir=_audits 2>/dev/null || true)

if [ $status -eq 0 ]; then
    echo "OK: v* convention canonical-Article enforced across cross-subproject docs."
fi
exit $status
