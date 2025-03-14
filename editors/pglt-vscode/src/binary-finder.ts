import { RelativePattern, Uri, workspace } from "vscode";
import {
  BinaryFindStrategy,
  downloadPgltStrategy,
  nodeModulesStrategy,
  pathEnvironmentVariableStrategy,
  vsCodeSettingsStrategy,
  yarnPnpStrategy,
} from "./binary-finder-strategies";
import { logger } from "./logger";

type Strategy = {
  strategy: BinaryFindStrategy;
  onSuccess: (u: Uri) => void;
  condition?: (path?: Uri) => Promise<boolean>;
};

const LOCAL_STRATEGIES: Strategy[] = [
  {
    strategy: vsCodeSettingsStrategy,
    onSuccess: (uri) =>
      logger.debug(`Found Binary in VSCode Settings (pglt.lsp.bin)`, {
        path: uri.fsPath,
      }),
  },
  {
    strategy: nodeModulesStrategy,
    onSuccess: (uri) =>
      logger.debug(`Found Binary in Node Modules`, {
        path: uri.fsPath,
      }),
  },
  {
    strategy: yarnPnpStrategy,
    onSuccess: (uri) =>
      logger.debug(`Found Binary in Yarn PnP`, {
        path: uri.fsPath,
      }),
  },
  {
    strategy: pathEnvironmentVariableStrategy,
    onSuccess: (uri) =>
      logger.debug(`Found Binary in PATH Environment Variable`, {
        path: uri.fsPath,
      }),
  },
  {
    strategy: downloadPgltStrategy,
    onSuccess: (uri) =>
      logger.debug(`Found downloaded binary`, {
        path: uri.fsPath,
      }),

    /**
     * We don't want to encourage users downloading the binary if they
     * could also install it via `npm` (or other Node package managers).
     */
    condition: async (path) =>
      !path || // `path` should never be falsy in a local strategy
      workspace
        .findFiles(new RelativePattern(path, "**/package.json"))
        .then((rs) => rs.length === 0),
  },
];

export class BinaryFinder {
  static async find(path: Uri) {
    const binary = await this.attemptFind(LOCAL_STRATEGIES, path);

    if (!binary) {
      logger.debug("Unable to find binary locally.");
    }

    return binary;
  }

  private static async attemptFind(strategies: Strategy[], path: Uri) {
    for (const { strategy, onSuccess, condition } of strategies) {
      if (condition && !(await condition(path))) {
        continue;
      }

      try {
        const binary = await strategy.find(path);
        if (binary) {
          onSuccess(binary);
          return { bin: binary };
        } else {
          logger.info(`Binary not found with strategy`, {
            strategy: strategy.name,
          });
        }
      } catch (err: unknown) {
        logger.error(`${strategy.name} returned an error`, { err });
        continue;
      }
    }
  }
}
