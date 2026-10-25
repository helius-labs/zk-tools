# Scalar-grouping module extraction

The private scalar-grouping implementation and its tests moved out of
`msm_affine.rs`. The change was adopted after native and portable tests,
independent proof checks and the prospective nonregression gate passed.

Grouping still uses owned hash-map keys, the same admission policy, array
types, reservations and field/drop order. The four function bodies and two
existing test bodies were copied byte-for-byte before formatting. Three new
tests cover ordered group metadata, concurrent callers and panic recovery.
The extraction adds no arithmetic algorithm, runtime switch or key table.

## Performance

AMD Ryzen 9 9950X, 16 physical cores / 32 logical CPUs; 32 Rayon workers.
Both variants use the same generic SDK driver, native compiler flags and
runtime configuration. Seven blocks rotate fresh processes across nine
shapes; each process makes three priming requests and 31 measured, verified
SDK requests. Separate warm profiles cover 1x2 and merge-54 with three blocks.

| Shape | Baseline ms | Extracted module ms | Paired ratio |
| --- | ---: | ---: | ---: |
| 1x2 | 8.624253 | 8.594267 | 0.996336 |
| 2x2 | 11.884072 | 11.697247 | 0.984343 |
| 4x2 | 16.255464 | 16.158262 | 0.997694 |
| 8x2 | 24.899371 | 24.477005 | 0.997987 |
| 12x2 | 34.928809 | 34.386319 | 0.981471 |
| 16x2 | 41.694141 | 40.867383 | 0.981703 |
| 24x2 | 69.786302 | 69.265160 | 0.986565 |
| merge-24x1 | 68.377620 | 68.150148 | 0.990599 |
| merge-54x1 | 177.480120 | 178.891799 | 1.010771 |

Times are medians of process medians. Ratios are medians of ratios paired by
block, so they need not equal the ratio of the displayed times.

| Prospective gate | Limit | Observed | Result |
| --- | ---: | ---: | --- |
| All-nine geometric mean of paired case ratios | <= 1.005 | 0.991899 | pass |
| Primary paired median | <= 1.01 | 0.996336 | pass |
| Each case paired median | <= 1.02 | maximum 1.010771 | pass |
| Every individual paired block | <= 1.05 | maximum 1.047105 | pass |

Merge-54 was 1.08% slower by its paired median, within the declared tolerance.
This is a structural refactor; the small timing differences do not demonstrate
a new arithmetic mechanism or a large speedup. Earlier results from the
six-argument standalone driver remain historical measurements.

The [generated report](artifacts/perf/report.md) includes all supplied M5 CPU
and Metal references. The new primary result is 4.305x the supplied CPU
reference and 2.793x the supplied Metal reference. None of the eight shapes
meets the original 10x CPU target. No new Apple, AWS or Aeglos measurement was
made here.

## Validation and memory

- 109 native and 92 portable core tests passed unfiltered. Both configurations
  also passed two raw-prover and six cubic-prover tests.
- Five named grouping tests include the two moved tests. Independent ordered
  metadata and scalar-weighted generator oracles cover G1 and G2.
- Custom Rayon pools use one, two and four workers with 512 KiB worker stacks,
  four concurrent callers and three iterations. Panic injection checks sibling
  completion and subsequent pool reuse.
- 4,980 SDK requests in the comparison produced 138 fresh, independently
  verified final exports. Another 52 requests produced 13 exports: portable
  proofs for all nine shapes at four workers, plus native primary/merge-54
  proofs at one and four workers. Public-input fixture hashes matched.
- Total: 5,032 requests, 151 independently verified exports and 302 proof/public
  files. Candidate requests account for 2,542 requests and 82 exports; the rest
  are baseline controls. All 12 profiles have one sampled process, no lost
  records and 345,101 samples in aggregate.

Maximum whole-process RSS among latency processes was 114.50 / 113.00 MiB
for baseline / candidate 1x2, and 1,850.98 / 1,887.63 MiB for merge-54.
Profiling-process peaks were 128.75 / 129.00 MiB and 1,689.64 / 1,668.96 MiB,
respectively. These include loading and priming. They establish neither
minimum RAM nor a memory reduction.

The extraction preserves grouping buffers, capacities and ownership. Previous
250/4999 pressure qualification belongs to the
[preceding implementation](../2026-10-25-g2-prefix-adoption/README.md).
Those large proofs and process-cap tests were not repeated for this extraction.

## Rust workflow

The maintained CPU experiment loop now uses `cargo xtask prepare`, `workflow`
and `perf`. Preparation verifies a pinned source snapshot and explicit overlay;
workflow runs build/test/link steps; perf measures latency and automatically
captures warm profiles. Completed successful commands have checked receipts
and can be reused without repeating proof generation.

Full content checks bracket each workflow or experiment. Intermediate checks
use file identity, length and nanosecond modification/change timestamps,
avoiding repeated streaming of large immutable images. The workflow validates
required tests by name and every full-suite summary's filtered count.

Twenty tooling unit tests, formatting and Clippy passed locally. A Linux CI
job now checks the task; remote CI has not been run for these changes. The
stricter full-suite parser and workflow identity checks were added after the
timing run. Existing native/portable outputs were revalidated through resume;
successful compiler, test and link commands were reused.

The legacy device autotuner and empty Python runner directory were removed.
The independent Python arithmetic oracle, two code generators and documented
device/key/quick-benchmark helpers retain their roles. Archived experiment
scripts were not rewritten. See the
[workflow documentation](../../../xtask/README.md).

## Artifacts

Numeric results, source snapshots, proof exports and command receipts are
published here. Raw perf captures and expanded sample streams remain in the
local `artifacts` archive. The artifact hash index covers the complete archive.

[Production manifest](artifacts/production-source-manifest.json),
[prospective protocol](artifacts/experiment.json),
[gate results](artifacts/gates.json),
[qualification](artifacts/qualification-result.json) and
[compatibility checks](artifacts/compat-validation.json) record the outcome.
`artifacts/` retains candidate sources, command/build/test receipts, raw timing
output, exported proofs and raw/decoded profiles. The exact measurement-time
Rust task source is in `xtask-at-measurement.tar`; IPC FIFOs are omitted.
The production copy removes two empty trailing lines from the measured parent
file. Both hashes and the exact whitespace diff are retained in
`production-normalization.json` and `production-normalization.diff`; the
measured candidate snapshot remains unchanged.

The large key images, compiler libraries and SDK executables are identified by
digest and external path rather than bundled. Protocol paths describe this
run's workspace before source promotion. [artifacts.sha256](artifacts.sha256)
checks every bundled artifact.
