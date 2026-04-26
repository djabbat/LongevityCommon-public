defmodule OntogenesisFrontendWeb.DetailLive do
  use OntogenesisFrontendWeb, :live_view

  alias OntogenesisFrontendWeb.CoreComponents

  @impl true
  def mount(%{"id" => id}, _session, socket) do
    if connected?(socket) do
      :timer.send_interval(60_000, self(), :refresh_detail)
    end

    {:ok,
      socket
      |> assign(:id, id)
      |> assign(:loading, true)
      |> assign(:error, nil)
      |> assign(:entity, nil)
      |> assign(:related_entities, [])
      |> load_entity()
    }
  end

  @impl true
  def handle_params(_params, _uri, socket) do
    {:noreply, socket}
  end

  @impl true
  def handle_info(:refresh_detail, socket) do
    {:noreply, socket |> load_entity()}
  end

  @impl true
  def handle_event("refresh", _params, socket) do
    {:noreply, socket |> assign(:loading, true) |> load_entity()}
  end

  defp load_entity(socket) do
    id = socket.assigns.id

    socket = case OntogenesisFrontendWeb.Clients.BackendClient.get_entity(id) do
      {:ok, %{entity: entity, related: related}} ->
        socket
        |> assign(:entity, entity)
        |> assign(:related_entities, related)
        |> assign(:loading, false)
        |> assign(:error, nil)
        |> assign(:page_title, "#{entity.name} - Ontogenesis")

      {:error, reason} ->
        socket
        |> assign(:loading, false)
        |> assign(:error, "Failed to load entity: #{inspect(reason)}")
    end

    socket
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-6">
      <!-- Header -->
      <div class="md:flex md:items-center md:justify-between">
        <div class="flex-1 min-w-0">
          <nav class="flex" aria-label="Breadcrumb">
            <ol role="list" class="flex items-center space-x-4">
              <li>
                <div>
                  <.link navigate={~p"/"} class="text-gray-400 hover:text-gray-500">
                    <Heroicons.home class="flex-shrink-0 h-5 w-5" />
                    <span class="sr-only">Dashboard</span>
                  </.link>
                </div>
              </li>
              <li>
                <div class="flex items-center">
                  <Heroicons.chevron_right class="flex-shrink-0 h-5 w-5 text-gray-400" />
                  <span class="ml-4 text-sm font-medium text-gray-500">Entity Details</span>
                </div>
              </li>
            </ol>
          </nav>
          <h2 class="mt-2 text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate">
            <%= if @entity do %>
              <%= @entity.name %>
            <% else %>
              Loading...
            <% end %>
          </h2>
        </div>
        <div class="mt-4 flex md:mt-0 md:ml-4">
          <.button phx-click="refresh" class="inline-flex items-center">
            <Heroicons.arrow_path class="-ml-0.5 mr-2 h-4 w-4" />
            Refresh
          </.button>
        </div>
      </div>

      <!-- Error Display -->
      <%= if @error do %>
        <CoreComponents.error_alert title="Failed to load entity">
          <p><%= @error %></p>
        </CoreComponents.error_alert>
      <% end %>

      <!-- Loading Spinner -->
      <%= if @loading do %>
        <div class="flex justify-center py-12">
          <CoreComponents.spinner />
        </div>
      <% else %>
        <%= if @entity do %>
          <!-- Entity Details Card -->
          <CoreComponents.card class="divide-y divide-gray-200">
            <div class="px-4 py-5 sm:px-6">
              <div class="flex justify-between items-start">
                <div>
                  <h3 class="text-lg font-medium leading-6 text-gray-900"><%= @entity.name %></h3>
                  <p class="mt-1 max-w-2xl text-sm text-gray-500"><%= @entity.description %></p>
                </div>
                <CoreComponents.badge status={@entity.status}>
                  <%= @entity.status %>
                </CoreComponents.badge>
              </div>
            </div>
            
            <div class="px-4 py-5 sm:p-6">
              <dl class="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
                <div>
                  <dt class="text-sm font-medium text-gray-500">Domain</dt>
                  <dd class="mt-1 text-sm text-gray-900"><%= @entity.domain %></dd>
                </div>
                <div>
                  <dt class="text-sm font-medium text-gray-500">Default Value / Range</dt>
                  <dd class="mt-1 text-sm text-gray-900"><%= @entity.range %></dd>
                </div>
                <div>
                  <dt class="text-sm font-medium text-gray-500">Units</dt>
                  <dd class="mt-1 text-sm text-gray-900"><%= @entity.units %></dd>
                </div>
                <div>
                  <dt class="text-sm font-medium text-gray-500">Source / Justification</dt>
                  <dd class="mt-1 text-sm text-gray-900"><%= @entity.source %></dd>
                </div>
                <%= if @entity.coupling_coefficient do %>
                  <div>
                    <dt class="text-sm font-medium text-gray-500">Coupling Coefficient (γ)</dt>
                    <dd class="mt-1 text-sm text-gray-900"><%= @entity.coupling_coefficient %></dd>
                  </div>
                <% end %>
              </dl>

              <!-- Algorithmic Parameters -->
              <%= if @entity.algorithmic do %>
                <div class="mt-6 pt-6 border-t border-gray-200">
                  <h4 class="text-sm font-medium text-gray-900">Algorithmic Configuration</h4>
                  <div class="mt-4 grid grid-cols-2 gap-4">
                    <div>
                      <dt class="text-sm font-medium text-gray-500">LCS Model Parameter</dt>
                      <dd class="mt-1 text-sm text-gray-900"><%= @entity.algorithmic.lcs_param || "N/A" %></dd>
                    </div>
                    <div>
                      <dt class="text-sm font-medium text-gray-500">Cross-domain Coupling</dt>
                      <dd class="mt-1 text-sm text-gray-900">
                        <%= if @entity.algorithmic.coupling_enabled do %>
                          Enabled (γ = <%= @entity.algorithmic.coupling_value %>)
                        <% else %>
                          Disabled (null hypothesis)
                        <% end %>
                      </dd>
                    </div>
                  </div>
                </div>
              <% end %>

              <!-- Value Visualization -->
              <%= if @entity.current_value && @entity.max_value do %>
                <div class="mt-6 pt-6 border-t border-gray-200">
                  <h4 class="text-sm font-medium text-gray-900 mb-4">Current Value</h4>
                  <CoreComponents.progress_bar 
                    value={@entity.current_value} 
                    max={@entity.max_value}
                    label={@entity.name}
                  />
                  <div class="mt-2 text-sm text-gray-500 text-center">
                    <%= @entity.current_value %> / <%= @entity.max_value %> <%= @entity.units %>
                  </div>
                </div>
              <% end %>

              <!-- Etagenesis Context -->
              <div class="mt-6 pt-6 border-t border-gray-200">
                <h4 class="text-sm font-medium text-gray-900">Etagenesis Context</h4>
                <p class="mt-2 text-sm text-gray-600">
                  This parameter is tracked across all three periods of etagenesis:
                  <span class="font-medium">Ontogenesis (0–25 years)</span>,
                  <span class="font-medium">Mesogenesis (25–50 years)</span>, and
                  <span class="font-medium">Gerontogenesis (50–120 years)</span>.
                </p>
                <p class="mt-2 text-sm text-gray-600">
                  According to Frolkis (1999): "Etagenesis is an age-associated development 
                  of an organism from the zygote to death."
                </p>
              </div>
            </div>
          </CoreComponents.card>

          <!-- Related Entities -->
          <%= if length(@related_entities) > 0 do %>
            <div class="bg-white shadow rounded-lg">
              <div class="px-4 py-5 sm:px-6">
                <h3 class="text-lg leading-6 font-medium text-gray-900">Related Parameters</h3>
                <p class="mt-1 max-w-2xl text-sm text-gray-500">
                  Cross-domain coupling may affect these related parameters
                </p>
              </div>
              <div class="border-t border-gray-200">
                <ul role="list" class="divide-y divide-gray-200">
                  <%= for related <- @related_entities do %>
                    <li class="px-6 py-4">
                      <div class="flex items-center justify-between">
                        <div class="flex-1 min-w-0">
                          <p class="text-sm font-medium text-gray-900 truncate">
                            <%= related.name %>
                          </p>
                          <p class="text-sm text-gray-500 truncate">
                            <%= related.domain %> • <%= related.range %>
                          </p>
                        </div>
                        <div class="ml-4 flex-shrink-0">
                          <.link 
                            navigate={~p"/detail/#{related.id}"}
                            class="font-medium text-indigo-600 hover:text-indigo-900"
                          >
                            View
                          </.link>
                        </div>
                      </div>
                    </li>
                  <% end %>
                </ul>
              </div>
            </div>
          <% end %>

          <!-- MCOA Integration -->
          <div class="bg-gray-50 rounded-lg p-6">
            <div class="flex items-center">
              <Heroicons.puzzle_piece class="h-5 w-5 text-gray-400 mr-2" />
              <h3 class="text-lg font-medium text-gray-900">MCOA Integration</h3>
            </div>
            <p class="mt-2 text-sm text-gray-600">
              Ontogenesis provides initial conditions D_i,0 for each MCOA counter at t = 25 years.
              This parameter contributes to the <%= @entity.domain %> counter family.
            </p>
            <div class="mt-4">
              <.link 
                href="https://longevitycommon.org/mcoa/counter-registry" 
                target="_blank"
                class="inline-flex items-center text-sm font-medium text-indigo-600 hover:text-indigo-900"
              >
                View MCOA counter registry →
              </.link>
            </div>
          </div>

        <% else %>
          <CoreComponents.error_alert title="Entity not found">
            <p>Entity with ID <%= @id %> was not found in the backend.</p>
          </CoreComponents.error_alert>
        <% end %>
      <% end %>
    </div>
    """
  end
end