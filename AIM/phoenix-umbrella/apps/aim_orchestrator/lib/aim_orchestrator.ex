defmodule AimOrchestrator do
  @moduledoc """
  Coordinates calls across Rust services (aim-llm, aim-rag, aim-medkb,
  aim-doctor, diffdx-api, ssa-api). Replaces agents/orchestrator.py +
  agents/ensemble.py + agents/debate.py + agents/reflexion.py.

  All TODO — this is the skeleton.
  """

  alias AimOrchestrator.Upstream

  @doc "Single LLM call via aim-llm."
  def chat(messages, opts \\ []) do
    Upstream.post("llm_url", "/v1/chat", %{
      messages: messages,
      model_hint: Keyword.get(opts, :model_hint)
    })
  end

  @doc "Run the doctor pipeline (intake → diagnose → plan)."
  def diagnose(case_id) do
    Upstream.post("doctor_url", "/v1/diagnose", %{case_id: case_id})
  end

  # ── stubs used by LiveViews until the Rust gateway lands the routes ────
  # Each returns {:ok, payload} or :error so the LiveViews' rescue blocks
  # render an empty/default state until the real RPC is wired.

  def dashboard_projects, do: {:ok, []}
  def dashboard_deadlines, do: {:ok, []}
  def health_snapshot, do: {:ok, %{status: "starting", crates: 178, tests: 2531}}

  def check_drug_regimen(_drugs), do: {:ok, %{findings: [], report: nil}}

  def list_user_keys, do: {:ok, []}
  def set_user_key(_provider, _value), do: :ok
  def clear_user_key(_provider), do: :ok
end

# Backwards-compat alias used by Phoenix LiveViews. Phoenix HEEx + tooling
# accept calls through `Orchestrator.foo/n` regardless of where the impl
# lives.
defmodule Orchestrator do
  @moduledoc false
  defdelegate dashboard_projects(), to: AimOrchestrator
  defdelegate dashboard_deadlines(), to: AimOrchestrator
  defdelegate health_snapshot(), to: AimOrchestrator
  defdelegate check_drug_regimen(drugs), to: AimOrchestrator
  defdelegate list_user_keys(), to: AimOrchestrator
  defdelegate set_user_key(provider, value), to: AimOrchestrator
  defdelegate clear_user_key(provider), to: AimOrchestrator
end
