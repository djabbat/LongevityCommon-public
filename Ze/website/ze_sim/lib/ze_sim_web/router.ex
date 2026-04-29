defmodule ZeSimWeb.Router do
  use ZeSimWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {ZeSimWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", ZeSimWeb do
    pipe_through :browser

    live "/", DynamicsLive
    get "/about", PageController, :home
    live "/thermo", ThermoLive
    live "/quantum", QuantumLive
    live "/repro", ReproLive
    live "/regime", RegimeLive
    live "/particles", ParticlesLive
    live "/slit", SlitLive
  end

  # Other scopes may use custom stacks.
  # scope "/api", ZeSimWeb do
  #   pipe_through :api
  # end
end
