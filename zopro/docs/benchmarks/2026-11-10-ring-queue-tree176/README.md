# Bounded queue trees in ring unit phases

The new count model reduces queue dependency depth and inverse calls, but **fails its declared componentwise work screen and is closed before native timing**. Covered fused-product events rise slightly. This is not a measured slowdown; all positive operation and depth reductions remain reported. Accepted production is unchanged.

The prior one-array ring completes colliding operands serially, reaching 199 rounds. This proposal executes pending work, reduces queued points for each destination through balanced trees, then attaches one root per bucket. At most 199 queued operands require at most eight tree levels plus one attachment stage. Ordinary full-batch processing retains its original LIFO take. The radix 128 recoder, nineteen positions, two partitions, 2,731 ordinary buckets, unit-phase endomorphism schedule, indexed leading window, axes and weighting remain unchanged. This is a different addition topology from guarded draining, mandatory one-by-one completion and full-window indexing; their earlier results are not repeated or relabeled.

The modeled generic affine addition totals stay 573,250/956,434. There are zero generic projective spills or normalizations. The stream processes 143/94 queue-tree completions containing 11,837/2,522 operands across primary/raw2x2. These are generic occupancy counts, not runtime or real-curve exception observations. Raw2x2 is diagnostic because production admits scalar grouping there.

|Covered event|Primary serial completion|Primary queue trees|Raw2x2 serial completion|Raw2x2 queue trees|
|---|---:|---:|---:|---:|
|Ordinary Fq products|4530736|4497710|7340483|7288031|
|Fused two-product events|2863707|2865166|4779972|4781476|
|Scalar Fq squares|5158|2240|4468|1460|
|Scalar inverse calls|4106|3323|6442|5190|

Total covered inverse calls fall 19.07%/19.43%, ordinary products 0.73%/0.71%, and scalar squares 56.57%/67.32%. Fused-product events rise 1,459/1,504, or 0.05095%/0.03146%. The fourteen frozen checks include no increase in each of these four categories versus serial completion; the two fused-product checks fail. All twelve other checks pass, including the vector proxy and inverse limits versus accepted. A conservative componentwise screen does not price the exchange between multiplication and inversion. Its failure therefore does not establish inferior CPU performance or rule out other queue-tree algorithms. No gate is changed after observation, no percentage gain floor applies, and no successful scan/test is repeated.

The covered source ledger excludes weighted projective postprocessing, linear operations, grouping/routing, packing, memory traffic and scheduling. Vector/scalar events have different throughput. Old top/weighted counts and source formulas are inherited; the new stream and queue trees are independently replayed. Scalar inverse call counts also include unchanged empty normalization boundaries; they are not equivalent fixed-duration work units.

Two new tests pass: 180 signed/zero/cancelling token states across boundary sizes and bucket counts, plus 10,000 hot-bucket inputs preserving 9,999 additions. They check conservation, termination, distinct pending destinations, queue/batch emptiness and 199-input/eight-level bounds. Five old tests are filtered. A separate same-agent reader uses dense u16 multiplicity halving and VecDeque instead of the producer's BTreeMap and explicit token pair trees, and native MAP/direct-division decoding instead of its recoder. The previously qualified Phi split is shared. It reconstructs every stream total, tree histogram, work row and all 14 checks; leading/weighted geometry remains exact. These formal token/model checks do not qualify native curve arithmetic, fallback, concurrency or proof correctness.

All six serialized Rust 1.98.1/xtask workflow commands succeed on first execution. Review checks 221 model pins; the reader workflow adds four frozen pins. Terminal publication validates all six exact commands/exits/stream hashes, 225 final input pins, 176 unchanged accepted production hashes and the accepted SDK identity. Initial setup encounters a stale historical production pin; the repaired preparer uses the current adoption manifest while keeping the already generated model exact. An offline setup helper also needs a borrow-check repair before any reader execution. Failed sources are retained; those helper commands have transcript evidence rather than complete xtask receipts.

No native implementation or allocation forecast is established. The bounded queue still needs grouping/routing, coordinate copies, pair records and attachment storage: 199 G2 affine points alone occupy 25,472 bytes per active tree. This is additional proposed working storage, not free reuse, measured RSS or minimum RAM. The model introduces no production table, expanded key, duplicate solver plan or new pool. No full-query/proof timing, SDK/Gnark export, PMU/profile, phone/ARM/TEE, huge-circuit, concurrent-throughput, pressure/swap, AWS or Aeglos result is added. The existing ring query regression remains separate evidence. Canonical proof inventory remains 44,265.

The following accepted local 32-worker complete-proof observations and supplied historical M5 references are inherited from the [latest full SDK campaign](../2026-11-10-frontend-overlap176/README.md); these are not timings of queue trees.

|Shape|Accepted CPU ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.783594|37 /4.754x|24 /3.083x|3.7|
|2x2|10.932731|59 /5.397x|30 /2.744x|5.9|
|4x2|15.757088|109 /6.918x|48 /3.046x|10.9|
|8x2|23.973295|189 /7.884x|74 /3.087x|18.9|
|12x2|33.827269|284 /8.396x|101 /2.986x|28.4|
|16x2|39.900917|335 /8.396x|124 /3.108x|33.5|
|24x2|66.041083|528 /7.995x|176 /2.665x|52.8|
|merge-24x1|65.717994|507 /7.715x|175 /2.663x|50.7|

The full verified CPU10x objective remains active and unmet, **0/8**. Primary16 is 9.016864 ms; merge54 is 162.170158 ms without a supplied M5 reference. Primary32 peak 111.25 MiB belongs to the preceding short 31-request campaign; older 133.50 MiB observations remain valid and establish no memory savings here.

[Analysis](artifacts/analysis.json), [independent replay](artifacts/review.json), [decision](artifacts/decision.json), [protocol](artifacts/design.md), [model](artifacts/model.rs), [reader](artifacts/review.rs), [six command receipts](artifacts/receipts.json), [preserved failures](artifacts/failures.md), [continuation and autoresearch loop](artifacts/next.md), [source/receipt archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Keys, points, scalar/witness/proof payloads, full ledgers, unchanged source snapshots and executable/library blobs are excluded. Reproduction needs the pinned private inputs.
