defmodule ZeSim.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      ZeSimWeb.Telemetry,
      {DNSCluster, query: Application.get_env(:ze_sim, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: ZeSim.PubSub},
      # Start a worker by calling: ZeSim.Worker.start_link(arg)
      # {ZeSim.Worker, arg},
      # Start to serve requests, typically the last entry
      ZeSimWeb.Endpoint
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: ZeSim.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    ZeSimWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
