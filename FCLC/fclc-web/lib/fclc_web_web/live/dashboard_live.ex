defmodule FclcWebWeb.DashboardLive do
  @moduledoc """
  LiveView dashboard for the FCLC federated learning coordinator.

  Displays:
  - Current FL round + AUC history chart
  - Registered clinic nodes + their DP budget status
  - Shapley contribution scores per node
  - Trigger round button (admin action)
  - Audit log (last 5 entries)

  Polling interval: 10 seconds (configurable via @refresh_interval_ms).
  """
  use FclcWebWeb, :live_view

  alias FclcWeb.FclcClient

  @refresh_interval_ms 10_000

  @impl true
  def mount(_params, session, socket) do
    if connected?(socket) do
      :timer.send_interval(@refresh_interval_ms, self(), :refresh)
    end

    # Check admin auth: session token must match FCLC_ADMIN_TOKEN env var.
    # If the env var is unset, the action is disabled entirely.
    admin_token = System.get_env("FCLC_ADMIN_TOKEN")
    is_admin = is_binary(admin_token) and admin_token != "" and
               Map.get(session, "admin_token") == admin_token

    {:ok, socket |> assign_defaults() |> assign(:is_admin, is_admin) |> load_data()}
  end

  @impl true
  def handle_info(:refresh, socket) do
    {:noreply, load_data(socket)}
  end

  @impl true
  def handle_event("trigger_round", _params, socket) do
    if socket.assigns.is_admin do
      case FclcClient.trigger_round() do
        {:ok, _} ->
          {:noreply, socket |> put_flash(:info, "Round triggered.") |> load_data()}
        {:error, msg} ->
          {:noreply, put_flash(socket, :error, "Failed: #{msg}")}
      end
    else
      {:noreply, put_flash(socket, :error, "Unauthorized: admin token required.")}
    end
  end

  # ── Private ───────────────────────────────────────────────────────────────────

  defp assign_defaults(socket) do
    assign(socket,
      metrics: nil,
      nodes: [],
      rounds: [],
      audit: [],
      shapley_scores: [],
      error: nil,
      last_updated: nil
    )
  end

  defp load_data(socket) do
    metrics = case FclcClient.get_metrics() do
      {:ok, m}    -> m
      {:error, e} -> %{"error" => e}
    end

    nodes = case FclcClient.list_nodes() do
      {:ok, n}    -> n
      {:error, _} -> []
    end

    rounds = case FclcClient.list_rounds() do
      {:ok, r}    -> Enum.take(r, 20)   # show last 20 rounds
      {:error, _} -> []
    end

    audit = case FclcClient.get_audit_chain() do
      {:ok, a}    -> Enum.take(a, 5)    # show last 5 audit entries
      {:error, _} -> []
    end

    # Shapley scores: fetch the latest score for each registered node.
    shapley_scores = nodes
      |> Enum.map(fn node ->
        node_id = node["node_id"] || node[:node_id]
        latest_score = case FclcClient.get_node_score(node_id) do
          {:ok, scores} when is_list(scores) and length(scores) > 0 ->
            scores |> List.last() |> Map.get("shapley_score")
          _ -> nil
        end
        %{node_name: node["node_name"] || "node-#{String.slice(to_string(node_id), 0, 6)}",
          score: latest_score}
      end)

    assign(socket,
      metrics: metrics,
      nodes: nodes,
      rounds: rounds,
      audit: audit,
      shapley_scores: shapley_scores,
      last_updated: DateTime.utc_now()
    )
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="min-h-screen bg-gray-50 p-6">
      <div class="max-w-7xl mx-auto">

        <!-- Header -->
        <div class="flex items-center justify-between mb-8">
          <div>
            <h1 class="text-3xl font-bold text-gray-900">FCLC Coordinator</h1>
            <p class="text-sm text-gray-500 mt-1">Federated Clinical Learning Cooperative</p>
          </div>
          <div class="text-right">
            <%= if @last_updated do %>
              <p class="text-xs text-gray-400">
                Updated: <%= Calendar.strftime(@last_updated, "%H:%M:%S") %> UTC
              </p>
            <% end %>
            <%= if @is_admin do %>
              <button phx-click="trigger_round"
                      class="mt-2 px-4 py-2 bg-blue-600 text-white text-sm rounded-lg hover:bg-blue-700 transition">
                ▶ Trigger Round
              </button>
            <% else %>
              <button disabled
                      class="mt-2 px-4 py-2 bg-gray-300 text-gray-500 text-sm rounded-lg cursor-not-allowed"
                      title="Admin token required">
                ▶ Trigger Round
              </button>
            <% end %>
          </div>
        </div>

        <!-- Flash messages -->
        <div :if={msg = live_flash(@flash, :info)}
             class="mb-4 p-3 bg-green-50 border border-green-200 rounded text-green-800 text-sm">
          <%= msg %>
        </div>
        <div :if={msg = live_flash(@flash, :error)}
             class="mb-4 p-3 bg-red-50 border border-red-200 rounded text-red-800 text-sm">
          <%= msg %>
        </div>

        <!-- Metrics cards -->
        <%= if @metrics && !Map.has_key?(@metrics, "error") do %>
          <div class="grid grid-cols-5 gap-4 mb-8">
            <div class="bg-white rounded-xl shadow-sm p-5">
              <p class="text-xs text-gray-500 uppercase tracking-wide">Current Round</p>
              <p class="text-4xl font-bold text-gray-900 mt-1">
                <%= @metrics["current_round"] || 0 %>
              </p>
            </div>
            <div class="bg-white rounded-xl shadow-sm p-5">
              <p class="text-xs text-gray-500 uppercase tracking-wide">Active Nodes</p>
              <p class="text-4xl font-bold text-blue-600 mt-1">
                <%= @metrics["node_count"] || 0 %>
              </p>
            </div>
            <div class="bg-white rounded-xl shadow-sm p-5">
              <p class="text-xs text-gray-500 uppercase tracking-wide">Latest AUC</p>
              <p class="text-4xl font-bold text-green-600 mt-1">
                <%= format_auc(List.last(@metrics["auc_history"] || [])) %>
              </p>
            </div>
            <div class="bg-white rounded-xl shadow-sm p-5">
              <p class="text-xs text-gray-500 uppercase tracking-wide">Avg Shapley</p>
              <p class="text-4xl font-bold text-purple-600 mt-1">
                <%= format_score(@metrics["avg_shapley"]) %>
              </p>
            </div>
            <div class="bg-white rounded-xl shadow-sm p-5">
              <p class="text-xs text-gray-500 uppercase tracking-wide">Rényi ε Saved</p>
              <p class="text-4xl font-bold text-indigo-600 mt-1">
                <%= format_score(@metrics["rdp_epsilon_savings"]) %>
              </p>
              <p class="text-xs text-gray-400 mt-1">vs. linear accounting</p>
            </div>
          </div>
        <% else %>
          <div class="mb-8 p-4 bg-yellow-50 border border-yellow-200 rounded-xl text-yellow-800">
            Cannot reach fclc-server — check connection and FCLC_SERVER_URL.
          </div>
        <% end %>

        <!-- Two-column layout: nodes + AUC chart -->
        <div class="grid grid-cols-2 gap-6 mb-6">

          <!-- Node registry table -->
          <div class="bg-white rounded-xl shadow-sm p-5">
            <h2 class="text-lg font-semibold text-gray-800 mb-3">Clinic Nodes</h2>
            <%= if Enum.empty?(@nodes) do %>
              <p class="text-sm text-gray-400">No nodes registered.</p>
            <% else %>
              <div class="overflow-x-auto">
                <table class="w-full text-sm">
                  <thead>
                    <tr class="border-b text-left text-gray-500">
                      <th class="pb-2 pr-4">Name</th>
                      <th class="pb-2 pr-4">ε spent</th>
                      <th class="pb-2">Registered</th>
                    </tr>
                  </thead>
                  <tbody>
                    <%= for node <- @nodes do %>
                      <tr class="border-b last:border-0 hover:bg-gray-50">
                        <td class="py-2 pr-4 font-medium text-gray-800">
                          <%= node["node_name"] %>
                        </td>
                        <td class="py-2 pr-4">
                          <span class={dp_badge_class(node["epsilon_spent"])}>
                            <%= format_score(node["epsilon_spent"]) %> / 10
                          </span>
                        </td>
                        <td class="py-2 text-gray-400 text-xs">
                          <%= format_date(node["registered_at"]) %>
                        </td>
                      </tr>
                    <% end %>
                  </tbody>
                </table>
              </div>
            <% end %>
          </div>

          <!-- AUC history (text-based chart) -->
          <div class="bg-white rounded-xl shadow-sm p-5">
            <h2 class="text-lg font-semibold text-gray-800 mb-3">AUC History</h2>
            <%= if Enum.empty?(@rounds) do %>
              <p class="text-sm text-gray-400">No rounds completed yet.</p>
            <% else %>
              <div class="space-y-1">
                <%= for round <- @rounds do %>
                  <div class="flex items-center gap-3 text-sm">
                    <span class="w-16 text-gray-500 text-xs">Round <%= round["round_number"] %></span>
                    <div class="flex-1 bg-gray-100 rounded-full h-4 overflow-hidden">
                      <div class="bg-green-500 h-4 rounded-full transition-all"
                           style={"width: #{min(round["auc"] * 100, 100)}%"}>
                      </div>
                    </div>
                    <span class="w-12 text-right font-mono text-xs text-gray-700">
                      <%= format_auc(round["auc"]) %>
                    </span>
                  </div>
                <% end %>
              </div>
            <% end %>
          </div>
        </div>

        <!-- Shapley contribution scores bar chart -->
        <div class="bg-white rounded-xl shadow-sm p-5 mb-6">
          <h2 class="text-lg font-semibold text-gray-800 mb-1">Shapley Contribution Scores</h2>
          <p class="text-xs text-gray-400 mb-3">
            Per-node fairness metric — fraction of model improvement attributable to each node.
            Sum ≈ 1.0. Nodes below 0.05 for 3 consecutive rounds are suspended.
          </p>
          <%= if Enum.empty?(@shapley_scores) do %>
            <p class="text-sm text-gray-400">No scores yet — complete at least one federated round.</p>
          <% else %>
            <div class="space-y-2">
              <%= for %{node_name: name, score: score} <- @shapley_scores do %>
                <div class="flex items-center gap-3 text-sm">
                  <span class="w-40 truncate text-gray-700 font-medium" title={name}>
                    <%= name %>
                  </span>
                  <div class="flex-1 bg-gray-100 rounded-full h-5 overflow-hidden">
                    <%= if score do %>
                      <div class={shapley_bar_class(score)}
                           style={"width: #{min(score * 100, 100)}%"}>
                      </div>
                    <% else %>
                      <div class="bg-gray-300 h-5 w-full rounded-full"></div>
                    <% end %>
                  </div>
                  <span class="w-16 text-right font-mono text-xs text-gray-700">
                    <%= format_shapley(score) %>
                  </span>
                </div>
              <% end %>
            </div>
          <% end %>
        </div>

        <!-- Audit log -->
        <div class="bg-white rounded-xl shadow-sm p-5">
          <h2 class="text-lg font-semibold text-gray-800 mb-3">Audit Log (last 5)</h2>
          <%= if Enum.empty?(@audit) do %>
            <p class="text-sm text-gray-400">No audit entries yet.</p>
          <% else %>
            <div class="font-mono text-xs space-y-1">
              <%= for entry <- @audit do %>
                <div class="flex gap-4 text-gray-600">
                  <span class="text-gray-400 w-12">R<%= entry["round_number"] %></span>
                  <span class="text-green-600">AUC: <%= format_auc(entry["mean_auc"]) %></span>
                  <span class="text-purple-600">n=<%= entry["participating"] %></span>
                  <span class="text-gray-400 truncate" title={entry["entry_hash"]}>
                    #<%= String.slice(entry["entry_hash"] || "", 0, 12) %>…
                  </span>
                </div>
              <% end %>
            </div>
          <% end %>
        </div>

      </div>
    </div>
    """
  end

  # ── Helpers ───────────────────────────────────────────────────────────────────

  defp format_auc(nil), do: "—"
  defp format_auc(auc) when is_float(auc), do: :erlang.float_to_binary(auc, decimals: 4)
  defp format_auc(_), do: "—"

  defp format_score(nil), do: "—"
  defp format_score(v) when is_float(v), do: :erlang.float_to_binary(v, decimals: 3)
  defp format_score(_), do: "—"

  defp format_date(nil), do: "—"
  defp format_date(dt_str) when is_binary(dt_str), do: String.slice(dt_str, 0, 10)
  defp format_date(_), do: "—"

  defp dp_badge_class(eps) when is_float(eps) and eps > 8.0,
    do: "text-red-600 font-semibold"
  defp dp_badge_class(eps) when is_float(eps) and eps > 5.0,
    do: "text-yellow-600 font-semibold"
  defp dp_badge_class(_), do: "text-gray-700"

  defp format_shapley(nil), do: "N/A"
  defp format_shapley(s) when is_float(s),
    do: :erlang.float_to_binary(s, decimals: 4)
  defp format_shapley(_), do: "N/A"

  # Bar colour: green if healthy (≥0.10), yellow if at-risk (0.05–0.10), red if below threshold.
  defp shapley_bar_class(s) when is_float(s) and s >= 0.10,
    do: "bg-green-500 h-5 rounded-full transition-all"
  defp shapley_bar_class(s) when is_float(s) and s >= 0.05,
    do: "bg-yellow-400 h-5 rounded-full transition-all"
  defp shapley_bar_class(_),
    do: "bg-red-500 h-5 rounded-full transition-all"
end
