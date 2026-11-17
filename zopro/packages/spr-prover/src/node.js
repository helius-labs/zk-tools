import { readFile } from "node:fs/promises";
import { initializer } from "./init.js";
import { close } from "./node-rayon.js";
import { nodeKeyStore } from "./node-key-store.js";
import { localProofServiceFactory } from "./local-service.js";

export const init = initializer(async (threads) => {
  const module =
    threads > 0
      ? await import("../runtime/threads/zopro_spr_node.js")
      : await import("../runtime/single/zopro_spr.js");
  const wasm = new URL(
    `../runtime/${threads > 0 ? "threads" : "single"}/zopro_spr_bg.wasm`,
    import.meta.url,
  );
  return {
    module,
    initialize: async () =>
      module.default({ module_or_path: await readFile(wasm) }),
    close,
  };
});

export const createLocalProofService = localProofServiceFactory(
  init,
  nodeKeyStore,
);
