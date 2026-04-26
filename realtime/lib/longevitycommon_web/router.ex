defmodule LongevityCommonRealtimeWeb.Router do
  use Phoenix.Router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", LongevityCommonRealtimeWeb do
    pipe_through :api
    get "/health", HealthController, :index
  end
end
