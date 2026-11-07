# Descriptor sign-mask collection in complete proofs

The native both-curve descriptor-mask candidate measures **7.988860 ms** at 32 workers, versus **8.032278 ms** for the accepted SDK and **8.010056 ms** for the original-source control. Paired gains are **0.476%** and **0.350%**, respectively, with both confidence intervals below 1. The full gate **fails two 16-worker checks**, so this build is **not adopted**. The calibrated 32-worker gain is retained; the failed broader gate is not described as a measured regression.

This separate complete-SDK study was declared as an engineering exception after the [caller screen](../2026-11-07-sign-mask-callers176/README.md) failed seven checks. It preserves the same helper and dispatch for both curves and removes diagnostic adapters only. There is no G1-only variant, compiler/layout tuning, sample extension, timing reroll or topology-subset adoption.

The helper replaces shifted-bit collection with a vector sign-mask intrinsic. The original loop already vectorizes its loads. Two masked loads read only the initialized encoded-index dwords of eight borrowed descriptors; unsupported layouts and platforms retain the original loop. No field arithmetic, solver, key ownership, queues, MSM geometry, resource schedule or pool changes. There is no added table, key copy or execution plan.

Thirty-one fixed rotated/reversed blocks compare three immutable binaries at 32 logical CPUs and 16 physical cores on the local Ryzen 9 9950X. Each process performs 3 untimed primes and 251 observed requests. The measured request includes solving, fresh randomness, proving, internal verification and serialization; loading, primes and proof export are excluded. All 186 latency processes finish before independent Gnark verification and the separate profiling campaign. No owned build, hashing, analysis, report or push overlaps recording. The original-source control is reused, not newly rebuilt; legacy analyzer labels `fresh` refer to that control.

|Workers|Accepted ms|Original control ms|Candidate ms|Candidate / accepted, paired 95% CI|Candidate / control, paired 95% CI|
|---|---:|---:|---:|---|---|
|32|8.032278|8.010056|7.988860|0.995245 [0.992915, 0.997603], 24/31 wins|0.996498 [0.993019, 0.999837], 21/31 wins|
|16|9.794663|9.725310|9.661972|0.978418 [0.946336, 1.011668], 19/31 wins|0.991499 [0.961990, 1.021205], 16/31 wins|

Times are medians of process means; paired estimates use all 31 block ratios and the unchanged fixed bootstrap utility. There is no minimum gain floor or dropped block. Both 32-worker comparisons and control calibration pass. At 16 workers, control/accepted has interval [0.956930, 1.018539], outside the declared [1/1.03, 1.03] calibration range. Candidate/control has upper bound 1.021205, above the declared 1.02 nonregression limit. Candidate/accepted passes 16-worker nonregression. These wide intervals prevent the intended broader qualification even though candidate medians are lower. The exact build is closed without an unchanged retry.

Six separate 31-request PMU panels and six 251-request user-cycle profiles pass their declared quality checks: 62,603 samples, at least 8,577 per profile, unknown symbols at most 1.759%, and zero LOST records. At 32 workers, candidate/control instructions decrease 0.356% and cycles decrease 1.593%; candidate/accepted decreases are 0.236% and 3.066%. These single panels have no repeated-panel uncertainty estimate. Flat profiles remain dominated by G1/G2 MSM streams, inversions and reductions; inlining prevents attribution of a precise time saving to sign collection. The profile data do not establish exclusive wall-time fractions or isolated causality.

Lifetime process peak RSS in matched latency processes is 137.75 / 138.75 / 138.25 MiB for accepted / original control / candidate at 32 workers, and 92 / 93.5 / 93.25 MiB at 16. This includes loading/runtime/primes and is neither minimum RAM nor a pressure/device qualification. The source adds no heap allocation; no separate resource-admission campaign follows a failed primary gate.

The 177-file snapshot recovers all 176 accepted sources: 175 are byte exact; only `msm.rs` dispatch/module inclusion and the qualified child differ. One coherent eight-library SDK set uses O3/native/fat LTO/one codegen unit and the unchanged benchmark driver/support. The successful generic/no_asm/no_ifma check is inherited from the caller through an audit of the native-only guards and removed diagnostics; it is not rerun. Accepted binary SHA starts `6d83f1d6`, original control `64bfa94a`, candidate `60fed132`.

Correctness preflights cover all nine shapes at 32 workers and primary at 1/16 workers for control/candidate. The complete stage performs **49,060 internally verified SDK requests and 220 fresh independently Gnark-verified exports**, with image/public/VK bindings, 164-byte proofs and 44-byte public inputs. The distinct-proof ledger advances **32,857 → 32,879 → 33,065 → 33,077**. Review checks 472 successful workflow receipts, source recovery, library/binary pins, raw stream hashes and unchanged statistics. No successful old test, model, recording, verifier, bootstrap analysis or decoder is replayed for bookkeeping.

Two review-reader assumptions required correction: source-manifest members can contain non-UTF8 bytes, and a structured `link-sdk` step records its actual compiler arguments separately from the plan. Original failures and repaired readers are retained. The repairs change only evidence handling; measurements, gates and successful workload/verifier/profile executions are unchanged.

The accepted SDK remains unchanged. Its latest matched observation is now 8.032278 ms; historical accepted measurements below 8 ms remain valid observations of the same binary. The 7.988860 ms candidate is not the production baseline. No cause is established for between-run variation.

The following all-shape accepted results and supplied M5 references are inherited from [adoption](../2026-10-30-g2-vector-sign-adoption176/README.md), not remeasured by this primary pilot. All eight still miss the requested verified 10× CPU target.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24×1|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference. No new AWS/Aeglos, Apple, mobile, TEE, forester, swap or large-circuit result is claimed. The original all-eight goal remains active, **0/8** achieved. Further work should reduce dominant MSM work through a distinct mechanism; this mask study supplies a small positive result at 32 workers, not a path to the required step change by itself.

[Protocol and sources/receipts](artifacts/evidence.tar.gz), [latency](artifacts/latency-analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [source audit](artifacts/source-audit.json), [native qualification](artifacts/sdk-native-qualification.json), [fallback audit](artifacts/inherited-portable.json), [decision](artifacts/decision.json), [review](artifacts/review.json), [checksums](SHA256SUMS).
