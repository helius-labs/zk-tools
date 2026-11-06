# G2 joint binary-ring work screen

The fixed radix-128 model reduces generic additions by **9.61% on primary** and **11.87% on raw 2×2**, after charging plane/part merges and both coordinate reductions. Independent review reproduces the counts. This passes a necessary work-count screen only: **no native implementation, timing, memory qualification or adoption** follows from these counts. Accepted production is unchanged.

The earlier [binary-ring](../2026-10-22-universal-binary-ring-native/README.md) and [hexagonal](../2026-10-22-universal-hexagonal-native/README.md) trials changed G1 only and remain rejected. This screen asks whether joint digits help the current G2 stream. It is an application of the same ring idea, not a new MSM algorithm or a repeat of those native builds. Radix 128, nineteen windows and the original two point-range parts were fixed before counting.

|Query|Original full dense model additions|Ring model additions|Ring / original|Original bucket φ maps|Ring bucket φ maps|
|---|---:|---:|---:|---:|---:|
|1x2-B|634405|573432|0.903889|41022|94042|
|2x2-B|1085418|956616|0.881334|41071|95162|

Primary digit arrivals fall from 613,891 to 521,649. The complete generic-add model falls from 634,405 to 573,432; 60,973 fewer additions accompany 53,020 extra affine φ maps, costing 106,040 additional Fq multiplications. Nineteen final axis-result φ maps are separate. Dense Horner doublings increase from 120 to 126. These different operations have no measured exchange rate. Raw 2×2 is not its admitted grouped runtime.

The scalar's existing signed Phi2 pair is recoded in Z[ω], ω²+ω+1=0. Six units partition 16,383 nonzero residues into 2,731 representative classes, including one three-element orbit. Three signed planes preserve the original points and apply endomorphisms to completed bucket sums. Merged representatives enter signed magnitude groups for each coordinate, followed by two dense weighted-fold models and an axis-result combination. No key-dependent point table, transformed key copy or retained duplicate plan is proposed.

Both recipes include every position in a hypothetical dense model, including their top. This is not the accepted indexed-top execution cost or a timing prediction. Formal noncancelling populations do not price equal-x handling, exceptional projective work, inversion and SIMD batch geometry, point conversion, recoding, allocation, weighted-tree projective operations or scheduling. A smaller count is not a lower bound on native latency and does not qualify a twofold proof improvement.

## Memory cost and next gate

The ring has 8,193 G2 affine bucket slots per part, **1,048,704 coordinate bytes versus 262,144** for the original 2,048 slots. If all modeled dense part arrays coexist, coordinate payload alone is about **38.005 MiB versus 5.5 MiB**. This is a material increase, not measured liveness, peak RSS or minimum RAM. Metadata, projective exceptions, batch queues, prefix/axis/tree scratch, key/witness/solver/FFT state, allocator behavior and stacks add costs. Scheduling fewer simultaneous parts needs an explicit ownership and admission design.

A proposed compact residue map would occupy 64 KiB, plus 5,462 bytes of representatives. Nineteen i16 digit codes occupy 38 bytes per scalar and may coexist with original split words during preparation. The diagnostic's larger Rust maps are not a proposed runtime layout or memory bound. See [memory accounting](artifacts/memory-cost.md).

The next stage is a complete operation census of actual indexed-top and coordinate weighted trees, plus a checked lifetime/space design for every new vector and original borrowed-input fallback. Larger bucket planes, extra φ work and altered batching may erase the apparent benefit. No native recording is selected until that design exists; portable/mobile/huge-circuit support must retain the original bounded fallback.

## Correctness and provenance

Three new algebraic tests pass on their first execution: every residue/unit including the three-element orbit; 77 signed coordinate boundary pairs and 20,000 random pairs; and weighted-plane conservation with changing formal points. Nineteen-digit termination has an exact conservative rational norm bound. All 82,363 real fixture rows reconstruct their signed coordinate pair exactly and the original Fr/lambda scalar identity; each window's weighted token total agrees in two small-prime modules. These are not new elliptic-curve or proof oracles. Decomposition and complete fixture Arkworks bindings are inherited.

The model uses J(a,b)=(-b,a-b), with J²+J+1=0, so P and φ(P) are distinct. The sealed design names another equivalent token basis from an earlier screen; the [scope clarification](artifacts/model-scope.md) records this wording discrepancy without editing its pins or rerunning the study.

The independent reviewer imports no study code. It derives sorted residue orbits with explicit unit formulas, divides via direct subtraction, reconstructs weighted integers, counts sorted population sets and extracts Booth digits bit by bit. It reproduces every ring and original row, both φ-map counts and the necessary gate, verifies 198 source/input/tool/library pins and four successful Rust/xtask command receipts and stream hashes. All 176 production source hashes remain exact. The exact original field library is used; existing field/core/prover qualification is inherited without rerun.

One reviewer preparation compile failed because serde_json::Value cannot be directly compared with i128. The retained initial source/log and a narrow integer comparison repair precede the first reviewer execution; no successful build, model test, count or performance recording was repeated. The four-step model workflow itself completes first attempt without failures.

A separate R260 source/history audit found no new hot-loop removal: earlier paths already retain R260 points/buckets, and their scalar inverse boundary keeps five vector products. Current ordinary mixed multiplication and earlier uniform fused multiplication already omit their respective final corrections. The audit selects no representation build or timing rerun. See [design](artifacts/design.md).

Zero new SDK requests, Gnark proofs, performance captures or profiles. Canonical distinct-proof ledger remains **30,708**. No new AWS/Aeglos, ARM/phone/TEE, pressure/swap, minimum-RAM, 250/4999 or concurrent-throughput result. The all-eight CPU10× goal remains active and unmet, **0/8**.

## Accepted CPU and Metal comparison, inherited

Accepted local 32-worker full-proof measurements and supplied historical M5 references follow. This screen remeasures none of them.

|Shape|Accepted ms|M5 CPU ms|M5 Metal ms|CPU/local|Metal/local|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37|24|4.667×|3.027×|3.7|
|2x2|11.151932|59|30|5.291×|2.690×|5.9|
|4x2|16.086386|109|48|6.776×|2.984×|10.9|
|8x2|24.179453|189|74|7.817×|3.060×|18.9|
|12x2|33.884455|284|101|8.381×|2.981×|28.4|
|16x2|40.077381|335|124|8.359×|3.094×|33.5|
|24x2|65.850777|528|176|8.018×|2.673×|52.8|
|merge-24x1|65.060937|507|175|7.793×|2.690×|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference. Accepted SDK SHA-256 is `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`.

[Counts](artifacts/analysis.json), [review](artifacts/review.json), [model source](artifacts/study.rs), [review source](artifacts/review.rs), [workflow](artifacts/plan.json), [exact receipts](artifacts/receipts.json), [checksums](SHA256SUMS). Reproduction uses the exact original field library, two private fixtures and compiler argv pinned in the workflow. Raw scalars/point memberships, points, keys, witnesses, binaries and libraries are excluded from publication. Private root: `/home/ubuntu/src/zopro-perf-work/g2-joint-ring-screen176`.
