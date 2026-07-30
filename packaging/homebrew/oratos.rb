# Homebrew Formula for Oratos
#
# Publish to latentmeta/homebrew-tap as Formula/oratos.rb
# Update url/sha256 on each release from GitHub Release assets + SHA256SUMS.
class Oratos < Formula
  desc "Website visibility intelligence for SEO, accessibility, structured data, and AI readiness"
  homepage "https://github.com/latentmeta/oratos"
  version "0.3.1"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/latentmeta/oratos/releases/download/v#{version}/oratos-v#{version}-macos-aarch64.tar.gz"
      sha256 "81eef79e6c0df5e23482bf9fbfb6de3e3c83e57b98950053c5b3164ac7b3c43a"
    end
    on_intel do
      url "https://github.com/latentmeta/oratos/releases/download/v#{version}/oratos-v#{version}-macos-x86_64.tar.gz"
      sha256 "a6117472500d2be4cf594466d065d3aa3cb2c1f43af5fdc719c1062f77aee274"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/latentmeta/oratos/releases/download/v#{version}/oratos-v#{version}-linux-aarch64.tar.gz"
      sha256 "28016b4807f97dac586f49e270c74fb547582c5dd48996b7a921bb179b0040cf"
    end
    on_intel do
      url "https://github.com/latentmeta/oratos/releases/download/v#{version}/oratos-v#{version}-linux-x86_64.tar.gz"
      sha256 "bffdee6f9f98bd104d75de8faefd2fdc6315dfcdb7e5874a43c3f3f602cef734"
    end
  end

  def install
    bin.install "oratos"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/oratos --version")
  end
end
