# ── Build stage ────────────────────────────────────────────────────────────────
FROM elixir:1.16-slim AS builder

ENV MIX_ENV=prod

WORKDIR /app

RUN apt-get update && apt-get install -y \
    nodejs npm build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install hex and rebar
RUN mix local.hex --force && mix local.rebar --force

COPY mix.exs mix.lock ./
RUN mix deps.get --only prod
RUN mix deps.compile

COPY assets/ assets/
COPY lib/ lib/
COPY priv/ priv/
COPY config/ config/

RUN mix assets.deploy
RUN mix compile
RUN mix release

# ── Runtime stage ──────────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    openssl libncurses5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/_build/prod/rel/fclc_web ./

ENV PHX_SERVER=true
EXPOSE 4000

CMD ["/app/bin/fclc_web", "start"]
