# Small-query core worker entry adoption

The native CPU prover now enters its existing Rayon pool once around solving when the canonical small fused-query path is eligible. Larger paths retain their entry behavior. No new thread pool, key field, query buffer, lookup table or solver plan is introduced. The change remains behind zopro_small_core_entry in the qualified native build; runtime ZOPRO_CPU_SMALL_CORE_ENTRY=0 disables it.

The final standard-CLI native release-profile comparison measures8.689205ms for1x2 versus9.013227ms accepted and8.986798ms identical OFF. Paired gains are3.46% and2.95%, with7/7 winning blocks against each. Pinned16 is10.189666ms versus10.493611ms accepted, also7/7 wins. Full-nine geometric ratios are0.991568 and0.992475. All ten cases preserve the inherited2% paired nonregression band;16x2 and24x2 have slightly less favorable observations, retained in the full report.

Primary observed peak RSS is112.25MiB versus121MiB accepted. A sustained192MiB hard-limit/no-swap test completes54 requests per variant with32MiB controller ballast; candidate cgroup peak is117.328MiB including ballast. Both compiler-target qualifications complete two proofs each on actual250 and4999 circuits under95GiB, with zero swap/OOM. Native4999 peak process RSS is94.693184GiB; this establishes feasibility at that cap, not minimum RAM.

Fresh native33field104core and baseline portable11field85core tests, each with two raw and six cubic tests, pass. The complete benchmark and qualification campaign includes15,104 SDK requests plus eight large proofs across controls/candidates,793 distinct independently verified exports and1,586 proof/public members rehashed before promotion. Native and server-compatible builds, ordinary callers, finite pressure transitions, retained allocator behavior, original CLI/default flag and pinned-worker receipts are covered. Source and library hashes are retained.

The user-supplied M5 CPU/Metal comparisons for all original eight shapes are in the full results. The original10× CPU objective remains unmet. No new AWS or phone-performance result is claimed. No reports or code are pushed by this adoption.

[Full performance results](../2026-10-25-small-core-entry-production/README.md), [native resource qualification](../2026-10-25-small-core-entry-native-qualification/README.md), [source change](source.patch), [exact flags and source pins](production-source-manifest.json).
