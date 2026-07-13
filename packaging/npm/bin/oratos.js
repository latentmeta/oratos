#!/usr/bin/env node
"use strict";

const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const binDir = path.join(__dirname, "..", "vendor");
const binName = process.platform === "win32" ? "oratos.exe" : "oratos";
const binPath = path.join(binDir, binName);

if (!fs.existsSync(binPath)) {
  console.error(
    "oratos binary not found. Re-run npm install, or install from https://github.com/latentmeta/oratos/releases"
  );
  process.exit(1);
}

const result = spawnSync(binPath, process.argv.slice(2), {
  stdio: "inherit",
});
process.exit(result.status === null ? 1 : result.status);
