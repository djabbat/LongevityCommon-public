defmodule CommonhealthRealtimeWeb.Router do
  use Phoenix.Router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", CommonhealthRealtimeWeb do
    pipe_through :api
    get "/health", HealthController, :index
  end
end
