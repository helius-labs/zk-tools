import init, { wbg_rayon_start_worker } from "../runtime/threads/zopro_spr.js";

self.addEventListener(
  "message",
  async ({ data: { module, memory, receiver } }) => {
    await init({ module_or_path: module, memory });
    self.postMessage({ type: "ready" });
    wbg_rayon_start_worker(receiver);
  },
  { once: true },
);
