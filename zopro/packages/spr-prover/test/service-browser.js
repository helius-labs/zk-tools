import { createLocalProofService } from "@heliuslabs/zopro-spr";
import catalog from "../src/proving-keys.json" with { type: "json" };
import { sdkInputs } from "./sdk-inputs.js";

export async function checkBrowserService({ threads, memory }) {
  const keyName = "transfer_confidential_1_2.key";
  const keyUrl = new URL(catalog.keys[keyName].file, catalog.baseUrl).href;
  const request = await (await fetch("/service-request.json")).text();
  if (memory) Object.defineProperty(globalThis, "caches", { value: undefined });
  const originalFetch = globalThis.fetch;
  let downloads = 0;
  let preloaded = false;
  globalThis.fetch = (input, options) => {
    const url = input instanceof Request ? input.url : String(input);
    if (url === keyUrl) {
      if (preloaded)
        throw new Error("A hot proof must not request key transport");
      downloads++;
      return originalFetch("/service-key.zopro", options);
    }
    return originalFetch(input, options);
  };
  let service;
  let originalMatch;
  try {
    service = await createLocalProofService(
      threads === undefined ? {} : { threads },
    );
    await service.preload(keyName);
    preloaded = true;
    if (downloads !== 1)
      throw new Error(`Expected one setup key download, got ${downloads}`);
    if (!memory && globalThis.caches) {
      const cache = await caches.open(
        "zopro-spr-proving-keys-78751f5d1c591777",
      );
      if (!(await cache.match(keyUrl)))
        throw new Error(
          "The production key was not stored in the browser cache",
        );
      originalMatch = Cache.prototype.match;
      Cache.prototype.match = () => {
        throw new Error("A hot proof must not read the persistent key store");
      };
    }
    const proof = await service.prove(sdkInputs(request));
    if (
      proof.a.length !== 64 ||
      proof.b.length !== 128 ||
      proof.c.length !== 64
    )
      throw new Error("Expected the SDK Proof byte format");
    return {
      service: true,
      preloaded: true,
      memory,
      downloads,
      proofBytes: [64, 128, 64],
    };
  } finally {
    globalThis.fetch = originalFetch;
    if (originalMatch) Cache.prototype.match = originalMatch;
    await service?.close();
  }
}
