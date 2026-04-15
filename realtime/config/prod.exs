import Config

config :commonhealth_realtime, CommonhealthRealtimeWeb.Endpoint,
  cache_static_manifest: "priv/static/cache_manifest.json",
  server: true

config :logger, level: :info
