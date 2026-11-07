# Descriptor sign-mask: first all-shape qualification

**Not adopted.** All eight new shape contexts pass control calibration and the 2% nonregression bound against both controls. The original-eight equal-log estimate improves about **0.09%**, but both stratified confidence intervals cross parity. The calibrated primary and 2×2 gains remain scoped positive results; a broad gain is not established. Production stays unchanged.

This study gathers the missing shape evidence as an explicit engineering exception to the preceding closure. It preserves the failed 16-worker gate and reuses the original 31-block primary measurements. No primary or 16-worker recording is repeated, and no source, compiler, layout or gate changes. The [prior study](../2026-11-07-sign-mask-sdk176/README.md) retains its two failed checks.

The immutable accepted, original-control and candidate SDKs are reused (SHA prefixes `6d83f1d6`, `64bfa94a`, `60fed132`). The helper changes only sign-bit collection for native borrowed descriptors, for both curves. It adds no field arithmetic, heap operation, key copy, lookup, plan, pool or resource-policy change. Existing all-nine correctness, source recovery, coherent eight-library builds and generic/no_asm/no_ifma qualification are inherited without replay.

On the local Ryzen 9950X, 15 fixed rotated/reversed blocks cover 2×2 through merge-54, three builds at 32 workers on CPUs 0–31. Each process has three untimed primes and 51 observed complete SDK requests. Solving, fresh randomness, proving, internal verification and serialization are timed; loading, primes and export are excluded. All 360 latency processes finish before 360 independent Gnark verifiers. Automatic counters and profiles follow latency; no owned build, hashing, analysis, report or push overlaps recording. Times below are medians of process means; paired ratios use all matched block means.

|Shape|Accepted ms|Original control ms|Candidate ms|Candidate / accepted, paired 95% CI|
|---|---:|---:|---:|---|
|1x2 (inherited)|8.032278|8.010056|7.988860|0.995245 [0.992915, 0.997603]|
|2x2|11.153166|11.148368|11.115683|0.995655 [0.992138, 0.999356]|
|4x2|16.022575|16.062814|15.966536|0.998149 [0.990037, 1.007141]|
|8x2|24.181596|24.106532|24.232061|1.002841 [0.996959, 1.008977]|
|12x2|33.799383|33.806805|33.855479|0.999899 [0.992873, 1.006308]|
|16x2|40.105142|40.131773|40.253045|1.000709 [0.994123, 1.007598]|
|24x2|66.276435|66.386650|66.396099|1.002414 [0.997267, 1.007870]|
|merge-24x1|65.290835|65.078945|65.094458|0.997906 [0.993053, 1.002821]|
|merge-54x1|161.681638|161.591610|161.757834|0.999676 [0.994831, 1.004821]|

At 2×2, candidate/accepted is 0.995655 [0.992138, 0.999356], with 12/15 wins; candidate/control is 0.994907 [0.991568, 0.998498], with 11/15 wins. This establishes about 0.43–0.51% lower latency in this context. Most other shape intervals cross parity. No minimum gain floor, block exclusion, extension or reroll is used.

The original-eight aggregate includes the retained 31 primary blocks and 15 blocks for each of seven other reference shapes, excluding merge-54. Each shape has equal log weight. The 20,000 fixed-seed stratified resamples use each shape's own block ratios. Candidate/accepted is **0.999099 [0.997067, 1.001138]**; candidate/control is **0.999117 [0.997006, 1.001253]**. Both required upper-bound-below-1 checks fail; these are the only new failed latency checks. Intervals assume exchangeable blocks and independent shape strata; cross-shape and shared-host drift are not independently qualified. Prior 16-worker uncertainty remains unresolved.

Six separate 7-request PMU panels and six 17-request cycle profiles cover 24×2, merge-54 and all three builds, after three primes each. All profiles qualify: **92,887 samples**, at least 8,905 per profile, unknown symbols at most 0.754%, zero LOST records, and checked process/thread/CPU/mapping/event/16M-period/FIFO gates. PMU running time meets 99.9%. Candidate/control instruction ratios are 0.999593 at 24×2 and 1.001099 at merge-54; user-cycle ratios are 1.012024 and 0.998839. Single panels do not estimate repeatability or exclusive wall time; lower instruction counts do not establish a proof gain.

Lifetime process peak RSS from matched latency processes follows. This includes loading/runtime/primes, varies with allocation and scheduling, and is not minimum RAM or memory-pressure qualification. No new resource-admission campaign or adoption follows the failed gate.

|Shape|Accepted peak MiB|Original control peak MiB|Candidate peak MiB|
|---|---:|---:|---:|
|1x2|137.750|138.750|138.250|
|2x2|297.250|294.000|294.500|
|4x2|340.000|336.750|340.250|
|8x2|539.500|539.750|546.000|
|12x2|771.250|759.750|750.750|
|16x2|860.250|861.500|891.750|
|24x2|1371.750|1383.051|1367.410|
|merge-24x1|1249.000|1237.500|1244.750|
|merge-54x1|1956.574|1964.062|1956.871|

The new stage performs **19,620 internally verified SDK requests and 372 fresh independently Gnark-verified exports**: 19,440 latency requests with 360 exports, plus 180 diagnostic requests with 12 exports. Image, public-input, verification-key and freshness bindings pass; proofs are 164 bytes and public inputs are 44 bytes. The distinct-proof ledger advances **33,077 → 33,437 → 33,449**. Review checks 774 successful workflow receipts, raw samples and stream hashes, 176-source recovery, immutable SDK provenance, per-shape ratios and equal-log aggregate point estimates. The qualified RNG is byte exact. Successful bootstrap, proof-verifier, native-test, old-model, recording and decoder executions are not repeated for bookkeeping. All new readers, captures and review pass on first execution.

These are current accepted 32-worker observations with the supplied historical M5 references. The primary row is inherited from the immediately preceding matched campaign; other rows are new here. Candidate results above do not replace the accepted baseline.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1x2|8.032278|37 / 4.606×|24 / 2.988×|3.7|
|2x2|11.153166|59 / 5.290×|30 / 2.690×|5.9|
|4x2|16.022575|109 / 6.803×|48 / 2.996×|10.9|
|8x2|24.181596|189 / 7.816×|74 / 3.060×|18.9|
|12x2|33.799383|284 / 8.403×|101 / 2.988×|28.4|
|16x2|40.105142|335 / 8.353×|124 / 3.092×|33.5|
|24x2|66.276435|528 / 7.967×|176 / 2.656×|52.8|
|merge-24x1|65.290835|507 / 7.765×|175 / 2.680×|50.7|

Merge-54 has no supplied M5 reference. The original all-eight verified CPU 10× goal remains active and unmet, **0/8**. No new AWS/Aeglos, Apple, phone, ARM, TEE, forester, huge-circuit, swap or concurrent-throughput result is claimed. This closes the exact mask build with its positive primary/2×2 evidence and unresolved all-eight gain; no unchanged timing retry or subset adoption follows.

[Prospective design](artifacts/design.md), [latency and every context](artifacts/latency-analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [decision](artifacts/decision.json), [review](artifacts/review.json), [source/readers/receipts/decoded profiles](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
