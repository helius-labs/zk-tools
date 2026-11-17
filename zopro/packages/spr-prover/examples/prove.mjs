import { readFile } from "node:fs/promises";
import { init } from "@heliuslabs/zopro-spr";

const [keyPath, requestPath, threads = "0"] = process.argv.slice(2);
if (!keyPath || !requestPath)
  throw new Error(
    "Usage: npm run example -- <key.zopro> <request.json> [threads]",
  );
const runtime = await init({ threads: Number(threads) });
let prover;
try {
  // Use a key whose checksum you checked against the deployment's key manifest.
  prover = new runtime.SprProver(await readFile(keyPath));
  const result = await prover.prove(await readFile(requestPath, "utf8"));
  console.log(result.proof);
  console.error(`prove: ${result.proveMs} ms; verify: ${result.verifyMs} ms`);
} finally {
  prover?.free();
  await runtime.close();
}
