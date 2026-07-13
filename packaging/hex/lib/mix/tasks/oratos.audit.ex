defmodule Mix.Tasks.Oratos.Audit do
  @shortdoc "Audit rendered HTML with Oratos (SEO, a11y, JSON-LD, LLM readiness)"
  @moduledoc """
  Runs `oratos audit` after ensuring the CLI binary is available.

  ## Examples

      mix oratos.audit
      mix oratos.audit ./priv/static --fail-under 85
      mix oratos.audit ./priv/static --strict --format sarif --output reports/oratos.sarif
      mix oratos.audit ./priv/static --changed-only

  When no target is given, defaults to `./priv/static` if that directory exists,
  otherwise `./dist`.

  Common CI gate:

      mix oratos.audit ./priv/static --fail-under 85 --strict

  See the [package README](readme.html) for Phoenix prerender workflows and aliases.
  """

  use Mix.Task

  @impl true
  def run(args) do
    bin = Oratos.ensure_binary!()
    args = maybe_default_target(args)
    {_, status} = System.cmd(bin, ["audit" | args], into: IO.stream(:stdio, :line))
    if status != 0, do: exit({:shutdown, status})
  end

  defp maybe_default_target([]), do: [default_target()]
  defp maybe_default_target(["--" | rest]), do: [default_target() | rest]

  defp maybe_default_target([first | _] = args) do
    if String.starts_with?(first, "-"), do: [default_target() | args], else: args
  end

  defp default_target do
    cond do
      File.dir?("priv/static") -> "priv/static"
      File.dir?("dist") -> "dist"
      true -> "priv/static"
    end
  end
end
