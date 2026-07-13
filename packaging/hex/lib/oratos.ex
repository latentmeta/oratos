defmodule Oratos do
  @moduledoc """
  Helpers for locating and ensuring the Oratos CLI binary.

  This package does **not** audit HTML inside the BEAM. It downloads and runs
  the native `oratos` binary from GitHub Releases.
  """

  @repo "latentmeta/oratos"
  @default_version "0.3.0"

  @doc "Configured Oratos version (without leading `v`)."
  def version do
    Application.get_env(:oratos, :version, @default_version)
    |> to_string()
    |> String.trim_leading("v")
  end

  @doc "Path to the managed binary under this package's priv directory."
  def binary_path do
    name = if match?({:win32, _}, :os.type()), do: "oratos.exe", else: "oratos"
    Path.join([:code.priv_dir(:oratos), "bin", name])
  end

  @doc """
  Ensure the Oratos binary is present, downloading it if needed.

  Returns `{:ok, path}` or `{:error, reason}`.
  """
  def ensure_binary! do
    case ensure_binary() do
      {:ok, path} -> path
      {:error, reason} -> Mix.raise("oratos: #{reason}")
    end
  end

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

    File.mkdir_p!(Path.dirname(dest))
    tmp = Path.join(System.tmp_dir!(), "oratos-#{:erlang.unique_integer([:positive])}")
    File.mkdir_p!(tmp)

    try do
      archive = Path.join(tmp, asset)
      Mix.shell().info("oratos: downloading #{url}")

      case System.cmd("curl", ["-fsSL", url, "-o", archive], stderr_to_stdout: true) do
        {_, 0} ->
          extract!(archive, ext, tmp)
          bin_name = if os == "windows", do: "oratos.exe", else: "oratos"
          src = Path.join(tmp, bin_name)

          if File.exists?(src) do
            File.cp!(src, dest)
            File.chmod!(dest, 0o755)
            {:ok, dest}
          else
            {:error, "binary missing after extract: #{src}"}
          end

        {out, status} ->
          {:error, "download failed (exit #{status}): #{String.trim(out)}"}
      end
    after
      File.rm_rf(tmp)
    end
  end

  defp extract!(archive, "tar.gz", dest) do
    System.cmd("tar", ["-xzf", archive, "-C", dest], stderr_to_stdout: true)
  end

  defp extract!(archive, "zip", dest) do
    System.cmd("unzip", ["-qo", archive, "-d", dest], stderr_to_stdout: true)
  end

  defp host_platform do
    os =
      case :os.type() do
        {:unix, :darwin} -> "macos"
        {:unix, _} -> "linux"
        {:win32, _} -> "windows"
      end

    arch =
      case :erlang.system_info(:system_architecture) |> to_string() do
        arch when arch in ["x86_64", "amd64"] or String.starts_with?(arch, "x86_64") ->
          "x86_64"

        arch when arch in ["aarch64", "arm64"] or String.contains?(arch, "aarch64") or String.contains?(arch, "arm64") ->
          "aarch64"

        other ->
          # Fall back via uname on Unix
          case System.cmd("uname", ["-m"], stderr_to_stdout: true) do
            {"arm64\n", 0} -> "aarch64"
            {"aarch64\n", 0} -> "aarch64"
            {"x86_64\n", 0} -> "x86_64"
            _ -> raise "oratos: unsupported architecture #{inspect(other)}"
          end
      end

    {os, arch}
  end
end
