# Homebrew Formula for Oratos
#
# Publish to latentmeta/homebrew-tap as Formula/oratos.rb
# Update url/sha256 on each release from GitHub Release assets + SHA256SUMS.
class Oratos < Formula
  desc "Website visibility intelligence for SEO, accessibility, structured data, and AI readiness"
  homepage "https://github.com/latentmeta/oratos"
  version "0.3.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/latentmeta/oratos/releases/download/v#{version}/oratos-v#{version}-macos-aarch64.tar.gz"
      # sha256 "REPLACE_AFTER_RELEASE"
    end
    on_intel do
      url "https://github.com/latentmeta/oratos/releases/download/v#{version}/oratos-v#{version}-macos-x86_64.tar.gz"
      # sha256 "REPLACE_AFTER_RELEASE"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/latentmeta/oratos/releases/download/v#{version}/oratos-v#{version}-linux-aarch64.tar.gz"
      # sha256 "REPLACE_AFTER_RELEASE"
    end
    on_intel do
      url "https://github.com/latentmeta/oratos/releases/download/v#{version}/oratos-v#{version}-linux-x86_64.tar.gz"
      # sha256 "REPLACE_AFTER_RELEASE"
    end
  end

  def install
    bin.install "oratos"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/oratos --version")
  end
end
