import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { fileURLToPath } from "node:url";
import { test } from "node:test";

for (const threads of [0, 4]) {
  test(
    `public Node proof service preloads its production key (${threads} threads) and exits`,
    { timeout: 90_000 },
    async () => {
      const { stdout } = await promisify(execFile)(
        process.execPath,
        [
          fileURLToPath(new URL("./service-node-proof.mjs", import.meta.url)),
          String(threads),
        ],
        { timeout: 80_000 },
      );
      assert.deepEqual(JSON.parse(stdout.trim()), {
        threads,
        preloaded: true,
        proofBytes: [64, 128, 64],
      });
    },
  );
}
