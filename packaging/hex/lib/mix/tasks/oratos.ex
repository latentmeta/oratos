defmodule Mix.Tasks.Oratos do
  @shortdoc "Run the Oratos CLI (help, generate, prompt, …)"
  @moduledoc """
  Thin wrapper around the Oratos CLI binary.

  With no arguments, prints CLI help. Otherwise forwards argv to `oratos`:

      mix oratos --version
      mix oratos generate llms ./priv/static --output priv/static/llms.txt
      mix oratos prompt phoenix priv/static/index.html --output tmp/fix.md

  For audits, prefer `mix oratos.audit`, which defaults the target to
  `priv/static` / `dist` and is nicer in Mix aliases.
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
