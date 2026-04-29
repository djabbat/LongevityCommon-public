defmodule ZeSimWeb.SlitLive do
  @moduledoc """
  Ze Vectors Theory — Double-Slit Prediction P1 (FoP article §5.5).

  Ze-theoretic prediction:
    V = 1 − 2·p_T          (tight equality)

  Standard quantum mechanics inequality:
    V² + D² ≤ 1            (wave-particle duality)

  where:
    V  — fringe visibility (0 = no fringes, 1 = full interference)
    D  — which-path distinguishability (0 = unknown, 1 = fully known)
    p_T — T-event rate of the which-path detector (0 ≤ p_T ≤ 0.5)

  Mapping:  D ≈ 1 − V  under Ze-tight bound, so D² + V² = (1−V)² + V² ≤ 1
  when V < 1. The Ze formula saturates the Englert inequality (V² + D² = 1)
  for all p_T, which is a stronger claim than the standard inequality.

  Falsification: if measured V deviates systematically from 1 − 2·p_T
  while satisfying V² + D² < 1, ZeVT Prediction P1 is falsified.
  """
  use ZeSimWeb, :live_view

  @impl true
  def mount(_params, _session, socket) do
    {:ok,
     assign(socket,
       p_t: 0.25,
       p_t_input: "0.25",
       input_error: nil
     )}
  end

  @impl true
  def render(assigns) do
    v_ze  = max(0.0, 1.0 - 2.0 * assigns.p_t)
    d_ze  = 1.0 - v_ze
    v_std_max = :math.sqrt(1.0 - d_ze * d_ze)

    ~H"""
    <div class="max-w-2xl mx-auto p-6 font-mono">
      <h1 class="text-2xl font-bold mb-1">Double-Slit: Ze vs Standard QM</h1>
      <p class="text-sm text-gray-500 mb-1">Prediction P1 · Ze Vectors Theory FoP §5.5</p>
      <p class="text-xs text-gray-400 mb-6">
        ZeVT: V = 1 − 2·p_T (tight equality) &nbsp;·&nbsp;
        Standard QM: V² + D² ≤ 1 (inequality)
      </p>

      <div class="bg-yellow-50 border border-yellow-300 rounded p-4 mb-6 text-sm">
        <p class="font-semibold mb-1">Falsifiability criterion</p>
        <p>If measured V deviates systematically from <strong>1 − 2·p_T</strong>
        while V² + D² &lt; 1, Prediction P1 is falsified. Only if V
        saturates the Englert bound (V² + D² = 1) for all p_T is P1 confirmed.</p>
      </div>

      <div class="mb-6">
        <label class="block text-sm font-semibold mb-2">
          T-event rate of which-path detector:&nbsp;
          <span class="font-mono text-blue-700">p_T = <%= Float.round(@p_t, 3) %></span>
        </label>
        <input type="range" min="0" max="0.5" step="0.001"
               value={@p_t}
               phx-change="update_pt"
               name="p_t"
               class="w-full" />
        <div class="flex justify-between text-xs text-gray-400 mt-1">
          <span>p_T = 0 (no which-path info)</span>
          <span>p_T = 0.5 (full which-path info)</span>
        </div>
        <div class="mt-2">
          <label class="text-xs text-gray-500">Or enter exact value:</label>
          <input type="text" phx-change="update_pt_text" name="p_t_text"
                 value={@p_t_input}
                 class="ml-2 border rounded px-2 py-0.5 text-sm font-mono w-24" />
          <%= if @input_error do %>
            <span class="text-red-500 text-xs ml-2"><%= @input_error %></span>
          <% end %>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4 mb-6">
        <div class={"rounded-lg p-4 border-2 bg-green-50 border-green-400"}>
          <p class="text-xs text-gray-500 mb-1">ZeVT Prediction P1</p>
          <p class="text-xs font-mono text-gray-400 mb-2">V = 1 − 2·p_T</p>
          <p class="text-2xl font-bold font-mono text-green-700">
            V = <%= Float.round(v_ze, 4) %>
          </p>
          <p class="text-xs text-gray-500 mt-1">D = <%= Float.round(d_ze, 4) %></p>
          <p class="text-xs font-mono text-gray-400 mt-1">
            V² + D² = <%= Float.round(v_ze*v_ze + d_ze*d_ze, 4) %>
          </p>
          <p class={"text-xs mt-1 #{if abs(v_ze*v_ze + d_ze*d_ze - 1.0) < 0.001, do: "text-green-600", else: "text-red-500"}"}>
            <%= if abs(v_ze*v_ze + d_ze*d_ze - 1.0) < 0.001, do: "✓ Saturates Englert bound", else: "⚠ Does not saturate" %>
          </p>
        </div>
        <div class="rounded-lg p-4 border-2 bg-blue-50 border-blue-400">
          <p class="text-xs text-gray-500 mb-1">Standard QM (Englert 1996)</p>
          <p class="text-xs font-mono text-gray-400 mb-2">V² + D² ≤ 1</p>
          <p class="text-2xl font-bold font-mono text-blue-700">
            V ≤ <%= Float.round(v_std_max, 4) %>
          </p>
          <p class="text-xs text-gray-500 mt-1">D = <%= Float.round(d_ze, 4) %> (same)</p>
          <p class="text-xs font-mono text-gray-400 mt-1">
            Max V² + D² = 1.0000 (by design)
          </p>
          <p class="text-xs mt-1 text-blue-600">
            Allows V &lt; V_Ze for same D
          </p>
        </div>
      </div>

      <div class="bg-gray-50 rounded p-4 mb-4">
        <p class="text-xs font-semibold text-gray-600 mb-2">Visual comparison</p>
        <div class="relative h-8 bg-gray-200 rounded overflow-hidden mb-1">
          <div class="absolute left-0 top-0 h-full bg-green-400 opacity-80 transition-all"
               style={"width: #{Float.round(v_ze * 100, 1)}%"}></div>
          <span class="absolute left-2 top-1 text-xs font-mono text-white font-bold">
            Ze: V = <%= Float.round(v_ze, 3) %>
          </span>
        </div>
        <div class="relative h-8 bg-gray-200 rounded overflow-hidden">
          <div class="absolute left-0 top-0 h-full bg-blue-400 opacity-80 transition-all"
               style={"width: #{Float.round(v_std_max * 100, 1)}%"}></div>
          <span class="absolute left-2 top-1 text-xs font-mono text-white font-bold">
            QM max: V = <%= Float.round(v_std_max, 3) %>
          </span>
        </div>
        <p class="text-xs text-gray-400 mt-2">
          Green = Ze prediction (tight); Blue = QM maximum allowed.
          Ze predicts the <em>exact</em> value; QM provides the upper bound.
        </p>
      </div>

      <div class="mt-6 text-xs text-gray-400 space-y-1">
        <p>Reference: Englert (1996) PRL 77:2154 · Scully et al. (1991) Nature 351:111</p>
        <p>Status: Testable with existing quantum optics data. No new experiment required for P1.</p>
        <p>Source: 5+_Ze_Foundations_of_Physics.md §5.5 · CONCEPT.md §8c</p>
      </div>
    </div>
    """
  end

  @impl true
  def handle_event("update_pt", %{"p_t" => val}, socket) do
    case Float.parse(val) do
      {f, _} when f >= 0.0 and f <= 0.5 ->
        {:noreply, assign(socket, p_t: f, p_t_input: "#{Float.round(f, 3)}", input_error: nil)}
      _ ->
        {:noreply, socket}
    end
  end

  def handle_event("update_pt_text", %{"p_t_text" => val}, socket) do
    case Float.parse(String.trim(val)) do
      {f, _} when f >= 0.0 and f <= 0.5 ->
        {:noreply, assign(socket, p_t: f, p_t_input: val, input_error: nil)}
      {f, _} when f < 0.0 or f > 0.5 ->
        {:noreply, assign(socket, p_t_input: val, input_error: "p_T must be in [0, 0.5]")}
      _ ->
        {:noreply, assign(socket, p_t_input: val, input_error: "Invalid number")}
    end
  end
end
