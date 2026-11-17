import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { cp, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url));
const consumer = await mkdtemp(join(tmpdir(), "spr-node-consumer-"));
async function command(cmd, args, cwd = consumer, env = process.env) {
  return await new Promise((resolve, reject) => {
    const child = spawn(cmd, args, { cwd, env, stdio: ["ignore", "pipe", "inherit"] });
    const output = [];
    child.stdout.on("data", (chunk) => output.push(chunk));
    child.on("error", reject);
    child.on("close", (code) => code === 0 ? resolve(Buffer.concat(output).toString()) : reject(new Error(`${cmd} exited ${code}`)));
  });
}
try {
  const [packed] = JSON.parse(await command("npm", ["pack", "--json", "--workspace", "@heliuslabs/spr-web-prover", "--pack-destination", consumer], root));
  await writeFile(join(consumer, "package.json"), JSON.stringify({ private: true, type: "module" }));
  await command("npm", ["install", "--ignore-scripts", "--no-audit", "--no-fund", join(consumer, packed.filename), "typescript@^7.0.2", "@types/node@^24"]);
  // No browser DOM library, bundler, native GPU binding, or protocol SDK is needed for Node CPU proofs.
  await assert.rejects(readFile(join(consumer, "node_modules/webgpu/package.json")), { code: "ENOENT" });
  await assert.rejects(readFile(join(consumer, "node_modules/@heliuslabs/spr/package.json")), { code: "ENOENT" });
  await command(process.execPath, ["--input-type", "module", "-e", `
    import { SprNodeProver } from "@heliuslabs/spr-web-prover/node";
    const prover = new SprNodeProver({ proverUrl: "http://localhost:3001", threads: 2, gpu: "off" });
    try { await prover.start(); } finally { prover.terminate(); }
  `]);
  // Test subprocesses expand V8 flags in execArgv. Workers must ignore them,
  // just as they ignore parent-only --input-type and loader options.
  await writeFile(join(consumer, "worker-flags.test.mjs"), `
    import { test } from "node:test";
    import assert from "node:assert/strict";
    import { SprNodeProver } from "@heliuslabs/spr-web-prover/node";
    test("prover workers do not inherit test/V8 flags", async () => {
      const prover = new SprNodeProver({ threads: 2, gpu: "off" });
      try { await prover.start(); assert.equal(prover.threads, 2); }
      finally { prover.terminate(); }
    });
  `);
  await command(process.execPath, ["--stack-trace-limit=20", "--test", "worker-flags.test.mjs"]);
  await cp(join(root, "fixtures"), join(consumer, "fixtures"), { recursive: true });
  await cp(join(root, "examples/node"), join(consumer, "examples/node"), { recursive: true });
  await command(join(consumer, "node_modules/.bin/tsc"), ["-p", "examples/node/tsconfig.json"]);
  await command(process.execPath, ["examples/node/prove.ts"]);
  const test = (await readFile(join(root, "scripts/test-node.mjs"), "utf8"))
    .replace('"../packages/web-prover/dist/node/index.js"', '"@heliuslabs/spr-web-prover/node"')
    .replaceAll('"../fixtures/', '"./fixtures/');
  await writeFile(join(consumer, "test-node.mjs"), test);
  console.log(await command(process.execPath, ["test-node.mjs"], consumer, { ...process.env, SPR_TEST_GPU: "0", SPR_TEST_NO_GPU: "1" }));
  if (process.env.SPR_TEST_GPU === "1") {
    await command("npm", ["install", "--no-audit", "--no-fund", "webgpu@0.6.2"]);
    console.log(await command(process.execPath, ["test-node.mjs"], consumer));
  }
  await command("npm", ["install", "--ignore-scripts", "--no-audit", "--no-fund", "@heliuslabs/spr@0.9.0-alpha"]);
  await writeFile(join(consumer, "sdk-consumer.ts"), `
    import { SprNodeProver } from "@heliuslabs/spr-web-prover/node";
    import { createLocalProofService } from "@heliuslabs/spr-web-prover/sdk";
    import type { ProofService } from "@heliuslabs/spr/client";
    const prover = new SprNodeProver({ gpu: "off" });
    const service: ProofService = createLocalProofService(prover);
    if (typeof service.prove !== "function" || typeof service.proveMerge !== "function") throw new Error("missing SDK service");
    prover.terminate();
  `);
  await command(join(consumer, "node_modules/.bin/tsc"), ["--noEmit", "--strict", "--skipLibCheck", "false", "--target", "ES2024", "--lib", "ES2024,DOM", "--module", "nodenext", "sdk-consumer.ts"]);
  await command(process.execPath, ["sdk-consumer.ts"]);
  console.log("Packed Node consumer: types, proofs, caching and worker lifecycle passed");
} finally { await rm(consumer, { recursive: true, force: true }); }
