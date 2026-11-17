export async function browserKeyStore(options, keySet, cacheStorage) {
  if (options.keyDirectory !== undefined)
    throw new TypeError("keyDirectory is only supported in Node");
  let cache;
  try {
    cache = await (
      cacheStorage === undefined ? globalThis.caches : cacheStorage
    )?.open(`zopro-spr-proving-keys-${keySet}`);
  } catch {
    /* Unavailable persistent storage uses a bounded in-memory entry. */
  }
  let memory;
  return {
    async read(artifact, url) {
      if (memory?.file === artifact.file) return memory.bytes;
      if (cache) {
        try {
          const response = await cache.match(url);
          if (response?.ok) return new Uint8Array(await response.arrayBuffer());
        } catch {
          cache = undefined;
        }
      }
    },
    async write(artifact, bytes, url) {
      if (cache) {
        try {
          await cache.put(url, new Response(bytes));
          memory = undefined;
          return;
        } catch {
          cache = undefined;
        }
      }
      // One key is resident in the prover; keep at most its source bytes here.
      memory = { file: artifact.file, bytes };
    },
    async sha256(bytes) {
      const digest = new Uint8Array(
        await crypto.subtle.digest("SHA-256", bytes),
      );
      return Array.from(digest, (byte) =>
        byte.toString(16).padStart(2, "0"),
      ).join("");
    },
    close() {
      memory = undefined;
      cache = undefined;
    },
  };
}
