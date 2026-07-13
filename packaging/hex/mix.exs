defmodule Oratos.MixProject do
  use Mix.Project

  @version "0.3.0"

  def project do
    [
      app: :oratos,
      version: @version,
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      description: description(),
      package: package(),
      docs: [
        main: "readme",
        extras: ["README.md"]
      ]
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:ex_doc, "~> 0.34", only: :dev, runtime: false}
    ]
  end

  defp description do
    """
    Mix wrapper for the Oratos CLI (website visibility audits).
    Downloads a prebuilt binary from GitHub Releases — no Rust required.
    """
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{
        "GitHub" => "https://github.com/latentmeta/oratos",
        "Docs" => "https://github.com/latentmeta/oratos/blob/main/docs/install.md"
      },
      files: ~w(lib mix.exs README.md)
    ]
  end
end
