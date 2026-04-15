defmodule CommonhealthRealtime.Repo do
  use Ecto.Repo,
    otp_app: :commonhealth_realtime,
    adapter: Ecto.Adapters.Postgres
end
