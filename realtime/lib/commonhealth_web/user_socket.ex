defmodule CommonhealthRealtimeWeb.UserSocket do
  use Phoenix.Socket

  channel "feed:*",   CommonhealthRealtimeWeb.FeedChannel
  channel "ze_clock", CommonhealthRealtimeWeb.ZeClockChannel
  channel "study:*",  CommonhealthRealtimeWeb.StudyChannel

  @impl true
  def connect(%{"token" => token}, socket, _connect_info) do
    case CommonhealthRealtime.Auth.verify_token(token) do
      {:ok, user_id} ->
        {:ok, assign(socket, :user_id, user_id)}
      {:error, _} ->
        :error
    end
  end

  def connect(_params, _socket, _connect_info), do: :error

  @impl true
  def id(socket), do: "user_socket:#{socket.assigns.user_id}"
end
