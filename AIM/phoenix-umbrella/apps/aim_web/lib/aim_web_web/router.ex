defmodule AimWeb.Router do
  use AimWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {AimWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
    plug AimWeb.Plugs.SecurityHeaders
    plug AimWeb.Plugs.Locale
  end

  pipeline :health do
    plug :accepts, ["json"]
  end

  scope "/", AimWeb do
    pipe_through :health
    get "/health", HealthController, :index
  end

  scope "/", AimWeb do
    pipe_through :browser

    live "/",         HomeLive,     :index
    live "/chat",     ChatLive,     :index
    live "/intake",   IntakeLive,   :new
    live "/cases",    CasesLive,    :index
    live "/cases/:id", CaseLive,    :show
    live "/consult",  ConsultLive,  :index
    live "/dashboard", DashboardLive, :index
    live "/drugs",    DrugInteractionsLive, :index
    live "/settings", SettingsLive,  :index
  end
end
