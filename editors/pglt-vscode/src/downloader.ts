import {
  ProgressLocation,
  QuickPickItem,
  Uri,
  window,
  workspace,
} from "vscode";
import { logger } from "./logger";
import { state } from "./state";
import { CONSTANTS } from "./constants";
import { fileExists } from "./utils";
import { getConfig } from "./config";
import { chmodSync } from "fs";
import { getAllReleases } from "./releases";

export async function downloadPglt(): Promise<Uri | null> {
  logger.debug(`Downloading PGLT`);

  const versionToDownload = await promptVersionToDownload();

  if (!versionToDownload) {
    logger.debug(`No version to download selected, aborting`);
    return null;
  }

  await window.withProgress(
    {
      title: `Downloading PGLT ${versionToDownload.label}`,
      location: ProgressLocation.Notification,
    },
    () => downloadPgltVersion(versionToDownload.label)
  );

  const downloaded = await getDownloadedVersion();

  return downloaded?.binPath ?? null;
}

async function downloadPgltVersion(version: string): Promise<void> {
  const url = `https://github.com/supabase-community/postgres_lsp/releases/download/${version}/${CONSTANTS.platformSpecificReleasedAssetName}`;

  logger.debug(`Attempting to download binary asset from Github`, { url });

  let binary: ArrayBuffer;

  try {
    binary = await fetch(url, {
      headers: {
        Accept: "application/octet-stream",
      },
    })
      .then((r) => r.blob())
      .then((b) => b.arrayBuffer());
  } catch (error: unknown) {
    logger.error(`Failed to download binary`, { error });
    window.showErrorMessage(
      `Failed to download binary version ${version} from ${url}.\n\n${error}`
    );
    return;
  }

  const binPath = getInstalledBinaryPath();

  try {
    await workspace.fs.writeFile(binPath, new Uint8Array(binary));
    chmodSync(binPath.fsPath, 0o755);
    const successMsg = `Downloaded PGLT ${version} to ${binPath.fsPath}`;
    logger.info(successMsg);
    window.showInformationMessage(successMsg);
    state.context.globalState.update("downloadedVersion", version);
  } catch (error) {
    logger.error(`Failed to save downloaded binary`, { error });
    window.showErrorMessage(`Failed to save binary.\n\n${error}`);
    return;
  }
}

export async function getDownloadedVersion(): Promise<{
  version: string;
  binPath: Uri;
} | null> {
  logger.debug(`Getting downloaded version`);

  const version = state.context.globalState.get<string>("downloadedVersion");
  if (!version) {
    logger.debug(`No downloaded version stored in global state context.`);
    return null;
  }

  const binPath = getInstalledBinaryPath();

  if (await fileExists(binPath)) {
    logger.debug(`Found downloaded version and binary`, {
      path: binPath.fsPath,
      version,
    });

    return {
      binPath,
      version,
    };
  }

  logger.info(
    `Downloaded version found in global state context, but binary does not exist.`,
    {
      binPath,
      version,
    }
  );

  return null;
}

async function promptVersionToDownload() {
  logger.debug(`Prompting user to select PGLT version to download`);

  const itemsPromise: Promise<QuickPickItem[]> = new Promise(
    async (resolve) => {
      const downloadedVersion = await getDownloadedVersion()
        .then((it) => it?.version)
        .catch(() => undefined);

      logger.debug(`Retrieved downloaded version`, { downloadedVersion });

      const withPrereleases =
        getConfig<boolean>("allowDownloadPrereleases") ?? false;

      const availableVersions = await getAllReleases({
        withPrereleases,
      }).catch(() => []);

      logger.debug(`Found ${availableVersions.length} downloadable versions`, {
        withPrereleases,
      });

      const items: QuickPickItem[] = availableVersions.map((release, index) => {
        const descriptions = [];

        if (index === 0) {
          descriptions.push("latest");
        }

        if (release.prerelease) {
          descriptions.push("prerelease");
        }

        return {
          label: release.tag_name,
          description: descriptions.join(", "),
          detail:
            downloadedVersion === release.tag_name
              ? "(currently installed)"
              : "",
          alwaysShow: index < 3,
        };
      });

      resolve(items);
    }
  );

  return window.showQuickPick(itemsPromise, {
    title: "Select PGLT version to download",
    placeHolder: "Select PGLT version to download",
  });
}

function getInstalledBinaryPath() {
  return Uri.joinPath(
    state.context.globalStorageUri,
    CONSTANTS.globalStorageFolderForBinary,
    CONSTANTS.platformSpecificBinaryName
  );
}
