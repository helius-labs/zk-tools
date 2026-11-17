import { Worker } from "node:worker_threads";
import { threadPool } from "./pool.js";

export const { startWorkers, close } = threadPool(() => {
  // The worker is plain JS; parent test/loader and V8 flags may be invalid here.
  const worker = new Worker(
    new URL("./node-rayon-worker.js", import.meta.url),
    { execArgv: [] },
  );
  return {
    send: (message) => worker.postMessage(message),
    terminate: () => worker.terminate(),
    listen(message, error) {
      const exited = (code) =>
        error(new Error(`Rayon worker exited before readiness (${code})`));
      worker.on("message", message);
      worker.on("error", error);
      worker.on("exit", exited);
      return () => {
        worker.off("message", message);
        worker.off("error", error);
        worker.off("exit", exited);
      };
    },
  };
});
