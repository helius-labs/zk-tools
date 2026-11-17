// Alternating shapes, each policy/backend in its own process. Real verified proofs.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { fork, execFileSync } from "node:child_process";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { availableParallelism, cpus, tmpdir, platform, arch } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { SprNodeProver, shapeByLabel } from "../packages/web-prover/dist/node/index.js";

const shapes = ["2x2", "8x2"];
const iterations = 6;
const threads = Math.min(18, availableParallelism());
const keys = JSON.parse(await readFile(new URL("../zopro-keys.lock", import.meta.url), "utf8")).keys;
const sha256 = async (path) => createHash("sha256").update(await readFile(new URL(path, import.meta.url))).digest("hex");
if (process.argv[2] === "--case") {
  const config = JSON.parse(process.argv[3]);
  const measurements = [];
  const prover = new SprNodeProver({ proverUrl: "http://localhost:3001", threads,
    gpu: config.backend === "gpu" ? "required" : "off", cacheDir: config.cacheDir,
    residentKeyBudgetBytes: config.budget,
    onMeasurement: (m) => measurements.push(m),
  });
  try {
    const requests = await Promise.all(shapes.map((s) => readFile(new URL(`../fixtures/transfer-${s}.json`, import.meta.url), "utf8")));
    for (const shape of shapes) await prover.ensureKey(shapeByLabel(shape));
    assert.equal((await prover.loadedKeys()).length, config.budget ? 2 : 1);
    measurements.length = 0;
    const samples = [];
    for (let cycle = 0; cycle <= iterations; cycle++) {
      for (let i = 0; i < shapes.length; i++) {
        const start = performance.now();
        const result = await prover.proveRequest(requests[i]);
        const ms = performance.now() - start;
        assert.equal(result.backend, config.backend);
        assert.equal(JSON.parse(result.proof).provingKeySha256, keys[shapeByLabel(shapes[i]).keyFile].key_sha256);
        samples.push({ shape: shapes[i], first: cycle === 0, ms });
      }
    }
    const loads = measurements.filter((m) => m.step === "key-fetch").length;
    assert.equal(loads, config.budget ? 0 : samples.length);
    process.send({ ...config, cacheDir: undefined, threads, samples, keyReloads: loads,
      peakRssMiB: process.resourceUsage().maxRSS / 1024,
      finalRssMiB: process.memoryUsage().rss / 1024 ** 2 });
  } finally { prover.terminate(); process.disconnect(); }
} else {
  const output = process.env.SPR_BENCH_OUTPUT ?? "target/node-benchmark/switching.json";
  const report = { date: new Date().toISOString(), cpu: cpus()[0]?.model, node: process.version,
    platform: platform(), arch: arch(),
    sourceCommit: execFileSync("git", ["rev-parse", "HEAD"], { encoding: "utf8" }).trim(),
    harnessSha256: await sha256("./bench-node-switching.mjs"),
    workerSha256: await sha256("../packages/web-prover/dist/prover.worker.js"),
    controllerSha256: await sha256("../packages/web-prover/dist/wasm-prover.js"),
    runtimeRevision: JSON.parse(await readFile(new URL("../runtime.lock.json", import.meta.url), "utf8")).zopro.revision,
    gpuBinding: JSON.parse(await readFile(new URL("../node_modules/webgpu/package.json", import.meta.url), "utf8")).version,
    shapes, iterations, results: [] };
  for (const backend of ["cpu", "gpu"]) for (const budget of [0, 512 * 1024 ** 2]) {
    const cacheDir = await mkdtemp(join(tmpdir(), "spr-switching-"));
    try {
      const result = await new Promise((resolve, reject) => {
        const child = fork(fileURLToPath(import.meta.url), ["--case", JSON.stringify({ backend, budget, cacheDir })], { stdio: ["ignore", "ignore", "inherit", "ipc"] });
        const timer = setTimeout(() => child.kill("SIGKILL"), 180_000);
        let answer;
        child.on("message", (m) => { answer = m; });
        child.on("error", reject);
        child.on("close", (code) => {
          clearTimeout(timer);
          if (code === 0 && answer) resolve(answer);
          else reject(new Error(`Benchmark failed: ${backend}/${budget}, exit ${code}`));
        });
      });
      report.results.push(result);
      console.log(JSON.stringify(result));
    } finally { await rm(cacheDir, { recursive: true, force: true }); }
  }
  await mkdir(dirname(output), { recursive: true });
  await writeFile(output, JSON.stringify(report, null, 2) + "\n");
}
