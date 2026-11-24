import { execFile } from "node:child_process";
import { randomUUID } from "node:crypto";
import { mkdir, readFile, readdir, rename, writeFile } from "node:fs/promises";
import { dirname, join, resolve, sep } from "node:path";
import {
  artifactMetadata,
  assertBenchmarkEnvironment,
  fileHash,
  hostMetadata,
  loadSnapshot,
  sourceMetadata,
  stableEnvironment,
} from "./provenance.mjs";
import {
  caseId,
  csv,
  digest,
  matchingRun,
  processFailure,
  stableJson,
  summarizeIterations,
} from "./records.mjs";
import { keyBytes } from "./keys.mjs";

const columns = [
  "case_index",
  "case_id",
  "key",
  "circuit",
  "inputs",
  "outputs",
  "threads",
  "status",
  "rounds",
  "warmup",
  "warmup_completed",
  "measured_completed",
  "key_bytes",
  "input_setup_ms",
  "key_read_ms",
  "init_ms",
  "key_load_ms",
  "prove_min_ms",
  "prove_median_ms",
  "prove_mean_ms",
  "prove_max_ms",
  "prove_sd_ms",
  "prove_cv_percent",
  "prove_p95_ms",
  "verify_median_ms",
  "verify_sd_ms",
  "verify_cv_percent",
  "verify_p95_ms",
  "peak_rss_mb",
  "session_id",
  "attempt",
  "error",
];

export async function atomicJson(path, value) {
  const temporary = `${path}.${randomUUID()}.tmp`;
  await writeFile(temporary, `${JSON.stringify(value, null, 2)}\n`);
  await rename(temporary, path);
}

export async function readIterations(path) {
  let content;
  try {
    content = await readFile(path, "utf8");
  } catch (error) {
    if (error.code === "ENOENT") return [];
    throw error;
  }
  return content
    .split("\n")
    .filter(Boolean)
    .map((line, index) => {
      try {
        return JSON.parse(line);
      } catch {
        throw new Error(`Malformed raw iteration record ${path}:${index + 1}`);
      }
    });
}

function alive(pid) {
  if (!Number.isSafeInteger(pid) || pid <= 0) return false;
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return error.code !== "ESRCH";
  }
}

export function runIdentity({ configuration, source, artifacts, environment }) {
  return digest(stableJson({ configuration, source, artifacts, environment }));
}

function childCase(script, args, options, started) {
  return new Promise((resolve, reject) => {
    const child = execFile(
      process.execPath,
      [...options.execArgv, script, ...args],
      { timeout: options.timeout, maxBuffer: 1024 * 1024 },
      (error, stdout, stderr) => {
        if (error) {
          error.stderr = stderr;
          reject(error);
        } else resolve({ stdout, stderr });
      },
    );
    started(child);
  });
}

async function nextAttempt(directory) {
  const names = await readdir(directory).catch((error) => {
    if (error.code === "ENOENT") return [];
    throw error;
  });
  return (
    1 +
    Math.max(
      0,
      ...names
        .filter((name) => /^attempt-\d+$/u.test(name))
        .map((name) => Number(name.slice(8))),
    )
  );
}

async function results(runDirectory, cases) {
  const rows = [];
  for (const entry of cases) {
    const path = join(runDirectory, "cases", caseId(entry), "result.json");
    try {
      rows.push(JSON.parse(await readFile(path, "utf8")));
    } catch (error) {
      if (error.code !== "ENOENT") throw error;
    }
  }
  return rows;
}

async function reports(runDirectory, cases, output, format) {
  const rows = await results(runDirectory, cases);
  const summary = `${columns.join(",")}\n${rows.map((row) => columns.map((column) => csv(row[column])).join(",")).join("\n")}${rows.length ? "\n" : ""}`;
  await writeFile(join(runDirectory, "summaries.csv"), summary);
  const allIterations = [];
  for (const entry of cases) {
    const directory = join(runDirectory, "cases", caseId(entry));
    const names = await readdir(directory).catch((error) => {
      if (error.code === "ENOENT") return [];
      throw error;
    });
    for (const attempt of names
      .filter((name) => /^attempt-\d+$/u.test(name))
      .sort((a, b) => Number(a.slice(8)) - Number(b.slice(8)))) {
      allIterations.push(
        ...(await readIterations(join(directory, attempt, "iterations.jsonl"))),
      );
    }
  }
  await writeFile(
    join(runDirectory, "iterations.jsonl"),
    allIterations.map((sample) => JSON.stringify(sample)).join("\n") +
      (allIterations.length ? "\n" : ""),
  );
  if (output) {
    await mkdir(dirname(resolve(output)), { recursive: true });
    await writeFile(
      output,
      format === "jsonl"
        ? rows.map((row) => JSON.stringify(row)).join("\n") +
            (rows.length ? "\n" : "")
        : summary,
    );
  }
  return rows;
}

/** One host, sequential fresh processes. Completed cases never combine attempts. */
export async function campaign({
  lock,
  cases,
  options,
  timeout,
  seed,
  runDirectory,
  resume,
  output,
  format,
  failFast,
  script,
}) {
  if (
    output &&
    (resolve(output) === runDirectory ||
      resolve(output).startsWith(runDirectory + sep))
  )
    throw new Error(
      "--output copy must be outside --run-dir; canonical reports are already saved there",
    );
  const source = await sourceMetadata(runDirectory);
  const artifacts = await artifactMetadata();
  const host = await hostMetadata();
  try {
    assertBenchmarkEnvironment(host);
  } catch (error) {
    await mkdir(runDirectory, { recursive: true });
    await atomicJson(
      join(runDirectory, "preflight-rejected-" + Date.now() + ".json"),
      { status: "rejected", source, host, diagnostics: error.diagnostics },
    );
    throw error;
  }
  const initialLoad = await loadSnapshot({ processes: true });
  const environment = stableEnvironment(host, initialLoad);
  const configuration = {
    schema: 2,
    sampling:
      options.warmup >= 3 && options.rounds >= 10
        ? "repeated_warmed_measurements"
        : "smoke_or_validation_only",
    scope:
      cases.length === Object.keys(lock.keys).length * 5 &&
      [...new Set(cases.map(({ threads }) => threads))]
        .sort((a, b) => a - b)
        .join(",") === "1,2,4,8,18"
        ? "full_205_case_matrix"
        : "filtered_matrix",
    rounds: options.rounds,
    warmup: options.warmup,
    case_timeout_ms: timeout,
    order_seed: seed,
    cases,
    key_set: lock.prefix,
    key_directory: options.keyDirectory,
    protocol:
      "public SDK prove wall time; setup excluded; explicit verification separate; all samples verified",
    statistics:
      "measured samples only; sample SD uses N-1; CV=100*SD/mean; p95 uses linear interpolation at (N-1)*0.95",
    randomness:
      "order seed controls only case scheduling; proof blinding uses the unchanged runtime OS CSPRNG",
  };
  const fingerprint = runIdentity({
    configuration,
    source,
    artifacts,
    environment,
  });
  const manifestPath = join(runDirectory, "manifest.json");
  const progressPath = join(runDirectory, "progress.json");
  const now = new Date().toISOString();
  let manifest;
  if (resume) {
    manifest = JSON.parse(await readFile(manifestPath, "utf8"));
    if (manifest.schema !== 2 || runIdentity(manifest) !== manifest.fingerprint)
      throw new Error(
        "Resume refused: manifest provenance is corrupt or unsupported",
      );
    matchingRun(manifest, { fingerprint });
    const previous = JSON.parse(await readFile(progressPath, "utf8"));
    if (alive(previous.parent_pid) || alive(previous.current_case?.child_pid))
      throw new Error(
        "Resume refused: recorded parent or case process is still alive",
      );
    for (const oldSession of manifest.sessions) {
      if (!oldSession.finished_at) {
        oldSession.interruption_detected_at = now;
        oldSession.outcome =
          "previous parent exited without finalizing this session";
      }
    }
  } else {
    await mkdir(dirname(runDirectory), { recursive: true });
    await mkdir(runDirectory);
    manifest = {
      schema: 2,
      run_id: runDirectory.split(/[\\/]/u).at(-1),
      fingerprint,
      status: "preparing",
      created_at: now,
      command: [process.execPath, ...process.argv.slice(1)],
      source,
      artifacts,
      host,
      environment,
      configuration,
      key_artifacts: Object.fromEntries(
        [...new Set(cases.map(({ key }) => key))].map((key) => [
          key,
          lock.keys[key],
        ]),
      ),
      expected_cases: cases.length,
      expected_warmup_proofs: cases.length * options.warmup,
      expected_measured_proofs: cases.length * options.rounds,
      sessions: [],
    };
  }
  const session = {
    id: randomUUID(),
    started_at: now,
    parent_pid: process.pid,
    source,
    host,
    initial_load: initialLoad,
    resume: Boolean(resume),
    command: [process.execPath, ...process.argv.slice(1)],
  };
  manifest.sessions.push(session);
  manifest.status = "preparing";
  await atomicJson(manifestPath, manifest);
  let current;
  let activeChild;
  let interrupted;
  const stop = (signal) => {
    interrupted = signal;
    activeChild?.kill("SIGTERM");
  };
  const sigint = () => stop("SIGINT");
  const sigterm = () => stop("SIGTERM");
  process.once("SIGINT", sigint);
  process.once("SIGTERM", sigterm);
  let rows = await reports(runDirectory, cases, output, format);
  const progress = async (status) => {
    await atomicJson(progressPath, {
      schema: 2,
      run_id: manifest.run_id,
      fingerprint,
      status,
      session_id: session.id,
      parent_pid: process.pid,
      updated_at: new Date().toISOString(),
      expected_cases: cases.length,
      completed_cases: rows.filter((row) => row.status === "ok").length,
      failed_cases: rows.filter((row) => row.status !== "ok").length,
      remaining_cases:
        cases.length - rows.filter((row) => row.status === "ok").length,
      current_case: current ?? null,
    });
  };
  await progress("preparing");
  if (!output && format === "csv") console.log(columns.join(","));
  try {
    const prefetch = performance.now();
    for (const key of new Set(cases.map(({ key }) => key))) {
      if (interrupted) break;
      await keyBytes(lock, key, options.keyDirectory);
    }
    session.key_prefetch_ms = performance.now() - prefetch;
    session.key_prefetch_completed_at = new Date().toISOString();
    manifest.status = "running";
    await atomicJson(manifestPath, manifest);
    for (const [index, entry] of cases.entries()) {
      if (interrupted) break;
      const id = caseId(entry);
      const previous = rows.find((row) => row.case_id === id);
      if (previous?.status === "ok") {
        if (previous.run_fingerprint !== fingerprint)
          throw new Error("Completed case has a foreign run fingerprint");
        const saved = await readIterations(
          join(
            runDirectory,
            "cases",
            id,
            `attempt-${previous.attempt}`,
            "iterations.jsonl",
          ),
        );
        if (stableJson(saved) !== stableJson(previous.iterations))
          throw new Error(
            "Resume refused: completed case raw samples differ from its result",
          );
        const derived = summarizeIterations(previous.iterations, options);
        if (
          Object.entries(derived).some(
            ([name, value]) => previous[name] !== value,
          )
        )
          throw new Error(
            "Resume refused: summary statistics differ from the saved raw samples",
          );
        continue;
      }
      const currentSource = await sourceMetadata(runDirectory);
      const currentArtifacts = await artifactMetadata();
      if (
        stableJson(currentSource) !== stableJson(source) ||
        stableJson(currentArtifacts) !== stableJson(artifacts)
      )
        throw new Error(
          "Source or build artifacts changed during the benchmark",
        );
      const before = await loadSnapshot();
      if (
        stableJson(stableEnvironment(host, before)) !== stableJson(environment)
      )
        throw new Error(
          "Machine power configuration changed before a benchmark case",
        );
      const directory = join(runDirectory, "cases", id);
      const attempt = await nextAttempt(directory);
      const attemptDirectory = join(directory, `attempt-${attempt}`);
      await mkdir(attemptDirectory, { recursive: true });
      current = {
        case_index: index + 1,
        case_id: id,
        ...entry,
        attempt,
        directory: attemptDirectory,
        started_at: new Date().toISOString(),
      };
      await atomicJson(join(attemptDirectory, "attempt.json"), {
        ...current,
        run_fingerprint: fingerprint,
        session_id: session.id,
        before,
      });
      await progress("running");
      console.error(
        `[${index + 1}/${cases.length}] ${entry.key}, ${entry.threads} threads; ${options.warmup} warmup + ${options.rounds} measured`,
      );
      let row;
      let startupProgress;
      try {
        const { stdout, stderr } = await childCase(
          script,
          [
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
            "--record-dir",
            attemptDirectory,
            "--run-fingerprint",
            fingerprint,
            "--session-id",
            session.id,
            "--attempt",
            String(attempt),
            "--case-index",
            String(index + 1),
          ],
          { timeout, execArgv: host.exec_argv.allowed },
          (child) => {
            activeChild = child;
            current.child_pid = child.pid;
            startupProgress = progress("running");
          },
        );
        await writeFile(join(attemptDirectory, "stderr.log"), stderr);
        row = JSON.parse(stdout);
        if (
          row.key !== entry.key ||
          row.threads !== entry.threads ||
          row.inputs !== entry.inputs ||
          row.outputs !== entry.outputs ||
          row.status !== "ok"
        )
          throw new Error("Child result does not match the scheduled case");
        if (stableJson(row.exec_arguments) !== stableJson(host.exec_argv))
          throw new Error(
            "Timed child Node flags differ from the recorded parent configuration",
          );
        const iterations = await readIterations(
          join(attemptDirectory, "iterations.jsonl"),
        );
        if (stableJson(iterations) !== stableJson(row.iterations))
          throw new Error("Raw iteration file differs from the child result");
        if (
          iterations.some(
            (sample) =>
              sample.case_id !== id ||
              sample.run_fingerprint !== fingerprint ||
              sample.session_id !== session.id ||
              sample.attempt !== attempt ||
              sample.case_index !== index + 1,
          )
        )
          throw new Error(
            "Raw iterations have a foreign case, attempt or run identity",
          );
        row = { ...row, ...summarizeIterations(iterations, options) };
      } catch (error) {
        await writeFile(
          join(attemptDirectory, "stderr.log"),
          error.stderr ?? "",
        );
        row = {
          ...entry,
          status: "error",
          error: processFailure(error, timeout),
          iterations: await readIterations(
            join(attemptDirectory, "iterations.jsonl"),
          ),
        };
      } finally {
        await startupProgress;
        activeChild = undefined;
      }
      const after = await loadSnapshot();
      let sourceError;
      try {
        if (
          stableJson(await sourceMetadata(runDirectory)) !==
            stableJson(source) ||
          stableJson(await artifactMetadata()) !== stableJson(artifacts)
        )
          sourceError =
            "Source or build artifacts changed during the benchmark case";
      } catch (error) {
        sourceError = error.message;
      }
      const environmentChanged =
        stableJson(stableEnvironment(host, after)) !== stableJson(environment);
      if (environmentChanged)
        row = {
          ...row,
          status: "environment_changed",
          error:
            "Machine power configuration changed during the benchmark case",
        };
      if (sourceError)
        row = { ...row, status: "source_changed", error: sourceError };
      row = {
        ...row,
        case_id: id,
        case_index: index + 1,
        attempt,
        session_id: session.id,
        run_fingerprint: fingerprint,
        rounds: options.rounds,
        warmup: options.warmup,
        started_at: current.started_at,
        finished_at: new Date().toISOString(),
        environment_before: before,
        environment_after: after,
      };
      await atomicJson(join(attemptDirectory, "result.json"), row);
      await atomicJson(join(directory, "result.json"), row);
      current = undefined;
      rows = await reports(runDirectory, cases, output, format);
      await progress("running");
      if (!output)
        console.log(
          format === "jsonl"
            ? JSON.stringify(row)
            : columns.map((column) => csv(row[column])).join(","),
        );
      if (environmentChanged || sourceError) throw new Error(row.error);
      if (row.status !== "ok" && failFast) break;
    }
    const successes = rows.filter((row) => row.status === "ok").length;
    manifest.status = interrupted
      ? "interrupted"
      : successes === cases.length
        ? "complete"
        : "incomplete";
    session.finished_at = new Date().toISOString();
    session.final_load = await loadSnapshot({ processes: true });
    manifest.updated_at = session.finished_at;
    manifest.completed_cases = successes;
    manifest.failed_cases = rows.filter((row) => row.status !== "ok").length;
    const recorded = await readIterations(
      join(runDirectory, "iterations.jsonl"),
    );
    manifest.proof_counts = {
      selected_successful_attempts: {
        warmup: rows
          .filter((row) => row.status === "ok")
          .flatMap((row) => row.iterations)
          .filter((sample) => sample.phase === "warmup" && sample.verified)
          .length,
        measured: rows
          .filter((row) => row.status === "ok")
          .flatMap((row) => row.iterations)
          .filter((sample) => sample.phase === "measured" && sample.verified)
          .length,
      },
      all_attempts: {
        records: recorded.length,
        verified: recorded.filter((sample) => sample.verified).length,
        failed: recorded.filter((sample) => sample.status !== "ok").length,
      },
    };
    manifest.datasets = {
      summaries: {
        file: "summaries.csv",
        ...(await fileHash(join(runDirectory, "summaries.csv"))),
      },
      iterations: {
        file: "iterations.jsonl",
        ...(await fileHash(join(runDirectory, "iterations.jsonl"))),
      },
    };
    await progress(manifest.status);
    if (manifest.status !== "complete")
      process.exitCode =
        interrupted === "SIGINT" ? 130 : interrupted === "SIGTERM" ? 143 : 1;
  } catch (error) {
    manifest.status = "incomplete";
    manifest.error = error.message;
    await progress("incomplete");
    throw error;
  } finally {
    process.off("SIGINT", sigint);
    process.off("SIGTERM", sigterm);
    await atomicJson(manifestPath, manifest);
  }
  console.error(`Benchmark ${manifest.status}: ${runDirectory}`);
  return manifest;
}
