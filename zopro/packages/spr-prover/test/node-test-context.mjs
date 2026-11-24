import assert from "node:assert/strict";
import { test } from "node:test";
import { init } from "@heliuslabs/zopro-spr";

test(
  "initialize and close Rayon from the Node test runner",
  { timeout: 15_000 },
  async () => {
    const runtime = await init({ threads: 2 });
    try {
      assert.equal(runtime.threads, 2);
    } finally {
      await runtime.close();
    }
    console.log("RAYON_INITIALIZED_AND_CLOSED");
  },
);
