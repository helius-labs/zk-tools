// Every result is a real, locally verified proof. Each shape/backend gets an
// empty disk cache and separate fresh/restart processes; failures remain rows.
import assert from "node:assert/strict";
import { fork, execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, mkdtemp, readFile, rm, writeFile, rename } from "node:fs/promises";
import { cpus, platform, release, arch, availableParallelism, totalmem, tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { gunzipSync } from "node:zlib";
import { SprNodeProver, TRANSFER_SHAPES, MERGE_SHAPES } from "../packages/web-prover/dist/node/index.js";

const root = fileURLToPath(new URL("../", import.meta.url));
const file = fileURLToPath(import.meta.url);
const fixtureBytes = await readFile(join(root, "fixtures/node-shapes.json.gz"));
const fixtures = JSON.parse(gunzipSync(fixtureBytes));
const shapeKeys = new Map([
  ...TRANSFER_SHAPES.map((shape) => [shape.label, shape]),
  ...MERGE_SHAPES.map((shape) => [`merge-${shape.inputs}x1`, shape]),
]);
const fixtureSha256 = createHash("sha256").update(fixtureBytes).digest("hex");
const keys = JSON.parse(await readFile(join(root, "zopro-keys.lock"), "utf8")).keys;

if (process.argv[2] === "--case") {
  const config = JSON.parse(process.argv[3]);
  const measurements = [];
  let stage = "initialization";
  const prover = new SprNodeProver({
    proverUrl: "http://127.0.0.1:3001", cacheDir: config.cacheDir,
    threads: config.threads, gpu: config.backend === "gpu" ? "required" : "off",
    onMeasurement: (measurement) => measurements.push(measurement),
    ...(config.phase === "restart" ? { fetch: async () => { throw new Error("Cached restart must not fetch"); } } : {}),
  });
  try {
    const body = JSON.stringify(fixtures.requests[config.shape]);
    const started = performance.now();
    await prover.start();
    const startupMs = performance.now() - started;
    stage = "first-proof";
    const first = await prover.proveRequest(body);
    const firstMs = performance.now() - started;
    assert.equal(first.backend, config.backend);
    assert.equal(JSON.parse(first.proof).provingKeySha256, keys[shapeKeys.get(config.shape).keyFile].key_sha256);
    if (config.phase === "restart") assert.ok(measurements.some((m) => m.note?.includes("raw-key cache")));
    stage = "warm-proofs";
    const samples = [];
    for (let i = 0; i < config.iterations; i++) {
      const start = performance.now();
      const proof = await prover.proveRequest(body);
      const totalMs = performance.now() - start;
      assert.equal(proof.backend, config.backend);
      samples.push({ totalMs, proveMs: proof.proveMs, verifyMs: proof.verifyMs });
    }
    process.send({ status: "passed", startupMs, firstMs, measurements, samples });
  } catch (error) {
    process.send({ status: "failed", stage, error: error.code ?? error.name ?? "Error" });
  } finally {
    prover.terminate();
    process.disconnect();
  }
} else {
  const selected = process.env.SPR_BENCH_SHAPES ?? "all";
  const shapes = selected === "all" ? [...shapeKeys.keys()] : selected.split(",");
  assert.equal(new Set(shapes).size, shapes.length, "Duplicate shapes");
  for (const shape of shapes) assert.ok(shapeKeys.has(shape) && fixtures.requests[shape], `Missing fixture/key: ${shape}`);
  const iterations = Number(process.env.SPR_BENCH_RUNS ?? 5);
  assert.ok(Number.isInteger(iterations) && iterations > 0);
  const threads = Math.min(18, availableParallelism());
  const modes = (process.env.SPR_BENCH_MODES ?? `cpu:${threads},gpu:${threads}`).split(",").map((mode) => {
    const [backend, count] = mode.split(":");
    const threads = Number(count);
    assert.ok(["cpu", "gpu"].includes(backend) && Number.isInteger(threads) && threads >= 0 && threads <= 64);
    return { threads, backend };
  });
  const output = process.env.SPR_BENCH_OUTPUT ?? join(root, "target/node-benchmark/all-shapes.json");
  const timeoutMs = Number(process.env.SPR_BENCH_TIMEOUT_MS ?? 300000);
  assert.ok(Number.isFinite(timeoutMs) && timeoutMs > 0);
  const report = {
    schema: 2, startedAt: new Date().toISOString(), completedAt: null,
    sourceCommit: execFileSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).trim(),
    harnessSha256: createHash("sha256").update(await readFile(file)).digest("hex"),
    fixtureSha256, protocolRevision: fixtures.protocolRevision, runtimeRevision: fixtures.runtimeRevision,
    gpuBinding: JSON.parse(await readFile(join(root, "node_modules/webgpu/package.json"), "utf8")).version,
    node: process.version, platform: platform(), osRelease: release(), arch: arch(),
    cpu: cpus()[0]?.model, availableParallelism: availableParallelism(), memoryGiB: totalmem() / 2 ** 30,
    modes, shapes, iterations, timeoutMs, results: [],
  };
  await mkdir(dirname(output), { recursive: true });
  async function save() {
    await writeFile(`${output}.tmp`, JSON.stringify(report, null, 2) + "\n");
    await rename(`${output}.tmp`, output);
  }
  await save();
  for (const shape of shapes) {
    for (const mode of modes) {
      const cacheDir = await mkdtemp(join(tmpdir(), "spr-shape-bench-"));
      const row = { shape, keyFile: shapeKeys.get(shape).keyFile, keyBytes: shapeKeys.get(shape).keyBytes, ...mode };
      try {
        row.fresh = await run({ shape, ...mode, cacheDir, iterations, phase: "fresh" }, timeoutMs);
        if (row.fresh.status === "passed") row.restart = await run({ shape, ...mode, cacheDir, iterations: 0, phase: "restart" }, timeoutMs);
        row.status = row.fresh.status === "passed" && row.restart?.status === "passed" ? "passed" : "failed";
      } finally { await rm(cacheDir, { recursive: true, force: true }); }
      report.results.push(row);
      await save();
      const warm = row.fresh.samples?.map((s) => s.totalMs).sort((a, b) => a - b);
      console.log(JSON.stringify({ shape, ...mode, status: row.status,
        coldMs: row.fresh.firstMs, cachedMs: row.restart?.firstMs,
        warmMedianMs: warm?.[Math.floor(warm.length / 2)], error: row.fresh.error ?? row.restart?.error }));
    }
  }
  report.completedAt = new Date().toISOString();
  await save();
  if (report.results.some((row) => row.status !== "passed")) process.exitCode = 1;
}

function run(config, timeoutMs) {
  return new Promise((resolve) => {
    const child = fork(file, ["--case", JSON.stringify(config)], { stdio: ["ignore", "ignore", "ignore", "ipc"] });
    let answer;
    let timedOut = false;
    const timer = setTimeout(() => { timedOut = true; child.kill("SIGKILL"); }, timeoutMs);
    child.on("message", (message) => { answer = message; });
    child.on("error", () => { answer = { status: "failed", error: "process-start-failed" }; });
    // Wait for actual process exit, including all its native/nested worker resources.
    child.on("close", (code, signal) => {
      clearTimeout(timer);
      resolve(timedOut ? { status: "failed", error: "timeout" }
        : code === 0 && answer ? answer : { status: "failed", error: signal ?? `process-exit-${code}` });
    });
  });
}
