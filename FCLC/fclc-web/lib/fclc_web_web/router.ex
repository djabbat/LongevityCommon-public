defmodule FclcWebWeb.Router do
  use FclcWebWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {FclcWebWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", FclcWebWeb do
    pipe_through :browser

    live "/", DashboardLive, :index
    live "/dashboard", DashboardLive, :index
  end

  # Other scopes may use custom stacks.
  # scope "/api", FclcWebWeb do
  #   pipe_through :api
  # end
end
