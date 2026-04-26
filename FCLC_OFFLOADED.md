# FCLC — offloaded to server 2026-04-26

The FCLC subproject directory is no longer kept on the desktop machine.

- **Server location:** `jaba@server:/home/jaba/web/fclc/`
- **Server backup of pre-rsync state:** `jaba@server:~/fclc_server_backup_2026-04-26.tar.gz` (412M)
- **Local pre-offload backup:** `/home/oem/.cache/offloaded_2026-04-26/FCLC/` (2.0M, restorable)
- **Sync direction during offload:** local → server via rsync (excluded `target/`, `_build/`, `deps/`, `node_modules/`, `.sqlx/`)

## To work on FCLC

```sh
ssh server
cd /home/jaba/web/fclc
```

## To restore locally

```sh
mv /home/oem/.cache/offloaded_2026-04-26/FCLC /home/oem/Desktop/LongevityCommon/FCLC
rm /home/oem/Desktop/LongevityCommon/FCLC_OFFLOADED.md
```

## Git note

LongevityCommon is a monorepo — FCLC was a tracked subdirectory.
After offload, `git status` in LongevityCommon shows all FCLC files as deleted.
The user should decide when to:
1. `git rm -r FCLC && git commit -m "Offload FCLC to server"` (drops FCLC from monorepo permanently — server becomes sole live copy)
2. Or restore locally via the command above (no commit needed).
