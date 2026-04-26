import Config

config :longevitycommon_realtime, LongevityCommonRealtimeWeb.Endpoint,
  url: [host: "localhost"],
  adapter: Phoenix.Endpoint.Cowboy2Adapter,
  render_errors: [formats: [json: LongevityCommonRealtimeWeb.ErrorJSON], layout: false],
  pubsub_server: LongevityCommonRealtime.PubSub,
  live_view: [signing_salt: "change_me"]

config :longevitycommon_realtime, LongevityCommonRealtime.Repo,
  adapter: Ecto.Adapters.Postgres

config :longevitycommon_realtime,
  jwt_secret: System.get_env("JWT_SECRET", "change_me_to_random_64_char_string")

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :phoenix, :json_library, Jason

import_config "#{config_env()}.exs"
