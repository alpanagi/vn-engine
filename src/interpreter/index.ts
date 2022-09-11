import { Command, generateCommands } from "./parser";
import { generateTokens } from "./tokenizer";

export { Command } from "./parser";
export const parseText = (text: string): Command[] => {
  const tokens = generateTokens(text);
  const commands = generateCommands(tokens);
  return commands;
};
