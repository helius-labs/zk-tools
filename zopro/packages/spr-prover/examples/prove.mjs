import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { init } from "@heliuslabs/zopro-spr";

const [keyPath, inputsPath, threads = "0"] = process.argv.slice(2);
if (!keyPath || !inputsPath)
  throw new Error(
    "Usage: npm run example -- <key.zopro> <inputs.mjs> [threads]",
  );
const runtime = await init({ threads: Number(threads) });
let prover;
try {
  // Use a key whose checksum you checked against the deployment's key manifest.
  prover = new runtime.SprProver(await readFile(keyPath));
  const { default: inputs } = await import(
    pathToFileURL(resolve(inputsPath)).href
  );
  const proof =
    "circuit" in inputs
      ? await prover.prove(inputs)
      : await prover.proveMerge(inputs);
  console.log(proof); // The SDK's { a, b, c } byte arrays.
} finally {
  prover?.free();
  await runtime.close();
}
