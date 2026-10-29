# Prior mechanisms retained as closed

2026-10-29 serial-phi175: entire decomposition sequential, uncertainty includes no gain and aggregate gate false. New candidate differs by releasing each partition's windows independently before the other decomposition joins.
2026-10-27 g2-scalar-grain: with_min_len128 retained global decomposition barrier; full gate failed. No new grain sweep.
2026-10-25 proof-dataflow-screen: inspected archived proof.rs source; both schedule_g2 callers globally par_iter_mut/with_min_len128 decompose all scalars BEFORE schedule_g2. That continuation experiment regressed. New candidate changes readiness across this preparation boundary; no claim all continuations are novel.
2026-10-23 G2fourparts/bucketbands and2026-10-28 whole-querywaves, extra worker pools, prefix pools, batched steal and empty epoch trials remain closed. No repeat of partition count, bucket bands, pool sizes, queues, fences or runtime tuning.
2026-10-29 current C7a PGO experiment rejected, production unchanged, no profile retraining. Profiles support investigating readiness/coordination but do not isolate memory, clock, scheduler or critical-path wall fractions. Preparation observer records foreign-query work during some join tails; that is not proof all such time is removable. This test measures full SDK outcomes directly.
