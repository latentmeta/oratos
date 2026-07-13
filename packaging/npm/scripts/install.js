#!/usr/bin/env node
"use strict";

const fs = require("node:fs");
const https = require("node:https");
const path = require("node:path");
const { execFileSync } = require("node:child_process");
const { createWriteStream } = require("node:fs");
const { pipeline } = require("node:stream/promises");

const pkg = require("../package.json");
const VERSION = process.env.ORATOS_VERSION || `v${pkg.version}`;
const REPO = "latentmeta/oratos";

function platformAsset() {
  const platform = process.platform;
  const arch = process.arch;
  let os;
  let cpu;
  if (platform === "darwin") os = "macos";
  else if (platform === "linux") os = "linux";
  else if (platform === "win32") os = "windows";
  else throw new Error(`unsupported platform: ${platform}`);

  if (arch === "x64") cpu = "x86_64";
  else if (arch === "arm64") cpu = "aarch64";
  else throw new Error(`unsupported architecture: ${arch}`);

  const ext = platform === "win32" ? "zip" : "tar.gz";
  const bin = platform === "win32" ? "oratos.exe" : "oratos";
  return {
    name: `oratos-${VERSION}-${os}-${cpu}.${ext}`,
    ext,
    bin,
  };
}

function get(url, redirects = 0) {
  return new Promise((resolve, reject) => {
    https
      .get(url, { headers: { "User-Agent": "oratos-npm-installer" } }, (res) => {
        if (
          res.statusCode >= 300 &&
          res.statusCode < 400 &&
          res.headers.location &&
          redirects < 5
        ) {
          res.resume();
          resolve(get(res.headers.location, redirects + 1));
          return;
        }
        if (res.statusCode !== 200) {
          reject(new Error(`GET ${url} → ${res.statusCode}`));
          res.resume();
          return;
        }
        resolve(res);
      })
      .on("error", reject);
  });
}

async function main() {
  const asset = platformAsset();
  const url = `https://github.com/${REPO}/releases/download/${VERSION}/${asset.name}`;
  const vendor = path.join(__dirname, "..", "vendor");
  fs.mkdirSync(vendor, { recursive: true });
  const archivePath = path.join(vendor, asset.name);

  console.log(`Downloading ${url}`);
  const res = await get(url);
  await pipeline(res, createWriteStream(archivePath));

  if (asset.ext === "tar.gz") {
    execFileSync("tar", ["-xzf", archivePath, "-C", vendor], { stdio: "inherit" });
  } else {
    // Prefer unzip if available; Node has no built-in unzip without deps.
    try {
      execFileSync("unzip", ["-qo", archivePath, "-d", vendor], {
        stdio: "inherit",
      });
    } catch {
      throw new Error(
        "unzip is required to extract Windows Oratos archives during npm install"
      );
    }
  }

  const binPath = path.join(vendor, asset.bin);
  if (!fs.existsSync(binPath)) {
    throw new Error(`expected binary missing after extract: ${binPath}`);
  }
  try {
    fs.chmodSync(binPath, 0o755);
  } catch {
    /* windows */
  }
  fs.unlinkSync(archivePath);
  console.log(`Installed ${binPath}`);
}

main().catch((err) => {
  console.error(`oratos postinstall failed: ${err.message}`);
  console.error(
    "Install a prebuilt binary from https://github.com/latentmeta/oratos/releases or use scripts/install.sh"
  );
  process.exit(1);
});
