import * as vne from "./engine";
import fs from "fs/promises";

fs.readFile("./assets/script.vn", "utf-8").then((script) => {
  let engine = vne.initEngine();
  engine = vne.parseScript(engine, script);
  console.log(engine);
});
