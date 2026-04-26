import Config

if config_env() == :prod do
  database_url = System.get_env("DATABASE_URL") ||
    raise "DATABASE_URL environment variable is not set"

  config :longevitycommon_realtime, LongevityCommonRealtime.Repo,
    url: database_url,
    pool_size: String.to_integer(System.get_env("POOL_SIZE", "10"))

  secret_key_base = System.get_env("SECRET_KEY_BASE") ||
    raise "SECRET_KEY_BASE is not set"

  config :longevitycommon_realtime, LongevityCommonRealtimeWeb.Endpoint,
    http: [ip: {0, 0, 0, 0}, port: String.to_integer(System.get_env("PORT", "4000"))],
    secret_key_base: secret_key_base

  config :longevitycommon_realtime,
    jwt_secret: System.fetch_env!("JWT_SECRET")
end
