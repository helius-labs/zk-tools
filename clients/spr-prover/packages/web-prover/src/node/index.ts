import { availableParallelism, homedir } from "node:os";
import { join } from "node:path";
import { WasmProver, type WasmProverOptions } from "../wasm-prover.js";
import { DiskCacheStorage } from "./key-cache.js";
import { NodeWorker } from "./worker.js";

export * from "../index.js";

export interface NodeProverOptions extends Omit<WasmProverOptions, "workerFactory" | "persistStorage" | "cacheStorage"> {
  /** Directory for verified downloads and decoded keys; false disables persistence. */
  readonly cacheDir?: string | false;
}

/** Local WASM proofs in Node, with Rayon threads and optional Dawn WebGPU. */
export class SprNodeProver extends WasmProver {
  constructor(options: NodeProverOptions) {
    const { cacheDir, ...shared } = options;
    super({
      ...shared,
      threads: options.threads ?? Math.min(18, availableParallelism()),
      ...(cacheDir === false ? {} : {
        cacheStorage: new DiskCacheStorage(cacheDir ?? join(homedir(), ".cache", "spr-prover")),
      }),
      workerFactory: () => new NodeWorker(new URL("../prover.worker.js", import.meta.url), {
        name: "spr-prover", gpu: options.gpu ?? "auto",
      }),
    });
  }
}
