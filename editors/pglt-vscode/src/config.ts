import {
  type ConfigurationScope,
  type WorkspaceFolder,
  workspace,
} from "vscode";

/**
 * This function retrieves a setting from the workspace configuration.
 * Settings are looked up under the "pglt" prefix.
 *
 * @param key The key of the setting to retrieve
 */
export const getConfig = <T>(
  key: string,
  options: {
    scope?: ConfigurationScope;
  } = {}
): T | undefined => {
  return workspace.getConfiguration("pglt", options.scope).get<T>(key);
};

/**
 * TODO: Can the "state.activeProject" also refer to a workspace, or just to a workspace-folder?
 */
export const isEnabledForFolder = (folder: WorkspaceFolder): boolean => {
  return !!getConfig<boolean>("enabled", { scope: folder.uri });
};
