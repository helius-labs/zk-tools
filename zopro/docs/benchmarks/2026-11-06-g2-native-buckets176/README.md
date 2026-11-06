# Native radix-52 state in signed G2 batches

**Rejected; not adopted.** Keeping temporary G2 bucket coordinates in radix-52 form makes the complete signed borrowed batch kernel **22–27% slower** across six contexts, with zero winning blocks against fresh OFF. All six G2 control calibrations pass. The two-size 32-worker aggregate is **26.09% slower**, ratio **1.260947 [1.259056, 1.270457]**, zero of fifteen wins. No query integration, full-proof trial or AWS measurement follows.

The candidate retires approximately 2% fewer instructions but consumes about 32% more user cycles in the two counter panels. Removing coordinate-conversion instructions did not make the complete kernel cheaper. These observations do not isolate an exclusive gather, cache, memory-bandwidth or stall cost.

## Mechanism and prior evidence

The accepted signed borrowed G2 kernel packs bucket X in its norm pass, packs X/Y again in its reverse pass, then unpacks updated X/Y. This candidate keeps four coordinate fields in canonical radix-52 storage across batches, using checked vector gathers and masked scatters. Input key points and their original constructors remain unchanged. Norm/prefix/inverse arithmetic, signs, scalar tails and point formulas are inherited from the current kernel; reversing eight substitutions recovers its exact source.

The field Store is byte-exact from the [earlier G1 native-bucket study](../2026-10-22-universal-native-bucket52/README.md). That rejected unsigned G1 window used a different batch/exception/tail algorithm. This study tests the current signed borrowed G2 prefix, with original scalar tails. It also differs from the [blocked-input codec](../2026-11-05-g2-blocked-codec176/README.md), which changed input-root representation, and from column-cohort and compact-tree reductions. No algorithmic novelty is claimed.

Before selecting this screen, an offline audit of the qualified original [G2 operation census](../2026-11-05-g2-operation-census176/README.md) found only modest ordinary-task batch-count skew: maximum/mean **1.036468** for primary and **1.004464** for grouped 2×2. All forty saved task records, component sums and cross-worker row multisets agree. Counts are not service times, but do not support expecting a large simple rebalancing gain. No scalar scan, query or generic timeline was rerun. Earlier component splitting and weighted scheduling recipes remain closed.

## Fixed kernel comparison

Ryzen 9950X, sixteen physical cores and 32 logical CPUs. Synthetic changing-state G2 batches contain 200 or 352 signed updates to distinct destinations in a 2,048-bucket state. Each logical worker owns its fixture; incoming points and signs are fixed, while bucket values change on every timed update. This is a kernel workload, not a replay of window scheduling or proof throughput. Unchanged signed G1/200 at all three worker counts is retained as a negative control.

Three arms use one private source snapshot and identical driver bytes: fresh cfg-disabled original API (`frozen`), candidate executable OFF using the original kernel, and the same executable ON using native bucket state. Actual consumer commands contain O3, native CPU, fat LTO (`-C lto` from Cargo `lto=true`) and one codegen unit. Individual workers pin to CPU 2 solo, CPUs 0–15 or CPUs 0–31, with 512 KiB stacks.

The [prospective protocol](artifacts/protocol.md) fixes fifteen rotated/reversed blocks and all gates before capture. Each process checks three 128-batch primes, resets both diagnostic states, then times 4,096 batches per fixture. Reference assembly, kernel work, dispatch and waits are timed; point/oracle construction and final checks are excluded. The fixed dual-state reset touches native storage last; there is no additional warmup after reset. These whole-loop results do not establish a sub-batch warm-state measurement.

Every final point matches an independent generator-coefficient oracle; intermediate timed states are not individually checked. All timed allocation counts are zero and scratch capacities stay stable. Twenty-seven preflights pass before all 405 latency processes, followed by separate counters and profiles. No owned build, analyzer, archive or second workload overlaps recording.

Times are microseconds per wave across the caller's fixtures. Paired ratios need not equal ratios of marginal medians.

| Context | Frozen µs | OFF µs | ON µs | Paired ON/OFF [lower, upper] | Wins/15 |
|---|---:|---:|---:|---|---:|
| G2/200/1 | 7.662 | 7.674 | 9.409 | 1.226971 [1.223111, 1.228079] | 0 |
| G2/200/16 | 8.934 | 8.978 | 11.271 | 1.253962 [1.250945, 1.256497] | 0 |
| G2/200/32 | 14.788 | 14.760 | 18.774 | 1.272310 [1.262917, 1.276976] | 0 |
| G2/352/1 | 12.953 | 12.963 | 16.081 | 1.240672 [1.238587, 1.242444] | 0 |
| G2/352/16 | 15.097 | 15.127 | 18.778 | 1.240283 [1.235787, 1.246678] | 0 |
| G2/352/32 | 25.379 | 25.478 | 31.881 | 1.255677 [1.239828, 1.267952] | 0 |
| Unchanged G1/200/1 | 4.197 | 4.206 | 4.203 | 0.999875 [0.998383, 1.000794] | 8 |
| Unchanged G1/200/16 | 4.763 | 4.769 | 4.771 | 1.000024 [0.995739, 1.002750] | 7 |
| Unchanged G1/200/32 | 6.910 | 7.909 | 6.939 | 0.997333 [0.861923, 1.017514] | 8 |

The G2 32-worker aggregate versus frozen is **1.263212 [1.257122, 1.264947]**, zero wins. Ordered ranks 4–12/15 provide nominal 96.484375% marginal median intervals under exchangeable independent blocks, without simultaneous coverage. No gain floor, exclusion, extension, subset promotion or unchanged reroll is used.

Twenty checks fail: twelve G2 nonregressions, four individual G2 gain checks, both G2 aggregate gains, the unchanged G1/32 calibration and its symmetric ON/OFF stability check. G1/32 OFF/frozen is 1.005809 [0.978678, 1.153354]; its upper uncertainty is not a measured 15.3% slowdown. That variable control does not rescue six independently calibrated G2 regressions. All original/control ratios and raw clocks remain in the analysis archive.

## Counters, profiles and storage

Six FIFO-gated PMU panels run 16,384 batches per fixture; six separate fixed 16-million-cycle profiles run 65,536. Primes and checks are outside perf enable. Every PMU event runs 100%. All profiles qualify with **70,278 samples**, at least 7,741 each, unknown share below 0.065% and zero lost records. Actual PID/TID/CPU, mappings, task events and event period are checked. ON profiles resolve to `apply_native`; controls resolve to the original borrowed kernel.

| 32-worker panel | OFF instructions/wave | ON instructions/wave | OFF cycles/wave | ON cycles/wave |
|---|---:|---:|---:|---:|
| G2/200 | 3.557219M | 3.488384M | 1.901012M | 2.519822M |
| G2/352 | 6.033103M | 5.911441M | 3.282293M | 4.343854M |

Generic cache-miss counts increase for 200 but decrease for 352 while both slow down. They do not establish a universal cache-level or DRAM explanation. Full symbols, counters, faults and context switches are retained.

Native coordinate payload is **160 bytes per bucket versus 128**, or 327,680 versus 262,144 bytes for 2,048 buckets: **64 KiB additional per state**. This replaces a temporary bucket representation, not a key table. Scratch records remain 16,000 bytes at 200 updates or 28,160 at 352 in every arm. The benchmark deliberately retains reference, original and native states for validation; that duplication is not a proposed prover allocation pattern. Diagnostic maximum RSS at G2/32 is 55.75/55.75 MiB OFF/ON for 200 and 58.75/59.00 MiB for 352. Equal diagnostic RSS does not negate the larger payload or prove equal lifetime peaks.

No runtime Window, resource-admission or portable path is changed. A future integration would have to account for all live states and host-output overlap, preserve exceptional-point routing and low-memory fallbacks, and qualify complete queries and proofs. This negative screen does not proceed there. There is no LUT, key/plan duplication, new pool, arbitrary RAM cap, device, swap/pressure or minimum-RAM claim.

## Validation and provenance

All 128 core tests pass, including three new signed/tail/mask/reuse/input-preservation and 1/4/16/32-worker tests. Two reused Store tests freshly pass against Arkworks and check bounds rejection; the original 35 field tests are filtered and inherited. The kernel retains its finite-point, distinct-destination and nonzero-denominator caller contract. New exceptional-Window qualification is not claimed.

The 178-file private snapshot contains 172 exact originals and four append-only parent exports/bridges; removing them recovers all 176 accepted sources. Existing field arithmetic and generated kernels remain exact. A separate checker verifies source recovery, Store provenance, arithmetic inversion audit, all 482 native/capture command argv and environment receipts,444 callers, paired distributions/gates, CPU/RSS, raw PMUs and every profile symbol bin. It succeeds on its first execution. The 444 callers perform 45,708,864 batch calls including primes; these are not proofs. No SDK/Gnark request occurs and the canonical ledger stays 30,846.

Preserved tooling failures: the initial source generator's inverse audit caught an ambiguous replacement anchor before writing any snapshot; the saved-census audit initially compared asynchronous row order rather than complete row multisets; capture preparation expected a different textual spelling of fat LTO; the first analyzer classified the new preflight-check helper as a CALLER process. Repairs change only those checks. Native tests/builds and all 475 capture commands complete once. The frozen controller and all receipts remain unchanged; the corrected analyzer is a separate postcapture file.

## Accepted proof comparison, unchanged

These are inherited local 32-worker full-proof times and supplied M5 references, **not results of this kernel**. Accepted source 176/SDK `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661` remains unchanged. Hardware differs, so these ratios do not establish equal hardware efficiency.

| Shape | Accepted CPU ms | M5 CPU ms | CPU/local | M5 Metal ms | Metal/local | CPU10× limit ms |
|---|---:|---:|---:|---:|---:|---:|
|1×2|7.928660|37|4.667×|24|3.027×|3.7|
|2×2|11.151932|59|5.291×|30|2.690×|5.9|
|4×2|16.086386|109|6.776×|48|2.984×|10.9|
|8×2|24.179453|189|7.817×|74|3.060×|18.9|
|12×2|33.884455|284|8.381×|101|2.981×|28.4|
|16×2|40.077381|335|8.359×|124|3.094×|33.5|
|24×2|65.850777|528|8.018×|176|2.673×|52.8|
|merge24×1|65.060937|507|7.793×|175|2.690×|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference. The all-eight CPU10× goal remains active and unmet, **0/8**. No new AWS/Aeglos comparison is established. Close this native-state recipe without a layout, size or worker-count rescue.

[Review](artifacts/review.json), [analysis](artifacts/analysis.json), [design](artifacts/design.md), [protocol](artifacts/protocol.md), [work audit](artifacts/work-audit.json), [source audit](artifacts/source-audit.json) and [checksums](SHA256SUMS) accompany lossless source, receipts/helpers and six raw profile archives. Private point/scalar/key/witness arrays, executables and libraries are excluded; synthetic generator fixture source is retained.
