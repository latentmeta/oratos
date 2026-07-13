defmodule Mix.Tasks.Oratos do
  @shortdoc "Show Oratos CLI help (wrapper for the native binary)"
  @moduledoc """
  Runs `oratos --help` after ensuring the CLI binary is installed.

  Prefer `mix oratos.audit` for audits.
  """

  use Mix.Task

  @impl true
  def run(args) do
    bin = Oratos.ensure_binary!()
    forwarded = if args == [], do: ["--help"], else: args
    {_, status} = System.cmd(bin, forwarded, into: IO.stream(:stdio, :line))
    if status != 0, do: Mix.raise("oratos exited with status #{status}")
  end
end
