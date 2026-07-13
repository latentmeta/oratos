defmodule Oratos.MixProject do
  use Mix.Project

  @version "0.3.1"

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
        extras: ["README.md"],
        source_url: "https://github.com/latentmeta/oratos",
        source_url_pattern:
          "https://github.com/latentmeta/oratos/blob/v#{@version}/packaging/hex/%{path}#L%{line}"
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
      {:req, "~> 0.5"},
      {:ex_doc, "~> 0.34", only: :dev, runtime: false}
    ]
  end

  defp description do
    """
    Mix tasks for Oratos — audit Phoenix/static HTML for SEO, accessibility,
    structured data, and LLM readiness. Downloads a prebuilt CLI (no Rust).
    """
  end

  defp package do
    [
      name: "oratos",
      licenses: ["MIT"],
      links: %{
        "GitHub" => "https://github.com/latentmeta/oratos",
        "Changelog" => "https://github.com/latentmeta/oratos/blob/main/CHANGELOG.md",
        "Phoenix guide" => "https://github.com/latentmeta/oratos/blob/main/docs/phoenix.md",
        "Install" => "https://github.com/latentmeta/oratos/blob/main/docs/install.md",
        "Rules" => "https://github.com/latentmeta/oratos/blob/main/docs/rules.md"
      },
      files: ~w(lib mix.exs README.md)
    ]
  end
end
