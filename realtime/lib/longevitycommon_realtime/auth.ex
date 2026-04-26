defmodule LongevityCommonRealtime.Auth do
  @moduledoc """
  JWT verification — validates tokens issued by Rust/Axum server.
  Both services share the same JWT_SECRET.
  """

  def verify_token(token) do
    secret = Application.fetch_env!(:longevitycommon_realtime, :jwt_secret)
    signer = Joken.Signer.create("HS256", secret)

    case Joken.verify(token, signer) do
      {:ok, claims} ->
        user_id = claims["sub"]
        {:ok, user_id}
      {:error, reason} ->
        {:error, reason}
    end
  end
end
