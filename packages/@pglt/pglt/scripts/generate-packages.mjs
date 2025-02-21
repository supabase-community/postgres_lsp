import assert from "node:assert";
import * as fs from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { format } from "node:util";

const CLI_ROOT = resolve(fileURLToPath(import.meta.url), "../..");
const PACKAGES_PGLT_ROOT = resolve(CLI_ROOT, "..");
const PGLT_ROOT = resolve(PACKAGES_PGLT_ROOT, "../..");
const MANIFEST_PATH = resolve(CLI_ROOT, "package.json");

async function downloadAsset(platform, os, arch, releaseTag, githubToken) {
  const buildName = getBuildName(platform, arch);
  const assetUrl = `https://github.com/supabase-community/postgres_lsp/releases/download/${releaseTag}/${buildName}`;

  const response = await fetch(assetUrl, {
    headers: {
      Authorization: `token ${githubToken}`,
      Accept: `application/octet-stream`,
    },
  });

  if (!response.ok) {
    throw new Error(`Failed to Fetch Asset from ${assetUrl}`);
  }

  const fileStream = fs.createWriteStream(getBinarySource(os, platform, arch));

  await new Promise((res, rej) => {
    response.body.pipeTo(fileStream);
    fileStream.on("error", rej);
    fileStream.on("finish", res);
  });

  console.log(`Downloaded asset for ${buildName} (v${releaseTag})`);
}

const rootManifest = JSON.parse(
  fs.readFileSync(MANIFEST_PATH).toString("utf-8")
);

function getBinaryExt(os) {
  return os === "windows" ? ".exe" : "";
}

function getBinarySource(os, platform, arch) {
  const ext = getBinaryExt(os);
  return resolve(PGLT_ROOT, `${getBuildName(platform, arch)}${ext}`);
}

function getBuildName(platform, arch) {
  return format(`pglt_${arch}_${platform}`, arch);
}

function getPackageName(platform, arch) {
  // trim the "unknwown" from linux
  const name = platform.split("-").slice(-2).join("-");
  return format(`@pglt/cli_${name}`, arch);
}

function copyBinaryToNativePackage(platform, arch) {
  const buildName = getBuildName(platform, arch);
  const packageRoot = resolve(PACKAGES_PGLT_ROOT, buildName);
  const packageName = getPackageName(platform, arch);

  // "unknow-linux-gnu", "apple-darwin" – take linux, apple, windows
  const os = platform.split("-").find((_, idx) => idx === 1);

  // Update the package.json manifest
  const { version, license, repository, engines } = rootManifest;

  const manifest = JSON.stringify(
    {
      name: packageName,
      version,
      license,
      repository,
      engines,
      os: [os],
      cpu: [arch],
      libc: (() => {
        switch (os) {
          case "linux":
            return "gnu";
          case "windows":
            return "msvc";
          default:
            return undefined;
        }
      })(),
    },
    null,
    2
  );

  const manifestPath = resolve(packageRoot, "package.json");
  console.info(`Update manifest ${manifestPath}`);
  fs.writeFileSync(manifestPath, manifest);

  // Copy the CLI binary
  const binarySource = getBinarySource(os, platform, arch);
  const ext = getBinaryExt(os);
  const binaryTarget = resolve(packageRoot, `pglt${ext}`);

  if (!fs.existsSync(binarySource)) {
    console.error(
      `Source for binary for ${buildName} not found at: ${binarySource}`
    );
    process.exit(1);
  }

  console.info(`Copy binary ${binaryTarget}`);
  fs.copyFileSync(binarySource, binaryTarget);
  fs.chmodSync(binaryTarget, 0o755);
}

(async function main() {
  const githubToken = process.env.GITHUB_TOKEN;
  const assetsUrl = process.env.ASSETS_URL;
  const releaseTag = process.env.RELEASE_TAG;

  assert(githubToken, "GITHUB_TOKEN not defined!");
  assert(assetsUrl, "ASSETS_URL not defined!");
  assert(releaseTag, "RELEASE_TAG not defined!");

  const PLATFORMS = ["windows-msvc", "apple-darwin", "unknown-linux-gnu"];
  const ARCHITECTURES = ["x86_64", "aarch64"];

  for (const platform of PLATFORMS) {
    for (const arch of ARCHITECTURES) {
      await downloadAsset(platform, os, arch, releaseTag, githubToken);
      copyBinaryToNativePackage(platform, arch);
    }
  }

  process.exit(0);
})();
