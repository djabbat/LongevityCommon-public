defmodule LongevityCommonRealtimeWeb.ZeClockChannel do
  @moduledoc """
  Ze·Clock channel — broadcasts weekly community Ze stats every Monday.
  Clients subscribe to receive:
    - mean community χ_Ze
    - best cohort
    - top intervention correlation
  """
  use Phoenix.Channel
  alias LongevityCommonRealtime.Repo
  import Ecto.Query

  @impl true
  def join("ze_clock", _params, socket) do
    # Send current stats on join
    send(self(), :send_stats)
    {:ok, socket}
  end

  @impl true
  def handle_info(:send_stats, socket) do
    stats = compute_weekly_stats()
    push(socket, "ze_clock_update", stats)
    {:noreply, socket}
  end

  defp compute_weekly_stats do
    # Aggregate χ_Ze from last 7 days across all verified samples
    # Runs against the same PostgreSQL as Rust API
    week_ago = DateTime.add(DateTime.utc_now(), -7 * 24 * 3600)

    query = """
    SELECT
      AVG(chi_ze_combined) as mean_chi_ze,
      COUNT(DISTINCT user_id) as active_users,
      COUNT(*) as total_samples
    FROM ze_samples
    WHERE is_verified = true
      AND recorded_at > $1
    """

    case Repo.query(query, [week_ago]) do
      {:ok, %{rows: [[mean_chi, active, total]]}} ->
        %{
          week_of: Date.to_iso8601(Date.utc_today()),
          mean_chi_ze: mean_chi && Float.round(mean_chi, 4),
          active_users: active,
          total_samples: total,
        }
      _ ->
        %{week_of: Date.to_iso8601(Date.utc_today()), mean_chi_ze: nil}
    end
  end
end
