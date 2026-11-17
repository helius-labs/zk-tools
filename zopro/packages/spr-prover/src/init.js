import { proverClass } from "./prover.js";

export function initializer(load) {
  let initialization;
  let configuredThreads;
  let closed = false;

  return async function init({ threads = 0 } = {}) {
    if (!Number.isInteger(threads) || threads < 0 || threads > 64) {
      throw new RangeError("threads must be an integer between 0 and 64");
    }
    if (closed)
      throw new Error(
        "This runtime is closed; start a new worker to initialize another",
      );
    if (initialization) {
      if (threads !== configuredThreads)
        throw new Error("The thread count is fixed for this worker");
      return initialization;
    }
    configuredThreads = threads;
    initialization = (async () => {
      const { module, initialize, close } = await load(threads);
      try {
        await initialize();
        if (threads > 0) await module.initThreadPool(threads);
      } catch (error) {
        closed = true;
        await close();
        throw error;
      }
      let closing;
      return Object.freeze({
        SprProver: proverClass(module.SprProver),
        threads,
        close() {
          closed = true;
          closing ??= Promise.resolve().then(close);
          return closing;
        },
      });
    })();
    // A failed startup may leave a partially initialized WASM/Rayon instance.
    initialization.catch(() => {
      closed = true;
    });
    return initialization;
  };
}
