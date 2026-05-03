# deploy/systemd/ — native Phoenix services

These systemd unit files run the Phoenix releases (ze-web, biosense-web,
fclc-web) **natively on the host**, not inside Docker containers. The
Docker pipeline (`deploy/docker-compose-all.yml`) is kept for build-time
artefact production but no longer for runtime.

## Why native

- One process tree under `systemctl`, no docker daemon dependency
- Direct `journalctl -u ze-web` log access
- Lower memory footprint (no per-service runtime overlay)
- nginx upstream is `127.0.0.1:<port>` either way; no observable
  difference for end users

## Layout on server

```
/opt/ze-web/         ← Phoenix release (extracted from Docker image once,
/opt/biosense-web/     then rebuilt natively via mix release after edits)
/opt/fclc-web/

/etc/systemd/system/ze-web.service
/etc/systemd/system/biosense-web.service
/etc/systemd/system/fclc-web.service
```

Each release ships its own ERTS (Erlang Runtime System) — no host-level
Erlang/Elixir is needed at runtime. Build environment (asdf-managed
Erlang OTP 27 + Elixir 1.17) lives in `~/.asdf/` for source rebuilds.

## Bootstrap (one-shot, already done 2026-05-04)

```bash
# 1. Extract release from existing Docker image (one-time, before
#    asdf-based native rebuild was ready)
docker create --name extract <image>
docker cp extract:/app /tmp/release
docker rm extract
sudo mv /tmp/release /opt/<service>
sudo chown -R jaba:jaba /opt/<service>
sudo mkdir -p /opt/<service>/tmp

# 2. Install systemd unit
sudo cp deploy/systemd/<service>.service /etc/systemd/system/
sudo systemctl daemon-reload

# 3. Stop the Docker container, start the native service
docker stop <container-name>
sudo systemctl enable --now <service>
```

## Rebuild after editing source

```bash
cd /home/jaba/web/longevitycommon/<Project>/<project-web>
. ~/.asdf/asdf.sh
MIX_ENV=prod mix deps.get --only prod
MIX_ENV=prod mix compile
MIX_ENV=prod mix release --overwrite
sudo systemctl stop <service>
sudo cp -r _build/prod/rel/<service>/* /opt/<service>/
sudo systemctl start <service>
```

## Ports (loopback only — public TLS via nginx + Cloudflare)

| Service       | Port | nginx upstream            |
|---------------|------|---------------------------|
| ze-web        | 4400 | ze.longevity.ge           |
| biosense-web  | 4501 | biosense.longevity.ge     |
| fclc-web      | 4003 | fclc.longevity.ge         |
