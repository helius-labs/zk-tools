globalThis.proveInWorker = (options) =>
  new Promise((resolve, reject) => {
    const worker = new Worker(new URL("./browser-proof.js", import.meta.url), {
      type: "module",
    });
    const timeout = setTimeout(() => {
      worker.terminate();
      reject(new Error("proof timed out"));
    }, 60_000);
    worker.onmessage = ({ data }) => {
      clearTimeout(timeout);
      worker.terminate();
      resolve(data);
    };
    worker.onerror = (event) => {
      clearTimeout(timeout);
      worker.terminate();
      reject(new Error(event.message));
    };
    worker.postMessage(options);
  });
