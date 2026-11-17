import { execFile } from "node:child_process";
import { createHash, randomUUID } from "node:crypto";
import { mkdir, open, readFile, rename, rm, writeFile } from "node:fs/promises";
import { availableParallelism, cpus, homedir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { parseArgs, promisify } from "node:util";

export const THREAD_COUNTS = Object.freeze([1, 2, 4, 8, 18]);
const lockPath = new URL("../../../tools/spr/zopro-keys.lock", import.meta.url);

export function circuitShape(name) {
  const match = /^(transfer_confidential|merge)_(\d+)_(\d+)\.key$/u.exec(name);
  if (!match)
    throw new Error(`Unsupported circuit in the key catalog: ${name}`);
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
  const ordered = [...values].sort((a, b) => a - b);
  const middle = Math.floor(ordered.length / 2);
  return ordered.length % 2
    ? ordered[middle]
    : (ordered[middle - 1] + ordered[middle]) / 2;
}

function integer(value, name, minimum = 1) {
  const number = Number(value);
  if (!Number.isSafeInteger(number) || number < minimum)
    throw new Error(`${name} must be an integer >= ${minimum}`);
  return number;
}

async function keyBytes(lock, key, directory) {
  const artifact = lock.keys[key];
  const path = join(directory, artifact.file);
  let bytes;
  let downloaded = false;
  try {
    bytes = await readFile(path);
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
    const response = await fetch(new URL(artifact.file, lock.baseUrl), {
      signal: AbortSignal.timeout(180_000),
    });
    if (!response.ok)
      throw new Error(
        `Key download failed (${response.status}): ${artifact.file}`,
      );
    bytes = new Uint8Array(await response.arrayBuffer());
    downloaded = true;
  }
  if (
    bytes.length !== artifact.size ||
    createHash("sha256").update(bytes).digest("hex") !== artifact.sha256
  ) {
    throw new Error(`Key size or SHA-256 mismatch: ${artifact.file}`);
  }
  if (downloaded) {
    await mkdir(directory, { recursive: true });
    const temporary = `${path}.${randomUUID()}.part`;
    try {
      await writeFile(temporary, bytes);
      await rename(temporary, path);
    } finally {
      await rm(temporary, { force: true });
    }
  }
  return bytes;
}

async function runCase(lock, entry, options) {
  // A new process owns each runtime: Rayon cannot change its pool size in place.
  const { benchmarkInputs } = await import("./inputs.mjs");
  const { init } = await import("@heliuslabs/zopro-spr");
  const inputs = await benchmarkInputs(entry.key);
  const artifact = lock.keys[entry.key];
  const reading = performance.now();
  const key = await keyBytes(lock, entry.key, options.keyDirectory);
  const readMs = performance.now() - reading;
  const starting = performance.now();
  const runtime = await init({ threads: entry.threads });
  const initMs = performance.now() - starting;
  let prover;
  try {
    const loading = performance.now();
    prover = new runtime.SprProver(key);
    const loadMs = performance.now() - loading;
    if (
      prover.digest() !== artifact.key_sha256 ||
      prover.inputs() !== entry.inputs ||
      prover.outputs() !== entry.outputs
    ) {
      throw new Error(
        "Loaded proving key does not match the benchmark circuit",
      );
    }
    const merge = entry.circuit === "merge";
    const prove = merge
      ? prover.proveMerge.bind(prover)
      : prover.prove.bind(prover);
    const verify = merge
      ? prover.verifyMerge.bind(prover)
      : prover.verify.bind(prover);
    const proving = [];
    const verifying = [];
    for (let round = 0; round < options.warmup + options.rounds; round++) {
      const started = performance.now();
      const proof = await prove(inputs);
      const proveMs = performance.now() - started;
      const checking = performance.now();
      if (!verify(inputs, proof))
        throw new Error("Benchmark proof failed verification");
      const verifyMs = performance.now() - checking;
      if (round >= options.warmup) {
        proving.push(proveMs);
        verifying.push(verifyMs);
      }
    }
    return {
      ...entry,
      status: "ok",
      rounds: options.rounds,
      warmup: options.warmup,
      key_bytes: key.byteLength,
      key_read_ms: readMs,
      init_ms: initMs,
      key_load_ms: loadMs,
      prove_min_ms: Math.min(...proving),
      prove_median_ms: median(proving),
      prove_max_ms: Math.max(...proving),
      verify_median_ms: median(verifying),
      peak_rss_mb: process.resourceUsage().maxRSS / 1024,
    };
  } finally {
    prover?.free();
    await runtime.close();
  }
}

const columns = [
  "key",
  "circuit",
  "inputs",
  "outputs",
  "threads",
  "status",
  "rounds",
  "warmup",
  "key_bytes",
  "key_read_ms",
  "init_ms",
  "key_load_ms",
  "prove_min_ms",
  "prove_median_ms",
  "prove_max_ms",
  "verify_median_ms",
  "peak_rss_mb",
  "error",
];
function csv(value) {
  const string =
    value === undefined
      ? ""
      : typeof value === "number" && !Number.isInteger(value)
        ? value.toFixed(3)
        : String(value);
  return /[",\r\n]/u.test(string)
    ? `"${string.replaceAll('"', '""')}"`
    : string;
}

export async function main(args = process.argv.slice(2)) {
  const { values } = parseArgs({
    args,
    options: {
      help: { type: "boolean", short: "h" },
      list: { type: "boolean" },
      circuits: { type: "string" },
      threads: { type: "string", default: THREAD_COUNTS.join(",") },
      rounds: { type: "string", default: "3" },
      warmup: { type: "string", default: "1" },
      "keys-dir": { type: "string" },
      "timeout-ms": { type: "string", default: "900000" },
      output: { type: "string" },
      format: { type: "string", default: "csv" },
      "fail-fast": { type: "boolean" },
      case: { type: "string" },
    },
  });
  if (values.help) {
    console.log(
      "Usage: npm run bench -- [--list] [--circuits 1x2,3x8,merge-8x1] [--threads 1,2,4,8,18] [--rounds 3] [--warmup 1] [--keys-dir PATH] [--output results.csv] [--format csv|jsonl] [--timeout-ms 300000] [--fail-fast]",
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
            ? `merge_${name.slice(6).replace("x", "_")}.key`
            : `transfer_confidential_${name.replace("x", "_")}.key`,
      );
    for (const name of names)
      if (!(name in lock.keys))
        throw new Error(`Unknown benchmark circuit: ${name}`);
    cases = cases.filter((entry) => names.includes(entry.key));
  }
  if (!["csv", "jsonl"].includes(values.format))
    throw new Error("format must be csv or jsonl");
  const timeout = integer(values["timeout-ms"], "timeout-ms");
  if (values.output)
    await mkdir(dirname(resolve(values.output)), { recursive: true });
  const file = values.output ? await open(values.output, "w") : undefined;
  const emit = async (line) => {
    if (file) await file.write(`${line}\n`);
    else process.stdout.write(`${line}\n`);
  };
  let failures = 0;
  try {
    if (values.format === "csv") await emit(columns.join(","));
    console.error(
      JSON.stringify({
        node: process.version,
        platform: `${process.platform}/${process.arch}`,
        cpu: cpus()[0]?.model,
        available_threads: availableParallelism(),
        circuits: new Set(cases.map((entry) => entry.key)).size,
        cases: cases.length,
        threads,
        rounds: options.rounds,
        warmup: options.warmup,
        key_set: lock.prefix,
      }),
    );
    for (const [index, entry] of cases.entries()) {
      let row = entry;
      if (!values.list) {
        console.error(
          `[${index + 1}/${cases.length}] ${entry.key}, ${entry.threads} threads`,
        );
        try {
          const { stdout } = await promisify(execFile)(
            process.execPath,
            [
              fileURLToPath(import.meta.url),
              "--case",
              entry.key,
              "--threads",
              String(entry.threads),
              "--rounds",
              String(options.rounds),
              "--warmup",
              String(options.warmup),
              "--keys-dir",
              options.keyDirectory,
            ],
            { timeout, maxBuffer: 1024 * 1024 },
          );
          row = JSON.parse(stdout);
        } catch (error) {
          failures++;
          row = {
            ...entry,
            status: "error",
            error: error.stderr?.trim() || error.message,
          };
        }
      }
      await emit(
        values.format === "jsonl"
          ? JSON.stringify(row)
          : columns.map((column) => csv(row[column])).join(","),
      );
      if (failures && values["fail-fast"]) break;
    }
  } finally {
    await file?.close();
  }
  if (failures) process.exitCode = 1;
}

if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  await main();
