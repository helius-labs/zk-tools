import { readFile } from "node:fs/promises";
import { availableParallelism } from "node:os";
import { parentPort, workerData, type Transferable } from "node:worker_threads";
import { NodeWorker } from "./worker.js";

if (parentPort === null) throw new Error("Prover bootstrap requires a worker thread");
const port = parentPort;
const events = new EventTarget();
// Browser compatibility is confined to the prover's worker. The application globals
// and the shared browser runtime stay untouched.
Object.assign(globalThis, {
  self: globalThis,
  Worker: NodeWorker,
  // wgpu detects a worker by this constructor before reading navigator.gpu.
  WorkerGlobalScope: EventTarget,
  crossOriginIsolated: true,
  addEventListener: events.addEventListener.bind(events),
  removeEventListener: events.removeEventListener.bind(events),
  postMessage: (data: unknown, transfer: Transferable[] = []) => port.postMessage(data, transfer),
});
Object.defineProperty(globalThis, "navigator", {
  value: { hardwareConcurrency: availableParallelism() }, configurable: true,
});

// wasm-bindgen's web target fetches a URL relative to its JS module. Node's fetch
// has no file: support; load only the package's WASM assets through this bridge.
const networkFetch = globalThis.fetch;
const runtimeRoot = new URL("../runtime/", import.meta.url).href;
globalThis.fetch = async (input, init) => {
  const url = input instanceof Request ? input.url : String(input);
  if (url.startsWith(runtimeRoot) && url.endsWith(".wasm")) {
    const bytes = await readFile(new URL(url));
    return new Response(bytes, { headers: { "content-type": "application/wasm" } });
  }
  return networkFetch(input, init);
};

if (workerData.gpu !== "off") {
  try {
    const { create, globals } = await import("webgpu");
    Object.assign(globalThis, globals);
    Object.defineProperty(navigator, "gpu", { value: create([]) });
  } catch {
    // The shared runtime enforces gpu: required and otherwise uses Rayon.
  }
}

await import(workerData.url);
// Start delivery only after the imported entry has installed its listeners.
port.on("message", (data: unknown) => events.dispatchEvent(new MessageEvent("message", { data })));
