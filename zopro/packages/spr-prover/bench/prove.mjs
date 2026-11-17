import { homedir, loadavg } from "node:os";
import { mkdir, open, readFile, realpath } from "node:fs/promises";
import { join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { parseArgs } from "node:util";
import { campaign } from "./campaign.mjs";
import { runtimeFlags } from "./provenance.mjs";
import { keyBytes } from "./keys.mjs";
import {
  DEFAULT_ORDER_SEED,
  caseId,
  csv,
  orderedCases,
  quantile,
  summarizeIterations,
} from "./records.mjs";
export { processFailure } from "./records.mjs";

export const THREAD_COUNTS = Object.freeze([1, 2, 4, 8, 18]);
export const DEFAULT_ROUNDS = 10;
export const DEFAULT_WARMUP = 3;
const lockPath = new URL("../../../tools/spr/zopro-keys.lock", import.meta.url);

export function circuitShape(name) {
  const match = /^(transfer_confidential|merge)_(\d+)_(\d+)\.key$/u.exec(name);
  if (!match)
    throw new Error("Unsupported circuit in the key catalog: " + name);
  return {
    circuit: match[1],
    inputs: Number(match[2]),
    outputs: Number(match[3]),
  };
}

export function matrix(keys, threads = THREAD_COUNTS) {
  return Object.keys(keys)
    .map((key) => ({ key, ...circuitShape(key) }))
    .sort(
      (a, b) =>
        a.circuit.localeCompare(b.circuit) ||
        a.inputs - b.inputs ||
        a.outputs - b.outputs,
    )
    .flatMap((shape) => threads.map((threads) => ({ ...shape, threads })));
}

export function median(values) {
  return quantile(values, 0.5);
}

function integer(value, name, minimum = 1) {
  const number = Number(value);
  if (!Number.isSafeInteger(number) || number < minimum)
    throw new Error(name + " must be an integer >= " + minimum);
  return number;
}

/** Setup is complete before any sample timer begins. The driver override is for boundary tests. */
export async function runCase(lock, entry, options, driver) {
  if (!driver) {
    const { benchmarkInputs } = await import("./inputs.mjs");
    const { init } = await import("@heliuslabs/zopro-spr");
    driver = { benchmarkInputs, init, keyBytes, now: () => performance.now() };
  }
  const clock = driver.now;
  const inputStart = clock();
  const inputs = await driver.benchmarkInputs(entry.key);
  const inputSetupMs = clock() - inputStart;
  const artifact = lock.keys[entry.key];
  const readStart = clock();
  const key = await driver.keyBytes(lock, entry.key, options.keyDirectory);
  const readMs = clock() - readStart;
  const initStart = clock();
  const runtime = await driver.init({ threads: entry.threads });
  const initMs = clock() - initStart;
  let prover;
  let samplesFile;
  const iterations = [];
  try {
    const loadStart = clock();
    prover = new runtime.SprProver(key);
    const loadMs = clock() - loadStart;
    if (
      prover.digest() !== artifact.key_sha256 ||
      prover.inputs() !== entry.inputs ||
      prover.outputs() !== entry.outputs
    ) {
      throw new Error(
        "Loaded proving key does not match the benchmark circuit",
      );
    }
    if (options.recordDirectory) {
      await mkdir(options.recordDirectory, { recursive: true });
      samplesFile = await open(
        join(options.recordDirectory, "iterations.jsonl"),
        "wx",
      );
    }
    const merge = entry.circuit === "merge";
    const prove = merge
      ? prover.proveMerge.bind(prover)
      : prover.prove.bind(prover);
    const verify = merge
      ? prover.verifyMerge.bind(prover)
      : prover.verify.bind(prover);
    for (let round = 0; round < options.warmup + options.rounds; round++) {
      const phase = round < options.warmup ? "warmup" : "measured";
      const sample = {
        case_id: caseId(entry),
        key: entry.key,
        threads: entry.threads,
        case_index: options.caseIndex ?? null,
        run_fingerprint: options.fingerprint ?? null,
        session_id: options.sessionId ?? null,
        attempt: options.attempt ?? 1,
        phase,
        iteration: phase === "warmup" ? round + 1 : round - options.warmup + 1,
        started_at: new Date().toISOString(),
        status: "error",
        verified: false,
      };
      const proving = clock();
      try {
        const proof = await prove(inputs);
        sample.prove_ms = clock() - proving;
        const verifying = clock();
        sample.verified = verify(inputs, proof);
        sample.verify_ms = clock() - verifying;
        if (!sample.verified)
          throw new Error("Benchmark proof failed verification");
        sample.status = "ok";
      } catch (error) {
        sample.error = error.message;
        throw error;
      } finally {
        sample.finished_at = new Date().toISOString();
        sample.load_avg = loadavg();
        sample.rss_bytes = process.memoryUsage.rss();
        iterations.push(sample);
        await samplesFile?.write(JSON.stringify(sample) + "\n");
      }
    }
    await samplesFile?.sync();
    return {
      ...entry,
      case_id: caseId(entry),
      status: "ok",
      rounds: options.rounds,
      warmup: options.warmup,
      ...summarizeIterations(iterations, options),
      key_bytes: key.byteLength,
      input_setup_ms: inputSetupMs,
      key_read_ms: readMs,
      init_ms: initMs,
      key_load_ms: loadMs,
      artifact_sha256: artifact.sha256,
      source_key_sha256: artifact.key_sha256,
      peak_rss_mb: process.resourceUsage().maxRSS / 1024,
      resource_usage: process.resourceUsage(),
      process_id: process.pid,
      node: process.version,
      exec_arguments: runtimeFlags(process.execArgv),
      iterations,
    };
  } finally {
    try {
      await samplesFile?.close();
    } finally {
      try {
        prover?.free();
      } finally {
        await runtime.close();
      }
    }
  }
}

export async function main(args = process.argv.slice(2)) {
  const { values } = parseArgs({
    args,
    options: {
      help: { type: "boolean", short: "h" },
      list: { type: "boolean" },
      circuits: { type: "string" },
      threads: { type: "string", default: THREAD_COUNTS.join(",") },
      rounds: { type: "string", default: String(DEFAULT_ROUNDS) },
      warmup: { type: "string", default: String(DEFAULT_WARMUP) },
      "keys-dir": { type: "string" },
      "timeout-ms": { type: "string", default: "1800000" },
      output: { type: "string" },
      format: { type: "string", default: "csv" },
      "run-dir": { type: "string" },
      resume: { type: "boolean" },
      seed: { type: "string", default: DEFAULT_ORDER_SEED },
      "fail-fast": { type: "boolean" },
      case: { type: "string" },
      "record-dir": { type: "string" },
      "run-fingerprint": { type: "string" },
      "session-id": { type: "string" },
      attempt: { type: "string" },
      "case-index": { type: "string" },
    },
  });
  if (values.help) {
    console.log(
      "Usage: npm run bench -- [--list] [--circuits 1x2,3x8,merge-8x1] [--threads 1,2,4,8,18] [--rounds 10] [--warmup 3] [--seed zopro-spr-matrix-v1] [--keys-dir PATH] [--run-dir RESULTS] [--resume] [--output summary.csv] [--format csv|jsonl] [--timeout-ms 1800000] [--fail-fast]",
    );
    return;
  }
  const lock = JSON.parse(await readFile(lockPath, "utf8"));
  const threads = values.threads
    .split(",")
    .map((value) => integer(value, "threads"));
  if (
    threads.some((count) => count > 64) ||
    new Set(threads).size !== threads.length
  )
    throw new Error("Use distinct thread counts between 1 and 64");
  const options = {
    rounds: integer(values.rounds, "rounds"),
    warmup: integer(values.warmup, "warmup", 0),
    keyDirectory: resolve(
      values["keys-dir"] ??
        process.env.ZOPRO_KEYS_DIR ??
        join(homedir(), ".cache", "zopro-wasm", lock.prefix.split("/").at(-1)),
    ),
    recordDirectory: values["record-dir"],
    fingerprint: values["run-fingerprint"],
    sessionId: values["session-id"],
    attempt: values.attempt ? integer(values.attempt, "attempt") : undefined,
    caseIndex: values["case-index"]
      ? integer(values["case-index"], "case-index")
      : undefined,
  };
  if (values.case) {
    if (!(values.case in lock.keys) || threads.length !== 1)
      throw new Error(
        "An isolated case needs one catalog key and one thread count",
      );
    console.log(
      JSON.stringify(
        await runCase(
          lock,
          {
            key: values.case,
            ...circuitShape(values.case),
            threads: threads[0],
          },
          options,
        ),
      ),
    );
    return;
  }
  let cases = matrix(lock.keys, threads);
  if (values.circuits) {
    const names = values.circuits
      .split(",")
      .map((name) =>
        name.endsWith(".key")
          ? name
          : name.startsWith("merge-")
            ? "merge_" + name.slice(6).replace("x", "_") + ".key"
            : "transfer_confidential_" + name.replace("x", "_") + ".key",
      );
    for (const name of names)
      if (!(name in lock.keys))
        throw new Error("Unknown benchmark circuit: " + name);
    cases = cases.filter((entry) => names.includes(entry.key));
  }
  cases = orderedCases(cases, values.seed);
  if (!["csv", "jsonl"].includes(values.format))
    throw new Error("format must be csv or jsonl");
  if (values.list) {
    if (values.format === "csv")
      console.log("key,circuit,inputs,outputs,threads");
    for (const entry of cases)
      console.log(
        values.format === "jsonl"
          ? JSON.stringify(entry)
          : ["key", "circuit", "inputs", "outputs", "threads"]
              .map((name) => csv(entry[name]))
              .join(","),
      );
    return;
  }
  if (values.resume && !values["run-dir"])
    throw new Error("--resume requires --run-dir");
  const runId =
    new Date().toISOString().replace(/[:.]/gu, "-") +
    "-" +
    options.warmup +
    "w" +
    options.rounds +
    "r";
  const runDirectory = resolve(
    values["run-dir"] ??
      fileURLToPath(new URL("./results/" + runId, import.meta.url)),
  );
  await campaign({
    lock,
    cases,
    options,
    timeout: integer(values["timeout-ms"], "timeout-ms"),
    seed: values.seed,
    runDirectory,
    resume: values.resume,
    output: values.output,
    format: values.format,
    failFast: values["fail-fast"],
    script: fileURLToPath(import.meta.url),
  });
}

const entryFile = process.argv[1]
  ? await realpath(resolve(process.argv[1])).catch(() => null)
  : null;
if (entryFile && import.meta.url === pathToFileURL(entryFile).href)
  await main();
