import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import {
  copyFile,
  mkdir,
  mkdtemp,
  readFile,
  rm,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { createHash } from "node:crypto";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import {
  THREAD_COUNTS,
  circuitShape,
  matrix,
  median,
  processFailure,
  DEFAULT_ROUNDS,
  DEFAULT_WARMUP,
  runCase,
} from "../bench/prove.mjs";
import {
  DEFAULT_ORDER_SEED,
  matchingRun,
  orderedCases,
  stableJson,
  statistics,
  summarizeIterations,
} from "../bench/records.mjs";
import { readIterations, runIdentity } from "../bench/campaign.mjs";
import {
  assertBenchmarkEnvironment,
  runtimeFlags,
  sourceMetadata,
  stableEnvironment,
} from "../bench/provenance.mjs";

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

test("process failures retain termination details when SDK warnings are on stderr", () => {
  const error = Object.assign(
    new Error("Command failed: node benchmark case"),
    {
      stderr: "using deprecated parameters for the initialization function",
      code: null,
      signal: "SIGTERM",
      killed: true,
    },
  );
  const reported = processFailure(error, 500);
  assert.match(reported, /Command failed/u);
  assert.match(reported, /signal=SIGTERM killed=true timeout_ms=500/u);
  assert.match(reported, /using deprecated parameters/u);
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
    orderedCases(matrix(lock.keys), DEFAULT_ORDER_SEED),
  );
});

test("full benchmark defaults are three warmups and ten measurements", () => {
  assert.equal(DEFAULT_WARMUP, 3);
  assert.equal(DEFAULT_ROUNDS, 10);
  assert.equal(
    matrix(lock.keys).length * (DEFAULT_WARMUP + DEFAULT_ROUNDS),
    2665,
  );
});

test("seeded case ordering is reproducible and preserves the entire matrix", () => {
  const original = matrix(lock.keys);
  const ordered = orderedCases(original);
  assert.deepEqual(ordered, orderedCases(original));
  assert.notDeepEqual(ordered, orderedCases(original, "different-order-seed"));
  assert.deepEqual(
    new Set(ordered.map(({ key, threads }) => key + ":" + threads)),
    new Set(original.map(({ key, threads }) => key + ":" + threads)),
  );
  assert.ok(
    new Set(ordered.slice(0, 10).map(({ threads }) => threads)).size > 1,
  );
  assert.deepEqual(original, matrix(lock.keys));
});

test("sample dispersion and interpolated p95 are explicit, without invented one-sample SD", () => {
  const result = statistics([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  assert.equal(result.count, 10);
  assert.equal(result.min_ms, 1);
  assert.equal(result.median_ms, 5.5);
  assert.equal(result.max_ms, 10);
  assert.equal(result.p95_ms, 9.549999999999999);
  assert.equal(result.sd_ms, Math.sqrt(82.5 / 9));
  assert.equal(result.cv_percent, (100 * Math.sqrt(82.5 / 9)) / 5.5);
  assert.equal(statistics([7]).sd_ms, null);
  assert.equal(statistics([7]).cv_percent, null);
  assert.throws(() => statistics([]), /nonempty/);
  assert.throws(() => statistics([NaN]), /finite/);
});

test("summaries exclude warmups and reject incomplete or unverified iterations", () => {
  const iterations = [
    ...Array.from({ length: 3 }, (_, index) => ({
      phase: "warmup",
      iteration: index + 1,
      prove_ms: 1000,
      verify_ms: 100,
      verified: true,
      status: "ok",
    })),
    ...Array.from({ length: 10 }, (_, index) => ({
      phase: "measured",
      iteration: index + 1,
      prove_ms: index + 1,
      verify_ms: 0.5,
      verified: true,
      status: "ok",
    })),
  ];
  const summary = summarizeIterations(iterations, { warmup: 3, rounds: 10 });
  assert.equal(summary.prove_median_ms, 5.5);
  assert.equal(summary.warmup_completed, 3);
  assert.equal(summary.measured_completed, 10);
  assert.throws(
    () => summarizeIterations(iterations.slice(1), { warmup: 3, rounds: 10 }),
    /Incomplete/,
  );
  assert.throws(
    () =>
      summarizeIterations(
        iterations.map((sample, index) =>
          index === 5 ? { ...sample, verified: false } : sample,
        ),
        { warmup: 3, rounds: 10 },
      ),
    /verification/,
  );
});

test("input/key setup and key decode are outside every raw sample timer", async () => {
  const directory = await mkdtemp(join(tmpdir(), "zopro-bench-timers-"));
  let time = 0;
  let freed = false;
  let closed = false;
  const driver = {
    now: () => time,
    benchmarkInputs: async () => {
      time += 17;
      return {};
    },
    keyBytes: async () => {
      time += 101;
      return new Uint8Array(8);
    },
    init: async () => {
      time += 200;
      return {
        SprProver: class {
          constructor() {
            time += 5000;
          }
          digest() {
            return "source-key";
          }
          inputs() {
            return 1;
          }
          outputs() {
            return 2;
          }
          async prove() {
            time += 2;
            return {};
          }
          verify() {
            time += 0.5;
            return true;
          }
          free() {
            freed = true;
          }
        },
        close: async () => {
          closed = true;
        },
      };
    },
  };
  try {
    const result = await runCase(
      {
        keys: {
          "transfer_confidential_1_2.key": {
            key_sha256: "source-key",
            sha256: "artifact",
          },
        },
      },
      {
        key: "transfer_confidential_1_2.key",
        circuit: "transfer_confidential",
        inputs: 1,
        outputs: 2,
        threads: 4,
      },
      { warmup: 3, rounds: 10, recordDirectory: directory },
      driver,
    );
    assert.deepEqual(
      [
        result.input_setup_ms,
        result.key_read_ms,
        result.init_ms,
        result.key_load_ms,
      ],
      [17, 101, 200, 5000],
    );
    assert.equal(result.prove_median_ms, 2);
    assert.equal(result.verify_median_ms, 0.5);
    assert.equal(result.iterations.length, 13);
    assert.ok(
      result.iterations.every(
        (sample) =>
          sample.prove_ms === 2 && sample.verify_ms === 0.5 && sample.verified,
      ),
    );
    assert.deepEqual(
      await readIterations(join(directory, "iterations.jsonl")),
      result.iterations,
    );
    assert.equal(freed, true);
    assert.equal(closed, true);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test("raw iteration corruption is explicit and resume identity covers code, options and environment", async () => {
  const directory = await mkdtemp(join(tmpdir(), "zopro-bench-records-"));
  try {
    const path = join(directory, "iterations.jsonl");
    await writeFile(path, '{"verified":true}\n{broken');
    await assert.rejects(
      readIterations(path),
      /Malformed raw iteration record/,
    );
    const identity = {
      configuration: { rounds: 10 },
      source: { sha: "a" },
      artifacts: { wasm: "x" },
      environment: { cpu: "M5", power: "AC" },
    };
    const fingerprint = runIdentity(identity);
    matchingRun(
      { fingerprint },
      { fingerprint: runIdentity(JSON.parse(JSON.stringify(identity))) },
    );
    for (const field of [
      "configuration",
      "source",
      "artifacts",
      "environment",
    ]) {
      assert.throws(
        () =>
          matchingRun(
            { fingerprint },
            {
              fingerprint: runIdentity({
                ...identity,
                [field]: { changed: true },
              }),
            },
          ),
        /Resume refused/,
      );
    }
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test("environment capture redacts unsupported arguments instead of publishing them", () => {
  const flags = runtimeFlags([
    "--jitless",
    "--max-old-space-size=4096",
    "--eval",
    "API_KEY=secret",
  ]);
  assert.deepEqual(flags.allowed, ["--jitless", "--max-old-space-size=4096"]);
  assert.equal(flags.redacted_count, 2);
  assert.ok(!JSON.stringify(flags).includes("secret"));
  assert.throws(
    () =>
      assertBenchmarkEnvironment({
        exec_argv: flags,
        selected_environment: { NODE_OPTIONS: null },
        instrumentation: {
          coverage_enabled: false,
          inspector_active: false,
          node_test_context: false,
        },
      }),
    /preflight rejected/,
  );
  assert.throws(
    () =>
      assertBenchmarkEnvironment({
        exec_argv: runtimeFlags([]),
        selected_environment: { NODE_OPTIONS: null },
        instrumentation: {
          coverage_enabled: true,
          inspector_active: false,
          node_test_context: false,
        },
      }),
    /preflight rejected/,
  );
  const host = { cpus: [{ model: "M5" }] };
  assert.equal(
    stableJson(
      stableEnvironment(host, {
        power: {
          source: "AC",
          configuration: { output: "normal" },
          battery: { output: "90%" },
        },
      }),
    ),
    stableJson(
      stableEnvironment(host, {
        power: {
          source: "AC",
          configuration: { output: "normal" },
          battery: { output: "100%" },
        },
        load_avg: [4, 3, 2],
      }),
    ),
  );
});

test("source cleanliness checks the entire checkout, not only the package subdirectory", async () => {
  const directory = await mkdtemp(join(tmpdir(), "zopro-bench-source-"));
  const git = (...args) => promisify(execFile)("git", args, { cwd: directory });
  try {
    await git("init", "-q");
    await mkdir(join(directory, "package"));
    await writeFile(join(directory, "outside.txt"), "committed\n");
    await git("add", ".");
    await git(
      "-c",
      "commit.gpgsign=false",
      "-c",
      "user.name=Benchmark Test",
      "-c",
      "user.email=benchmark@example.invalid",
      "commit",
      "-qm",
      "fixture",
    );
    const output = join(directory, "results[one]");
    assert.equal(
      (await sourceMetadata(output, join(directory, "package"))).clean,
      true,
    );
    await mkdir(output);
    await writeFile(join(output, "raw.jsonl"), "{}\n");
    assert.equal(
      (await sourceMetadata(output, join(directory, "package"))).clean,
      true,
    );
    await writeFile(join(directory, "outside.txt"), "modified\n");
    await assert.rejects(
      sourceMetadata(output, join(directory, "package")),
      /must be clean/,
    );
    await writeFile(join(directory, "outside.txt"), "committed\n");
    await mkdir(join(directory, "resultso"));
    await writeFile(join(directory, "resultso/dirty.txt"), "untracked\n");
    await assert.rejects(
      sourceMetadata(output, join(directory, "package")),
      /resultso\/dirty.txt/,
    );
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test(
  "campaign preserves failed attempts, resumes only matching runs, and refuses raw-data corruption",
  { timeout: 30_000 },
  async () => {
    // This is an orchestration fixture, not a cryptographic/performance measurement.
    const directory = await mkdtemp(join(tmpdir(), "zopro-bench-campaign-"));
    const source = join(directory, "source");
    const root = join(source, "zopro/packages/spr-prover");
    const output = join(directory, "results");
    const keyDirectory = join(directory, "keys");
    const fixtureFile = async (path, content) => {
      await mkdir(dirname(path), { recursive: true });
      await writeFile(path, content);
    };
    const git = (...args) => promisify(execFile)("git", args, { cwd: source });
    const env = {
      ...process.env,
      BENCH_FIXTURE_MARKER: join(directory, "second-attempt"),
    };
    for (const name of [
      "NODE_TEST_CONTEXT",
      "NODE_OPTIONS",
      "NODE_V8_COVERAGE",
    ])
      delete env[name];
    try {
      await mkdir(join(root, "bench"), { recursive: true });
      for (const name of [
        "prove.mjs",
        "campaign.mjs",
        "records.mjs",
        "provenance.mjs",
        "keys.mjs",
      ]) {
        await copyFile(
          new URL("../bench/" + name, import.meta.url),
          join(root, "bench", name),
        );
      }
      await fixtureFile(
        join(root, "bench/inputs.mjs"),
        "export async function benchmarkInputs() { return {}; }\n",
      );
      await fixtureFile(
        join(root, "package.json"),
        JSON.stringify({
          name: "@heliuslabs/zopro-spr",
          type: "module",
          exports: { ".": "./src/node.js" },
        }),
      );
      await fixtureFile(join(root, "package-lock.json"), "{}\n");
      await fixtureFile(join(source, "zopro/Cargo.lock"), "# fixture\n");
      await fixtureFile(join(source, ".gitignore"), "node_modules/\n");
      const key = Buffer.from('{"digest":"fixture-source"}');
      const keyName = "transfer_confidential_1_2.key";
      const keyFile = "transfer_confidential_1_2.zopro";
      await fixtureFile(
        join(source, "zopro/tools/spr/zopro-keys.lock"),
        JSON.stringify({
          prefix: "fixture",
          baseUrl: "https://not-used.invalid/",
          keys: {
            [keyName]: {
              file: keyFile,
              size: key.length,
              sha256: createHash("sha256").update(key).digest("hex"),
              key_sha256: "fixture-source",
            },
          },
        }),
      );
      await fixtureFile(join(keyDirectory, keyFile), key);
      await fixtureFile(
        join(root, "src/node.js"),
        `
      import {readFile,writeFile} from 'node:fs/promises';
      export async function init({threads}) {
        return { SprProver: class {
          constructor(bytes) { this.key=JSON.parse(new TextDecoder().decode(bytes)); }
          digest(){return this.key.digest;} inputs(){return 1;} outputs(){return 2;}
          async prove(){
            if(threads===2) { try{await readFile(process.env.BENCH_FIXTURE_MARKER);}catch(error){
              if(error.code!=='ENOENT')throw error;
              await writeFile(process.env.BENCH_FIXTURE_MARKER,'attempted');
              throw new Error('intentional fixture failure');
            }}
            return {};
          }
          verify(){return true;} free(){}
        },close:async()=>{}};
      }
    `,
      );
      await fixtureFile(
        join(root, "node_modules/@heliuslabs/spr/package.json"),
        JSON.stringify({
          name: "@heliuslabs/spr",
          version: "0.0.0-fixture",
          type: "module",
          exports: { "./client": "./dist/client/index.js" },
        }),
      );
      await fixtureFile(
        join(root, "node_modules/@heliuslabs/spr/dist/client/index.js"),
        "export {};\n",
      );
      for (const path of [
        "single/zopro_spr.js",
        "single/zopro_spr_bg.wasm",
        "threads/zopro_spr.js",
        "threads/zopro_spr_node.js",
        "threads/zopro_spr_bg.wasm",
      ]) {
        await fixtureFile(join(root, "runtime", path), "fixture artifact\n");
      }
      await git("init", "-q");
      await git("add", ".");
      await git(
        "-c",
        "commit.gpgsign=false",
        "-c",
        "user.name=Benchmark Test",
        "-c",
        "user.email=benchmark@example.invalid",
        "commit",
        "-qm",
        "fixture",
      );
      const args = [
        "--max-old-space-size=512",
        join(root, "bench/prove.mjs"),
        "--threads",
        "1,2",
        "--run-dir",
        output,
        "--keys-dir",
        keyDirectory,
      ];
      await assert.rejects(
        promisify(execFile)(process.execPath, args, { env, timeout: 10_000 }),
        { code: 1 },
      );
      const first = JSON.parse(
        await readFile(join(output, "manifest.json"), "utf8"),
      );
      assert.equal(first.status, "incomplete");
      assert.equal(first.completed_cases, 1);
      assert.equal(first.failed_cases, 1);
      await promisify(execFile)(process.execPath, [...args, "--resume"], {
        env,
        timeout: 10_000,
      });
      const completed = JSON.parse(
        await readFile(join(output, "manifest.json"), "utf8"),
      );
      assert.equal(completed.status, "complete");
      assert.equal(completed.sessions.length, 2);
      assert.equal(completed.completed_cases, 2);
      assert.deepEqual(completed.host.exec_argv.allowed, [
        "--max-old-space-size=512",
      ]);
      const firstCase = JSON.parse(
        await readFile(
          join(output, "cases/transfer_confidential_1_2.t1/result.json"),
          "utf8",
        ),
      );
      assert.deepEqual(firstCase.exec_arguments.allowed, [
        "--max-old-space-size=512",
      ]);
      assert.deepEqual(completed.proof_counts.selected_successful_attempts, {
        warmup: 6,
        measured: 20,
      });
      assert.deepEqual(completed.proof_counts.all_attempts, {
        records: 27,
        verified: 26,
        failed: 1,
      });
      const failed = JSON.parse(
        await readFile(
          join(
            output,
            "cases/transfer_confidential_1_2.t2/attempt-1/result.json",
          ),
          "utf8",
        ),
      );
      assert.equal(failed.status, "error");
      assert.match(failed.error, /intentional fixture failure/);
      await assert.rejects(
        promisify(execFile)(
          process.execPath,
          [...args, "--resume", "--rounds", "11"],
          { env, timeout: 10_000 },
        ),
        /Resume refused/,
      );
      await writeFile(
        join(
          output,
          "cases/transfer_confidential_1_2.t1/attempt-1/iterations.jsonl",
        ),
        "",
      );
      await assert.rejects(
        promisify(execFile)(process.execPath, [...args, "--resume"], {
          env,
          timeout: 10_000,
        }),
        /raw samples differ/,
      );
    } finally {
      await rm(directory, { recursive: true, force: true });
    }
  },
);

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
