defmodule LongevityCommonRealtime.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      LongevityCommonRealtime.Repo,
      {Phoenix.PubSub, name: LongevityCommonRealtime.PubSub},
      LongevityCommonRealtimeWeb.Endpoint,
    ]

    opts = [strategy: :one_for_one, name: LongevityCommonRealtime.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    LongevityCommonRealtimeWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
