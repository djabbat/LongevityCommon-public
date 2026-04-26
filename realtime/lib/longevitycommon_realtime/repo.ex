defmodule LongevityCommonRealtime.Repo do
  use Ecto.Repo,
    otp_app: :longevitycommon_realtime,
    adapter: Ecto.Adapters.Postgres
end
