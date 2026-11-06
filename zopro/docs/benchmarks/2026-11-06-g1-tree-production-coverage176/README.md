# Production coverage of the G1 tree experiment

The [complete-caller experiment](../2026-11-06-g1-tree-callers176/README.md) reduced the cost and memory of large grouped reductions but did not pass its acceptance gate. This read-only audit checks which accepted proof paths it changes. It reuses existing accepted 1×2 profiles; no new proving time is measured.

The large-group component gain does not establish a proof breakthrough. Source and saved-profile coverage redirect further optimization toward the production streaming addition kernels and their useful work, while retaining the smaller tree result as unadopted evidence.

At 32 workers, accepted 1×2 flat cycle samples place 20.203% in G1 streamed addition, 15.634% in G2 streamed addition, 5.378% in G1 tree reduce_layer, 5.827% in G2 tree reduce_layer, 7.650% in the shared scalar Fq inverse and 8.279% in Crossbeam with_handle. At physical16, the corresponding fractions are 25.113%, 18.242%, 4.930%, 6.511%, 8.685% and 2.697%. Both accepted profiles already qualified; this audit conserves all 20,837 existing samples and their shares without recapture or redecode.

The G1 tree candidate's stream_g1_impl body is byte-exact with accepted production (SHA256 9a750af6da629accac099c9bdaa0568326bb1eababcbb871cd85211dbf71a464). Its changed tree storage is used in grouped/indexed/weighted reductions. Weighted leaves remain 256 buckets, indexed G1 leaves at most 2048 points; their scratch geometry differs from 32 workers each repeatedly reducing the same 35,973-point group. Grouped equal-scalar spans can be larger, but their actual sizes/frequencies are not established by this static audit. A fixture length matching an input-vector length is not evidence that production runs that entire vector as one reduction group on every worker.

These flat sample percentages cannot be converted into exclusive critical-path fractions, removable wall time or an Amdahl latency ceiling. The shared inverse includes callers from both curves and other phases. Saved profiles explicitly do not validate absolute sampled-period coverage. The earlier observer diagnostic gate remains false at physical16 (control calibration and observer overhead); its phase spans are not substituted for accepted scheduling. No new SDK measurement, proof, native test, profile or production change was made.

Existing experiments already reject making weighted groups serial on their window worker, increasing their iterator grain, and sharing the final weighted frontiers in their tested organizations. High Crossbeam sample share alone does not select a new scheduler or imply that share is removable. Another candidate must identify a concrete useful-work/dependency/lifetime change in the streamed production path and compare it against those closed designs before implementation. The current tree experiment remains closed with all ten failures; no full-SDK candidate or production adoption is selected by this audit.


## Exact source coverage

The accepted 176 production files and SDK6d83 are hash-checked. The separate G1 tree candidate retains the streaming kernel byte-for-byte. Source anchors are conditional paths, not captured execution counts.

|Source|Line|Role|
|---|---:|---|
|[crates/zopro-core/src/lib.rs](../../../crates/zopro-core/src/lib.rs#L2189)|2189|native canonical solver and conditional A / fused K+H / G2 query fork|
|[crates/zopro-core/src/msm_affine.rs](../../../crates/zopro-core/src/msm_affine.rs#L152)|152|equal-scalar base sums call group_affine before msm_raw|
|[crates/zopro-core/src/msm_affine.rs](../../../crates/zopro-core/src/msm_affine.rs#L1681)|1681|exclusive B2 scalars choose separate G2 decomposition/grouping|
|[crates/zopro-core/src/msm_affine.rs](../../../crates/zopro-core/src/msm_affine.rs#L670)|670|narrow top digits collect order and call indexed tree plus weighted reduction|
|[crates/zopro-core/src/msm_affine.rs](../../../crates/zopro-core/src/msm_affine.rs#L469)|469|ordinary bucket filling and final weighted reduction are different functions|
|[crates/zopro-core/src/msm.rs](../../../crates/zopro-core/src/msm.rs#L219)|219|equal-scalar groups copy host-limb points into worker scratch then reduce layers|
|[crates/zopro-core/src/msm.rs](../../../crates/zopro-core/src/msm.rs#L273)|273|G1 indexed leaf tiles use 2048 points, then grouped partial sums|
|[crates/zopro-core/src/msm.rs](../../../crates/zopro-core/src/msm.rs#L313)|313|256-bucket parallel chunks allocate 2*n-1 host-limb tree slots and call weighted_sum|
|[crates/zopro-core/src/msm.rs](../../../crates/zopro-core/src/msm.rs#L1082)|1082|each weighted level copies even points into corrections before reduce_layer|
|[crates/zopro-core/src/msm.rs](../../../crates/zopro-core/src/msm.rs#L843)|843|separate streamed addition kernel; byte-exact between accepted and candidate|
|[crates/zopro-core/src/msm_affine/fused_kh.rs](../../../crates/zopro-core/src/msm_affine/fused_kh.rs#L17)|17|fused K/H selects borrowed pending path on supported native inputs|
|[crates/zopro-core/src/msm_affine/borrowed_pending/balanced.rs](../../../crates/zopro-core/src/msm_affine/borrowed_pending/balanced.rs#L64)|64|balanced K/H has20 windows then256-bucket weighted groups; no narrow-top special branch|

## Existing accepted profile locations

Counts and shares below are flat user-cycle samples from the accepted SDK, not the instrumented candidate. They conserve all saved bins. Ownership/period/loss validation is inherited from the [existing coarse-phase review](../2026-11-06-coarse-proof-phase-universal176/README.md); no raw capture or decode was repeated. The saved absolute-period-coverage limitation and failed observer gate remain visible in the analysis. These two profiles do not establish coverage for all shapes.

|Location|16-worker samples / share|32-worker samples / share|
|---|---:|---:|
|G1 streamed add|2160 / 25.113%|2472 / 20.203%|
|G2 streamed add|1569 / 18.242%|1913 / 15.634%|
|G1 tree reduce_layer|424 / 4.930%|658 / 5.378%|
|G2 tree reduce_layer|560 / 6.511%|713 / 5.827%|
|shared scalar Fq inverse|747 / 8.685%|936 / 7.650%|
|Crossbeam with_handle|232 / 2.697%|1013 / 8.279%|
|other|2793 / 32.473%|4319 / 35.297%|
|unknown|116 / 1.349%|212 / 1.733%|

High sampled Crossbeam use at 32 workers is useful evidence, but it does not establish a new scheduler optimization. The [window-local weighted](../2026-10-22-universal-window-local-weighted-sdk/README.md), [weighted task-grain](../2026-10-22-universal-weighted-task-grain/README.md) and [shared-frontier](../2026-10-31-weighted-frontier176/README.md) experiments already tested distinct attempts in that direction and remain rejected.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.




## Continuation and artifacts

The [next step](artifacts/next.md) is a useful-work/dependency audit of the accepted streamed kernels before choosing another implementation. No new runtime candidate, full-SDK trial or adoption is selected. The full all-eight CPU10× goal remains active and unmet.

The one read-only Rust/xtask step passed on its first execution. All source pins, saved sample conservation checks and exact stream-body comparison passed. The helper archive contains the audit source, design, command/exit/output receipts and publication sources; it does not contain SDK/library binaries, private inputs, keys/witnesses/proofs or raw profiles. Existing profile binaries/recordings and their original ownership evidence remain at their source experiment.

[Analysis](artifacts/analysis.json), [design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [audit source](artifacts/audit.rs), [plan](artifacts/plan.json), [input pins](artifacts/pins.json), [helpers and receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS).
