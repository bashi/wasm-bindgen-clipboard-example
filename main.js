import init, { read_clipboard } from "./pkg/wasm_bindgen_clipboard.js";

async function run() {
  await init();
  console.log(await read_clipboard());
}

run();
