import { createHash } from "node:crypto";

export const DEFAULT_ORDER_SEED = "zopro-spr-matrix-v1";

export function stableJson(value) {
  return JSON.stringify(value, (_, item) =>
    item && typeof item === "object" && !Array.isArray(item)
      ? Object.fromEntries(
          Object.entries(item).sort(([a], [b]) => a.localeCompare(b)),
        )
      : item,
  );
}

export const digest = (value) =>
  createHash("sha256").update(value).digest("hex");
export const caseId = ({ key, threads }) =>
  `${key.replace(/\.key$/u, "")}.t${threads}`;

export function processFailure(error, timeout) {
  const message = error.message || String(error);
  const status = `code=${error.code ?? "none"} signal=${error.signal ?? "none"} killed=${Boolean(error.killed)} timeout_ms=${timeout}`;
  const stderr = error.stderr?.trim();
  return [
    message,
    status,
    stderr && !message.includes(stderr) ? stderr : undefined,
  ]
    .filter(Boolean)
    .join("\n");
}

/** Only execution order is seeded. Proof blinding keeps the runtime's OS RNG. */
export function orderedCases(cases, seed = DEFAULT_ORDER_SEED) {
  return [...cases].sort((a, b) => {
    const hash = (entry) => digest(`${seed}\0${entry.key}\0${entry.threads}`);
    return hash(a).localeCompare(hash(b)) || caseId(a).localeCompare(caseId(b));
  });
}

export function quantile(values, probability) {
  if (!values.length) throw new Error("A quantile needs at least one sample");
  if (!Number.isFinite(probability) || probability < 0 || probability > 1)
    throw new Error("Quantile probability must be between zero and one");
  const sorted = [...values].sort((a, b) => a - b);
  const index = (sorted.length - 1) * probability;
  const lower = Math.floor(index);
  const fraction = index - lower;
  return (
    sorted[lower] +
    fraction * ((sorted[lower + 1] ?? sorted[lower]) - sorted[lower])
  );
}

export function statistics(values) {
  if (
    !values.length ||
    values.some((value) => !Number.isFinite(value) || value < 0)
  ) {
    throw new Error("Timing samples must be nonempty, finite and nonnegative");
  }
  const mean = values.reduce((sum, value) => sum + value, 0) / values.length;
  const sd =
    values.length < 2
      ? null
      : Math.sqrt(
          values.reduce((sum, value) => sum + (value - mean) ** 2, 0) /
            (values.length - 1),
        );
  return {
    count: values.length,
    min_ms: Math.min(...values),
    median_ms: quantile(values, 0.5),
    mean_ms: mean,
    max_ms: Math.max(...values),
    sd_ms: sd,
    cv_percent: sd === null || mean === 0 ? null : (100 * sd) / mean,
    p95_ms: quantile(values, 0.95),
  };
}

export function summarizeIterations(iterations, { warmup, rounds }) {
  const warmups = iterations.filter((sample) => sample.phase === "warmup");
  const measured = iterations.filter((sample) => sample.phase === "measured");
  if (
    warmups.length !== warmup ||
    measured.length !== rounds ||
    iterations.length !== warmup + rounds
  ) {
    throw new Error("Incomplete iteration set");
  }
  for (const [index, sample] of iterations.entries()) {
    const phase = index < warmup ? "warmup" : "measured";
    const iteration = phase === "warmup" ? index + 1 : index - warmup + 1;
    if (
      sample.phase !== phase ||
      sample.iteration !== iteration ||
      sample.verified !== true ||
      sample.status !== "ok"
    ) {
      throw new Error("Iteration phase, order or verification is invalid");
    }
  }
  const prefixed = (prefix, values) =>
    Object.fromEntries(
      Object.entries(statistics(values)).map(([name, value]) => [
        `${prefix}_${name}`,
        value,
      ]),
    );
  return {
    warmup_completed: warmups.length,
    measured_completed: measured.length,
    ...prefixed(
      "prove",
      measured.map((sample) => sample.prove_ms),
    ),
    ...prefixed(
      "verify",
      measured.map((sample) => sample.verify_ms),
    ),
  };
}

export function matchingRun(expected, candidate) {
  if (expected.fingerprint !== candidate.fingerprint) {
    throw new Error(
      "Resume refused: source, artifacts, options or machine environment differ",
    );
  }
}

export function csv(value) {
  const string =
    value === undefined || value === null
      ? ""
      : typeof value === "number" && !Number.isInteger(value)
        ? value.toFixed(6)
        : String(value);
  return /[",\r\n]/u.test(string)
    ? `"${string.replaceAll('"', '""')}"`
    : string;
}
