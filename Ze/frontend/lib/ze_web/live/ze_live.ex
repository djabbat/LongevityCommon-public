defmodule ZeWeb.ZeLive do
  @moduledoc """
  Единый LiveView для Ze Theory с тремя вкладками:
    - impedance (ODE)
    - chsh (Bell + Ze-деформация)
    - autowaves (1D reaction-diffusion)
  Данные идут из Rust backend `ze_backend`.
  """
  use ZeWeb, :live_view

  @impedance_scenarios [
    {"routine", "Routine (низкое σ)"},
    {"novelty", "Novelty (σ-step at τ=5)"},
    {"meditation", "Meditation (σ↓, λ↑)"},
    {"cheating", "Cheating spike (τ=10)"}
  ]

  @impl true
  def mount(_params, _session, socket) do
    socket =
      socket
      |> assign(
        tab: "impedance",
        imp_scenario: "novelty",
        imp_horizon: 50,
        chsh_h: 0.5,
        chsh_alpha: 0.03,
        chsh_delta: 0.05,
        aw_n: 200,
        aw_steps: 2000,
        impedance_scenarios: @impedance_scenarios,
        backend_url: Ze.BackendClient.base_url(),
        loading: false,
        error: nil,
        data: nil
      )

    {:ok, maybe_load(socket)}
  end

  defp maybe_load(socket), do: if(connected?(socket), do: run(socket), else: socket)

  @impl true
  def handle_event("tab", %{"tab" => tab}, socket) when tab in ~w(impedance chsh autowaves) do
    {:noreply, socket |> assign(tab: tab, data: nil, error: nil) |> run()}
  end

  def handle_event("update-imp", params, socket) do
    {:noreply,
     socket
     |> assign(
       imp_scenario: Map.get(params, "scenario", socket.assigns.imp_scenario),
       imp_horizon: clamp_int(params["horizon"], socket.assigns.imp_horizon, 5, 200)
     )
     |> run()}
  end

  def handle_event("update-chsh", params, socket) do
    {:noreply,
     socket
     |> assign(
       chsh_h: clamp_float(params["h"], socket.assigns.chsh_h, 0.0, 1.0),
       chsh_alpha: clamp_float(params["alpha"], socket.assigns.chsh_alpha, 0.0, 0.5),
       chsh_delta: clamp_float(params["delta"], socket.assigns.chsh_delta, 0.0, 0.5)
     )
     |> run()}
  end

  def handle_event("update-aw", params, socket) do
    {:noreply,
     socket
     |> assign(
       aw_n: clamp_int(params["n"], socket.assigns.aw_n, 20, 400),
       aw_steps: clamp_int(params["steps"], socket.assigns.aw_steps, 200, 10_000)
     )
     |> run()}
  end

  def handle_event("run", _, socket), do: {:noreply, run(socket)}

  defp clamp_int(nil, fallback, _, _), do: fallback
  defp clamp_int(s, fallback, lo, hi) when is_binary(s) do
    case Integer.parse(s) do
      {n, _} -> n |> max(lo) |> min(hi)
      _ -> fallback
    end
  end

  defp clamp_float(nil, fallback, _, _), do: fallback
  defp clamp_float(s, fallback, lo, hi) when is_binary(s) do
    case Float.parse(s) do
      {f, _} -> f |> max(lo) |> min(hi)
      _ -> fallback
    end
  end

  defp run(socket) do
    socket = assign(socket, loading: true, error: nil)

    result =
      case socket.assigns.tab do
        "impedance" ->
          Ze.BackendClient.impedance(socket.assigns.imp_scenario, socket.assigns.imp_horizon)
        "chsh" ->
          Ze.BackendClient.chsh(socket.assigns.chsh_h, socket.assigns.chsh_alpha, socket.assigns.chsh_delta)
        "autowaves" ->
          Ze.BackendClient.autowaves(socket.assigns.aw_n, socket.assigns.aw_steps, max(div(socket.assigns.aw_steps, 10), 1))
      end

    case result do
      {:ok, data} ->
        socket
        |> assign(data: data, loading: false)
        |> push_event("ze-data", %{tab: socket.assigns.tab, data: data})

      {:error, reason} ->
        assign(socket, loading: false, error: "backend: #{inspect(reason)}")
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="mx-auto max-w-5xl p-6 space-y-6">
      <header class="space-y-1">
        <h1 class="text-3xl font-bold">Ze Theory — simulators</h1>
        <p class="text-sm opacity-80">Entropic-Geometric TOE · backend: <code>{@backend_url}</code></p>
      </header>

      <nav class="flex gap-2 border-b pb-2">
        <.tab_btn active={@tab == "impedance"} id="impedance">Impedance ODE</.tab_btn>
        <.tab_btn active={@tab == "chsh"} id="chsh">CHSH (Bell)</.tab_btn>
        <.tab_btn active={@tab == "autowaves"} id="autowaves">Autowaves</.tab_btn>
      </nav>

      <%= if @error do %>
        <div class="border border-red-400 bg-red-50 text-red-700 p-3 rounded text-sm">
          {@error}
        </div>
      <% end %>

      <%= cond do %>
        <% @tab == "impedance" -> %>
          <.impedance_panel
            scenario={@imp_scenario}
            horizon={@imp_horizon}
            scenarios={@impedance_scenarios}
            data={@data}
          />
        <% @tab == "chsh" -> %>
          <.chsh_panel h={@chsh_h} alpha={@chsh_alpha} delta={@chsh_delta} data={@data} />
        <% @tab == "autowaves" -> %>
          <.autowaves_panel n={@aw_n} steps={@aw_steps} data={@data} />
      <% end %>

      <%= if @loading do %>
        <div class="text-sm opacity-70">Loading…</div>
      <% end %>

      <footer class="text-xs opacity-60 pt-6 border-t">
        Ze Theory · см. <code>~/Desktop/LongevityCommon/Ze/CONCEPT.md</code> · источник концепции: <code>~/Desktop/5.md</code>
      </footer>
    </div>
    """
  end

  attr :active, :boolean, default: false
  attr :id, :string, required: true
  slot :inner_block, required: true

  defp tab_btn(assigns) do
    ~H"""
    <button
      type="button"
      phx-click="tab"
      phx-value-tab={@id}
      class={[
        "px-3 py-1 rounded-t border-b-2 text-sm",
        @active && "border-black font-semibold" || "border-transparent opacity-70"
      ]}
    >
      {render_slot(@inner_block)}
    </button>
    """
  end

  attr :scenario, :string, required: true
  attr :horizon, :integer, required: true
  attr :scenarios, :list, required: true
  attr :data, :any, required: true

  defp impedance_panel(assigns) do
    ~H"""
    <form phx-change="update-imp" phx-submit="run" class="grid grid-cols-1 md:grid-cols-3 gap-4 p-4 border rounded">
      <label class="flex flex-col gap-1">
        <span class="text-sm font-medium">Сценарий</span>
        <select name="scenario" class="border rounded p-2">
          <%= for {val, label} <- @scenarios do %>
            <option value={val} selected={val == @scenario}>{label}</option>
          <% end %>
        </select>
      </label>
      <label class="flex flex-col gap-1">
        <span class="text-sm font-medium">Горизонт τ</span>
        <input type="number" name="horizon" value={@horizon} min="5" max="200" step="5" class="border rounded p-2" />
      </label>
      <div class="flex items-end">
        <button type="submit" class="w-full px-4 py-2 rounded bg-black text-white">Run</button>
      </div>
    </form>

    <%= if @data do %>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3 text-sm">
        <.metric label="I₀" value={round4(List.first(@data["i"]))} />
        <.metric label="I(T)" value={round4(List.last(@data["i"]))} />
        <.metric label="𝒞(T)" value={round4(List.last(@data["consciousness"]))} />
        <.metric label="Φ_Ze" value={round4(@data["phi_ze"])} />
      </div>
    <% end %>

    <div id="imp-charts" phx-hook="ZeCharts" phx-update="ignore" data-tab="impedance"
         class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="border rounded p-2"><canvas id="chart-I" height="160"></canvas></div>
      <div class="border rounded p-2"><canvas id="chart-t" height="160"></canvas></div>
      <div class="border rounded p-2"><canvas id="chart-C" height="160"></canvas></div>
      <div class="border rounded p-2"><canvas id="chart-K" height="160"></canvas></div>
    </div>
    """
  end

  attr :h, :float, required: true
  attr :alpha, :float, required: true
  attr :delta, :float, required: true
  attr :data, :any, required: true

  defp chsh_panel(assigns) do
    ~H"""
    <form phx-change="update-chsh" phx-submit="run" class="grid grid-cols-1 md:grid-cols-4 gap-4 p-4 border rounded">
      <label class="flex flex-col gap-1">
        <span class="text-sm font-medium">H (энтропия)</span>
        <input type="number" name="h" value={@h} min="0" max="1" step="0.05" class="border rounded p-2" />
      </label>
      <label class="flex flex-col gap-1">
        <span class="text-sm font-medium">α</span>
        <input type="number" name="alpha" value={@alpha} min="0" max="0.2" step="0.005" class="border rounded p-2" />
      </label>
      <label class="flex flex-col gap-1">
        <span class="text-sm font-medium">δ₀</span>
        <input type="number" name="delta" value={@delta} min="0" max="0.5" step="0.01" class="border rounded p-2" />
      </label>
      <div class="flex items-end">
        <button type="submit" class="w-full px-4 py-2 rounded bg-black text-white">Run</button>
      </div>
    </form>

    <%= if @data do %>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3 text-sm">
        <.metric label="S_QM" value={round4(@data["s_qm"])} />
        <.metric label="S_Ze" value={round4(@data["s_ze"])} />
        <.metric label="ΔS (singlet)" value={round4(@data["s_shift"])} />
        <.metric label="S(H)" value={round4(@data["s_damped_h"])} />
      </div>
      <div class="text-xs opacity-70">
        5σ при N_required ≈ {fmt_sci(@data["sigma_5sigma_coincidences"])} совпадений.
      </div>
    <% end %>

    <div id="chsh-charts" phx-hook="ZeCharts" phx-update="ignore" data-tab="chsh"
         class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="border rounded p-2"><canvas id="chart-sweep" height="200"></canvas></div>
      <div class="border rounded p-2"><canvas id="chart-shift" height="200"></canvas></div>
    </div>
    """
  end

  attr :n, :integer, required: true
  attr :steps, :integer, required: true
  attr :data, :any, required: true

  defp autowaves_panel(assigns) do
    ~H"""
    <form phx-change="update-aw" phx-submit="run" class="grid grid-cols-1 md:grid-cols-3 gap-4 p-4 border rounded">
      <label class="flex flex-col gap-1">
        <span class="text-sm font-medium">Сетка N</span>
        <input type="number" name="n" value={@n} min="20" max="400" step="10" class="border rounded p-2" />
      </label>
      <label class="flex flex-col gap-1">
        <span class="text-sm font-medium">Шагов</span>
        <input type="number" name="steps" value={@steps} min="200" max="10000" step="100" class="border rounded p-2" />
      </label>
      <div class="flex items-end">
        <button type="submit" class="w-full px-4 py-2 rounded bg-black text-white">Run</button>
      </div>
    </form>

    <%= if @data do %>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3 text-sm">
        <.metric label="snapshots" value={length(@data["snapshots"])} />
        <.metric label="I_mean(T)" value={round4(List.last(@data["i_mean"]))} />
        <.metric label="x_mean(T)" value={round4(List.last(@data["x_mean"]))} />
        <.metric label="y_mean(T)" value={round4(List.last(@data["y_mean"]))} />
      </div>
    <% end %>

    <div id="aw-charts" phx-hook="ZeCharts" phx-update="ignore" data-tab="autowaves"
         class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="border rounded p-2"><canvas id="chart-means" height="200"></canvas></div>
      <div class="border rounded p-2"><canvas id="chart-snap" height="200"></canvas></div>
    </div>
    """
  end

  attr :label, :string, required: true
  attr :value, :any, required: true

  defp metric(assigns) do
    ~H"""
    <div class="border rounded p-3">
      <div class="text-xs opacity-70">{@label}</div>
      <div class="text-xl font-mono">{@value}</div>
    </div>
    """
  end

  defp round4(nil), do: "—"
  defp round4(x) when is_number(x), do: Float.round(x / 1, 4)

  defp fmt_sci(x) when is_number(x) do
    cond do
      x == :infinity or not is_float(x) -> "∞"
      x > 1.0e6 -> :io_lib.format(~c"~.2e", [x]) |> List.to_string()
      true -> :io_lib.format(~c"~.2f", [x]) |> List.to_string()
    end
  end
  defp fmt_sci(_), do: "—"
end
