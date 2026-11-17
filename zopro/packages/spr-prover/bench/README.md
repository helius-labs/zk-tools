# Node WASM prover benchmark

The full default workload is every key in `tools/spr/zopro-keys.lock`:
38 confidential transfer input/output shapes plus the 8-, 24- and 52-input
merges, each at **1, 2, 4, 8 and 18 threads**. That is **205 cases**.
One thread uses the threaded WASM build with a one-worker Rayon pool.

Each case starts a fresh Node process, loads its key once, runs **3 warmup
proofs**, then records **10 measured proofs**. Every proof is verified by
the runtime and explicitly verified again by the harness: 615 warmup plus
2,050 measured calls, or **2,665 verified proving calls** for an
uninterrupted full run. Inputs are all-real positive notes with unique
nullifiers, valid state paths and balanced totals. Proof blinding continues
to use the runtime's OS randomness.

## Run from a clean, pinned checkout

Build before opening the timing window. Stop this campaign's other local
builds, tests and benchmark processes before measurement; remote CI can
continue. Do not change power settings while running.

```sh
npm ci
npm run build
npm run bench -- --list

# Output may live in the primary checkout while this executes in an isolated,
# clean source checkout. Keep that source checkout until any resume is done.
node bench/prove.mjs \
  --threads 1,2,4,8,18 --warmup 3 --rounds 10 \
  --seed zopro-spr-matrix-v1 \
  --run-dir /absolute/path/to/primary-worktree/zopro/packages/spr-prover/bench/results/full-3w10r \
  --fail-fast
```

The default output is a new timestamped `bench/results/<run-id>/`
directory. An existing directory requires explicit `--resume`; existing
evidence is never overwritten as a new run. A source worktree must be
clean, apart from its own newly created output directory if that directory
is inside the checkout. Output paths use literal Git pathspecs.

Cases run sequentially. The runner sorts SHA-256 hashes of the scheduling
seed, key name and thread count; the manifest saves the exact order.
This spreads thread counts across the campaign instead of always running
1 through 18 in blocks. The seed never controls cryptographic randomness.
Ordering reduces systematic order bias; it does not eliminate thermal
drift or shared-machine noise.

## Measurement boundaries

All source/artifact checks and an initial download/checksum pass for every
selected key happen before cases start. Within each fresh process these
setup stages also precede every proof timer:

- `input_setup_ms`: fixture and witness-input construction.
- `key_read_ms`: source key bytes and artifact checksum verification.
- `init_ms`: WASM and Rayon pool initialization.
- `key_load_ms`: the key constructor, including decode and solver setup.

The **only** operation inside each `prove_ms` timer is the public typed
SDK-compatible proving call. It includes witness construction,
serialization, the proof, the runtime's internal verification and SDK
proof decoding. Additional explicit verification is timed separately as
`verify_ms`. Setup and key loading never enter either timing. Warmups
have the same raw records and verification requirements as measured
samples, but do not contribute to summary statistics.

`peak_rss_mb` is in MiB (Node reports `maxRSS` in KiB; the conversion
divides by 1,024). It is the fresh process's peak including key bytes,
WASM memory and worker threads. It includes setup rather than claiming
to isolate only the proof's incremental memory.

## Artifacts and statistical interpretation

Each run saves:

- `manifest.json`: exact clean Git SHA/tree, configuration, full case order,
  Node/V8/OS/CPU/available-thread/memory information, power/load snapshots,
  source key catalog and SHA-256s of the catalog, Rust/npm locks, installed
  SDK metadata, both WASM binaries and generated host glue. It records
  every resume session and verified proof counts.
- `progress.json`: current case/attempt/PIDs and explicit completed,
  failed and remaining counts, updated atomically.
- `summaries.csv`: one latest result per case, with setup costs, measured
  minimum/median/mean/maximum, sample standard deviation, coefficient of
  variation and p95. Verification has corresponding dispersion fields.
- `iterations.jsonl`: aggregate raw warmup and measured records, including
  unsuccessful attempts. Records retain case/attempt/session identities,
  timestamps, verification outcome, load averages and resident memory.
- `cases/<case-id>/attempt-N/`: raw iterations, stderr, start metadata
  and result for each attempt. A failed partial attempt is retained when
  the whole case is retried. Aggregated records are in case/attempt order;
  timestamps and session IDs describe the actual execution timeline.

Standard deviation uses **N-1**. CV is `100 * SD / mean`. The p95 is
linear interpolation at sorted index `(N-1) * 0.95` (Type 7).
Single-sample smoke runs report no SD/CV rather than inventing zero
variance. These are descriptive statistics of the recorded samples;
10 samples do not establish a tail-latency SLA or significance. Inspect
raw drift and variability, and treat differences comparable to the noise
as inconclusive. No outliers are silently removed.

Initial/final and case-boundary snapshots capture power and background
load; per-iteration records include load averages. Power settings,
available thread count or artifact/source changes stop the campaign.
The harness publishes only allowlisted environment/configuration fields.
Unsupported Node options, loaders, coverage or debugging instrumentation
are rejected at preflight with redacted counts/hashes. Validated
allowlisted flags are forwarded to the timed child and checked there.
No credentials, full environment dump or active command arguments are
written to results.

## Resume and failures

```sh
# Same source checkout, Node binary, arguments, keys and machine/power setup.
node bench/prove.mjs --threads 1,2,4,8,18 --warmup 3 --rounds 10 \
  --seed zopro-spr-matrix-v1 --run-dir /absolute/path/to/the/run --resume --fail-fast
```

Resume requires an exact fingerprint of source, build artifacts, sampling
configuration and stable machine/power identity. Recorded old parent and
case PIDs must be gone. Already successful cases are revalidated against
their raw records and skipped; failed/interrupted cases get a new attempt
with all 3+10 proofs. Partial samples are never merged into a successful
case's statistics. Session boundaries and extra attempted calls remain
explicit. Corrupt records or a differing environment are errors.

Every case failure preserves available raw records and an error row;
exit code is nonzero and the run remains `incomplete`. `--fail-fast`
stops after that case; otherwise subsequent cases are attempted. Timeout
and signal errors retain process message, code, signal, killed flag,
configured timeout and stderr. `--timeout-ms` is a per-case safety bound
(default 1,800,000 ms), not a campaign time budget.

## Filters and correctness smoke runs

```sh
npm run bench -- --circuits 1x2 --rounds 1 --warmup 0 --output smoke.csv
npm run bench -- --circuits 1x4,3x8,49x2,merge-8x1 --threads 4,18 --output selected.csv
```

Filtered and reduced-sample configurations are labeled in the manifest.
A one-proof/no-warmup result is correctness smoke evidence, not the full
performance comparison. `--output` optionally copies the summary outside
the run directory; canonical raw/provenance artifacts are always kept.
`--format jsonl` selects JSONL for that summary copy/stdout. Keys cache in
`~/.cache/zopro-wasm/<key-set>/`; `--keys-dir` or `ZOPRO_KEYS_DIR`
selects another cache. Unknown catalog shapes fail instead of silently
reducing coverage.

Method references: [Node high-resolution timing](https://nodejs.org/api/perf_hooks.html#performancenow),
[Node resource usage](https://nodejs.org/api/process.html#processresourceusage),
and [NIST measures of scale](https://www.itl.nist.gov/div898/handbook/eda/section3/eda356.htm).
