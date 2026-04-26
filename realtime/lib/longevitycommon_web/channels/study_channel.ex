defmodule LongevityCommonRealtimeWeb.StudyChannel do
  use Phoenix.Channel

  @impl true
  def join("study:" <> study_id, _params, socket) do
    socket = assign(socket, :study_id, study_id)
    {:ok, socket}
  end

  # Broadcast enrollment count updates
  def handle_info({:enrollment_updated, payload}, socket) do
    push(socket, "enrollment_updated", payload)
    {:noreply, socket}
  end
end
