# Full-proof coarse phase diagnostic

The recorder now covers both CPU proving paths, and all proofs and graph checks pass. **The combined diagnostic fails its declared qualification because the 16-worker control and overhead intervals are too wide.** No proving optimization is adopted. The 32-worker checks pass individually; they do not override the combined failure or establish an uninstrumented critical path.

The accepted local implementation remains at 7.928660 ms for 1×2. This experiment adds no AWS or Aeglos comparison. The all-eight CPU10× objective remains unmet, 0/8.

## Measurements and decision

Ryzen 9950X; primary 1×2; 15 fixed rotated/reversed process blocks per worker count; 31 timed SDK requests after three primes. Each request solves, blinds, serializes and verifies a fresh proof. Accepted is the retained production executable. OFF and ON use the same newly linked diagnostic executable, selected by `ZOPRO_COARSE_PHASE_ENABLE=0/1`. Key loading is excluded. No other owned build, analysis, verifier or workload overlaps recording.

| Workers | Accepted ms | OFF ms | ON ms | Maximum process RSS, accepted/OFF/ON MiB |
|---|---:|---:|---:|---:|
| 16 | 9.359536 | 9.345951 | 9.498859 | 81.50 / 80.50 / 82.25 |
| 32 | 7.906618 | 7.901638 | 7.931826 | 114.25 / 110.75 / 110.25 |

Times are medians of process medians. RSS includes process startup and is neither a minimum-RAM requirement nor an isolated recorder allocation measurement.

| Workers | Comparison | Median paired ratio | Interval | Result |
|---|---|---:|---|---|
| 16 | OFF / accepted | 0.989676 | [0.840204, 1.022253] | Calibration fails |
| 16 | ON / OFF | 1.000893 | [0.972888, 1.055377] | Overhead qualification fails |
| 32 | OFF / accepted | 0.999370 | [0.989156, 1.009640] | Calibration passes |
| 32 | ON / OFF | 1.002903 | [0.996296, 1.011351] | Overhead qualification passes |

Calibration requires both OFF/accepted intervals within [1/1.03, 1.03]; observer qualification requires both ON/OFF upper bounds ≤1.03. Intervals use ranks 4–12 of 15 paired ratios, with nominal 96.484375% marginal coverage conditional on exchangeability, without a familywise guarantee. The 16-worker upper bound is not a measured 5.5% tracing slowdown: its paired median is +0.089%. No observations are excluded, and no extension, reroll or threshold change rescues this capture.

## Recorded schedule

These are wall spans in the instrumented schedule, including nested Rayon helping and waiting. They cannot be added as exclusive CPU costs or treated as removable time. The failed combined qualification limits inference to recorded behavior.

| Phase, median ms across 45 fixed saved graphs per context | 16 workers | 32 workers |
|---|---:|---:|
| Request | 9.441121 | 7.944338 |
| Pipeline | 8.658646 | 7.111518 |
| Solver | 1.049048 | 1.187369 |
| Prepare scalars | 0.129644 | 0.189528 |
| FFT / quotient | 0.514540 | 0.556660 |
| Query group | 6.977866 | 5.147806 |
| A callback | 6.925277 | 5.063718 |
| KH callback | 6.972186 | 5.106638 |
| G2 callback | 6.957177 | 5.141594 |
| Finalize | 0.134954 | 0.144522 |
| Verify | 0.455108 | 0.487599 |

The median query-marker-to-callback interval is 0.731/1.332/1.703 µs for A/G2/KH at 16 workers, and 1.152/1.753/2.194 µs at 32. The last two callbacks finish a median 7.985/9.358 µs apart. Across all 465 timed enabled requests per context, the last callback is A/G2/KH in 14/164/287 cases at 16 workers and 64/183/218 at 32. These traces do not show a unique late-launch G2 tail. Near-simultaneous returns also do not establish balanced exclusive work: callbacks can execute one another's tasks while waiting.

The general CPU query marker precedes helper preparation and the correction join; its offset is not pure scheduler delay. Its G2 scope includes existing B subgroup/grouping work and optional B1. These distinctions matter when interpreting larger-shape preflight graphs.

## Implementation, correctness and memory

The diagnostic retains 18 fixed slots and an 888-byte recorder payload, excluding Arc bookkeeping, allocator overhead, compiler frames and up to three saved snapshots. One collector is reused outside request timers. Internal timestamp, TID and CPU writes are inside the timer; reset, session setup and snapshot validation are outside. The warm PMU interval also includes between-request metadata work. No field, witness or key values are recorded, and metadata does not grow with the proving key or worker count.

Caller TLS selects ownership once. An explicit Arc parameter crosses the existing small-core-entry worker hop; borrowed references reach callbacks and the ordinary `compute_parts`/`cpu_msms` route. No worker TLS installation, additional pool, key table, execution plan, arithmetic or operation-order change is introduced. The shared stage is named Pipeline because it covers canonical and ordinary CPU solving.

Full native qualification passes 130 core tests, two prover unit tests and six cubic integration tests. Five distinct recorder cases cover ownership, nested sessions, reuse, unwinding, duplicate phases, concurrent owners, outstanding spans and worker-entry transport with 1/4/16/32 workers and 512 KiB stacks. Earlier cases are reapplied to the changed integration; they are not counted as new cases repeatedly. All 176 original source files are independently recovered by reversing edits; 174 remain byte-exact, with one additional module giving 177 files. Coherent fresh libraries and actual O/native/fat-LTO/one-codegen-unit consumer flags are checked.

The first bootstrap stops on an indentation assertion before changed core source is written. Its original helper and logs are preserved. The first native compilation then fails with E0061 because a raw basis API caller lacks the new private argument; no tests or libraries run in that attempt. The repair preserves that source, pins, plan, logs and failed workflow, adds the cfg-gated `None` argument, and recovers every original source file again. Only the failed stage is repaired. Reproduction follows bootstrap then `repair-basis-argument.rs`; the initial generator alone intentionally preserves the failed version.

## Capture and independent review

All nine shapes at 32 workers plus primary at 16 pass accepted/OFF/ON preflights: 30 rows. Three exact accepted preflight caller/verifier/proof pairs are reused from the two preserved predecessor attempts, with identities checked. They are not new executions or ledger additions. All 90 latency callers complete before six warm PMU panels and six separate fixed-period 16M `cycles:u` profiles. Remaining Gnark verification and profile decoding follow all recording. SDK taskset masks restrict the allowed CPU range; individual workers are not pinned to particular CPUs.

There are 132 qualified caller rows, describing 4,908 internally verified SDK requests including 12 reused requests: **4,896 newly executed requests**. Their 129 fresh 164-byte exports pass pinned independent Gnark verification, image/public/VK identity and proof uniqueness checks. The canonical distinct-proof ledger advances **30,717 → 30,846**. Three reused exports remain present once. The public archive contains 264 proof/public members for all 132 rows. Across the initial, entry-transport and full-pipeline attempts, 4,934 SDK requests actually execute; planned but unexecuted calls in failed attempts are excluded.

Independent review reconstructs the source recovery, 819 pin occurrences, 295 successful new native/link/capture commands, six reused command receipts, the preserved compiler failure, proof freshness, ratios, gates and all phase summaries. It checks 112 saved graphs, including 90 latency graphs, for ownership, phase coverage, timestamp containment and CPU range. All 62,918 cycle samples are rechecked for actual PID/TID, permitted CPUs, period, MMAP/FORK ownership and zero loss. Profile unknown shares and sample counts pass; all PMU events run at least 99.9%. Raw PMU parsing is performed by the capture helper and is not independently reparsed by the reviewer. Absolute profile period coverage is not validated.

No portable/phone, memory-pressure, swap, minimum-RAM or concurrent-throughput qualification is claimed. The bounded recorder tests are not a production concurrency benchmark. Production source and runtime remain unchanged.

## Accepted comparison, unchanged

The following accepted local measurements and supplied M5 references are inherited, not newly measured here. Cross-machine timings do not establish equal hardware efficiency.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | CPU10× target ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 52.8 |
| merge24 | 65.060937 | 507 | 175 | 50.7 |

Merge54 remains 161.883899 ms without a supplied M5 reference. Accepted SDK SHA-256 is `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`; diagnostic SDK is `a4696f424a06fb79a0f59a8e82e3fe442785ced97b4b7ffb2da3e4f746bf989c`.

[Analysis](artifacts/analysis.json), [review](artifacts/review.json), [raw rows](artifacts/rows.json), [graphs](artifacts/graphs.json), [profile symbols](artifacts/profile-symbols.json), [design](artifacts/design.md), [identities](artifacts/identities.json), [proof inventory](artifacts/proof-inventory.json), [ledger](artifacts/proof-sha256-after.json), [public proofs](artifacts/public-proofs.hex.json.gz), [source snapshot](artifacts/source-snapshot.hex.json.gz), [receipts and helpers](artifacts/receipts.json.gz), [raw profile inventory](artifacts/raw-profile-inventory.json), [checksums](SHA256SUMS). Six raw perf files are losslessly compressed beside the inventory. Every archive is decompressed and byte-checked during publication. Executables, libraries and private inputs remain local with recorded hashes; reproduction requires pinned tools, dependencies and private inputs.

Private root: `/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176`. This completed diagnostic is closed without an unchanged retry. The next question is the scheduling churn associated with slow 16-worker observations, not a relaxation of the failed calibration.
