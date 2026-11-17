import { parentPort } from "node:worker_threads";
import init, {
  wbg_rayon_start_worker,
} from "../runtime/threads/zopro_spr_node.js";

parentPort.once("message", async ({ module, memory, receiver }) => {
  await init({ module_or_path: module, memory });
  parentPort.postMessage({ type: "ready" });
  wbg_rayon_start_worker(receiver);
});
