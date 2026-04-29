defmodule ZeSimWeb.ReproLive do
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
  def handle_event("run", %{"tau0" => tau0, "chains" => chains, "dim" => dim}, socket) do
    params = %{
      tau0:   String.to_integer(tau0),
      chains: String.to_integer(chains),
      dim:    String.to_integer(dim)
    }
    socket = assign(socket, loading: true, error: nil, result: nil, params: params)

    case Simulator.run_repro(tau0: params.tau0, chains: params.chains, dim: params.dim) do
      {:ok, result} -> {:noreply, assign(socket, result: result, loading: false)}
      {:error, msg} -> {:noreply, assign(socket, error: msg, loading: false)}
    end
  end

  defp default_params, do: %{tau0: 200, chains: 500, dim: 4}

  @impl true
  def render(assigns) do
    ~H"""
    <div class="sim-page">
      <h1>Ze Reproduction Simulator — Level 4</h1>
      <p class="subtitle">
        <strong>Axiom Z4:</strong> T-events spawn daughter Ze-observers.
        Double-slit: S-event = no spawn (interference); T-event = spawn (which-path).<br/>
        <strong>Ze prediction P4:</strong> V<sub>Ze</sub> = 1 − 2p<sub>T</sub> (linear).
        <strong>QM bound:</strong> V<sub>QM</sub> = √(1 − p<sub>T</sub>²) (Englert 1996).
        Ze predicts strictly less visibility than QM for 0 &lt; p<sub>T</sub> &lt; 0.5 — a
        falsifiable distinction.
      </p>

      <form phx-submit="run" class="sim-form">
        <label>Initial τ_Z
          <input type="number" name="tau0" value={@params.tau0} min="10" max="1000" />
        </label>
        <label>Chains
          <input type="number" name="chains" value={@params.chains} min="50" max="2000" />
        </label>
        <label>Hilbert dim
          <input type="number" name="dim" value={@params.dim} min="2" max="8" />
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
          <h2>Results — τ₀={@result.tau0}, {@result.n_chains} chains, dim={@result.dim}</h2>

          <table class="metrics">
            <tr><th>Metric</th><th>Born rule</th><th>Uniform</th></tr>
            <tr class="row-born">
              <td>Mean chain depth</td>
              <td><strong>{Float.round(@result.born_depth_mean, 1)}</strong></td>
              <td>{Float.round(@result.uniform_depth_mean, 1)}</td>
            </tr>
            <tr>
              <td>Mean T-event rate</td>
              <td>{Float.round(@result.born_t_rate_mean, 4)}</td>
              <td>{Float.round(@result.uniform_t_rate_mean, 4)}</td>
            </tr>
            <tr>
              <td>Theorem 5.1 + P5</td>
              <td colspan="2">
                <%= if @result.born_depth_mean > @result.uniform_depth_mean do %>
                  ✅ Born chains deeper (P5 confirmed)
                <% else %>
                  ⚠️ Check θ_Q or dim
                <% end %>
              </td>
            </tr>
          </table>

          <div class="chart-container">
            <canvas id="repro-chart" width="800" height="300"
              phx-hook="ReproChart"
              data-born={Jason.encode!(@result.born_first_chain.history_tau)}
              data-uniform={Jason.encode!(@result.uniform_first_chain.history_tau)}
              data-ds={Jason.encode!(@result.double_slit_visibility)}
            ></canvas>
          </div>

          <h3 style="margin-top:1.5rem;">Double-Slit Visibility: Ze Prediction P4 vs QM Complementarity</h3>
          <p style="font-size:0.9em; color:#555;">
            Ze: V = 1 − 2p<sub>T</sub>  vs  QM (Englert 1996): V = √(1 − p<sub>T</sub>²).
            For 0 &lt; p<sub>T</sub> &lt; 0.5, Ze predicts strictly less visibility — a falsifiable difference.
          </p>
          <div class="chart-container">
            <canvas id="repro-ds-chart" width="800" height="300"></canvas>
          </div>

          <p class="theorem-note">
            Born chain depth <strong>{@result.born_first_chain.chain_depth}</strong> vs
            Uniform <strong>{@result.uniform_first_chain.chain_depth}</strong> (first chains).
            Born rule generates deeper Ze-genealogies — Axiom Z4 + Theorem 5.1 ✓.
          </p>
        </div>
      <% end %>
    </div>
    """
  end
end
