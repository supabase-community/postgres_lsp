import { LogLevel, window } from "vscode";
import { CONSTANTS } from "./constants";

type LogArguments = Record<string, unknown>;

/**
 * Messages logged to this logger will be displayed in the `PGLT` output
 * channel in the Output panel. This logger respects the user's settings for
 * logging verbosity, so only messages with the appropriate log level will be
 * displayed.
 */
class Logger {
  private output = window.createOutputChannel(
    `${CONSTANTS.displayName} (${CONSTANTS.activationTimestamp})`,
    {
      log: true,
    }
  );

  private log(
    message: string,
    level: LogLevel = LogLevel.Info,
    args?: LogArguments
  ) {
    if (args) {
      message += `\n\t${Object.entries(args)
        .map(([key, value]) => `${key}=${JSON.stringify(value)}`)
        .join("\n\t")}`;
    }

    switch (level) {
      case LogLevel.Error:
        return this.output.error(message);
      case LogLevel.Warning:
        return this.output.warn(message);
      case LogLevel.Info:
        return this.output.info(message);
      case LogLevel.Debug:
        return this.output.debug(message);
      default:
        return this.output.debug(message);
    }
  }

  public error(message: string, args?: LogArguments) {
    this.log(message, LogLevel.Error, args);
  }
  public warn(message: string, args?: LogArguments) {
    this.log(message, LogLevel.Warning, args);
  }
  public info(message: string, args?: LogArguments) {
    this.log(message, LogLevel.Info, args);
  }
  public debug(message: string, args?: LogArguments) {
    this.log(message, LogLevel.Debug, args);
  }

  /**
   * Clears the logger
   *
   * This function does not actually clear the logger, but rather appends a
   * few newlines to the logger to ensure that the logger so that logs from a
   * previous run are visually separated from the current run. We need to do
   * this because of a bug in VS Code where the output channel is not cleared
   * properly when calling `clear()` on it.
   *
   * @see https://github.com/microsoft/vscode/issues/224516
   */
  clear() {
    this.output.append("\n\n\n\n\n");
  }
}

export const logger = new Logger();
