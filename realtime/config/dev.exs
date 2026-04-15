import Config

config :commonhealth_realtime, CommonhealthRealtimeWeb.Endpoint,
  http: [ip: {0, 0, 0, 0}, port: 4000],
  check_origin: false,
  code_reloader: true,
  debug_errors: true,
  secret_key_base: "dev_secret_key_base_change_in_production_min_64_chars_xxxxxxxxxxxxxxxxxx"

config :commonhealth_realtime, CommonhealthRealtime.Repo,
  username: "postgres",
  password: "password",
  hostname: "localhost",
  database: "commonhealth",
  stacktrace: true,
  show_sensitive_data_on_connection_error: true,
  pool_size: 10

config :logger, level: :debug
