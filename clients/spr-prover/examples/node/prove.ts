import { readFile } from "node:fs/promises";
import { SprNodeProver, shapeByLabel } from "@heliuslabs/spr-web-prover/node";

const prover = new SprNodeProver({
  proverUrl: "http://localhost:3001", // SDK routing identity; no server needed here.
});

try {
  await prover.ensureKey(shapeByLabel("2x2"));
  const request = await readFile(new URL("../../fixtures/transfer-2x2.json", import.meta.url), "utf8");
  const started = performance.now();
  const result = await prover.proveRequest(request);
  console.log({ backend: result.backend, milliseconds: Math.round(performance.now() - started) });
  console.log(result.proof);
} finally {
  prover.terminate();
}
