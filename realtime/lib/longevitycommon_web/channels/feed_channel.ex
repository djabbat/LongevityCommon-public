defmodule LongevityCommonRealtimeWeb.FeedChannel do
  use Phoenix.Channel

  alias Phoenix.PubSub

  @impl true
  def join("feed:public", _params, socket) do
    # Subscribe to feed updates published by Rust API via pg_notify
    PubSub.subscribe(LongevityCommonRealtime.PubSub, "feed:public")
    {:ok, socket}
  end

  def join("feed:" <> user_id, _params, socket) do
    if user_id == socket.assigns.user_id do
      {:ok, socket}
    else
      {:error, %{reason: "unauthorized"}}
    end
  end

  # Broadcast new post score updates to all subscribers
  def handle_info({:post_score_updated, payload}, socket) do
    push(socket, "score_updated", payload)
    {:noreply, socket}
  end

  # Broadcast new post to feed
  def handle_info({:new_post, payload}, socket) do
    push(socket, "new_post", payload)
    {:noreply, socket}
  end
end
