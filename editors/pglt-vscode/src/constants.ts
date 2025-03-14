import { workspace } from "vscode";
import packageJson from "../package.json";

export enum OperatingMode {
  SingleFile = "single_file", // unsupported
  SingleRoot = "single_root",
  MultiRoot = "multi_root",
}

const npmPackageName = "pglt_testrelease-nightly-3";
const binaryNpmPackageName = npmPackageName.replace("_", "-");

/**
 * platform and arch are values injected into the node runtime.
 * We use the values documented on https://nodejs.org.
 */
const PACKAGE_NAMES: Record<string, Record<string, string>> = {
  win32: {
    x64: `${binaryNpmPackageName}-cli-x86_64-windows-msvc`,
    arm64: `${binaryNpmPackageName}-cli-aarch64-windows-msvc`,
  },
  darwin: {
    x64: `${binaryNpmPackageName}-cli-x86_64-apple-darwin`,
    arm64: `${binaryNpmPackageName}-cli-aarch64-apple-darwin`,
  },
  linux: {
    x64: `${binaryNpmPackageName}-cli-x86_64-linux-gnu`,
    arm64: `${binaryNpmPackageName}-cli-aarch64-linux-gnu`,
  },
};

const platformMappings: Record<string, string> = {
  darwin: "apple-darwin",
  linux: "unknown-linux-gnu",
  win32: "pc-windows-msvc",
};

const archMappings: Record<string, string> = {
  arm64: "aarch64",
  x64: "x86_64",
};

const _CONSTANTS = {
  displayName: packageJson.name,

  activationTimestamp: Date.now(),

  platformSpecificBinaryName: (() => {
    return `pglt${process.platform === "win32" ? ".exe" : ""}`;
  })(),

  /**
   * The name under which pglt is published on npm.
   */
  npmPackageName,

  platformSpecificNodePackageName: (() => {
    const platform: string = process.platform;
    const arch: string = process.arch;

    const pkg = PACKAGE_NAMES[platform]?.[arch];

    // TS won't pick up on the possibility of this being undefined
    return pkg as string | undefined;
  })(),

  platformSpecificReleasedAssetName: (() => {
    let assetName = "pglt";

    for (const [nodeArch, rustArch] of Object.entries(archMappings)) {
      if (nodeArch === process.arch) {
        assetName += `_${rustArch}`;
      }
    }

    for (const [nodePlatform, rustPlatform] of Object.entries(
      platformMappings
    )) {
      if (nodePlatform === process.platform) {
        assetName += `-${rustPlatform}`;
      }
    }

    return assetName;
  })(),

  currentMachineSupported: (() => {
    // In future release, we should also check whether the toolchain matches (Linux musl, GNU etc.)
    return !!(platformMappings[process.platform] && archMappings[process.arch]);
  })(),

  operatingMode: ((): OperatingMode => {
    if (workspace.workspaceFolders === undefined) {
      return OperatingMode.SingleFile;
    }

    if (workspace.workspaceFolders.length > 1) {
      return OperatingMode.MultiRoot;
    }

    return OperatingMode.SingleRoot;
  })(),

  platformIdentifier: (() => {
    return `${process.platform}-${process.arch}`;
  })(),

  globalStorageFolderForBinary: "global-bin",
  globalStorageFolderTmp: "tmp-bin",
};

export const CONSTANTS: typeof _CONSTANTS = new Proxy(_CONSTANTS, {
  get(target, prop, receiver) {
    return Reflect.get(target, prop, receiver);
  },
  set: () => true,
  deleteProperty: () => true,
});
