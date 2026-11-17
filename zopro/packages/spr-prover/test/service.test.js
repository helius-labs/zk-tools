import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdtemp, readdir, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { localProofServiceFactory } from "../src/local-service.js";
import { nodeKeyStore } from "../src/node-key-store.js";
import { browserKeyStore } from "../src/browser-key-store.js";

const hash = (bytes) => createHash("sha256").update(bytes).digest("hex");
const first = "transfer_confidential_1_2.key";
const second = "transfer_confidential_2_2.key";
const merge = "merge_8_1.key";
const files = new Map([
  ["first.zopro", Uint8Array.of(1)],
  ["second.zopro", Uint8Array.of(2)],
  ["merge.zopro", Uint8Array.of(3)],
]);
const catalog = {
  baseUrl: "https://keys.example/test/",
  keys: Object.fromEntries(
    [
      [first, "first.zopro"],
      [second, "second.zopro"],
      [merge, "merge.zopro"],
    ].map(([name, file]) => [
      name,
      { file, size: 1, sha256: hash(files.get(file)), keySha256: name },
    ]),
  ),
};
const pins = Object.fromEntries(
  Object.entries(catalog.keys).map(([name, key]) => [name, key.keySha256]),
);
const transfer = (count = 1) => ({
  circuit: "transfer",
  payload: { inputs: Array(count).fill({}), outputs: [{}, {}] },
});
const proof = Object.freeze({
  a: new Uint8Array(64),
  b: new Uint8Array(128),
  c: new Uint8Array(64),
});

function fixture({
  cached = new Map(),
  fetchBytes,
  digest,
  proving,
  wrongShape = false,
} = {}) {
  const events = [];
  const store = {
    read: async (key) => {
      events.push(`read:${key.file}`);
      return cached.get(key.file);
    },
    write: async (key, bytes) => {
      events.push(`write:${key.file}`);
      cached.set(key.file, bytes);
    },
    sha256: hash,
  };
  const create = localProofServiceFactory(
    async ({ threads }) => {
      events.push(`init:${threads}`);
      return {
        SprProver: class {
          constructor(bytes) {
            this.index = bytes[0];
            events.push(`decode:${this.index}`);
          }
          digest() {
            return digest ?? [undefined, first, second, merge][this.index];
          }
          inputs() {
            return wrongShape ? 99 : [undefined, 1, 2, 8][this.index];
          }
          outputs() {
            return [undefined, 2, 2, 1][this.index];
          }
          async prove(inputs, context) {
            events.push(`prove:${this.index}`);
            return proving ? proving(inputs, context) : proof;
          }
          async proveMerge(inputs, context) {
            return this.prove(inputs, context);
          }
          free() {
            events.push(`free:${this.index}`);
          }
        },
        close: async () => {
          events.push("runtime:close");
        },
      };
    },
    () => store,
    {
      catalog,
      pins,
      fetch: async (url) => {
        const file = new URL(url).pathname.split("/").at(-1);
        events.push(`fetch:${file}`);
        return new Response(fetchBytes ?? files.get(file));
      },
    },
  );
  return { create, events, cached };
}

test("preload verifies and decodes the key before any proof, and same-key proofs reuse it", async () => {
  const { create, events } = fixture();
  const service = await create({ threads: 4 });
  await service.preload(first);
  assert.deepEqual(events, [
    "init:4",
    "read:first.zopro",
    "fetch:first.zopro",
    "write:first.zopro",
    "decode:1",
  ]);
  await service.preload(first);
  assert.equal(await service.prove(transfer()), proof);
  assert.equal(await service.prove(transfer()), proof);
  await service.close();
  await service.close();
  assert.deepEqual(events.slice(5), [
    "prove:1",
    "prove:1",
    "free:1",
    "runtime:close",
  ]);
  assert.equal("proofCount" in service, false);
  await assert.rejects(service.preload(first), /closed/);
  await assert.rejects(service.prove(transfer()), /closed/);
});

test("shape switches and close serialize behind accepted proofs", async () => {
  let finish;
  const { create, events } = fixture({
    proving: () =>
      new Promise((resolve) => {
        finish = () => resolve(proof);
      }),
  });
  const service = await create();
  await service.preload(first);
  const pending = service.prove(transfer());
  await new Promise((resolve) => setImmediate(resolve));
  const preloading = service.preload(second);
  const closing = service.close();
  await assert.rejects(service.preload(first), /closed/);
  assert.equal(events.includes("free:1"), false);
  finish();
  await pending;
  await preloading;
  await closing;
  assert.ok(events.indexOf("prove:1") < events.indexOf("free:1"));
  assert.ok(events.indexOf("free:1") < events.indexOf("decode:2"));
  assert.deepEqual(events.slice(-2), ["free:2", "runtime:close"]);
});

test("merge calls select their own key and unsupported shapes are not sent to a server", async () => {
  const { create, events } = fixture();
  const service = await create();
  assert.equal(
    await service.proveMerge({ inputs: Array(8).fill({}), ringProgramId: 0n }),
    proof,
  );
  await assert.rejects(
    service.prove({ circuit: "transferRing", payload: {} }),
    { code: "CLIENT_PROVER_INPUT" },
  );
  await assert.rejects(
    service.prove(transfer(99)),
    /No pinned local proving key/,
  );
  await assert.rejects(
    service.preload("unknown.key"),
    /No pinned local proving key/,
  );
  await service.close();
  assert.ok(events.includes("decode:3"));
});

test("cached and downloaded artifact corruption is rejected before WASM decoding", async () => {
  for (const options of [
    { cached: new Map([["first.zopro", Uint8Array.of(9)]]) },
    { fetchBytes: Uint8Array.of(9) },
  ]) {
    const { create, events } = fixture(options);
    const service = await create();
    await assert.rejects(service.preload(first), /size or SHA-256 mismatch/);
    assert.equal(
      events.some((event) => event.startsWith("decode:")),
      false,
    );
    assert.equal(
      events.some((event) => event.startsWith("write:")),
      false,
    );
    await service.close();
  }
});

test("loaded source digests are checked and rejected candidates are freed", async () => {
  const { create, events } = fixture({ digest: "foreign key" });
  const service = await create();
  await assert.rejects(
    service.preload(first),
    /Source proving key digest mismatch/,
  );
  await service.close();
  assert.deepEqual(events.slice(-3), ["decode:1", "free:1", "runtime:close"]);
});

test("preload refuses and frees a decoded key with the wrong circuit dimensions", async () => {
  const { create, events } = fixture({ wrongShape: true });
  const service = await create();
  await assert.rejects(
    service.preload(first),
    /Decoded proving key shape mismatch/,
  );
  await service.close();
  assert.deepEqual(events.slice(-3), ["decode:1", "free:1", "runtime:close"]);
});

test("catalog/source pin mismatches fail before runtime initialization", async () => {
  let started = false;
  const create = localProofServiceFactory(
    () => {
      started = true;
    },
    () => ({}),
    { catalog, pins: {} },
  );
  await assert.rejects(create(), /does not match the SDK deployment/);
  assert.equal(started, false);
});

test("one service owns the singleton runtime", async () => {
  const { create, events } = fixture();
  const service = await create();
  await assert.rejects(create(), /already owns/);
  await service.close();
  await assert.rejects(create(), /already owns/);
  assert.deepEqual(events, ["init:0", "runtime:close"]);
});

test("invalid options do not claim a runtime and corrected options can initialize", async () => {
  const { create, events } = fixture();
  for (const threads of [-1, 1.5, 65, NaN, "4", null, false])
    await assert.rejects(create({ threads }), RangeError);
  for (const options of [
    null,
    "invalid",
    [],
    { keyDirectory: 1 },
    { keyDirectory: "" },
    { keyDirectory: "bad\0path" },
  ]) {
    await assert.rejects(create(options), TypeError);
  }
  assert.deepEqual(events, []);
  const service = await create({ threads: 4 });
  await service.close();
  assert.deepEqual(events, ["init:4", "runtime:close"]);
});

test("a racing duplicate store and a failed initializer release their storage", async () => {
  let stores = 0;
  const closed = [];
  const create = localProofServiceFactory(
    async () => ({ close: async () => {}, SprProver: class {} }),
    async () => {
      const id = ++stores;
      await Promise.resolve();
      return {
        close: () => {
          closed.push(id);
        },
      };
    },
    { catalog, pins },
  );
  const [one, two] = await Promise.allSettled([create(), create()]);
  assert.equal(one.status, "fulfilled");
  assert.equal(two.status, "rejected");
  assert.deepEqual(closed, [2]);
  await one.value.close();
  assert.deepEqual(closed, [2, 1]);
  let released = false;
  const failed = localProofServiceFactory(
    async () => {
      throw new Error("WASM initialization failed");
    },
    () => ({
      close: () => {
        released = true;
      },
    }),
    { catalog, pins },
  );
  await assert.rejects(failed(), /WASM initialization failed/);
  assert.equal(released, true);
  await assert.rejects(failed(), /already owns/);
});

test("cancelled queued work does not load another key", async () => {
  let finish;
  const { create, events } = fixture({
    proving: () =>
      new Promise((resolve) => {
        finish = () => resolve(proof);
      }),
  });
  const service = await create();
  await service.preload(first);
  const pending = service.prove(transfer());
  await new Promise((resolve) => setImmediate(resolve));
  const controller = new AbortController();
  const cancelled = service.prove(transfer(2), { signal: controller.signal });
  controller.abort();
  finish();
  await pending;
  await assert.rejects(cancelled, { code: "CLIENT_ABORTED" });
  assert.equal(events.includes("read:second.zopro"), false);
  await service.close();
});

test("Node key storage reads misses and atomically writes a single checked artifact", async () => {
  const directory = await mkdtemp(join(tmpdir(), "zopro-service-store-"));
  try {
    const store = nodeKeyStore({ keyDirectory: directory }, "test");
    const artifact = catalog.keys[first];
    assert.equal(await store.read(artifact), undefined);
    await store.write(artifact, files.get(artifact.file));
    assert.deepEqual(
      new Uint8Array(await store.read(artifact)),
      files.get(artifact.file),
    );
    assert.equal(
      await store.sha256(await store.read(artifact)),
      artifact.sha256,
    );
    assert.deepEqual(await readdir(directory), [artifact.file]);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test("browser storage survives unavailable Cache API and quota errors with one memory entry", async () => {
  for (const storage of [
    null,
    {
      open: async () => {
        throw new Error("disabled");
      },
    },
    {
      open: async () => ({
        put: async () => {
          throw new Error("quota");
        },
      }),
    },
  ]) {
    const store = await browserKeyStore({}, "test", storage);
    await store.write(
      catalog.keys[first],
      files.get("first.zopro"),
      "https://keys.example/first",
    );
    assert.equal(
      await store.sha256(await store.read(catalog.keys[first])),
      catalog.keys[first].sha256,
    );
    await store.write(
      catalog.keys[second],
      files.get("second.zopro"),
      "https://keys.example/second",
    );
    assert.equal(await store.read(catalog.keys[first]), undefined);
    assert.deepEqual(
      await store.read(catalog.keys[second]),
      files.get("second.zopro"),
    );
    store.close();
    assert.equal(await store.read(catalog.keys[second]), undefined);
  }
  await assert.rejects(
    browserKeyStore({ keyDirectory: "/node-only" }, "test"),
    /only supported in Node/,
  );
});

test("browser persistent Cache API storage uses versioned URLs and reads its stored bytes", async () => {
  const kept = new Map();
  let cacheName;
  const store = await browserKeyStore({}, "fixture-set", {
    open: async (name) => {
      cacheName = name;
      return {
        match: async (url) =>
          kept.has(url) ? new Response(kept.get(url)) : undefined,
        put: async (url, response) => {
          kept.set(url, new Uint8Array(await response.arrayBuffer()));
        },
      };
    },
  });
  await store.write(
    catalog.keys[first],
    files.get("first.zopro"),
    "https://keys.example/first",
  );
  assert.deepEqual(
    await store.read(catalog.keys[first], "https://keys.example/first"),
    files.get("first.zopro"),
  );
  assert.equal(cacheName, "zopro-spr-proving-keys-fixture-set");
});
