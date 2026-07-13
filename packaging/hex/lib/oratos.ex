defmodule Oratos do
  @moduledoc """
  Manages the native Oratos CLI for Elixir and Phoenix projects.

  Oratos audits **rendered HTML** for SEO, accessibility, structured data, and
  LLM readiness. This Hex package downloads a prebuilt binary from GitHub
  Releases — it does **not** parse HTML inside the BEAM.

  ## Quick start (Phoenix)

      # mix.exs — not needed in production
      {:oratos, "~> 0.3.1", only: [:dev, :test], runtime: false}

      mix deps.get
      mix assets.deploy && mix phx.digest && mix phoenix.prerender
      mix oratos.audit ./priv/static --fail-under 85

  See the [Hex package README](readme.html) for Mix aliases, CI, `oratos.toml`,
  and how Oratos relates to `phoenix_seo`.

  ## Configuration

      config :oratos,
        version: "0.3.1",
        # path: "/usr/local/bin/oratos",
        prefer_path: false
  """

  @repo "latentmeta/oratos"
  @default_version "0.3.1"

  @doc "Configured Oratos CLI version (without leading `v`)."
  def version do
    Application.get_env(:oratos, :version, @default_version)
    |> to_string()
    |> String.trim_leading("v")
  end

  @doc "Path to the managed binary under this package's `priv/bin` directory."
  def binary_path do
    name = if match?({:win32, _}, :os.type()), do: "oratos.exe", else: "oratos"
    Path.join([:code.priv_dir(:oratos), "bin", name])
  end

  @doc """
  Ensure the Oratos binary is present, downloading it if needed.

  Raises via `Mix.raise/1` on failure.
  """
  def ensure_binary! do
    case ensure_binary() do
      {:ok, path} -> path
      {:error, reason} -> Mix.raise("oratos: #{reason}")
    end
  end

  @doc """
  Ensure the Oratos binary is present, downloading it if needed.

  Returns `{:ok, path}` or `{:error, reason}`.

  Resolution order:

  1. `config :oratos, path: "..."` if set
  2. `oratos` on `PATH` when `prefer_path: true`
  3. Cached binary under `priv/bin`
  4. Download from GitHub Releases for `version/0`
  """
  def ensure_binary do
    path = binary_path()

    cond do
      Application.get_env(:oratos, :path) ->
        custom = Application.get_env(:oratos, :path)

        if File.exists?(custom),
          do: {:ok, custom},
          else: {:error, "configured :path not found: #{custom}"}

      System.find_executable("oratos") && Application.get_env(:oratos, :prefer_path, false) ->
        {:ok, System.find_executable("oratos")}

      File.exists?(path) ->
        {:ok, path}

      true ->
        download(path)
    end
  end

  defp download(dest) do
    {os, arch} = host_platform()
    tag = "v" <> version()
    ext = if os == "windows", do: "zip", else: "tar.gz"
    asset = "oratos-#{tag}-#{os}-#{arch}.#{ext}"
    url = "https://github.com/#{@repo}/releases/download/#{tag}/#{asset}"
    bin_name = if os == "windows", do: "oratos.exe", else: "oratos"
    # GitHub redirect URLs are application/octet-stream without a .tar.gz path,
    # so pick the decoder from the asset we requested instead of MIME sniffing.
    decode = if os == "windows", do: &Req.ZIP.decode/1, else: &Req.Tar.decode/1

    Mix.shell().info("oratos: downloading #{url}")
    {:ok, _} = Application.ensure_all_started(:req)

    case Req.get(url, decode_body: false, redirect: true) do
      {:ok, %{status: 200, body: body}} when is_binary(body) ->
        case decode.(body) do
          {:ok, entries} -> install_binary(entries, dest, bin_name)
          {:error, exception} -> {:error, Exception.message(exception)}
        end

      {:ok, %{status: status}} ->
        {:error, "download failed: HTTP #{status} for #{url}"}

      {:error, exception} ->
        {:error, "download failed: #{Exception.message(exception)}"}
    end
  end

  defp install_binary(entries, dest, bin_name) do
    match =
      Enum.find(entries, fn {name, _} ->
        name |> to_string() |> Path.basename() == bin_name
      end)

    case match do
      {_, contents} ->
        File.mkdir_p!(Path.dirname(dest))
        File.write!(dest, contents)
        unless match?({:win32, _}, :os.type()), do: File.chmod!(dest, 0o755)
        {:ok, dest}

      nil ->
        names = Enum.map_join(entries, ", ", fn {n, _} -> to_string(n) end)
        {:error, "archive missing #{bin_name} (found: #{names})"}
    end
  end

  defp host_platform do
    os =
      case :os.type() do
        {:unix, :darwin} -> "macos"
        {:unix, _} -> "linux"
        {:win32, _} -> "windows"
      end

    arch_string = :erlang.system_info(:system_architecture) |> to_string()

    arch =
      cond do
        arch_string in ["x86_64", "amd64"] or String.starts_with?(arch_string, "x86_64") ->
          "x86_64"

        arch_string in ["aarch64", "arm64"] or String.contains?(arch_string, "aarch64") or
            String.contains?(arch_string, "arm64") ->
          "aarch64"

        true ->
          case System.cmd("uname", ["-m"], stderr_to_stdout: true) do
            {"arm64\n", 0} -> "aarch64"
            {"aarch64\n", 0} -> "aarch64"
            {"x86_64\n", 0} -> "x86_64"
            _ -> raise "oratos: unsupported architecture #{inspect(arch_string)}"
          end
      end

    {os, arch}
  end
end
