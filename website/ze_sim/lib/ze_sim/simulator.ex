defmodule ZeSim.Simulator do
  @moduledoc """
  Calls the ze-runner Rust binary and parses JSON results.
  The binary must be compiled and placed at priv/ze-runner.
  """

  @runner_path Application.compile_env(:ze_sim, :runner_path,
    Path.join(:code.priv_dir(:ze_sim), "ze-runner"))

  @doc """
  Run the thermodynamic simulation.

  Options:
    - molecules: integer (default 100)
    - steps: integer (default 500)
    - demon: boolean (default false)
    - seed: integer (default 42)
  """
  def run_thermo(opts \\ []) do
    base = [
      "thermo",
      "--molecules", to_string(opts[:molecules] || 100),
      "--steps",     to_string(opts[:steps] || 500),
      "--seed",      to_string(opts[:seed] || 42)
    ]
    demon_flag      = if opts[:demon],               do: ["--demon"],       else: []
    cold_start_flag = if opts[:cold_start] != false, do: ["--cold-start"],  else: []
    call_runner(base ++ demon_flag ++ cold_start_flag)
  end

  @doc """
  Run the quantum simulation.

  Options:
    - dim: integer (default 4)
    - steps: integer (default 2000)
    - states: integer (default 50)
    - seed: integer (default 42)
  """
  def run_quantum(opts \\ []) do
    args = [
      "quantum",
      "--dim",    to_string(opts[:dim] || 4),
      "--steps",  to_string(opts[:steps] || 2000),
      "--states", to_string(opts[:states] || 50),
      "--seed",   to_string(opts[:seed] || 42)
    ]

    call_runner(args)
  end

  @doc """
  Run Ze-Reproduction simulation (Axiom Z4 + double-slit).

  Options: tau0, chains, dim, seed
  """
  def run_repro(opts \\ []) do
    args = [
      "repro",
      "--tau0",   to_string(opts[:tau0]   || 200),
      "--chains", to_string(opts[:chains] || 500),
      "--dim",    to_string(opts[:dim]    || 4),
      "--seed",   to_string(opts[:seed]   || 42)
    ]
    call_runner(args)
  end

  defp call_runner(args) do
    case System.cmd(@runner_path, args, stderr_to_stdout: true) do
      {output, 0} ->
        case Jason.decode(output, keys: :atoms) do
          {:ok, result} -> {:ok, result}
          {:error, err} -> {:error, "JSON parse error: #{inspect(err)}"}
        end

      {output, code} ->
        {:error, "ze-runner exited #{code}: #{output}"}
    end
  end
end
