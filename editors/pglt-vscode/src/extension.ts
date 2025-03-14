import {
  type ConfigurationChangeEvent,
  commands,
  window,
  workspace,
} from "vscode";
import { UserFacingCommands } from "./commands";
import { restart, start, stop } from "./lifecycle";
import { logger } from "./logger";
import { state } from "./state";
import { debounce } from "./utils";
import { updateHidden } from "./status-bar";

/**
 * This function is responsible for booting the PGLT extension. It is called
 * when the extension is activated.
 */
export const createExtension = async () => {
  registerUserFacingCommands();
  await start();
  listenForConfigurationChanges();
  listenForActiveTextEditorChange();
};

/**
 * This function is responsible for shutting down the PGLT extension. It is
 * called when the extension is deactivated and will trigger a cleanup of the
 * extension's state and resources.
 */
export const destroyExtension = async () => {
  await stop();
};

const registerUserFacingCommands = () => {
  state.context.subscriptions.push(
    commands.registerCommand("pglt.start", UserFacingCommands.start),
    commands.registerCommand("pglt.stop", UserFacingCommands.stop),
    commands.registerCommand("pglt.restart", UserFacingCommands.restart),
    commands.registerCommand("pglt.download", UserFacingCommands.download),
    commands.registerCommand("pglt.reset", UserFacingCommands.reset),
    commands.registerCommand(
      "pglt.currentVersion",
      UserFacingCommands.currentVersion
    )
  );

  logger.info("User-facing commands registered");
};

/**
 * This function sets up a listener for configuration changes in the `pglt`
 * namespace. When a configuration change is detected, the extension is
 * restarted to reflect the new configuration.
 */
const listenForConfigurationChanges = () => {
  const debouncedConfigurationChangeHandler = debounce(
    (event: ConfigurationChangeEvent) => {
      if (event.affectsConfiguration("pglt")) {
        logger.info("Configuration change detected.");
        if (!["restarting", "stopping"].includes(state.state)) {
          restart();
        }
      }
    }
  );

  state.context.subscriptions.push(
    workspace.onDidChangeConfiguration(debouncedConfigurationChangeHandler)
  );

  logger.info("Started listening for configuration changes");
};

/**
 * This function listens for changes to the active text editor and updates the
 * active project accordingly. This change is then reflected throughout the
 * extension automatically. Notably, this triggers the status bar to update
 * with the active project.
 */
const listenForActiveTextEditorChange = () => {
  state.context.subscriptions.push(
    window.onDidChangeActiveTextEditor((editor) => {
      updateHidden(editor);
    })
  );

  logger.info("Started listening for active text editor changes");

  updateHidden(window.activeTextEditor);
};
