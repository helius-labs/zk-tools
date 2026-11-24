import assert from "node:assert/strict";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createLocalProofService } from "@heliuslabs/zopro-spr";
import { sdkInputs } from "./sdk-inputs.js";
import {
  productionArtifact,
  productionKey,
  productionKeyBytes,
} from "./service-key.js";

const directory = await mkdtemp(join(tmpdir(), "zopro-service-proof-"));
let service;
const originalFetch = globalThis.fetch;
try {
  await writeFile(
    join(directory, productionArtifact.file),
    await productionKeyBytes(),
  );
  service = await createLocalProofService({
    threads: Number(process.argv[2]),
    keyDirectory: directory,
  });
  await service.preload(productionKey);
  await rm(join(directory, productionArtifact.file));
  globalThis.fetch = () => {
    throw new Error("A preloaded proof must not fetch another key");
  };
  const inputs = sdkInputs(
    await readFile(
      new URL("../../../tools/spr/fixtures/transfer-1x2.json", import.meta.url),
      "utf8",
    ),
  );
  const proof = await service.prove(inputs);
  assert.deepEqual(
    [proof.a.length, proof.b.length, proof.c.length],
    [64, 128, 64],
  );
  assert.equal("proofCount" in service, false);
  console.log(
    JSON.stringify({
      threads: Number(process.argv[2]),
      preloaded: true,
      proofBytes: [64, 128, 64],
    }),
  );
} finally {
  globalThis.fetch = originalFetch;
  await service?.close();
  await rm(directory, { recursive: true, force: true });
}
