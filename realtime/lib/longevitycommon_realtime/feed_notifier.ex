defmodule LongevityCommonRealtime.FeedNotifier do
  @moduledoc """
  Postgres LISTEN/NOTIFY bridge between the Rust social-server and the
  Phoenix `feed:lobby` channel (Phase 4.5, 2026-05-08).

  Subscribes to channel `feed_events` on the same Postgres DB the Rust
  server writes to (`longevitycommon_social`). Each message is a JSON
  blob like `{"kind":"new_post","post_id":"<uuid>",...}`. The notifier
  decodes and broadcasts to the `feed:lobby` Phoenix Channel; clients
  subscribed via the `useRealtime` hook (web/src/hooks/useRealtime.ts)
  invalidate their feed query and refetch.

  Why LISTEN/NOTIFY instead of HTTP webhook: zero coupling to a port,
  survives Phoenix restart (events are at-most-once though — accept
  loss; clients also poll on 60s fallback), single source of truth
  for "post created" event = the DB transaction itself.
  """
  use GenServer
  require Logger

  @channel "feed_events"
  @phoenix_topic "feed:lobby"

  def start_link(_opts) do
    GenServer.start_link(__MODULE__, nil, name: __MODULE__)
  end

  @impl true
  def init(_) do
    repo_config = LongevityCommonRealtime.Repo.config()
    case Postgrex.Notifications.start_link(repo_config) do
      {:ok, pid} ->
        ref = Postgrex.Notifications.listen!(pid, @channel)
        Logger.info("FeedNotifier: LISTEN #{@channel} (ref=#{inspect(ref)})")
        {:ok, %{conn: pid, ref: ref}}
      {:error, reason} ->
        Logger.error("FeedNotifier: failed to connect to Postgres for LISTEN: #{inspect(reason)}")
        # Don't crash the supervisor — service is still useful for direct broadcasts.
        :ignore
    end
  end

  @impl true
  def handle_info({:notification, _pid, _ref, @channel, payload}, state) do
    Logger.info("FeedNotifier: notify received (#{byte_size(payload)} bytes)")
    case Jason.decode(payload) do
      {:ok, %{"kind" => kind} = msg} when kind in ["new_post", "post_updated", "post_deleted"] ->
        Logger.info("FeedNotifier: broadcast #{kind} to #{@phoenix_topic}")
        LongevityCommonRealtimeWeb.Endpoint.broadcast(@phoenix_topic, kind, msg)
      {:ok, %{"kind" => other}} ->
        Logger.warning("FeedNotifier: unknown kind=#{other}, ignoring")
      {:error, e} ->
        Logger.warning("FeedNotifier: invalid JSON payload: #{inspect(e)}; raw=#{payload}")
    end
    {:noreply, state}
  end

  @impl true
  def handle_info(other, state) do
    Logger.debug("FeedNotifier: unexpected message #{inspect(other)}")
    {:noreply, state}
  end
end
