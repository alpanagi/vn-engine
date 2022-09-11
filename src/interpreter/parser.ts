import { Token } from "./tokenizer";

interface Text {
  kind: "Text";
  speaker: string;
  text: string;
}
interface DisplayBackground {
  kind: "DisplayBackground";
  path: string;
}
export type Command = Text | DisplayBackground;

const Text = (speaker: string, text: string): Text => ({
  kind: "Text",
  speaker,
  text,
});
const DisplayBackground = (path: string): DisplayBackground => ({
  kind: "DisplayBackground",
  path,
});

export const generateCommands = (tokens: Token[]): Command[] => {
  let commands: Command[] = [];
  let queue = tokens;

  while (queue.length > 0) {
    if (
      queue[0]?.kind === "Text" &&
      queue[1]?.kind === "Text" &&
      queue[2]?.kind === "NewLine"
    ) {
      commands.push(Text(queue[0].text, queue[1].text));
      queue = queue.slice(3);
    } else if (queue[0]?.kind === "Text" && queue[1]?.kind === "NewLine") {
      commands.push(Text("", queue[0].text));
      queue = queue.slice(2);
    } else if (queue[0]?.kind === "Symbol" && queue[1]?.kind === "Symbol") {
      if (queue[0].text === "background") {
        commands.push(DisplayBackground(queue[1].text));
      } else {
        throw "Syntax error";
      }
      queue = queue.slice(2);
    } else if (queue[0]?.kind === "NewLine") {
      queue = queue.slice(1);
    } else {
      throw "Syntax error";
    }
  }

  return commands;
};
