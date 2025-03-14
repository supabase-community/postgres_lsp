import { ExtensionContext } from "vscode";

type Project = any;
type Session = any;

export type State = {
  state:
    | "initializing"
    | "starting"
    | "restarting"
    | "started"
    | "stopping"
    | "stopped"
    | "error";

  activeProject?: Project;
  activeSession?: Session;
  context: ExtensionContext;
  hidden: boolean;
};

const _state: State = {
  state: "initializing",
  hidden: false,
} as State;

export const state = new Proxy(_state, {
  get(target, prop, receiver) {
    return Reflect.get(target, prop, receiver);
  },
  set(target, prop, value, receiver) {
    if (Reflect.set(target, prop, value, receiver)) {
      // udpate something
      return true;
    }

    return false;
  },
});
