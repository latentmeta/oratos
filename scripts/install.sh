#!/usr/bin/env bash
# Install Oratos from GitHub Releases (no Rust toolchain required).
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/latentmeta/oratos/main/scripts/install.sh | sh
#   ORATOS_VERSION=v0.3.0 sh scripts/install.sh
#   ORATOS_INSTALL_DIR=/usr/local/bin sh scripts/install.sh
set -euo pipefail

REPO="${ORATOS_REPO:-latentmeta/oratos}"
INSTALL_DIR="${ORATOS_INSTALL_DIR:-${HOME}/.local/bin}"
VERSION="${ORATOS_VERSION:-}"

detect_platform() {
  local os arch
  os="$(uname -s | tr '[:upper:]' '[:lower:]')"
  arch="$(uname -m)"

  case "${os}" in
    linux) os="linux" ;;
    darwin) os="macos" ;;
    mingw*|msys*|cygwin*) os="windows" ;;
    *)
      echo "error: unsupported OS: $(uname -s)" >&2
      exit 1
      ;;
  esac

  case "${arch}" in
    x86_64|amd64) arch="x86_64" ;;
    aarch64|arm64) arch="aarch64" ;;
    *)
      echo "error: unsupported architecture: ${arch}" >&2
      exit 1
      ;;
  esac

  echo "${os}-${arch}"
}

latest_tag() {
  curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' \
    | head -n1
}

download() {
  local url="$1" dest="$2"
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "${url}" -o "${dest}"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "${dest}" "${url}"
  else
    echo "error: need curl or wget" >&2
    exit 1
  fi
}

verify_checksum() {
  local archive="$1" sums_file="$2"
  local expected actual base
  base="$(basename "${archive}")"
  expected="$(awk -v f="${base}" '$2 == f { print $1; exit }' "${sums_file}")"
  if [[ -z "${expected}" ]]; then
    echo "warning: no checksum entry for ${base}; skipping verification" >&2
    return 0
  fi
  if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "${archive}" | awk '{print $1}')"
  elif command -v shasum >/dev/null 2>&1; then
    actual="$(shasum -a 256 "${archive}" | awk '{print $1}')"
  else
    echo "warning: no sha256 tool found; skipping verification" >&2
    return 0
  fi
  if [[ "${actual}" != "${expected}" ]]; then
    echo "error: checksum mismatch for ${base}" >&2
    echo "  expected: ${expected}" >&2
    echo "  actual:   ${actual}" >&2
    exit 1
  fi
}

main() {
  local platform archive_ext archive_name tmp bin_name
  platform="$(detect_platform)"

  if [[ -z "${VERSION}" ]]; then
    VERSION="$(latest_tag)"
  fi
  if [[ -z "${VERSION}" ]]; then
    echo "error: could not determine release version" >&2
    exit 1
  fi
  # Accept 0.3.0 or v0.3.0
  case "${VERSION}" in
    v*) ;;
    *) VERSION="v${VERSION}" ;;
  esac

  if [[ "${platform}" == windows-* ]]; then
    archive_ext="zip"
    bin_name="oratos.exe"
  else
    archive_ext="tar.gz"
    bin_name="oratos"
  fi

  archive_name="oratos-${VERSION}-${platform}.${archive_ext}"
  tmp="$(mktemp -d)"
  trap 'rm -rf "${tmp}"' EXIT

  echo "Installing Oratos ${VERSION} (${platform}) → ${INSTALL_DIR}"
  download "https://github.com/${REPO}/releases/download/${VERSION}/${archive_name}" \
    "${tmp}/${archive_name}"
  download "https://github.com/${REPO}/releases/download/${VERSION}/SHA256SUMS" \
    "${tmp}/SHA256SUMS" || true

  if [[ -f "${tmp}/SHA256SUMS" ]]; then
    verify_checksum "${tmp}/${archive_name}" "${tmp}/SHA256SUMS"
  fi

  mkdir -p "${INSTALL_DIR}"
  case "${archive_ext}" in
    tar.gz)
      tar -xzf "${tmp}/${archive_name}" -C "${tmp}"
      ;;
    zip)
      if command -v unzip >/dev/null 2>&1; then
        unzip -qo "${tmp}/${archive_name}" -d "${tmp}"
      else
        echo "error: unzip is required to install Windows archives" >&2
        exit 1
      fi
      ;;
  esac

  install -m 0755 "${tmp}/${bin_name}" "${INSTALL_DIR}/${bin_name}"
  echo "Installed ${INSTALL_DIR}/${bin_name}"
  if ! command -v oratos >/dev/null 2>&1; then
    echo "Add ${INSTALL_DIR} to your PATH, then run: oratos --version"
  else
    oratos --version || true
  fi
}

main "$@"
