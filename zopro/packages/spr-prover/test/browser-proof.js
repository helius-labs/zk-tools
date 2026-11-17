import { init } from "../src/browser.js";
import { checkProofs } from "./proof-check.js";

self.addEventListener(
  "message",
  async ({ data: { threads, keyDigest, noCrypto } }) => {
    try {
      const key = new Uint8Array(
        await (await fetch("/key.zopro")).arrayBuffer(),
      );
      const request = await (await fetch("/request.json")).text();
      if (noCrypto)
        Object.defineProperty(globalThis, "crypto", { value: undefined });
      self.postMessage({
        result: await checkProofs(init, key, request, threads, keyDigest),
      });
    } catch (error) {
      self.postMessage({ error: String(error) });
    }
  },
  { once: true },
);
