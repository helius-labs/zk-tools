# G2 partition completion: native correctness

The isolated last-finisher scheduler passes native correctness qualification. **Performance is unmeasured; production is unchanged.** This stage changes G2 partition coordination while retaining the accepted arithmetic, widths, two partitions and existing Rayon pool.

The original small-query path creates parallel window parents, each waiting for an inner two-part collection. The candidate submits those same fills directly to one Rayon scope. Each part moves its bucket vector into its own slot; a completion counter lets the last part continue on that thread into the original merge and weighting functions, in original part order. The final Horner fold is unchanged. It removes the per-window partition join, not every nested join or the query's final completion barrier.

This is distinct from prior cross-query phase barriers, whole-query serialization and queue/idle-policy experiments. The [latest full-proof profiles](../2026-11-06-window-batch-proof-capture176/README.md) motivate examining coordination, but their epoch-symbol shares do not establish removable wall time or predict this scheduler's performance. No arithmetic or previously rejected bounds variant is reopened.

## Qualification and memory

All six native commands succeed once. Fresh OFF runs the four new focused tests with125 inherited tests filtered out; ON passes129 unfiltered core tests. The four new test families are completion/part ordering and last-thread ownership; panic/drop cleanup; failed-admission input preservation; and full-width G2 generator-coefficient oracles. They exercise1/4/16/32-worker pools with512KiB stacks. The arithmetic oracle uses accepted curve/field operations on independent generator coefficients; no new independent Arkworks field qualification or Gnark proof is claimed.

The native/source review passes. Of176 original source files,175 remain byte-identical. Removing the one parent's hook/module declaration recovers the remaining original. One child gives177 files total. The original hb_raw function body is exact and remains the fallback. Original field1200 and dependencies are reused without rebuilding or replaying field tests. Original source/library/tool identities and all six command streams are checked.

The new two-arrival control records and result slots are allocated before exclusive scalar words are changed. Checked overflow, admission or allocation failure returns to the original path. The resource-aware analysis reservation charges the requested vector capacities and stays live through result folding. It uses the existing dynamic policy rather than the legacy table-builder ceiling. The two existing bucket arrays move into the original computation; neither point array is cloned.

The native layout model is72bytes per pair record and208bytes per result slot, totaling3,080bytes for eleven windows. This is the control-vector payload, **not complete new workspace or minimum RAM**. Rayon scoped task heap allocations, allocator overhead and stacks are additional and have not been measured/accounted completely. They must be included in broader resource qualification before adoption. No new production pool, retained table, proving-key copy, solver plan, field representation or unsafe block is introduced.

## Next stage and goal

Freeze a complete-query timing/profiling/review protocol before measurement. It must include the original reference and fresh OFF/ON, primary and an admitted-grouped shape,1/16/32workers, observed dispatch/fallbacks and complete allocation/scheduling/drop costs. Explicit final links must use optimization,native CPU,fatLTO and one codegen unit. A query result alone cannot adopt this implementation:verified all-eight-plus-merge54 SDK comparisons,independent Gnark,portable and resource qualification remain required.

There are zero new SDK requests,Gnark exports,timings or profiles in this stage. The canonical ledger remains31,387. No AWS/Aeglos,phone,huge-circuit,pressure,swap or throughput result/resource is introduced. The Aeglos reference branch still points to0290613,which was already audited;no new peer implementation was discovered. The fixed-base[preprint](https://www.preprints.org/manuscript/202604.0045) uses shifted-base and chunk-pair tables and is not selected for the requested memory architecture.

The all-eight CPU10× goal remains active and unmet0/8. These are inherited new matched local timings for the exact accepted SDK6d83 from the completed prior capture,not this candidate's performance.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | CPU10× target ms |
|---|---:|---:|---:|---:|
|1×2|7.915095|37|24|3.7|
|2×2|11.169812|59|30|5.9|
|4×2|16.085882|109|48|10.9|
|8×2|24.517520|189|74|18.9|
|12×2|33.838626|284|101|28.4|
|16×2|40.083672|335|124|33.5|
|24×2|65.506198|528|176|52.8|
|merge24|65.200982|507|175|50.7|

Accepted merge54 is161.718956ms without a supplied M5 reference. Its prior whole-process RSS observation and the primary's112.5MiB are not new measurements or minimum-RAM requirements. Historical adopted primary7.928660ms/merge54161.883899ms remain labeled historical.

[Qualification](artifacts/qualification.json),[source audit](artifacts/source-audit.json),[prospective design](artifacts/design.md),[archive index](artifacts/archive-index.json). The archive contains the changed parent/child,helpers,plans,pins and exact native receipts with extraction hashes. Executable/library,key/image/witness and full proof-ledger payloads are excluded. Local root:`/home/ubuntu/src/zopro-perf-work/g2-last-finisher176`.
