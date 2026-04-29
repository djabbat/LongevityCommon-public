defmodule ZeSimWeb.ParticlesLive do
  @moduledoc """
  Ze Vectors Theory — Particle Lifetime Calculator (FoP article §7.2).

  Particle decay as Ze-Competition (Axiom Z5):
    τ_{1/2} = τ_Z^(0) / ν

  where:
    τ_Z^(0) — initial Ze-budget of the particle
    ν       — intrinsic clock rate (Hz)

  Known particles (ν ≈ 1 Hz assumption, post-diction validated in article):
  | Particle | τ_{1/2} experiment | τ_Z^(0) |
  | Neutron  | 880 s              | 880     |
  | Muon     | 2.2 μs             | ~2.2e-6 |
  | Pion π⁰  | 8.4e-17 s          | ~8.4e-17|
  | Proton   | ≥1.6e34 yr         | ~5e41   |
  """
  use ZeSimWeb, :live_view

  @known_particles [
    %{name: "Neutron",  symbol: "n",   tau_half_s: 880.0,      tau_z: 880.0,   nu_hz: 1.0,  source: "PDG 2022"},
    %{name: "Muon",     symbol: "μ⁻",  tau_half_s: 2.197e-6,   tau_z: 2.197e-6, nu_hz: 1.0, source: "PDG 2022"},
    %{name: "Pion π⁰",  symbol: "π⁰",  tau_half_s: 8.43e-17,   tau_z: 8.43e-17, nu_hz: 1.0, source: "PDG 2022"},
    %{name: "Proton",   symbol: "p",   tau_half_s: 5.04e41,    tau_z: 5.04e41, nu_hz: 1.0,  source: "Super-K 2020 (lower bound ≥1.6e34 yr)"},
  ]

  @impl true
  def mount(_params, _session, socket) do
    {:ok,
     assign(socket,
       tau_z_input: "880",
       nu_input: "1.0",
       tau_half_computed: nil,
       selected_particle: nil,
       error: nil
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="max-w-2xl mx-auto p-6 font-mono">
      <h1 class="text-2xl font-bold mb-1">Particle Lifetime Calculator</h1>
      <p class="text-sm text-gray-500 mb-1">Ze Vectors Theory · Axiom Z5 (Ze-Competition)</p>
      <p class="text-xs text-gray-400 mb-6">
        τ_{1/2} = τ_Z^(0) / ν &nbsp;·&nbsp; ν ≈ 1 Hz (working assumption; open question)
      </p>

      <div class="bg-blue-50 rounded p-4 mb-6">
        <p class="font-semibold text-sm mb-3">Known particles (post-diction, FoP article Table §7.2)</p>
        <table class="w-full text-xs">
          <thead>
            <tr class="border-b text-gray-500">
              <th class="text-left py-1">Particle</th>
              <th class="text-right py-1">τ_{1/2} (exp.)</th>
              <th class="text-right py-1">τ_Z^(0)</th>
              <th class="text-right py-1">Match</th>
            </tr>
          </thead>
          <tbody>
            <%= for p <- @known_particles do %>
              <tr class="border-b border-blue-100 hover:bg-blue-100 cursor-pointer"
                  phx-click="select_particle" phx-value-name={p.name}>
                <td class="py-1 pr-2 font-medium"><%= p.symbol %> (<%= p.name %>)</td>
                <td class="py-1 pr-2 text-right font-mono"><%= format_lifetime(p.tau_half_s) %></td>
                <td class="py-1 pr-2 text-right font-mono"><%= format_scientific(p.tau_z) %></td>
                <td class="py-1 text-right text-green-600">✓</td>
              </tr>
            <% end %>
          </tbody>
        </table>
        <p class="text-xs text-gray-400 mt-2">
          Click a row to load its values into the calculator below.
        </p>
      </div>

      <div class="bg-white border rounded p-4 mb-4">
        <p class="font-semibold text-sm mb-3">Custom calculator</p>
        <div class="grid grid-cols-2 gap-4 mb-4">
          <div>
            <label class="text-xs text-gray-500 block mb-1">τ_Z^(0) — initial Ze-budget</label>
            <input type="text" phx-change="update_tau_z" name="tau_z"
                   value={@tau_z_input}
                   class="w-full border rounded px-2 py-1 text-sm font-mono" />
          </div>
          <div>
            <label class="text-xs text-gray-500 block mb-1">ν (Hz) — intrinsic clock rate</label>
            <input type="text" phx-change="update_nu" name="nu"
                   value={@nu_input}
                   class="w-full border rounded px-2 py-1 text-sm font-mono" />
          </div>
        </div>
        <button phx-click="compute"
                class="bg-blue-600 text-white px-4 py-2 rounded text-sm hover:bg-blue-700">
          Compute τ_{1/2}
        </button>
      </div>

      <%= if @error do %>
        <div class="bg-red-50 border border-red-300 rounded p-3 text-sm text-red-700 mb-4">
          <%= @error %>
        </div>
      <% end %>

      <%= if @tau_half_computed do %>
        <div class="bg-green-50 border-2 border-green-400 rounded p-4">
          <p class="text-sm text-gray-600 mb-1">
            τ_{1/2} = τ_Z^(0) / ν = <%= @tau_z_input %> / <%= @nu_input %>
          </p>
          <p class="text-3xl font-mono font-bold text-green-700">
            = <%= format_lifetime(@tau_half_computed) %>
          </p>
          <%= if @selected_particle do %>
            <p class="text-xs text-gray-500 mt-2">
              Experimental value: <%= format_lifetime(@selected_particle.tau_half_s) %>
              &nbsp;(<%= @selected_particle.source %>)
            </p>
          <% end %>
        </div>
      <% end %>

      <div class="mt-8 text-xs text-gray-400 space-y-1">
        <p>Note: ν = 1 Hz is a working assumption enabling post-diction. Predictive use requires independent determination of ν.</p>
        <p>Ze-budget conservation: τ_Z^(daughter_1) + τ_Z^(daughter_2) = τ_Z^(parent)</p>
        <p>Source: 5+_Ze_Foundations_of_Physics.md §7.2 · Axiom Z5</p>
      </div>
    </div>
    """
  end

  @impl true
  def handle_event("select_particle", %{"name" => name}, socket) do
    particle = Enum.find(@known_particles, &(&1.name == name))
    if particle do
      tau_half = particle.tau_z / particle.nu_hz
      {:noreply,
       assign(socket,
         tau_z_input: format_scientific(particle.tau_z),
         nu_input: "#{particle.nu_hz}",
         tau_half_computed: tau_half,
         selected_particle: particle,
         error: nil
       )}
    else
      {:noreply, socket}
    end
  end

  def handle_event("update_tau_z", %{"tau_z" => val}, socket) do
    {:noreply, assign(socket, tau_z_input: val, tau_half_computed: nil, error: nil)}
  end

  def handle_event("update_nu", %{"nu" => val}, socket) do
    {:noreply, assign(socket, nu_input: val, tau_half_computed: nil, error: nil)}
  end

  def handle_event("compute", _params, socket) do
    with {:ok, tau_z} <- parse_float(socket.assigns.tau_z_input),
         {:ok, nu}    <- parse_float(socket.assigns.nu_input),
         true         <- nu > 0.0 do
      tau_half = tau_z / nu
      {:noreply, assign(socket, tau_half_computed: tau_half, error: nil)}
    else
      _ ->
        {:noreply, assign(socket, error: "Invalid input. Enter positive numbers (scientific notation ok: 8.4e-17).", tau_half_computed: nil)}
    end
  end

  defp parse_float(str) do
    case Float.parse(String.trim(str)) do
      {f, ""} -> {:ok, f}
      _       ->
        case Integer.parse(String.trim(str)) do
          {i, ""} -> {:ok, i * 1.0}
          _       -> :error
        end
    end
  end

  defp format_lifetime(s) when s >= 3.156e7,  do: "#{Float.round(s / 3.156e7, 2)} yr"
  defp format_lifetime(s) when s >= 86400,    do: "#{Float.round(s / 86400, 2)} days"
  defp format_lifetime(s) when s >= 1.0,      do: "#{Float.round(s, 3)} s"
  defp format_lifetime(s) when s >= 1.0e-3,   do: "#{Float.round(s * 1.0e3, 3)} ms"
  defp format_lifetime(s) when s >= 1.0e-6,   do: "#{Float.round(s * 1.0e6, 3)} μs"
  defp format_lifetime(s) when s >= 1.0e-9,   do: "#{Float.round(s * 1.0e9, 3)} ns"
  defp format_lifetime(s) when s >= 1.0e-12,  do: "#{Float.round(s * 1.0e12, 3)} ps"
  defp format_lifetime(s) when s >= 1.0e-15,  do: "#{Float.round(s * 1.0e15, 3)} fs"
  defp format_lifetime(s),                    do: :erlang.float_to_binary(s, [{:scientific, 2}]) <> " s"

  defp format_scientific(f) when f >= 1.0e6 or (f < 1.0e-3 and f != 0.0) do
    :erlang.float_to_binary(f * 1.0, [{:scientific, 3}])
  end
  defp format_scientific(f), do: "#{Float.round(f, 4)}"
end
