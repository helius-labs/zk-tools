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
  await command(process.execPath, ["--input-type", "module", "-e", `
    import { SprNodeProver } from "@heliuslabs/spr-web-prover/node";
    const prover = new SprNodeProver({ proverUrl: "http://localhost:3001", threads: 2, gpu: "off" });
    try { await prover.start(); } finally { prover.terminate(); }
  `]);
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
  console.log("Packed Node consumer: types, proofs, caching and worker lifecycle passed");
} finally { await rm(consumer, { recursive: true, force: true }); }
