# Balanced G2 windows: exact count screen

30 October 2026. Necessary opportunity check passes; no latency or proving-speed gain is measured. Production remains unchanged.

## Geometry and scope

The current Phi2 G2 path uses eleven 12-bit spans: ten ordinary 2,048-bucket windows and a narrow indexed top. The candidate uses eleven spans covering 127 bits, with widths `[11,12,11,12,11,12,11,12,11,12,12]` and starts `[0,11,23,34,46,57,69,80,92,103,115]`. Each uses the original borrowed ordinary Window, high component then phi then low. The original early-stopping LIFO queue is retained; the failed queue-scan candidate is absent. Existing batch sizes are 152/200. There is no parameter sweep.

Six 2,048-bucket and five 1,024-bucket outputs total **17,408 slots per part versus 20,544**, a 15.26% reduction. All eleven windows use ordinary streaming, removing the highest window's indexed input routing, point tiles and reduction layers. More affine updates, smaller batches, collisions and changed task dependencies can offset that saving. Slot counts are not arithmetic counts or latency.

The accepted offset Booth reader from balanced K/H is source-extracted exactly. It reads variable spans directly from the same two signed Phi components; no digit array, key lookup table, point copy, duplicate plan, pool or per-key state is added. The scalar split, original point operands and field formulas remain unchanged. This applies the geometry to a different G2 caller; the accepted K/H gain is not evidence of a G2 gain. Older uniform-width/GLS4/routed-round and queue experiments keep their outcomes. [History and research limits](artifacts/history-audit.md).

## Exact checks and work counts

Both immutable local fixtures and their successful independent curve oracles are reused from the preceding complete-query study, without new witness solves or oracle execution. Primary grouping declines; 2×2 admits 28,852 scalar classes from 52,274 rows. Original grouping policy and stable first-seen classes are preserved. A separate review checks every original class value and verifies both token dot products before and after grouping.

The model uses two deterministic full-width Fr-token populations. Every signed Phi/Booth reconstruction, ordinary bucket, direct sign-bin sum and final weighted whole sum agrees. Formal occupied buckets remain occupied after token cancellation: this is an ordinary noncancelling scheduling model, not actual curve-exception handling. Actual identities, doubling, cancellation, field costs, normalization, allocation, hardware and proof scheduling require native/caller qualification. Raw key points/scalars and unblinded results remain local.

Baseline ordinary counts exclude its indexed top. Candidate ordinary counts cover all eleven windows. The baseline indexed top is listed separately; its actual arithmetic and inversion costs are not modeled, so these counts **do not establish fewer total inversions**.

|Shape|Policy and scope|Arrivals|First writes|Affine updates|Batch inversions|Projective arrivals|Finish merges|Queue pushes|
|---|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|original: ten ordinary|558614|40960|516476|2605|1178|429|30907|
|1x2|balanced: eleven ordinary|614329|34282|578320|3336|1727|879|42870|
|2x2|original: ten ordinary|576560|40960|535224|2688|376|360|29861|
|2x2|balanced: eleven ordinary|634122|34282|599223|3450|617|588|41690|

|Baseline indexed top|Arrivals across both parts|Within-sign binary reduction pairs|Nonempty sign bins|
|---|---:|---:|---:|
|1x2|55277|54829|448|
|2x2|57135|56687|448|

For primary, the new ordinary route performs 578,320 affine updates versus 516,476 in the old ordinary route plus 54,829 within-sign top tree pairs. Those pair counts omit sign/component/part merges and exceptions; their sum is not a complete baseline operation total. Candidate projective arrivals also increase. This preserves the adverse work changes rather than inferring speed from capacity alone.

Two new tests pass on their first executions: signed variable-offset integer reconstruction across 8,192 full-width magnitude samples plus boundaries and signs, and weighted bucket-model sums across 40 empty/odd/full/tail/skew cases under both geometries. Original unchanged model tests and core/field/prover suites are inherited; prover source is not changed in this screen. Every original uniform digit also matches the source-extracted reader on both real populations. The four-step corrected workflow completes test compilation, two tests, probe compilation and real-input counts.

Two test-compilation attempts first caught unsupported Fr reference operators and missing Iterator::Sum support, before any test executed. Explicit value dereferences and a zero-initialized fold fix the tool code; initial sources, plans, pins and compiler receipts are retained. No successful test or count was repeated. A separate post-count grouping/source review adds no new solve or proof.

## Memory and next decision

The existing G2 forecast already charges eleven parents with half=2,048, two point partitions and maximum batch200, plus extra indexed-top workspace and a conservative scalar-copy allowance. Every candidate parent and nonforking leaf fits that envelope, with no indexed-top route or additional scalar buffer. Eleven `(start,width)` pairs request 176 stack bytes; compiler frames and actual peak RSS remain unmeasured. This is a capacity argument, not minimum-RAM or memory-pressure qualification.

The necessary opportunity check passes: exact algebra and schedule checks, bounded existing queue/batch maxima, smaller bucket capacity and an unchanged conservative heap envelope select native implementation. **No runtime candidate, latency result, peak prover RSS or adoption is established here.** Next: actual-curve tests for full sums and exceptional points, whole-query latency/profiles against frozen/OFF controls, then full all-nine plus primary16 SDK/independent-proof/resource/portable/final-artifact qualification if warranted. No minimum gain floor, favorable subset or unchanged reroll is selected.

## Accepted proof comparisons

These inherited local measurements and supplied historical M5 references are not count-screen timings. All eight CPU10× targets remain unmet.

|Shape|Accepted local ms|RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|134.00|37.0|4.612×|24.0|2.992×|3.7|
|2x2|11.335969|275.00|59.0|5.205×|30.0|2.646×|5.9|
|4x2|15.945544|315.00|109.0|6.836×|48.0|3.010×|10.9|
|8x2|24.090485|512.50|189.0|7.845×|74.0|3.072×|18.9|
|12x2|33.844855|707.25|284.0|8.391×|101.0|2.984×|28.4|
|16x2|39.931557|826.25|335.0|8.389×|124.0|3.105×|33.5|
|24x2|66.233309|1328.75|528.0|7.972×|176.0|2.657×|52.8|
|merge-24x1|65.273224|1175.50|507.0|7.767×|175.0|2.681×|50.7|
|merge-54x1|162.250464|1831.87|—|—|—|—|—|

Latest AWS primary remains Zopro11.915591ms at32workers versus pinned Aeglos Sorted7.959184ms at96; no new AWS run. The accepted physical16 calibration exception is retained. No mobile/ARM/TEE, huge-circuit, swap/pressure, minimum RAM or concurrent-throughput result follows. Proof ledger23,971, with zero new solves, SDK requests or Gnark exports. [Current benchmark snapshot](../2026-10-30-benchmark-status-report/README.md), [prior query comparison](../2026-10-30-g2-queue-scan175/README.md).

[All counts](artifacts/analysis.json), [design](artifacts/design.md), [geometry](artifacts/geometry.rs), [accepted digit reader](artifacts/digit-at.rs), [model](artifacts/model.rs), [count probe](artifacts/probe.rs), [independent grouping review](artifacts/review.rs), [workflow](artifacts/plan-final.json), [pins](artifacts/pins-final.json), [sources](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [records](artifacts/records.json.gz), [archive review](artifacts/archive-review.json), [unchanged accepted rows](accepted-rows.json).
