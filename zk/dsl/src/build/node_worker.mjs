import { Worker, parentPort, workerData } from "node:worker_threads";
import { Console } from "node:console";
import { writeSync } from "node:fs";
import { Writable } from "node:stream";

let workers = [];

export async function startWorkers(module, memory, builder) {
  if (builder.numThreads() === 0) throw new Error("Prover thread count must be positive");
  const spawned = [];
  try {
    const ready = Array.from({ length: builder.numThreads() }, async () => {
      const worker = new Worker(new URL(import.meta.url), {
        name: "zolana-prover",
        execArgv: [],
        stdout: true,
        stderr: true,
        workerData: {
          kind: "zolana-prover-thread",
          module,
          memory,
          receiver: builder.receiver(),
          mainJS: builder.mainJS(),
        },
      });
      spawned.push(worker);
      await new Promise((resolve, reject) => {
        const timeout = setTimeout(
          () => reject(new Error("Prover worker initialization timed out")),
          30_000,
        );
        worker.once("error", (error) => {
          clearTimeout(timeout);
          reject(error);
        });
        worker.once("exit", (code) => {
          clearTimeout(timeout);
          reject(new Error(`Prover worker exited during initialization (${code})`));
        });
        worker.once("message", (message) => {
          clearTimeout(timeout);
          if (message !== "zolana-prover-ready") {
            reject(new Error("Invalid prover worker initialization message"));
            return;
          }
          resolve();
        });
      });
    });
    await Promise.all(ready);
    builder.build();
    workers = spawned;
    for (const worker of workers) worker.unref();
  } catch (error) {
    await Promise.allSettled(spawned.map((worker) => worker.terminate()));
    throw error;
  }
}

if (workerData?.kind === "zolana-prover-thread") {
  // Write worker diagnostics directly without holding the parent's stdio ports open.
  globalThis.console = new Console({ stdout: diagnosticStream(1), stderr: diagnosticStream(2) });
  // Finish evaluating this module before importing the WASM module that imports it.
  Promise.resolve()
    .then(async () => {
      const wasm = await import(workerData.mainJS);
      await wasm.default({ module_or_path: workerData.module, memory: workerData.memory });
      parentPort.postMessage("zolana-prover-ready");
      wasm.wbg_rayon_start_worker(workerData.receiver);
    })
    .catch((error) => {
      throw error;
    });
}

function diagnosticStream(fd) {
  return new Writable({
    write(chunk, encoding, done) {
      try {
        writeSync(fd, typeof chunk === "string" ? Buffer.from(chunk, encoding) : chunk);
        done();
      } catch (error) {
        done(error);
      }
    },
  });
}
