import { threadPool } from "./pool.js";

export const { startWorkers, close } = threadPool(() => {
  const worker = new Worker(
    new URL("./browser-rayon-worker.js", import.meta.url),
    { type: "module" },
  );
  return {
    send: (message) => worker.postMessage(message),
    terminate: () => worker.terminate(),
    listen(message, error) {
      const received = (event) => message(event.data);
      const failed = (event) => {
        event.preventDefault();
        error(new Error("Rayon worker failed to initialize"));
      };
      worker.addEventListener("message", received);
      worker.addEventListener("error", failed);
      worker.addEventListener("messageerror", failed);
      return () => {
        worker.removeEventListener("message", received);
        worker.removeEventListener("error", failed);
        worker.removeEventListener("messageerror", failed);
      };
    },
  };
});
