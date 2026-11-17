import { mkdtemp, readdir, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { afterEach, expect, it } from "vitest";
import { DiskCacheStorage } from "../src/node/key-cache.js";

const directories: string[] = [];
afterEach(async () => {
  await Promise.all(directories.splice(0).map((path) => rm(path, { recursive: true, force: true })));
});
async function storage() {
  const directory = await mkdtemp(join(tmpdir(), "spr-key-cache-"));
  directories.push(directory);
  return { directory, store: new DiskCacheStorage(directory) };
}

it("persists bytes and headers across instances, with independent namespaces", async () => {
  const { directory, store } = await storage();
  const url = "https://keys.example/test?key=1";
  const cache = await store.open("generation/a");
  await cache.put(url, new Response(new Uint8Array([0, 255, 7]), { headers: { "x-source": "pin" } }));
  const other = await new DiskCacheStorage(directory).open("generation/a");
  const hit = await other.match(url);
  expect(hit?.headers.get("x-source")).toBe("pin");
  expect(new Uint8Array(await hit!.arrayBuffer())).toEqual(new Uint8Array([0, 255, 7]));
  expect(await (await store.open("generation/b")).match(url)).toBeUndefined();
  expect((await store.keys()).sort()).toEqual(["generation/a", "generation/b"]);
  await store.delete("generation/a");
  expect(await other.match(url)).toBeUndefined();
});

it("publishes whole entries under concurrent writes and ignores a damaged file", async () => {
  const { directory, store } = await storage();
  const cache = await store.open("keys");
  await Promise.all(["a", "b"].map((value) => cache.put("key", new Response(value.repeat(65536)))));
  const value = await (await cache.match("key"))!.text();
  expect(["a".repeat(65536), "b".repeat(65536)]).toContain(value);
  const folder = join(directory, Buffer.from("keys").toString("base64url"));
  const files = await readdir(folder);
  expect(files).toHaveLength(1);
  await writeFile(join(folder, files[0]!), new Uint8Array([255, 255, 255, 255]));
  expect(await cache.match("key")).toBeUndefined();
  await cache.put("key", new Response("recovered"));
  expect(await (await cache.match("key"))!.text()).toBe("recovered");
});
