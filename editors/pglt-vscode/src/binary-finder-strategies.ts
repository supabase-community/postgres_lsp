import { Uri, window } from "vscode";
import { logger } from "./logger";
import { delimiter, dirname, join } from "node:path";
import { CONSTANTS } from "./constants";
import { fileExists } from "./utils";
import { createRequire } from "node:module";
import { getConfig } from "./config";
import { downloadPglt, getDownloadedVersion } from "./downloader";

export interface BinaryFindStrategy {
  name: string;
  find(path: Uri): Promise<Uri | null>;
}

/**
 * The user can specify a PGLT binary in the VSCode settings.
 *
 * This can be done in two ways:
 *
 * 1. A static string that points to a binary. The extension will try to retrieve the binary from there.
 *
 * 2. An object with OS & arch combinations as keys and binary paths as values.
 * The extension will try to retrieve the binary from the key matching the current OS and arch.
 *
 * Config Example:
 * ```json
 * {
 *   "pglt.bin": {
 *   	"linux-x64": "/path/to/pglt",
 *    "darwin-arm64": "/path/to/pglt",
 *    "win32-x64": "/path/to/pglt.exe"
 *   }
 * }
 */
export const vsCodeSettingsStrategy: BinaryFindStrategy = {
  name: "VSCode Settings Strategy",
  async find(path: Uri) {
    logger.debug("Trying to find PGLT binary via VSCode Settings");

    type BinSetting = string | Record<string, string> | undefined;
    let binSetting: BinSetting = getConfig("bin", {
      scope: path,
    });

    if (!binSetting) {
      logger.debug("Binary path not set in VSCode Settings");
      return null;
    }

    if (typeof binSetting === "object") {
      logger.debug(
        "Binary Setting is an object, extracting relevant platform",
        { binSetting }
      );

      const relevantSetting = binSetting[CONSTANTS.platformIdentifier];
      if (relevantSetting) {
        logger.debug(
          "Found matching setting for platform in VSCode Settings, assigning as string",
          {
            setting: relevantSetting,
            platformIdentifier: CONSTANTS.platformIdentifier,
          }
        );
        binSetting = relevantSetting;
      }
    }

    if (typeof binSetting === "string") {
      logger.debug("Binary Setting is a string", { binSetting });

      const resolvedPath = binSetting.startsWith(".")
        ? Uri.joinPath(path, binSetting).toString()
        : binSetting;

      logger.debug("Looking for binary at path", { resolvedPath });

      const pglt = Uri.file(resolvedPath);

      if (await fileExists(pglt)) {
        return pglt;
      }
    }

    logger.debug("No PGLT binary found in VSCode settings.");

    return null;
  },
};

/**
 * Task:
 * Search the binary in node modules.
 * Search for the sub-packages that the binary tries to use with npm.
 * Use node's `createRequire` – what's that?
 * Resolve the *main* package.json – the one used by @pglt/pglt.
 * In those node_modules, you should see the installed optional dependency.
 */
export const nodeModulesStrategy: BinaryFindStrategy = {
  name: "Node Modules Strategy",
  async find(path: Uri) {
    logger.debug("Trying to find PGLT binary in Node Modules");

    if (!path) {
      logger.debug("No local path, skipping.");
      return null;
    }

    const pgltPackageNameJson = `${CONSTANTS.npmPackageName}/package.json`;

    logger.info(`Searching for node_modules package`, { pgltPackageNameJson });

    let requirePgltPackage: NodeJS.Require;
    try {
      /**
       * Create a scoped require function that can require modules from the
       * package installed via npm.
       *
       * We're essentially searching for the installed package in the current dir, and requiring from its node_modules.
       * `package.json` serves as a target to resolve the root of the package.
       */
      requirePgltPackage = createRequire(
        require.resolve(pgltPackageNameJson, {
          paths: [path.fsPath], // note: global ~/.node_modules is always searched
        })
      );
    } catch (err: unknown) {
      if (
        err instanceof Error &&
        err.message.toLowerCase().includes("cannot find module")
      ) {
        logger.debug(`User does not use node_modules`);
        return null;
      } else {
        throw err;
      }
    }

    logger.debug("Created require function!");

    const packageName = CONSTANTS.platformSpecificNodePackageName;
    if (packageName === undefined) {
      logger.debug(
        `No package for current platform available in node_modules`,
        {
          os: process.platform,
          arch: process.arch,
        }
      );
      return null;
    }

    logger.debug(`Resolving bin package at nested ${packageName}/package.json`);

    const binPackage = dirname(
      requirePgltPackage.resolve(`${packageName}/package.json`)
    );

    logger.debug(`Resolved binpackage`, { binPackage });

    const pgltPath = join(binPackage, CONSTANTS.platformSpecificBinaryName);
    const pglt = Uri.file(pgltPath);

    if (await fileExists(pglt)) {
      return pglt;
    }

    logger.debug(`Unable to find PGLT in path ${pgltPath}`);

    return null;
  },
};

export const yarnPnpStrategy: BinaryFindStrategy = {
  name: "Yarn PnP Strategy",
  async find(path: Uri) {
    logger.debug("Trying to find PGLT binary in Yarn Plug'n'Play");

    if (!path) {
      logger.debug("No local path, skipping.");
      return null;
    }

    for (const ext of ["cjs", "js"]) {
      const pnpFile = Uri.joinPath(path, `.pnp.${ext}`);

      if (!(await fileExists(pnpFile))) {
        logger.debug(`Couldn't find Plug'n'Play file with ext '${ext}'`);
        continue;
      }

      /**
       * Load the pnp file, so we can use the exported
       * `resolveRequest` method.
       *
       * `resolveRequest(request, issuer)` takes a request for a dependency and an issuer
       * that depends on said dependency.
       */
      const yarnPnpApi = require(pnpFile.fsPath);

      /**
       * Issue a request to the PGLT package.json from the current dir.
       */
      const pgltPackage = yarnPnpApi.resolveRequest(
        `${CONSTANTS.npmPackageName}/package.json`,
        path.fsPath
      );

      if (!pgltPackage) {
        logger.debug("Unable to find PGLT package via Yarn Plug'n'Play API");
        continue;
      }

      const packageName = CONSTANTS.platformSpecificNodePackageName;
      if (packageName === undefined) {
        logger.debug(`No package for current platform available in yarn pnp`, {
          os: process.platform,
          arch: process.arch,
        });
        return null;
      }

      /**
       * Return URI to the platform-specific binary that the found main package depends on.
       */
      return Uri.file(
        yarnPnpApi.resolveRequest(
          `${packageName}/${CONSTANTS.platformSpecificBinaryName}`,
          pgltPackage
        )
      );
    }

    logger.debug("Couldn't find PGLT binary via Yarn Plug'n'Play");

    return null;
  },
};

export const pathEnvironmentVariableStrategy: BinaryFindStrategy = {
  name: "PATH Env Var Strategy",
  async find() {
    const pathEnv = process.env.PATH;

    logger.debug("Trying to find PGLT binary in PATH env var");

    if (!pathEnv) {
      logger.debug("Path env var not found");
      return null;
    }

    for (const dir of pathEnv.split(delimiter)) {
      logger.debug(`Checking ${dir}`);

      const pglt = Uri.joinPath(
        Uri.file(dir),
        CONSTANTS.platformSpecificBinaryName
      );

      if (await fileExists(pglt)) {
        return pglt;
      }
    }

    logger.debug("Couldn't determine binary in PATH env var");

    return null;
  },
};

export const downloadPgltStrategy: BinaryFindStrategy = {
  name: "Download PGLT Strategy",
  async find() {
    logger.debug(`Trying to find downloaded PGLT binary`);

    const downloadedVersion = await getDownloadedVersion();

    if (downloadedVersion) {
      logger.info(
        `Using previously downloaded version ${downloadedVersion.version} at ${downloadedVersion.binPath.fsPath}`
      );

      return downloadedVersion.binPath;
    }

    const proceed =
      (await window.showInformationMessage(
        "You've opened a supported file outside of a PGLT project, and no installed PGLT binary could not be found on your system. Would you like to download and install PGLT?",
        "Download and install",
        "No"
      )) === "Download and install";

    if (!proceed) {
      logger.debug(`Decided not to download binary, aborting`);
      return null;
    }

    return await downloadPglt();
  },
};
