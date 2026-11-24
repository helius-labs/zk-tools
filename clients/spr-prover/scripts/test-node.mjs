import assert from "node:assert/strict";
import { mkdtemp, readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { SprNodeProver, WasmProverError, shapeByLabel } from "../packages/web-prover/dist/node/index.js";

const cacheDir = await mkdtemp(join(tmpdir(), "spr-node-test-"));
const endpoint = "http://127.0.0.1:3001";
const body = await readFile(new URL("../fixtures/transfer-2x2.json", import.meta.url), "utf8");
const request = JSON.parse(body);
const sentinel = "private-witness-must-not-escape";
const originalGlobals = { Worker: globalThis.Worker, caches: globalThis.caches, navigator: globalThis.navigator };
const modes = [{ threads: 4, gpu: "off" }, { threads: 0, gpu: "off" }];
if (process.env.SPR_TEST_NO_GPU === "1") modes.push({ threads: 4, gpu: "auto" });
if (process.env.SPR_TEST_GPU === "1") modes.push({ threads: 4, gpu: "required" });
try {
  if (process.env.SPR_TEST_NO_GPU === "1") {
    const required = new SprNodeProver({ proverUrl: endpoint, threads: 2, gpu: "required", cacheDir });
    try { await assert.rejects(required.start(), { code: "wasm_gpu_unavailable" }); }
    finally { required.terminate(); }
  }
  let cached = false;
  for (const mode of modes) {
    const measurements = [];
    const prover = new SprNodeProver({
      proverUrl: endpoint, cacheDir, ...mode, residentKeyBudgetBytes: 512 * 1024 ** 2,
      onMeasurement: (m) => measurements.push(m),
      ...(cached ? { fetch: async () => { throw new Error("Cache reuse must not fetch"); } } : {}),
    });
    try {
      let ticks = 0;
      const timer = setInterval(() => ticks++, 5);
      const proof = await prover.proveRequest(body).finally(() => clearInterval(timer));
      assert.ok(ticks > 2, "proving must not block Node's event loop");
      assert.equal(proof.backend, mode.gpu === "required" ? "gpu" : "cpu");
      assert.equal(prover.threads, mode.threads);
      assert.equal(prover.gpuAvailable, mode.gpu === "required");
      const parsed = JSON.parse(proof.proof);
      assert.ok(Array.isArray(parsed.ar) && Array.isArray(parsed.bs) && Array.isArray(parsed.krs));
      assert.ok(parsed.provingKeySha256);
      if (cached) assert.ok(measurements.some((m) => m.note?.includes("raw-key cache")));
      cached = true;

      const localFetch = prover.createFetch();
      const response = await localFetch(`${endpoint}/prove/transfer_confidential_2_2`, { method: "POST", body });
      assert.equal(response.status, 200);

      const invalid = structuredClone(request);
      invalid.inputs[0].nullifierSecret = sentinel;
      await assert.rejects(prover.proveRequest(JSON.stringify(invalid)), (error) => {
        assert.ok(error instanceof WasmProverError);
        assert.ok(!String(error.stack).includes(sentinel));
        return true;
      });
      assert.ok((await prover.proveRequest(body)).proof, "worker must recover after a rejected witness");

      // A well-formed but false witness must not produce an accepted proof either.
      const unsatisfied = structuredClone(request);
      unsatisfied.inputs[0].utxo.amount = "0x01";
      await assert.rejects(prover.proveRequest(JSON.stringify(unsatisfied)), WasmProverError);

      // A queued cancellation must leave the active proof alone.
      const active = prover.proveRequest(body);
      const queued = prover.proveRequest(body, AbortSignal.abort());
      await assert.rejects(queued, { name: "AbortError" });
      await active;

      // Cancel a running proof and rebuild the entire thread pool for the next call.
      const controller = new AbortController();
      const interrupted = prover.proveRequest(body, controller.signal);
      setTimeout(() => controller.abort(), 10);
      await assert.rejects(interrupted, { name: "AbortError" });
      assert.ok((await prover.proveRequest(body)).proof);
      assert.deepEqual(await prover.loadedKeys(), [shapeByLabel("2x2").keyFile]);
      // Switching back must select the right resident prover, not reload its key.
      await prover.ensureKey(shapeByLabel("8x2"));
      assert.equal((await prover.loadedKeys()).length, 2);
      measurements.length = 0;
      assert.equal(JSON.parse((await prover.proveRequest(body)).proof).provingKeySha256, parsed.provingKeySha256);
      assert.equal(measurements.filter((m) => m.step === "key-fetch").length, 0);
      // Touching 2x2 makes 8x2 the eviction candidate when 4x2 arrives.
      await prover.ensureKey(shapeByLabel("4x2"));
      assert.deepEqual(await prover.loadedKeys(), [shapeByLabel("2x2").keyFile, shapeByLabel("4x2").keyFile]);
      const wide = await readFile(new URL("../fixtures/transfer-8x2.json", import.meta.url), "utf8");
      const wideProof = await prover.proveRequest(wide);
      assert.equal(wideProof.backend, mode.gpu === "required" ? "gpu" : "cpu");
      assert.notEqual(JSON.parse(wideProof.proof).provingKeySha256, parsed.provingKeySha256);
      console.log(JSON.stringify({ mode, verified: true, responsive: true, cached: true, cancellation: true, residentSwitching: true, eviction: true }));
    } finally { prover.terminate(); }
  }
  assert.deepEqual({ Worker: globalThis.Worker, caches: globalThis.caches, navigator: globalThis.navigator }, originalGlobals);
} finally { await rm(cacheDir, { recursive: true, force: true }); }
// The process must exit naturally after terminating Rayon and decoder workers.
