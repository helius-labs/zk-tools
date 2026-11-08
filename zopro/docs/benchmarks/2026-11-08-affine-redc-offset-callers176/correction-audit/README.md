# Deferred affine-offset correction audit

The complete-caller candidate remains rejected. Four new offline decodes of its exact OFF/ON G1/G2 functions identify two new raw-correction branches in G1 and five in G2. The saved cycle profiles put some samples in these correction regions, supporting a focused control-path experiment. They do not identify the addresses responsible for branch misses or establish causal latency. No runtime code, native call, proof, timing block or profile recording is added.

|Caller|OFF instruction sites|ON instruction sites|New ON raw-correction conditional branches|
|---|---:|---:|---:|
|G1|4425|4411|2|
|G2|8590|8444|5|

Static sites cover each entire compiled function, including tails, fallbacks and checks. They are not dynamically retired instructions. Both functions are smaller in ON, consistent with the counter observation of fewer retired instructions; smaller code alone does not imply lower latency.

|ON profile|Total samples|Point-function samples|New correction-region samples|Share of total|
|---|---:|---:|---:|---:|
|cycles-G1-200-w32-ON|3820|3209|105|2.749%|
|cycles-G1-352-w32-ON|6325|5754|167|2.640%|
|cycles-G2-200-w32-ON|7800|7202|308|3.949%|
|cycles-G2-352-w32-ON|13373|12808|549|4.105%|

All 57943 point-function samples across eight saved profiles relocate to exact decoded instruction boundaries. Relocation uses the actual executable's ELF executable PT_LOAD segment plus that recording's MMAP2 base and file offset. Here virtual addresses differ from file offsets by 0x1000; omitting that difference would misattribute samples. Every input stream and executable is bound by SHA-256. Four decoding commands have successful Rust/xtask receipts.

Each new branch is a `vpcmpnltuq` / `kortestb` / forward `je` sequence attributed to `offset_reduce_raw` at field source line 230. A correction region starts at that compare and ends immediately before its forward skip target; it includes subtraction, borrow propagation and blends on the fallthrough path. Region samples are not branch-taken frequencies, exclusive costs or a prediction of removable time. Hardware cycle sampling can skid. OFF has no instances of this newly introduced helper; zero OFF samples in this category is definitional, not evidence of zero original correction cost.

The [parent caller report](../README.md) retains the calibrated G2 regressions and automatic counter panels. Those panels show G2 branch misses rising roughly 2.5–3.5× while instructions decrease. Combined with the identified branches, that makes an unconditional masked correction worth one distinct experiment. This is an inference, not a measured speedup. It would change the correction mechanism and require new native/caller correctness, source identity and a fresh prospectively frozen complete comparison. The rejected implementation is not rerun unchanged or promoted as a G1-only subset. No multi-gigabyte table, larger group record, new production pool or key copy is implicated.

The goal remains verified single-proof CPU AVX latency at least 10× faster than all supplied M5 CPU timings, reporting CPU and Metal comparisons. It is unmet, 0/8. Production176/accepted SDK6d83 and canonical inventory 36,477 are unchanged. This audit adds no new SDK, Gnark, AWS/Aeglos, full-proof or memory-pressure result.

[Detailed rows and pins](analysis.json), [decoder/analysis sources and receipts](evidence.json.gz), [checksums](SHA256SUMS). The original saved profile streams are in the parent report's checked [evidence archive](../artifacts/evidence.json.gz).
