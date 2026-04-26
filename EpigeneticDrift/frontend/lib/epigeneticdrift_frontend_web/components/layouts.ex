defmodule EpigeneticDriftFrontendWeb.Layouts do
  use EpigeneticDriftFrontendWeb, :html

  embed_templates "layouts/*"

  def app(assigns) do
    ~H"""
    <!DOCTYPE html>
    <html lang="en" class="h-full">
      <head>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
        <meta name="csrf-token" content={get_csrf_token()} />
        <title>Epigenetic Drift · LongevityCommon</title>
        <link phx-track-static rel="stylesheet" href={~p"/assets/app.css"}/>
        <script defer phx-track-static type="text/javascript" src={~p"/assets/app.js"}></script>
      </head>
      <body class="h-full bg-gray-50">
        <.live_title_button phx-click={show_sidebar()} class="sr-only">
          Epigenetic Drift Dashboard
        </.live_title_button>
        <div class="flex h-full">
          <.sidebar />
          <div class="flex-1 overflow-auto">
            <.header />
            <main class="p-6">
              <%= @inner_block %>
            </main>
            <.footer />
          </div>
        </div>
      </body>
    </html>
    """
  end

  attr :page_title, :string, default: "Epigenetic Drift Dashboard"
  slot :inner_block, required: true
  slot :page_action
  def root(assigns) do
    ~H"""
    <!DOCTYPE html>
    <html lang="en" class="h-full bg-white">
      <head>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
        <meta name="csrf-token" content={get_csrf_token()} />
        <.live_title suffix=" · LongevityCommon"><%= @page_title %></.live_title>
        <link phx-track-static rel="stylesheet" href={~p"/assets/app.css"}/>
        <script defer phx-track-static type="text/javascript" src={~p"/assets/app.js"}></script>
        <script src="https://cdn.plot.ly/plotly-latest.min.js"></script>
      </head>
      <body class="h-full">
        <%= @inner_block %>
      </body>
    </html>
    """
  end

  defp sidebar(assigns) do
    ~H"""
    <nav class="hidden md:flex md:w-64 md:flex-col md:fixed md:inset-y-0">
      <div class="flex flex-col flex-grow pt-5 bg-white border-r border-gray-200 overflow-y-auto">
        <div class="flex items-center flex-shrink-0 px-4">
          <h1 class="text-xl font-bold text-gray-900">Epigenetic Drift</h1>
        </div>
        <div class="mt-8 flex-grow flex flex-col">
          <div class="flex-1 px-2 space-y-1">
            <.nav_link to="/" active={@socket.view == EpigeneticDriftFrontendWeb.DashboardLive}>
              Dashboard
            </.nav_link>
            <.nav_link to="/counter_registry" active={@socket.view == EpigeneticDriftFrontendWeb.CounterRegistryLive}>
              Counter Registry
            </.nav_link>
            <.nav_link to="/sobol" active={@socket.view == EpigeneticDriftFrontendWeb.SobolSensitivityLive}>
              Sobol Sensitivity
            </.nav_link>
            <.nav_link to="/lineage" active={@socket.view == EpigeneticDriftFrontendWeb.HSCTrackingLive}>
              HSC Lineage Tracking
            </.nav_link>
          </div>
        </div>
      </div>
    </nav>
    """
  end

  defp header(assigns) do
    ~H"""
    <header class="bg-white shadow">
      <div class="max-w-7xl mx-auto py-4 px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between items-center">
          <h2 class="text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate">
            <%= @page_title %>
          </h2>
          <div class="flex items-center space-x-4">
            <.live_patch to="/admin/dashboard" class="text-sm text-gray-500 hover:text-gray-700">
              System Metrics
            </.live_patch>
            <div class="text-sm text-gray-500">
              <%= DateTime.utc_now() |> DateTime.to_iso8601() %>
            </div>
          </div>
        </div>
      </div>
    </header>
    """
  end

  defp footer(assigns) do
    ~H"""
    <footer class="bg-white border-t border-gray-200">
      <div class="max-w-7xl mx-auto py-4 px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between items-center text-sm text-gray-500">
          <div>
            LongevityCommon Subproject · Epigenetic Drift (Counter #4)
          </div>
          <div>
            v<%= Application.spec(:epigeneticdrift_frontend, :vsn) %> ·
            <span class="font-mono"><%= System.get_env("BACKEND_URL", "local") %></span>
          </div>
        </div>
      </div>
    </footer>
    """
  end

  defp nav_link(assigns) do
    ~H"""
    <a
      href={@to}
      class={[
        "group flex items-center px-2 py-2 text-sm font-medium rounded-md",
        @active && "bg-gray-100 text-gray-900",
        !@active && "text-gray-600 hover:bg-gray-50 hover:text-gray-900"
      ]}
    >
      <%= render_slot(@inner_block) %>
    </a>
    """
  end
end