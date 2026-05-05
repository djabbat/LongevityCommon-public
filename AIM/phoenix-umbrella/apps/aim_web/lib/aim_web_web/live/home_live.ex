defmodule AimWeb.HomeLive do
  @moduledoc """
  Landing page for AIM. Promotes the dashboard / drugs / chat /
  settings entry points and explains what AIM is in 7 languages
  (UN-6 + Georgian).
  """
  use AimWeb, :live_view

  def mount(_params, _session, socket), do: {:ok, socket}

  def render(assigns) do
    ~H"""
    <main class="aim-landing">
      <section class="hero">
        <div class="brand-mark">⌬</div>
        <h1><%= t("home.heading", @locale) %></h1>
        <p class="tagline"><%= t("home.tagline", @locale) %></p>

        <div class="cta-row">
          <a href="/dashboard" class="cta primary">
            <%= t("home.cta.dashboard", @locale) %>
          </a>
          <a href="/chat" class="cta">
            <%= t("home.cta.chat", @locale) %>
          </a>
        </div>
      </section>

      <section class="features">
        <div class="feature">
          <div class="feature-icon">🩺</div>
          <p><%= t("home.feature.diagnostics", @locale) %></p>
        </div>
        <div class="feature">
          <div class="feature-icon">🔐</div>
          <p><%= t("home.feature.privacy", @locale) %></p>
        </div>
        <div class="feature">
          <div class="feature-icon">🦀</div>
          <p><%= t("home.feature.stack", @locale) %></p>
        </div>
      </section>

      <section class="quicknav">
        <a href="/dashboard" class="card">
          <h3>📊 <%= t("home.menu.dashboard", @locale) %></h3>
          <p><%= t("dashboard.health", @locale) %></p>
        </a>
        <a href="/chat" class="card">
          <h3>💬 <%= t("nav.chat", @locale) %></h3>
          <p><%= t("home.cta.chat", @locale) %></p>
        </a>
        <a href="/intake" class="card">
          <h3>📝 <%= t("nav.intake", @locale) %></h3>
          <p><%= t("intake.heading", @locale) %></p>
        </a>
        <a href="/cases" class="card">
          <h3>📁 <%= t("nav.cases", @locale) %></h3>
          <p><%= t("cases.heading", @locale) %></p>
        </a>
        <a href="/drugs" class="card">
          <h3>💊 <%= t("home.menu.drugs", @locale) %></h3>
          <p><%= t("drugs.prompt", @locale) %></p>
        </a>
        <a href="/settings" class="card">
          <h3>⚙️ <%= t("home.menu.settings", @locale) %></h3>
          <p><%= t("settings.keys", @locale) %></p>
        </a>
      </section>
    </main>
    """
  end
end
