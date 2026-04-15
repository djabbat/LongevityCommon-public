defmodule CommonhealthRealtimeWeb.HealthController do
  use Phoenix.Controller

  def index(conn, _params) do
    json(conn, %{status: "ok", service: "commonhealth-realtime"})
  end
end
