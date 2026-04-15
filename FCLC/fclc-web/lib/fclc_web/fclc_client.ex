defmodule FclcWeb.FclcClient do
  @moduledoc """
  HTTP client for the FCLC orchestrator (fclc-server REST API).

  All requests include the Bearer token from `FCLC_API_TOKEN` env var.
  Base URL is configured via `FCLC_SERVER_URL` (default: http://localhost:3000).
  """

  @default_url "http://localhost:3000"

  defp base_url, do: System.get_env("FCLC_SERVER_URL", @default_url)

  defp headers do
    case System.get_env("FCLC_API_TOKEN") do
      nil   -> [{"content-type", "application/json"}]
      token -> [{"authorization", "Bearer #{token}"}, {"content-type", "application/json"}]
    end
  end

  @doc "GET /api/metrics — aggregated dashboard metrics."
  def get_metrics do
    Req.get("#{base_url()}/api/metrics", headers: headers())
    |> handle_response()
  end

  @doc "GET /api/rounds — list all completed rounds."
  def list_rounds do
    Req.get("#{base_url()}/api/rounds", headers: headers())
    |> handle_response()
  end

  @doc "GET /api/nodes — list all registered clinic nodes."
  def list_nodes do
    Req.get("#{base_url()}/api/nodes", headers: headers())
    |> handle_response()
  end

  @doc "GET /api/nodes/:node_id/score — Shapley score history for a node."
  def get_node_score(node_id) do
    Req.get("#{base_url()}/api/nodes/#{node_id}/score", headers: headers())
    |> handle_response()
  end

  @doc "POST /api/rounds/trigger — trigger a federated aggregation round."
  def trigger_round do
    Req.post("#{base_url()}/api/rounds/trigger", headers: headers(), json: %{})
    |> handle_response()
  end

  @doc "GET /api/audit — tamper-evident hash-chain audit log."
  def get_audit_chain do
    Req.get("#{base_url()}/api/audit", headers: headers())
    |> handle_response()
  end

  @doc "GET /api/model/current — current global model weights and round."
  def get_current_model do
    Req.get("#{base_url()}/api/model/current", headers: headers())
    |> handle_response()
  end

  defp handle_response({:ok, %{status: status, body: body}}) when status in 200..299 do
    {:ok, body}
  end
  defp handle_response({:ok, %{status: status, body: body}}) do
    {:error, "HTTP #{status}: #{inspect(body)}"}
  end
  defp handle_response({:error, reason}) do
    {:error, "Connection error: #{inspect(reason)}"}
  end
end
