import { Worker as Thread, type Transferable } from "node:worker_threads";

/** Web Worker message surface backed by a Node thread, including nested Rayon workers. */
export class NodeWorker extends EventTarget {
  readonly #thread: Thread;
  #stopped = false;

  override addEventListener(type: "message", listener: (event: MessageEvent) => void): void;
  override addEventListener(type: string, listener: EventListenerOrEventListenerObject | null, options?: AddEventListenerOptions | boolean): void;
  override addEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject | ((event: MessageEvent) => void) | null,
    options?: AddEventListenerOptions | boolean,
  ): void {
    super.addEventListener(type, listener as EventListenerOrEventListenerObject | null, options);
  }

  constructor(url: URL, options?: { name?: string; gpu?: "auto" | "off" | "required" }) {
    super();
    this.#thread = new Thread(new URL("./worker-bootstrap.js", import.meta.url), {
      workerData: { url: url.href, gpu: options?.gpu ?? "off" },
      ...(options?.name === undefined ? {} : { name: options.name }),
      // These are compiled JS workers. Parent loaders, test and V8 flags do not
      // belong to their runtime, and some are rejected by worker_threads.
      execArgv: [],
    });
    this.#thread.on("message", (data: unknown) => this.dispatchEvent(new MessageEvent("message", { data })));
    this.#thread.on("messageerror", () => this.dispatchEvent(new Event("messageerror")));
    this.#thread.on("error", () => this.dispatchEvent(new Event("error", { cancelable: true })));
    this.#thread.on("exit", () => {
      if (!this.#stopped) this.dispatchEvent(new Event("error", { cancelable: true }));
    });
  }

  postMessage(message: unknown, transfer: Transferable[] = []): void {
    this.#thread.postMessage(message, transfer);
  }

  terminate(): void {
    this.#stopped = true;
    void this.#thread.terminate();
  }
}
