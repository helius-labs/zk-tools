# Lehmer inversion in complete MSM addition callers

The complete-caller gate is **false**. Production remains unchanged. These are synthetic valid-curve caller measurements, including inversion and surrounding AVX arithmetic; they are not proof latency.

The primary borrowed logical32 aggregate is 0.999705890 versus OFF, bounds [0.996959395, 1.001823231], with17/31wins: a 0.029% estimated saving that is not distinguishable from parity. Versus frozen it is 0.999239129 [0.995305531, 1.002475096], with16/31wins. Both declared gain checks fail. All candidate nonregression checks pass; one OFF/frozen calibration fails for owned G1/352/logical32, ratio1.015797974 [1.003700857, 1.044476360]. This exact implementation closes without an SDK trial or adoption.

The descriptive all18 aggregate retains a 0.677% saving versus OFF and 0.205% versus frozen, with both intervals below parity. Owned G1 gains versus OFF do not establish the same gains versus the frozen baseline: all six owned ON/frozen intervals cross parity. These positives remain reported, without promoting a different subset after timing. The ON profiles contain the Lehmer simulation and finish functions; they do not show a removal of the inversion work or establish an exclusive proof-wall cost.

The candidate replaces native batched binary GCD with fixed-size variable-time Lehmer extended GCD. A leading-word projection combines Euclidean quotient steps; ordinary coefficient magnitudes grow without a modular reduction after every matrix. One final multiplication restores Montgomery form. Exact quotient handling and the original inverse cover rejected batches. Fixed stack arrays add no heap, per-key table, point copy or execution plan. The leading-word recurrence is adapted from Go math/big; [BSD license](artifacts/GO-LICENSE) and exact source are retained.

Ryzen 9950X, 16 physical/32 logical CPUs. Solo CPU2; physical16 CPUs0–15; logical32 CPUs0–31. Thirty-one fixed rotated/reversed blocks compare accepted frozen libraries, fresh OFF and ON. Each worker runs 8192 changing complete additions after three checked 1024-call primes. A common barrier surrounds every worker timer; independent generator sums and capacities are checked outside the interval. All timed allocation counts are zero. No owned build/helper/hash/archive/analyzer/other workload overlaps captures.

Times are microseconds per complete batch wave, including the slowest worker/barrier tail. Paired ratios can differ from ratios of marginal medians.

|Context|Frozen µs|OFF µs|ON µs|ON/OFF [bounds]|Wins/31|
|---|---:|---:|---:|---|---:|
|g1-owned-200-solo|4.130423|4.225143|4.115794|0.969200352 [0.966892619, 0.978833693]|31|
|g1-owned-200-physical16|4.654001|4.748946|4.633469|0.976987454 [0.974436163, 0.981198425]|31|
|g1-owned-200-logical32|6.880365|6.984040|6.880202|0.983561019 [0.978847121, 0.991687617]|28|
|g1-owned-352-solo|6.746085|6.838441|6.730914|0.987492759 [0.979880796, 0.990572683]|31|
|g1-owned-352-physical16|7.716473|7.805889|7.679692|0.985021301 [0.983288040, 0.990008410]|27|
|g1-owned-352-logical32|11.469356|11.657733|11.485890|0.983065429 [0.960174191, 0.993150090]|26|
|g1-borrowed-signed-200-solo|4.353804|4.345116|4.334691|0.994078667 [0.992667655, 1.003882750]|16|
|g1-borrowed-signed-200-physical16|4.878202|4.882571|4.875519|0.998260596 [0.995888322, 1.000439131]|21|
|g1-borrowed-signed-200-logical32|7.078439|7.101760|7.096669|0.999647549 [0.996257595, 1.002957538]|16|
|g1-borrowed-signed-352-solo|7.108347|7.107989|7.103635|1.001686065 [0.993476086, 1.006124696]|15|
|g1-borrowed-signed-352-physical16|8.114067|8.087950|8.108975|1.000505682 [0.997853908, 1.005603667]|15|
|g1-borrowed-signed-352-logical32|11.816619|11.824687|11.814867|0.998131238 [0.995033828, 1.003609968]|19|
|g2-borrowed-signed-200-solo|8.497814|8.496070|8.479990|0.999267016 [0.993049239, 1.004293430]|17|
|g2-borrowed-signed-200-physical16|9.933395|9.908351|9.901623|0.999864965 [0.992460417, 1.001906191]|16|
|g2-borrowed-signed-200-logical32|15.960854|15.932661|15.937063|0.998409816 [0.997312628, 1.003596593]|19|
|g2-borrowed-signed-352-solo|14.281466|14.286400|14.263947|0.998673538 [0.993142482, 1.005774073]|17|
|g2-borrowed-signed-352-physical16|16.888932|16.913758|16.909381|0.998039821 [0.988000680, 1.002667493]|19|
|g2-borrowed-signed-352-logical32|27.590594|27.493427|27.495004|1.000407724 [0.995528374, 1.005890370]|13|

All 18 OFF/frozen calibration intervals must fit factor1.03. Every ON/control upper ratio must be<=1.02. The primary equal-log aggregate consists of four borrowed logical32 contexts (two curves × two sizes), requiring upper<1 and at least21/31wins against both controls. The initial design miscounted these as six; [the prospective clarification](artifacts/capture-design.md) fixed the matrix before any timing. All18 and logical32-six aggregates are descriptive. Ranks10–22 of31 provide nominal97.055% marginal coverage conditional on exchangeable blocks, without a familywise guarantee. No gain floor, exclusions, extension, changed threshold or unchanged reroll.

|Aggregate|Ratio [bounds]|Wins/31|
|---|---|---:|
|all18/on/frozen|0.997950013 [0.996507896, 0.999383501]|24|
|all18/on/off|0.993228315 [0.990663738, 0.995610428]|30|
|borrowed_logical32_4/on/frozen|0.999239129 [0.995305531, 1.002475096]|16|
|borrowed_logical32_4/on/off|0.999705890 [0.996959395, 1.001823231]|17|
|logical32_6/on/frozen|0.999680496 [0.997005805, 1.002136790]|16|
|logical32_6/on/off|0.993446430 [0.990349886, 0.996879532]|25|

Failed checks: `["borrowed_logical32_4/on/frozen/gain","borrowed_logical32_4/on/off/gain","g1-owned-352-logical32/off/frozen"]`. All comparisons and positive/negative estimates remain in [compressed analysis](artifacts/analysis.json.gz). A caller pass selects separate full-SDK qualification; it does not adopt a proof change.

Field OFF passes33 tests; checked and optimized ON each pass36. New independent checks cover14327 BigInt matrices/148027 exact Euclidean quotient steps and9573 complete inverses per modulus against Arkworks and the original inverse. Repeated checked/optimized execution does not double distinct inputs. Both caller builds pass128 core tests, including four complete point adapters. Field arithmetic is byte-identical between qualified and caller snapshots. Three source edits (one runtime parent, two test-only manifests) reverse all175 originals; the caller adds two diagnostic exports and one adapter. Runtime point formulas are unchanged. No portable/resource/final-SDK qualification is claimed.

Four fixed16Mcycles:u profiles follow all latency, covering borrowed G1/G2 200-point callers at logical32, OFF/ON. FIFO gates exclude primes and output checks. Metadata requires the exact binary owner PID, recorded thread membership, samples on each worker's pinned CPU, every worker sampled, fixed period/time/CPU/mappings, >=1000samples, <=5%unknown period and zeroLOST. Profiles are separate from latency; symbol shares are not exclusive costs or removable proof wall time.

|Profile|Samples|Unknown period share|Qualified|
|---|---:|---:|---|
|profile-g1-borrowed-signed-200-logical32-off|15553|0.000257|true|
|profile-g1-borrowed-signed-200-logical32-on|15727|0.000382|true|
|profile-g2-borrowed-signed-200-logical32-off|33941|0.000442|true|
|profile-g2-borrowed-signed-200-logical32-on|33935|0.000619|true|

All1732 caller processes and1752 capture/postprocessing commands finish. Independent publication review recomputes60 paired distributions and56 decisions from raw per-process integer wall times and verifies1763 successful workflow commands plus one retained failed native freeze command. Sources, receipts and helpers round-trip byte-exact through compressed archives. Raw perf files, executables and libraries remain local with hashes. Component lifetime RSS and scratch capacities are retained in raw rows; they are not whole-prover RAM, compiler-stack bounds or minimum RAM.

Setup stopped on a missing Debian Go license path; the upstream matching BSD license and a separate resume helper completed source audit without repeating source edits. A missing workflow-version field was corrected before native commands. A copied native freezer then used a binary path as its receipt directory; successful OFF128 tests/library build were reused, and a separate helper corrected only receipt paths. The continuation preparer's initial error-text assertion was corrected from ENOENT to the actual ENOTDIR, before it wrote a plan. Original helpers, logs and failures are archived. No arithmetic repair or successful test/build/capture retry replaced an outcome.

The first capture preparation stopped before writing a plan because its source review decoded an unchanged binary constants file as UTF-8. The repaired review hashes unchanged members as bytes and decodes only modified text parents. After all captures completed, the original finisher stopped because it expected a literal `PERIOD` sample-type label. Fixed-period perf events can store that period in the event header; every decoded sample here carries16000000. A separate analyzer validates the exact constant period/header and preserves every per-sample PID/TID/CPU/time/loss check. Four new parser tests reject frequency mode, a wrong period and missing time; the original five tests are inherited unchanged. Original helpers and failures remain archived. No native or performance capture was repeated. [Repair](artifacts/analysis-repair.md), [independent check](artifacts/analysis-repair-review.json), [source-review repair](artifacts/capture-preparation-repair.md).

## Accepted proof comparisons

These are inherited accepted-production timings and supplied historical M5 references, not new Lehmer proof measurements. The accepted physical16 calibration engineering exception remains recorded. No new AWS/Aeglos comparison was made.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|8.022165|37 / 4.612×|24 / 2.992×|3.7|
|2x2|11.335969|59 / 5.205×|30 / 2.646×|5.9|
|4x2|15.945544|109 / 6.836×|48 / 3.010×|10.9|
|8x2|24.090485|189 / 7.845×|74 / 3.072×|18.9|
|12x2|33.844855|284 / 8.391×|101 / 2.984×|28.4|
|16x2|39.931557|335 / 8.389×|124 / 3.105×|33.5|
|24x2|66.233309|528 / 7.972×|176 / 2.657×|52.8|
|merge-24x1|65.273224|507 / 7.767×|175 / 2.681×|50.7|
|merge-54x1|162.250464|—|—|—|

All eight CPU10x targets remain unmet. Accepted primary is8.022165ms/134MiB;54→1 is162.250464ms/1831.867MiB with no supplied M5 reference. Latest retained C7a primary: Zopro11.915591ms/32workers versus Aeglos7.959184ms/96workers in their best tested configurations. Exact nine local/AWS timing/CPU/RSS rows remain in [accepted rows](accepted-rows.json). No SDK/Gnark proof is generated here; verified ledger26203 is unchanged. There is no new mobile/ARM/TEE, huge250/4999, pressure/swap/minimum-RAM or concurrent-throughput result. The all-eight goal remains active.

The full analysis and [workflow](artifacts/run-plan.json.gz) are compressed for storage. Decompression matches the original reviewed JSON byte for byte; no samples, protocol or decisions change. Restore the two JSON files before replaying the original archive reviewer. [Storage check](artifacts/storage-review.json).

[Protocol](artifacts/design.md), [capture protocol](artifacts/capture-design.md), [source review](artifacts/bundle-review.json), [independent review](artifacts/review.json), [archive check](artifacts/post-publication-review.json), [field source](artifacts/source.json.gz), [caller source](artifacts/caller-source.json.gz), [helpers and raw observations](artifacts/helpers.json.gz), [receipts](artifacts/receipts.json.gz), [checksums](SHA256SUMS).
