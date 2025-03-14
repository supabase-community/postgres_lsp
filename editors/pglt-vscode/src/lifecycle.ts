import { logger } from "./logger";
import { createActiveSession, destroySession } from "./session";
import { state } from "./state";

/**
 * Starts the PGLT extension
 */
export const start = async () => {
  state.state = "starting";
  await doStart();
  state.state = "started";
  logger.info("PGLT extension started");
};

/**
 * Stops the PGLT extension
 */
export const stop = async () => {
  state.state = "stopping";
  await doStop();
  state.state = "stopped";
  logger.info("PGLT extension stopped");
};

export const restart = async () => {
  if (state.state === "restarting") {
    // If we are already restarting, we can skip the restart
    return;
  }

  state.state = "restarting";
  await doStop();
  await doStart();
  state.state = "started";
  logger.info("PGLT extension restarted");
};

const doStart = async () => {
  try {
    await createActiveSession();
  } catch (e: unknown) {
    if (e instanceof Error) {
      logger.error(e.message);
    }
    logger.error("Failed to start PGLT extension", { error: e });
    state.state = "error";
  }
};

const doStop = async () => {
  // If we end up here following a configuration change, we need to wait
  // for the notification to be processed before we can stop the LSP session,
  // otherwise we will get an error. This is a workaround for a race condition
  // that occurs when the configuration change notification is sent while the
  // LSP session is already stopped.
  await new Promise((resolve) => setTimeout(resolve, 1000));

  if (state.activeSession) {
    destroySession(state.activeSession);
  }
};
