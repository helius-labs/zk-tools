import { initializer } from "./init.js";
import { close } from "./browser-rayon.js";

export const init = initializer(async (threads) => {
  if (
    typeof DedicatedWorkerGlobalScope === "undefined" ||
    !(globalThis instanceof DedicatedWorkerGlobalScope)
  ) {
    throw new Error("Initialize the browser prover in a dedicated Web Worker");
  }
  if (
    threads > 0 &&
    (!globalThis.crossOriginIsolated ||
      typeof SharedArrayBuffer === "undefined")
  ) {
    throw new Error(
      "Threaded proving requires cross-origin isolation; use threads: 0 otherwise",
    );
  }
  const module =
    threads > 0
      ? await import("../runtime/threads/zopro_spr.js")
      : await import("../runtime/single/zopro_spr.js");
  return { module, initialize: () => module.default(), close };
});
