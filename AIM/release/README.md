# release/ — AIM distribution packaging

Builds three platform-specific archives:
- `aim-<VERSION>-linux.tar.gz`
- `aim-<VERSION>-macos.tar.gz`
- `aim-<VERSION>-windows.zip`

Each archive contains a self-contained AIM source tree (no venv, no
build artefacts, no patient data, no DBs) plus a platform-specific
`README-INSTALL.txt` with the right install command.

## Build

```bash
./release/build_packages.sh                # → release/dist/        (full source, internal docs included)
./release/build_packages.sh --public       # → release/dist-public/ (CONCEPT/CLAUDE/TODO/PARAMETERS removed)
```

`release/dist/` and `release/dist-public/` are gitignored — binaries
are distributed via **GitHub Releases**, not committed to history.

## Publish

```bash
# Private (full) — to djabbat/AIM
gh release create v$(cat release/VERSION) \
  --repo djabbat/AIM \
  --title "AIM v$(cat release/VERSION)" \
  --notes-file release/RELEASE_NOTES.md \
  release/dist/aim-*

# Public (sanitised) — to djabbat/AIM-public
gh release create v$(cat release/VERSION) \
  --repo djabbat/AIM-public \
  --title "AIM v$(cat release/VERSION)" \
  --notes-file release/RELEASE_NOTES.md \
  release/dist-public/aim-*
```

After release, the OS-detection block on hive.longevity.ge points
users to `https://github.com/djabbat/LongevityCommon-public` for
source clone or to the GitHub Releases page for direct download.

## Versioning

`release/VERSION` is the canonical version string. Bump it before
each release. Format: `MAJOR.MINOR.PATCH`.
