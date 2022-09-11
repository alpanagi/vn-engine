import { Command, parseText } from "./interpreter";

interface Screen {
  readonly background: string;
  readonly text: string;
}

interface VNEngine {
  commands: Command[];
  history: { command: Command; screen: Screen }[];
}

export const currentScreen = (engine: VNEngine): Screen | undefined =>
  engine.history.at(-1)?.screen;

export const forward = (engine: VNEngine): VNEngine => {
  const screen = currentScreen(engine) ?? { background: "", text: "" };
  const command = engine.commands.at(0);

  if (command !== undefined) {
    return {
      commands: engine.commands.slice(1),
      history: [
        ...engine.history,
        { command, screen: runCommand(command, screen) },
      ],
    };
  } else {
    return engine;
  }
};

const runCommand = (command: Command, screen: Screen): Screen => {
  if (command.kind === "DisplayBackground") {
    return { ...screen, background: command.path };
  } else if (command.kind === "Text") {
    return { ...screen, text: command.text };
  } else {
    throw "Unknown command";
  }
};

export const backwards = (engine: VNEngine): VNEngine => {
  const historyEvent = engine.history.at(-1);
  if (historyEvent !== undefined) {
    return {
      commands: [historyEvent.command, ...engine.commands],
      history: engine.history.slice(0, engine.history.length - 1),
    };
  } else {
    return engine;
  }
};

export const initEngine = (): VNEngine => ({ commands: [], history: [] });

export const parseScript = (engine: VNEngine, script: string): VNEngine => {
  const commands = parseText(script);
  return { ...engine, commands: [...engine.commands, ...commands] };
};
