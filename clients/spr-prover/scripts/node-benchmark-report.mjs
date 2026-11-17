import assert from "node:assert/strict";
import { readFile, writeFile } from "node:fs/promises";
import { basename, resolve } from "node:path";
import { fileURLToPath } from "node:url";

export function statistics(samples) {
  assert.ok(samples.length > 0, "No warm samples");
  const sorted = samples.map((sample) => sample.totalMs).sort((a, b) => a - b);
  assert.ok(sorted.every((value) => Number.isFinite(value) && value >= 0), "Invalid timing");
  const middle = Math.floor(sorted.length / 2);
  return { median: sorted.length % 2 ? sorted[middle] : (sorted[middle - 1] + sorted[middle]) / 2,
    min: sorted[0], max: sorted.at(-1) };
}

export function render(report, source = "results.json") {
  assert.equal(report.schema, 2);
  assert.ok(report.completedAt, "Refusing to publish an incomplete benchmark");
  const rows = new Map();
  const key = (shape, mode) => `${shape}/${mode.backend}/${mode.threads}`;
  for (const row of report.results) {
    const id = key(row.shape, row);
    assert.ok(!rows.has(id), `Duplicate result: ${id}`);
    rows.set(id, row);
    if (row.status === "passed") {
      assert.equal(row.fresh.status, "passed");
      assert.equal(row.restart.status, "passed");
      assert.equal(row.fresh.samples.length, report.iterations);
      statistics(row.fresh.samples);
      assert.ok([row.fresh.firstMs, row.restart.firstMs].every((n) => Number.isFinite(n) && n >= 0));
    }
  }
  assert.equal(rows.size, report.shapes.length * report.modes.length, "Missing results");
  for (const shape of report.shapes) for (const mode of report.modes) assert.ok(rows.has(key(shape, mode)));
  const shapes = [...report.shapes].sort((a, b) => {
    const parts = (s) => [Number(s.startsWith("merge-")), ...s.replace("merge-", "").split("x").map(Number)];
    const aa = parts(a), bb = parts(b);
    return aa[0] - bb[0] || aa[1] - bb[1] || aa[2] - bb[2];
  });
  const modeName = (mode) => `${mode.backend === "gpu" ? "GPU +" : "CPU"} ${mode.threads || 1} thread${mode.threads === 1 || mode.threads === 0 ? "" : "s"}`;
  const shapeName = (shape) => `${shape.startsWith("merge-") ? "Merge" : "Transfer"} ${shape.replace("merge-", "").replace("x", " → ")}`;
  const passed = report.results.filter((r) => r.status === "passed").length;
  const lines = [
    "# Node prover time to proof", "",
    `Measured ${report.startedAt.slice(0, 10)} on **${report.cpu}**, ${report.platform}/${report.arch}, Node ${report.node}, ${report.memoryGiB} GiB RAM. GPU binding: Dawn \`webgpu@${report.gpuBinding}\`.`, "",
    `${report.shapes.length} shapes × ${report.modes.length} modes; **${passed}/${report.results.length} cases passed**. Every returned proof was locally verified; GPU cases require actual GPU execution. Requests are synthetic.`, "",
    "- **Warm:** median of repeated `proveRequest()` calls with the shape already resident. Includes witness generation, proving, local verification and worker messaging; excludes key preparation.",
    "- **Empty cache:** first proof in a fresh process and empty disk cache. Includes worker/GPU initialization, CDN download, key decoding, cache writes, proving and verification. Network and CDN conditions affect this value.",
    "- **Cached restart:** first proof in another process using the decoded disk cache. Network fetches are forbidden in this phase. Includes worker/GPU startup and loading the cached key.",
    `- Each shape/mode gets its own processes and cache. Cases run sequentially. Warm sample count: ${report.iterations}; the ranges below are observed min–max, not percentiles.`, "",
    "Circuit labels mean input notes → output notes. Merge 52→1 exceeds the WASM32 memory budget.", "",
    "## Warm verified proof latency", "", "Milliseconds: **median** (min–max).", "",
    `| Circuit | ${report.modes.map(modeName).join(" | ")} |`,
    `| --- | ${report.modes.map(() => "---:").join(" | ")} |`,
  ];
  for (const shape of shapes) lines.push(`| ${shapeName(shape)} | ${report.modes.map((mode) => {
    const row = rows.get(key(shape, mode));
    if (row.status !== "passed") return "Failed";
    const s = statistics(row.fresh.samples);
    return `**${s.median.toFixed(1)}** (${s.min.toFixed(1)}–${s.max.toFixed(1)})`;
  }).join(" | ")} |`);
  lines.push("", "## First verified proof latency", "", "Seconds, measured once per case.", "",
    `| Circuit | ${report.modes.flatMap((m) => [`${modeName(m)} empty`, `${modeName(m)} cached`]).join(" | ")} |`,
    `| --- | ${report.modes.flatMap(() => ["---:", "---:"]).join(" | ")} |`);
  for (const shape of shapes) lines.push(`| ${shapeName(shape)} | ${report.modes.flatMap((mode) => {
    const row = rows.get(key(shape, mode));
    return [row.fresh, row.restart].map((phase) => phase?.status === "passed" ? (phase.firstMs / 1000).toFixed(2) : "Failed");
  }).join(" | ")} |`);
  const failures = report.results.filter((r) => r.status !== "passed");
  if (failures.length) {
    lines.push("", "## Failed cases", "");
    for (const row of failures) lines.push(`- ${shapeName(row.shape)}, ${modeName(row)}: ${row.fresh.error ?? row.restart?.error}. No successful timing is substituted.`);
  }
  lines.push("", "## Reproduce", "", "```sh", "npm ci", "npm run build:prover",
    `SPR_BENCH_SHAPES=all SPR_BENCH_MODES=${report.modes.map((m) => `${m.backend}:${m.threads}`).join(",")} \\`,
    `SPR_BENCH_RUNS=${report.iterations} SPR_BENCH_OUTPUT=target/node-benchmark/all-shapes.json npm run bench:node`,
    "node scripts/node-benchmark-report.mjs target/node-benchmark/all-shapes.json target/node-benchmark/all-shapes.md", "```", "",
    `Raw samples, preparation timings, failures and machine metadata: [${basename(source)}](${basename(source)}).`, "",
    "Metadata in the raw file records source revisions and SHA-256 digests of the harness and fixtures.", "",
    "These timings apply to this host and run. Empty-cache latency also depends on CDN/network conditions.", "");
  return lines.join("\n");
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const [input, output] = process.argv.slice(2);
  assert.ok(input && output, "Usage: node scripts/node-benchmark-report.mjs results.json report.md");
  await writeFile(output, render(JSON.parse(await readFile(input, "utf8")), input));
}
