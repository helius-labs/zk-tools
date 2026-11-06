# Worker attribution in saved G2 query profiles

All epoch-pin samples in the four saved primary32 profiles belong to registered Rayon workers. The external caller has none. Every worker has borrowed affine-apply samples. This rules out an external-caller explanation for these epoch samples; it does not identify an individual proof's critical path or justify another scheduler change.

This is a read-only aggregation of the [queue/index composition](../2026-11-06-g2-queue-index-composition176/README.md), commit `3de327d01b863d2a651396c114140fc08c79534f`. No new query, recording, native curve test, proof or AWS result is produced. That experiment's failed screen and all earlier scheduler decisions remain unchanged.

|Configuration|Total samples|Caller samples|Worker epoch-pin samples|Worker other-epoch samples|Worker borrowed-apply samples|
|---|---:|---:|---:|---:|---:|
|ORIGINAL|20,502|19|1,989|284|8,313|
|DRAIN|20,393|19|1,963|271|8,300|
|PACK|18,493|20|2,101|278|8,094|
|BOTH|18,419|18|2,048|277|8,151|

Caller epoch-pin and borrowed-apply counts are zero in every configuration. The caller accounts for less than 0.11% of each profile's samples. All 8,101 epoch-pin samples belong to workers. The larger total of 9,211 samples includes other epoch symbols; the two categories are kept separate.

|Configuration|Workers with borrowed-apply samples|Minimum / maximum per worker|Mean|Coefficient of variation|
|---|---:|---:|---:|---:|
|ORIGINAL|32|231 /403|259.781|0.111432|
|DRAIN|32|206 /389|259.375|0.109277|
|PACK|32|218 /361|252.938|0.097630|
|BOTH|32|219 /377|254.719|0.102920|

These counts aggregate 1,024 queries per profile. They neither prove balanced individual proofs nor measure exclusive idle/stall time, task readiness, utilization or removable wall time. The modest aggregate dispersion is not a bound on individual stragglers. Flat symbols include inlined work, and the parent profile's absolute-period-coverage limitation remains. Packing's reduction/inverse savings do not remove most ordinary affine-apply work; the data do not isolate a causal scheduler or memory bottleneck.

## Validation and decision

Both workflow commands succeed on their first executions. Nineteen pinned inputs bind the analyzer/design, parent summaries and caller identities, four exact raw profiles and their decoded streams/exit hashes. Every sample reconciles with the parent's qualified count and flat symbol bins. PID, worker TID, recorded CPU and fixed period are checked; zero-sample registered workers remain represented.

A separate Rust reviewer uses raw mangled symbol tokens without importing the analyzer or demangler. It independently checks all per-thread totals, epoch-pin/borrowed classifications, CPU ownership and distribution statistics, plus all command argv/environment/exit/stream hashes. It reparses all 77,807 saved samples and passes on its first execution. It performs no new recording or query. Full source, raw/decode data and original profile qualification remain in the linked parent report; [pins](artifacts/pins.json) bind their exact local identities.

The audit closes the caller-versus-worker question. It selects no scheduler implementation. Empty-victim probing, collection cadence, bounded scanner admission and fused TLS pinning already have distinct historical trials. Reading the actual [C7a bounded-scanner report](../2026-10-26-bounded-steal-search-c7a/README.md) also resolves the older local report's stale pending-target note: the historical target screen passed at 64/96 workers, without production adoption; its local failure remains. No new target result is inferred.

## Arithmetic reading and next question

The relevant sections of [Costello–Smith's Montgomery survey](https://arxiv.org/pdf/1703.01863) require a difference argument for xADD and describe an exponential signed-combination input set for the cited full-dimensional Brown construction. That is unsuitable as a direct large-MSM replacement here. [Chung–Costello–Smith](https://arxiv.org/pdf/1510.03174), section5.3/Table4, supports short-Weierstrass scalar multiplication and full-point recovery, but does not supply the missing differences for our arbitrary bucket additions. This rejects a direct substitution, not every possible x-only approach. [Reading scope and history](artifacts/reading.md) record the sections read and the limits of that assessment; no paper speed claim transfers to this prover.

The next separate question is two interleaved borrowed-G2 prefix chains in the existing streaming forward/finishing passes. Unlike the earlier four-chain phased trial, the proposed fixed version would not materialize every inverse norm or add scratch passes. It would add three root-combination vector products and fixed local vector state, keeping two-vector records. This is an incremental dependency experiment with possible instruction/register costs, not a new algorithm or a promised large gain. No native implementation or performance/resource qualification follows from this report alone.

Accepted source176/SDK `6d83…` and full-proof local 1x2 **7.928660 ms** remain unchanged. The corresponding speedups are 4.667× versus the supplied M5 CPU reference and 3.027× versus Metal. The [all-eight CPU/Metal comparison](../2026-11-06-g2-queue-index-composition176/README.md#accepted-comparisons-remain-unchanged) remains inherited; the all-eight CPU10× goal is active/unmet, **0/8**. Merge54 remains 161.883899 ms without a supplied M5 reference. The canonical proof ledger remains 30,846. No new memory-peak/minimum-RAM, pressure, mobile, huge-circuit, concurrency or Aeglos comparison is claimed.
