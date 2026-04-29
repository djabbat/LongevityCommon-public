defmodule ZeSimWeb.RegimeLive do
  @moduledoc """
  Ze Vectors Theory — v* Regime Selector (CDATA v6, 2026-04-06).

  Allows the user to classify a system as passive or active, then displays
  the corresponding optimal Ze-velocity v* and explains the decision.

  Passive regime:  v* = 1 − ln(2) ≈ 0.3069  (Shannon entropy maximum)
  Active  regime:  v* ≈ 0.456               (resource-constrained agent)
  Cost of agency:  Δv = v*_active − v*_passive ≈ 0.1491
  """
  use ZeSimWeb, :live_view

  @v_passive 0.3069   # 1 - ln(2), exact
  @v_active  0.456    # clinical HRV + cognitive data; closed form open Q1 2027
  @z_star    0.7311   # logistic fixed point: 1/(1+e^{-1})

  @impl true
  def mount(_params, _session, socket) do
    {:ok,
     assign(socket,
       system_type: nil,
       agency: false,
       tau_z: true,
       result: nil
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="max-w-2xl mx-auto p-6 font-mono">
      <h1 class="text-2xl font-bold mb-2">Ze Regime Selector</h1>
      <p class="text-sm text-gray-500 mb-6">
        Ze Vectors Theory v6 · CONCEPT.md §8 · 2026-04-06
      </p>

      <div class="bg-gray-50 rounded p-4 mb-6 text-sm">
        <p class="font-semibold mb-2">Two optimal Ze-velocities</p>
        <table class="w-full text-xs">
          <tr class="border-b">
            <td class="py-1 pr-4 font-medium">v*_passive</td>
            <td class="py-1 pr-4 font-mono text-blue-700"><%= Float.round(@v_passive, 4) %></td>
            <td class="py-1 text-gray-500">1 − ln 2, Shannon entropy max</td>
          </tr>
          <tr class="border-b">
            <td class="py-1 pr-4 font-medium">v*_active</td>
            <td class="py-1 pr-4 font-mono text-green-700"><%= Float.round(@v_active, 4) %></td>
            <td class="py-1 text-gray-500">Resource-constrained agent (approx.)</td>
          </tr>
          <tr>
            <td class="py-1 pr-4 font-medium">Δv (cost of agency)</td>
            <td class="py-1 pr-4 font-mono text-red-700"><%= Float.round(@v_active - @v_passive, 4) %></td>
            <td class="py-1 text-gray-500">Price paid for having τ_Z budget</td>
          </tr>
        </table>
      </div>

      <div class="mb-4">
        <p class="font-semibold mb-2">Step 1 — What type of system?</p>
        <div class="flex gap-3">
          <button phx-click="set_type" phx-value-type="physical"
                  class={"px-4 py-2 rounded border text-sm " <> if @system_type == "physical", do: "bg-blue-100 border-blue-400 font-bold", else: "bg-white"}>
            Physical particle
          </button>
          <button phx-click="set_type" phx-value-type="biological"
                  class={"px-4 py-2 rounded border text-sm " <> if @system_type == "biological", do: "bg-green-100 border-green-400 font-bold", else: "bg-white"}>
            Biological organism
          </button>
          <button phx-click="set_type" phx-value-type="computational"
                  class={"px-4 py-2 rounded border text-sm " <> if @system_type == "computational", do: "bg-purple-100 border-purple-400 font-bold", else: "bg-white"}>
            Computational agent
          </button>
        </div>
      </div>

      <%= if @system_type do %>
        <div class="mb-4">
          <p class="font-semibold mb-2">Step 2 — Does it make predictions and spend τ_Z budget?</p>
          <div class="flex gap-3">
            <button phx-click="set_agency" phx-value-agency="true"
                    class={"px-4 py-2 rounded border text-sm " <> if @agency, do: "bg-yellow-100 border-yellow-400 font-bold", else: "bg-white"}>
              Yes — active predictor
            </button>
            <button phx-click="set_agency" phx-value-agency="false"
                    class={"px-4 py-2 rounded border text-sm " <> if !@agency, do: "bg-gray-100 border-gray-400 font-bold", else: "bg-white"}>
              No — passive counter
            </button>
          </div>
        </div>
      <% end %>

      <%= if @result do %>
        <div class={"mt-6 p-4 rounded-lg border-2 " <> regime_color(@result.regime)}>
          <p class="text-lg font-bold mb-1"><%= @result.regime %> regime</p>
          <p class="text-3xl font-mono mb-2">v* = <span class="font-bold"><%= Float.round(@result.v_star, 4) %></span></p>
          <p class="text-sm text-gray-600 mb-3"><%= @result.explanation %></p>
          <div class="text-xs text-gray-500 space-y-1">
            <%= for ex <- @result.examples do %>
              <p>• <%= ex %></p>
            <% end %>
          </div>
          <div class="mt-3 pt-3 border-t text-xs font-mono text-gray-500">
            Z* (optimal T-fraction) = <%= Float.round(@z_star, 4) %>
            &nbsp;·&nbsp; Cost of agency Δv = <%= Float.round(@v_active - @v_passive, 4) %>
          </div>
        </div>
      <% end %>

      <div class="mt-8 text-xs text-gray-400">
        <p>Open question: closed form for v*_active — target Q1 2027.</p>
        <p>Source: 5+_Ze_Foundations_of_Physics.md §8.1 · CONCEPT.md §8</p>
      </div>
    </div>
    """
  end

  @impl true
  def handle_event("set_type", %{"type" => type}, socket) do
    result = compute_result(type, socket.assigns.agency)
    {:noreply, assign(socket, system_type: type, result: result)}
  end

  def handle_event("set_agency", %{"agency" => agency_str}, socket) do
    agency = agency_str == "true"
    result = compute_result(socket.assigns.system_type, agency)
    {:noreply, assign(socket, agency: agency, result: result)}
  end

  defp compute_result(nil, _), do: nil
  defp compute_result(type, agency) do
    if agency do
      %{
        regime: "Active",
        v_star: @v_active,
        explanation: "System has τ_Z budget and makes deliberate predictions. " <>
                     "Pays cost of agency Δv ≈ 0.1491 above passive optimum. " <>
                     "v*_active ≈ 0.456 (approx.; closed form open Q1 2027).",
        examples: active_examples(type)
      }
    else
      %{
        regime: "Passive",
        v_star: @v_passive,
        explanation: "System registers T/S events without active prediction. " <>
                     "Optimal at Shannon entropy maximum: v* = 1 − ln 2 ≈ 0.3069 (exact).",
        examples: passive_examples(type)
      }
    end
  end

  defp active_examples("physical"),     do: ["Quantum measurement device", "Particle detector with threshold", "Interferometer"]
  defp active_examples("biological"),   do: ["Human brain (HRV data: v*≈0.456)", "Immune system (antigen recognition)", "Stem cell niche (CDATA: τ_Z ≡ D_crit − D(t))"]
  defp active_examples("computational"),do: ["Reinforcement learning agent", "Ze-Competition winner (Born rule)", "Bayesian predictor with resource constraint"]
  defp active_examples(_),              do: []

  defp passive_examples("physical"),     do: ["Free neutron (τ_{1/2} = τ_Z/ν)", "Muon decay counter", "Chaotic trajectory"]
  defp passive_examples("biological"),   do: ["Cell dividing without niche signals", "Passive ion channel"]
  defp passive_examples("computational"),do: ["Lookup table", "Deterministic counter", "Random walk"]
  defp passive_examples(_),              do: []

  defp regime_color("Active"),  do: "bg-green-50 border-green-400"
  defp regime_color("Passive"), do: "bg-blue-50 border-blue-400"
  defp regime_color(_),         do: "bg-gray-50 border-gray-300"

  defp v_passive, do: @v_passive
  defp v_active,  do: @v_active
  defp z_star,    do: @z_star
end
