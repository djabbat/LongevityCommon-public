defmodule ZeSimWeb.QuantumLive do
  use ZeSimWeb, :live_view
  alias ZeSim.Simulator

  @impl true
  def mount(_params, _session, socket) do
    {:ok,
     socket
     |> assign(params: default_params())
     |> assign(result: nil, error: nil, loading: false)}
  end

  @impl true
  def handle_event("run", %{"dim" => dim, "steps" => steps, "states" => states}, socket) do
    params = %{
      dim: String.to_integer(dim),
      steps: String.to_integer(steps),
      states: String.to_integer(states)
    }

    socket = assign(socket, loading: true, error: nil, result: nil, params: params)

    case Simulator.run_quantum(
           dim: params.dim,
           steps: params.steps,
           states: params.states
         ) do
      {:ok, result} ->
        {:noreply, assign(socket, result: result, loading: false)}

      {:error, msg} ->
        {:noreply, assign(socket, error: msg, loading: false)}
    end
  end

  defp default_params, do: %{dim: 4, steps: 2000, states: 50}

  @impl true
  def render(assigns) do
    ~H"""
    <div class="sim-page">
      <h1>Ze Quantum Simulator — Level 3</h1>
      <p class="subtitle">
        <strong>Theorem 5.1 (Conditional Optimality):</strong> given that nature follows Born rule,
        Born strategy q<sub>i</sub> = p<sub>i</sub> uniquely minimises the T-event rate.
        NOTE-Z5: Born rule is assumed as Axiom QM — this simulation verifies conditional optimality,
        not a derivation of Born rule from Ze axioms.
      </p>

      <form phx-submit="run" class="sim-form">
        <label>Hilbert space dim d
          <input type="number" name="dim" value={@params.dim} min="2" max="16" />
        </label>
        <label>Steps
          <input type="number" name="steps" value={@params.steps} min="100" max="5000" />
        </label>
        <label>States per step
          <input type="number" name="states" value={@params.states} min="10" max="200" />
        </label>
        <button type="submit" disabled={@loading}>
          <%= if @loading, do: "Running…", else: "Run Simulation" %>
        </button>
      </form>

      <%= if @error do %>
        <div class="error"><strong>Error:</strong> {@error}</div>
      <% end %>

      <%= if @result do %>
        <div class="results">
          <h2>Results — d={@result.dim}, θ_Q={@result.theta_q}</h2>
          <table class="metrics">
            <tr>
              <th>Strategy</th>
              <th>Final τ_Z</th>
              <th>T-rate (sim)</th>
              <th>T-rate (theory)</th>
            </tr>
            <tr class="row-born">
              <td><strong>Born (optimal)</strong></td>
              <td><strong>{@result.born_tau_final}</strong></td>
              <td>{Float.round(@result.born_t_rate, 4)}</td>
              <td>{Float.round(@result.born_theory_rate, 4)}</td>
            </tr>
            <tr>
              <td>Uniform</td>
              <td>{@result.uniform_tau_final}</td>
              <td>{Float.round(@result.uniform_t_rate, 4)}</td>
              <td>{Float.round(@result.uniform_theory_rate, 4)}</td>
            </tr>
            <tr class="row-anti">
              <td>Anti-Born (worst)</td>
              <td>{@result.anti_born_tau_final}</td>
              <td>{Float.round(@result.anti_born_t_rate, 4)}</td>
              <td>{Float.round(@result.anti_born_theory_rate, 4)}</td>
            </tr>
          </table>

          <div class="chart-container">
            <canvas id="quantum-chart" width="800" height="350"
              phx-hook="QuantumChart"
              data-born={Jason.encode!(@result.history_born)}
              data-uniform={Jason.encode!(@result.history_uniform)}
              data-anti={Jason.encode!(@result.history_anti_born)}
            ></canvas>
          </div>

          <p class="theorem-note">
            τ_Z(Born) = <strong>{@result.born_tau_final}</strong> vs
            τ_Z(Uniform) = {@result.uniform_tau_final},
            τ_Z(Anti-Born) = {@result.anti_born_tau_final}.
            Born rule maximises Ze proper-time. Theorem 5.1 verified:
            <%= if @result.theorem_5_1_holds do %>
              ✅ born_theory ≤ uniform_theory ≤ anti_theory
            <% else %>
              ⚠️ Theorem 5.1 violated — check parameters
            <% end %>
          </p>
        </div>
      <% end %>
    </div>
    """
  end
end
