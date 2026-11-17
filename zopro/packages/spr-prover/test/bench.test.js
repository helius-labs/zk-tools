import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { readFile } from "node:fs/promises";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import {
  THREAD_COUNTS,
  circuitShape,
  matrix,
  median,
} from "../bench/prove.mjs";

const lock = JSON.parse(
  await readFile(
    new URL("../../../tools/spr/zopro-keys.lock", import.meta.url),
    "utf8",
  ),
);

test("benchmark covers every catalog key at 1, 2, 4, 8 and 18 threads", () => {
  const cases = matrix(lock.keys);
  assert.deepEqual(THREAD_COUNTS, [1, 2, 4, 8, 18]);
  assert.equal(
    cases.length,
    Object.keys(lock.keys).length * THREAD_COUNTS.length,
  );
  assert.equal(
    new Set(cases.map(({ key, threads }) => `${key}:${threads}`)).size,
    cases.length,
  );
  for (const key of Object.keys(lock.keys)) {
    assert.deepEqual(
      cases.filter((entry) => entry.key === key).map(({ threads }) => threads),
      THREAD_COUNTS,
    );
  }
  assert.ok(
    cases.some(({ inputs, outputs }) => inputs === 49 && outputs === 2),
  );
  assert.ok(
    cases.some(({ circuit, inputs }) => circuit === "merge" && inputs === 52),
  );
  assert.ok(cases.some(({ outputs }) => outputs === 16));
  assert.throws(() => circuitShape("unknown.key"), /Unsupported circuit/);
});

test("median sorts without changing the samples", () => {
  const samples = [5, 1, 3];
  assert.equal(median(samples), 3);
  assert.deepEqual(samples, [5, 1, 3]);
  assert.equal(median([5, 1, 3, 7]), 4);
});

test("benchmark list exposes the whole matrix without downloading or initializing WASM", async () => {
  const { stdout } = await promisify(execFile)(process.execPath, [
    fileURLToPath(new URL("../bench/prove.mjs", import.meta.url)),
    "--list",
    "--format",
    "jsonl",
  ]);
  assert.deepEqual(
    stdout
      .trim()
      .split("\n")
      .map((line) => JSON.parse(line)),
    matrix(lock.keys),
  );
});

test("benchmark refuses unknown shapes rather than omitting them", async () => {
  await assert.rejects(
    promisify(execFile)(process.execPath, [
      fileURLToPath(new URL("../bench/prove.mjs", import.meta.url)),
      "--list",
      "--circuits",
      "999x999",
    ]),
    /Unknown benchmark circuit/,
  );
});
