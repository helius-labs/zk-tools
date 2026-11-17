import { init } from "@heliuslabs/zopro-spr";
import { checkProofs } from "./proof-check.js";
import { checkBrowserService } from "./service-browser.js";

self.addEventListener(
  "message",
  async ({
    data: { threads, keyDigest, noCrypto, merge, service, memory },
  }) => {
    try {
      if (service) {
        self.postMessage({
          result: await checkBrowserService({ threads, memory }),
        });
        return;
      }
      const key = new Uint8Array(
        await (
          await fetch(merge ? "/merge-key.zopro" : "/key.zopro")
        ).arrayBuffer(),
      );
      const request = await (
        await fetch(merge ? "/merge-request.json" : "/request.json")
      ).text();
      if (noCrypto)
        Object.defineProperty(globalThis, "crypto", { value: undefined });
      self.postMessage({
        result: await checkProofs(
          init,
          key,
          request,
          threads,
          keyDigest,
          merge,
        ),
      });
    } catch (error) {
      self.postMessage({ error: String(error) });
    }
  },
  { once: true },
);
