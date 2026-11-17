import assert from "node:assert/strict";
import { test } from "node:test";
import { render, statistics } from "./node-benchmark-report.mjs";

test("statistics preserve fractional timings and compute even/odd medians", () => {
  assert.deepEqual(statistics([9.2, 1.1, 2.3].map((totalMs) => ({ totalMs }))), { min: 1.1, median: 2.3, max: 9.2 });
  assert.equal(statistics([1, 2, 4, 9].map((totalMs) => ({ totalMs }))).median, 3);
  assert.throws(() => statistics([]));
  assert.throws(() => statistics([{ totalMs: NaN }]));
});

test("report rejects missing/duplicate samples and preserves failed cases", () => {
  const report = { schema: 2, completedAt: "now", startedAt: "2026-11-17", shapes: ["2x2"], modes: [{ backend: "gpu", threads: 4 }], iterations: 2,
    results: [{ shape: "2x2", backend: "gpu", threads: 4, status: "passed",
      fresh: { status: "passed", firstMs: 3456, samples: [{ totalMs: 7.5 }, { totalMs: 8.5 }] },
      restart: { status: "passed", firstMs: 123 } }] };
  assert.match(render(report), /\*\*8\.0\*\* \(7\.5–8\.5\)/);
  assert.match(render(report), /3\.46 \| 0\.12/);
  assert.throws(() => render({ ...report, completedAt: null }));
  assert.throws(() => render({ ...report, results: [] }));
  assert.throws(() => render({ ...report, results: [...report.results, ...report.results] }));
  assert.throws(() => render({ ...report, iterations: 5 }));
  report.results[0] = { shape: "2x2", backend: "gpu", threads: 4, status: "failed", fresh: { status: "failed", error: "timeout" } };
  assert.match(render(report), /Failed cases/);
  assert.match(render(report), /0\/1 cases passed/);
  assert.match(render(report), /timeout/);
});
