import assert from "node:assert";
import * as fs from "node:fs";
import { pipeline } from "node:stream";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { format, promisify } from "node:util";

const CLI_ROOT = resolve(fileURLToPath(import.meta.url), "../..");
const PACKAGES_PGLT_ROOT = resolve(CLI_ROOT, "..");
const PGLT_ROOT = resolve(PACKAGES_PGLT_ROOT, "../..");
const MANIFEST_PATH = resolve(CLI_ROOT, "package.json");

const streamPipeline = promisify(pipeline);

async function downloadSchema(releaseTag, githubToken) {
  const assetUrl = `https://github.com/supabase-community/postgres_lsp/releases/download/${releaseTag}/schema.json`;

  const response = await fetch(assetUrl, {
    headers: {
      Authorization: `token ${githubToken}`,
      Accept: `application/octet-stream`,
    },
  });

  if (!response.ok) {
    throw new Error(`Failed to Fetch Asset from ${assetUrl}`);
  }

  // download to root.
  const fileStream = fs.createWriteStream(resolve(PGLT_ROOT, "schema.json"));

  await streamPipeline(response.body, fileStream);

  console.log(`Downloaded schema for ${releaseTag}`);
}

async function downloadAsset(platform, arch, os, releaseTag, githubToken) {
  const buildName = getBuildName(platform, arch);

  // https://github.com/supabase-community/postgres_lsp/releases/download/0.1.0/pglt_x86_64_pc-windows-msvc
  // https://github.com/supabase-community/postgres_lsp/releases/download/0.1.0/pglt_x86_64-pc-windows-msvc
  const assetUrl = `https://github.com/supabase-community/postgres_lsp/releases/download/${releaseTag}/${buildName}`;

  const response = await fetch(assetUrl, {
    headers: {
      Authorization: `token ${githubToken}`,
      Accept: `application/octet-stream`,
    },
  });

  if (!response.ok) {
    const error = await response.text();
    throw new Error(`Failed to Fetch Asset from ${assetUrl}. Reason: ${error}`);
  }

  // just download to root.
  const fileStream = fs.createWriteStream(getBinarySource(platform, arch, os));

  await streamPipeline(response.body, fileStream);

  console.log(`Downloaded asset for ${buildName} (v${releaseTag})`);
}

const rootManifest = JSON.parse(
  fs.readFileSync(MANIFEST_PATH).toString("utf-8")
);

function getBinaryExt(os) {
  return os === "windows" ? ".exe" : "";
}

function getBinarySource(platform, arch, os) {
  const ext = getBinaryExt(os);
  return resolve(PGLT_ROOT, `${getBuildName(platform, arch)}${ext}`);
}

function getBuildName(platform, arch) {
  return `pglt_${arch}-${platform}`;
}

function getPackageName(platform, arch) {
  // trim the "unknown" from linux
  const name = platform.split("-").slice(-2).join("-");
  return `@pglt/cli-${arch}-${name}`;
}

function getOs(platform) {
  return platform.split("-").find((_, idx) => idx === 1);
}

function copyBinaryToNativePackage(platform, arch, os) {
  const buildName = getBuildName(platform, arch);
  const packageRoot = resolve(PACKAGES_PGLT_ROOT, buildName);
  const packageName = getPackageName(platform, arch);

  // "unknow-linux-gnu", "apple-darwin" – take linux, apple, windows

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
  const binarySource = getBinarySource(platform, arch, os);
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

function copySchemaToNativePackage(platform, arch) {
  const buildName = getBuildName(platform, arch);
  const packageRoot = resolve(PACKAGES_PGLT_ROOT, buildName);

  const schemaSrc = resolve(packageRoot, `schema.json`);
  const schemaTarget = resolve(packageRoot, `schema.json`);

  if (!fs.existsSync(schemaSrc)) {
    console.error(`Schema.json not found at: ${schemaSrc}`);
    process.exit(1);
  }

  console.info(`Copying schema.json`);
  fs.copyFileSync(schemaSrc, schemaTarget);
  fs.chmodSync(schemaTarget, 0o666);
}

(async function main() {
  const githubToken = process.env.GITHUB_TOKEN;
  const releaseTag = process.env.RELEASE_TAG;

  assert(githubToken, "GITHUB_TOKEN not defined!");
  assert(releaseTag, "RELEASE_TAG not defined!");

  await downloadSchema(releaseTag, githubToken);

  const PLATFORMS = ["pc-windows-msvc", "apple-darwin", "unknown-linux-gnu"];
  const ARCHITECTURES = ["x86_64", "aarch64"];

  for (const platform of PLATFORMS) {
    const os = getOs(platform);

    for (const arch of ARCHITECTURES) {
      await downloadAsset(platform, arch, os, releaseTag, githubToken);
      copyBinaryToNativePackage(platform, arch, os);
      copySchemaToNativePackage(platform, arch);
    }
  }

  process.exit(0);
})();
