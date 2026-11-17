import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { fileURLToPath } from "node:url";
import { test } from "node:test";

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
