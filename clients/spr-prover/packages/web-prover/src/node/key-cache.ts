import { createHash, randomUUID } from "node:crypto";
import { openAsBlob } from "node:fs";
import { mkdir, open, readdir, rename, rm } from "node:fs/promises";
import { join, resolve } from "node:path";
import type { KeyCache, KeyCacheStorage } from "../key-store.js";

/** Only public proving keys are persisted. Atomic files prevent partial/concurrent writes. */
export class DiskCacheStorage implements KeyCacheStorage {
  readonly #root: string;
  constructor(directory: string) { this.#root = resolve(directory); }
  #directory(name: string): string { return join(this.#root, Buffer.from(name).toString("base64url")); }

  async keys(): Promise<string[]> {
    try {
      return (await readdir(this.#root, { withFileTypes: true }))
        .filter((entry) => entry.isDirectory())
        .map((entry) => Buffer.from(entry.name, "base64url").toString());
    } catch { return []; }
  }

  async delete(name: string): Promise<boolean> {
    await rm(this.#directory(name), { recursive: true, force: true });
    return true;
  }

  async open(name: string): Promise<KeyCache> {
    const directory = this.#directory(name);
    await mkdir(directory, { recursive: true, mode: 0o700 });
    const path = (url: string) => join(directory, createHash("sha256").update(url).digest("hex"));
    return {
      async match(url) {
        try {
          // Status/prefetch checks need only headers, not hundreds of MB of key bytes.
          const data = await openAsBlob(path(url));
          const prefix = Buffer.from(await data.slice(0, 4).arrayBuffer());
          const end = prefix.readUInt32LE(0) + 4;
          if (end > data.size || end > 65_536) return undefined;
          const headers = JSON.parse(await data.slice(4, end).text()) as Record<string, string>;
          return new Response(data.slice(end), { headers });
        } catch { return undefined; }
      },
      async put(url, response) {
        const headers = Buffer.from(JSON.stringify(Object.fromEntries(response.headers)));
        const length = Buffer.alloc(4);
        length.writeUInt32LE(headers.length);
        const data = Buffer.from(await response.arrayBuffer());
        const temporary = `${path(url)}.${randomUUID()}.tmp`;
        try {
          await mkdir(directory, { recursive: true, mode: 0o700 });
          const file = await open(temporary, "wx", 0o600);
          try {
            await file.writeFile(length);
            await file.writeFile(headers);
            await file.writeFile(data);
          } finally { await file.close(); }
          await rename(temporary, path(url));
        } finally { await rm(temporary, { force: true }); }
      },
      async delete(url) {
        await rm(path(url), { force: true });
        return true;
      },
    };
  }
}
