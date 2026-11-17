import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { fileURLToPath } from "node:url";
import { test } from "node:test";

test(
  "Node test-runner flags do not reach the Rayon workers",
  { timeout: 20_000 },
  async () => {
    const env = { ...process.env };
    delete env.NODE_TEST_CONTEXT;
    const { stdout } = await promisify(execFile)(
      process.execPath,
      [
        "--stack-trace-limit=10",
        "--test",
        "--test-reporter=tap",
        fileURLToPath(new URL("./node-test-context.mjs", import.meta.url)),
      ],
      { env, timeout: 18_000 },
    );
    assert.match(stdout, /^# tests 1$/m);
    assert.match(stdout, /^# pass 1$/m);
    assert.match(stdout, /^# RAYON_INITIALIZED_AND_CLOSED$/m);
  },
);

for (const threads of [0, 1, 2, 4]) {
  test(
    `Node: ${threads} threads, valid randomized proofs and clean process exit`,
    { timeout: 90_000 },
    async () => {
      const { stdout } = await promisify(execFile)(
        process.execPath,
        [
          fileURLToPath(new URL("./node-proof.mjs", import.meta.url)),
          String(threads),
        ],
        { timeout: 80_000 },
      );
      const result = JSON.parse(stdout.trim());
      assert.equal(result.threads, threads);
      assert.ok(result.proveMs >= 0);
    },
  );
}

test(
  "Node: SDK proveMerge with an 8-input key",
  { timeout: 120_000 },
  async () => {
    const { stdout } = await promisify(execFile)(
      process.execPath,
      [
        fileURLToPath(new URL("./node-proof.mjs", import.meta.url)),
        "4",
        "merge",
      ],
      { timeout: 110_000 },
    );
    assert.equal(JSON.parse(stdout.trim()).threads, 4);
  },
);
