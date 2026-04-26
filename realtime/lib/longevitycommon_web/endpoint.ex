defmodule LongevityCommonRealtimeWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :longevitycommon_realtime

  socket "/socket", LongevityCommonRealtimeWeb.UserSocket,
    websocket: true,
    longpoll: false

  plug CORSPlug, origin: "*"
  plug Plug.RequestId
  plug Plug.Telemetry, event_prefix: [:phoenix, :endpoint]

  plug Plug.Parsers,
    parsers: [:urlencoded, :multipart, :json],
    pass: ["*/*"],
    json_decoder: Phoenix.json_library()

  plug Plug.MethodOverride
  plug Plug.Head
  plug LongevityCommonRealtimeWeb.Router
end
