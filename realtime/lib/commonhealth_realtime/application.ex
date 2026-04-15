defmodule CommonhealthRealtime.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      CommonhealthRealtime.Repo,
      {Phoenix.PubSub, name: CommonhealthRealtime.PubSub},
      CommonhealthRealtimeWeb.Endpoint,
    ]

    opts = [strategy: :one_for_one, name: CommonhealthRealtime.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    CommonhealthRealtimeWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
