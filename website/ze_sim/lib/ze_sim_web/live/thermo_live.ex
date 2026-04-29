defmodule ZeSimWeb.ThermoLive do
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
  def handle_event("run", %{"molecules" => mol, "steps" => steps, "demon" => demon}, socket) do
    params = %{
      molecules: String.to_integer(mol),
      steps: String.to_integer(steps),
      demon: demon == "true"
    }

    socket = assign(socket, loading: true, error: nil, result: nil, params: params)

    # Run simulation synchronously (fast — ~50ms for 100 mols × 500 steps)
    case Simulator.run_thermo(
           molecules: params.molecules,
           steps: params.steps,
           demon: params.demon
         ) do
      {:ok, result} ->
        {:noreply, assign(socket, result: result, loading: false)}

      {:error, msg} ->
        {:noreply, assign(socket, error: msg, loading: false)}
    end
  end

  defp default_params, do: %{molecules: 100, steps: 500, demon: false}

  @impl true
  def render(assigns) do
    ~H"""
    <div class="sim-page">
      <h1>Ze Thermodynamic Simulator — Level 2</h1>
      <p class="subtitle">
        Demonstrates: Second Law follows from Ze-budget monotonicity (Axiom Z2).
        Cold start (v=0): both S_Ze and S_Boltzmann increase during thermalization.
        NOTE-Z4: S_Ze and S_Boltzmann are distinct quantities — S_Ze is a Ze information-theoretic
        entropy, S_Boltzmann measures kinetic energy variance. Positive Spearman ρ during
        thermalization confirms co-monotone behaviour.
      </p>

      <form phx-submit="run" class="sim-form">
        <label>Molecules
          <input type="number" name="molecules" value={@params.molecules} min="10" max="500" />
        </label>
        <label>Steps
          <input type="number" name="steps" value={@params.steps} min="100" max="2000" />
        </label>
        <label>Maxwell's Demon
          <select name="demon">
            <option value="false" selected={!@params.demon}>No</option>
            <option value="true" selected={@params.demon}>Yes</option>
          </select>
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
          <h2>Results</h2>
          <table class="metrics">
            <tr><th>Metric</th><th>Value</th></tr>
            <tr><td>Pearson r (S_Ze vs S_Boltz, full series)</td><td>{Float.round(@result.correlation, 4)}</td></tr>
            <tr><td>Spearman ρ (thermalization phase, first 50 steps)</td><td><strong>{Float.round(@result.spearman_thermalization, 4)}</strong></td></tr>
            <tr><td>T-event rate</td><td>{Float.round(@result.t_event_rate, 4)}</td></tr>
            <tr><td>τ_Z depletion rate / step</td><td>{Float.round(@result.tau_depletion_rate, 4)}</td></tr>
            <tr><td>Final mean τ_Z</td><td>{Float.round(@result.final_tau_total, 1)}</td></tr>
            <tr><td>S_Ze final</td><td>{Float.round(@result.s_ze_final, 4)}</td></tr>
            <tr><td>S_Boltzmann final</td><td>{Float.round(@result.s_boltz_final, 4)}</td></tr>
            <%= if @result.demon_cost do %>
              <tr><td>Demon Ze-cost (τ_Z units)</td><td>{@result.demon_cost}</td></tr>
            <% end %>
          </table>

          <div class="chart-container">
            <canvas id="thermo-chart" width="800" height="350"
              phx-hook="ThermoChart"
              data-s-ze={Jason.encode!(@result.history_s_ze)}
              data-s-boltz={Jason.encode!(@result.history_s_boltz)}
              data-tau={Jason.encode!(@result.history_tau)}
            ></canvas>
          </div>
        </div>
      <% end %>
    </div>
    """
  end
end
