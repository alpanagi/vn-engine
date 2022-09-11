enum State {
  Start,
  String,
  Symbol,
}

interface NewLine {
  kind: "NewLine";
}
interface Text {
  kind: "Text";
  text: string;
}
interface Symbol {
  kind: "Symbol";
  text: string;
}
export type Token = NewLine | Text | Symbol;

const NewLine = (): Token => ({ kind: "NewLine" });
const Text = (text: string): Token => ({ kind: "Text", text });
const Symbol = (text: string): Token => ({ kind: "Symbol", text });

type ParserState = { state: State; stack: string; tokens: Token[] };

const startState = (parserState: ParserState, ch: string): ParserState => {
  if (ch === '"') {
    return { ...parserState, state: State.String };
  } else if (ch.match(/[\n]/)) {
    return { ...parserState, tokens: [...parserState.tokens, NewLine()] };
  } else if (ch.match(/\s/)) {
    return parserState;
  } else if (ch.match(/[a-zA-Z_]/)) {
    return {
      ...parserState,
      stack: parserState.stack + ch,
      state: State.Symbol,
    };
  } else {
    throw "Tokenizer error";
  }
};

const stringState = (parserState: ParserState, ch: string): ParserState => {
  if (ch == '"') {
    return {
      tokens: [...parserState.tokens, Text(parserState.stack)],
      stack: "",
      state: State.Start,
    };
  } else {
    return { ...parserState, stack: parserState.stack + ch };
  }
};

const symbolState = (parserState: ParserState, ch: string): ParserState => {
  if (ch.match(/[\w.]/)) {
    return { ...parserState, stack: parserState.stack + ch };
  } else if (ch.match(/[\n]/)) {
    return {
      tokens: [...parserState.tokens, Symbol(parserState.stack), NewLine()],
      stack: "",
      state: State.Start,
    };
  } else if (ch.match(/\s/)) {
    return {
      tokens: [...parserState.tokens, Symbol(parserState.stack)],
      stack: "",
      state: State.Start,
    };
  } else {
    throw "Tokenizer error";
  }
};

export const generateTokens = (text: string): Token[] => {
  let parserState: ParserState = {
    state: State.Start,
    stack: "",
    tokens: [],
  };

  for (const ch of text + "\n") {
    switch (parserState.state) {
      case State.Start:
        parserState = startState(parserState, ch);
        break;
      case State.String:
        parserState = stringState(parserState, ch);
        break;
      case State.Symbol:
        parserState = symbolState(parserState, ch);
        break;
    }
  }

  return parserState.tokens;
};
