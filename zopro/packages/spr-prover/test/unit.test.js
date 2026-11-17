import assert from "node:assert/strict";
import { test } from "node:test";
import { initializer } from "../src/init.js";
import { threadPool } from "../src/pool.js";

test("reject invalid thread counts before loading WASM", async () => {
  const init = initializer(() => {
    throw new Error("must not load");
  });
  for (const threads of [-1, 0.5, 65, NaN, Infinity, "4"]) {
    await assert.rejects(init({ threads }), RangeError);
  }
});

test("initialize once, fix the pool size, and close once", async () => {
  let loads = 0;
  let starts = 0;
  let closes = 0;
  const init = initializer(async () => {
    loads++;
    return {
      module: {
        SprProver: class {},
        initThreadPool: async () => {
          starts++;
        },
      },
      initialize: async () => {},
      close: async () => {
        closes++;
      },
    };
  });
  const [first, second] = await Promise.all([
    init({ threads: 2 }),
    init({ threads: 2 }),
  ]);
  assert.equal(first, second);
  await assert.rejects(init({ threads: 4 }), /thread count is fixed/);
  await Promise.all([first.close(), second.close()]);
  assert.throws(
    () => new first.SprProver(new Uint8Array()),
    /runtime is closed/,
  );
  await assert.rejects(init({ threads: 2 }), /closed/);
  assert.deepEqual(
    { loads, starts, closes },
    { loads: 1, starts: 1, closes: 1 },
  );
});

test("failed pool initialization cleans up and cannot reuse the runtime", async () => {
  let closed = false;
  const init = initializer(async () => ({
    module: {
      initThreadPool: async () => {
        throw new Error("startup failed");
      },
    },
    initialize: async () => {},
    close: async () => {
      closed = true;
    },
  }));
  await assert.rejects(init({ threads: 2 }), /startup failed/);
  assert.equal(closed, true);
  await assert.rejects(init({ threads: 2 }), /closed/);
});

test("one failed worker terminates every worker and cancels sibling startup waits", async () => {
  let spawned = 0;
  let terminated = 0;
  let listeners = 0;
  const pool = threadPool(() => {
    const failing = ++spawned === 1;
    let fail;
    return {
      listen(message, error) {
        listeners++;
        fail = error;
        return () => {
          listeners--;
        };
      },
      send() {
        if (failing) queueMicrotask(() => fail(new Error("worker failed")));
      },
      terminate() {
        terminated++;
      },
    };
  });
  await assert.rejects(
    pool.startWorkers(
      {},
      {},
      {
        numThreads: () => 4,
        receiver: () => 1,
        build: () => {
          throw new Error("pool must not build");
        },
      },
    ),
    /worker failed/,
  );
  assert.deepEqual(
    { spawned, terminated, listeners },
    { spawned: 4, terminated: 4, listeners: 0 },
  );
  await pool.close();
  assert.equal(terminated, 4);
});
