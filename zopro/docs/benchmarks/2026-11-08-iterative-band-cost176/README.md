# Four-pass bands: SIMD, inversion and queue work

The fuller model preserves the earlier **5.34–5.62% reduction in point additions**, but exposes a substantial primary queue tradeoff: queued projective points rise **1,178→16,424**, and normalization points **429→8,598**. The primary vector-IFMA proxy falls **8.03%** and base-field inversion calls fall **30.77%**, while elementary products rise **0.78%**. These are source-shaped counts; **no CPU latency result or production change is established**.

The new declared work screen requires fewer vector-IFMA proxy operations and elementary products, with no increase in scalar inversions, on both populations. Primary fails the elementary-product condition; grouped 2×2 passes all three. No favorable-only native trial is selected. This preserves the failed selection rather than treating either a product-count increase or an inversion decrease as proof of speed. The prior count-dominance screen remains separately failed and unchanged.

## Algorithm and scope

This follows the [four-band count study](../2026-11-08-iterative-bands176/README.md) without changing its width 14, four sequential bands, 2,048 physical buckets per ordinary pass, two source partitions or signed Phi2 components. The source comparison uses the accepted width 12 route. Primary has 30,089 scalar terms; 2×2 has 28,852 retained grouped terms from 52,274 raw rows. Grouping and point-nonidentity evidence are inherited; runtime preparation/admission and actual grouped-point construction are not repeated.

New accounting follows the original batch 200/queue 199 policy, high/low phase flushes, signed indexed top with 1,024-point tiles, grouped 256-point trees, 512-point merges, weighted-tree layers, scalar tails, projective identity fast paths and normalization. Nonidentity, noncancelling ordinary bucket sums are assumed. Actual equal-X, doubling/opposite trajectories remain unmodeled.

The proposed runtime would obtain each band's ordinary total by returning the existing `running` value from `weighted_affine` alongside its weighted result. No additional reduction or input-sized table is assumed free. Sequential bands would retain four pairs of projective results, not four simultaneous Window arrays. No native implementation, conservative concurrent-memory admission or capacity/RSS measurement exists yet. The analysis model's histograms and digit vectors are tooling storage.

## New work counts

V is an eight-lane ordinary Fq multiplication; F is a fused two-product vector operation. M, S and Fs are scalar Fq multiplication, square and fused-two-product operations. The vector proxy is `100V+150F`; elementary products are `8V+16F+M+S+2Fs`. Neither expression converts operations into equivalent CPU cycles. Inversions are counted separately, with no constant cost assigned to differing operands.

|Quantity|1×2 original|1×2 four-pass|Grouped 2×2 original|Grouped 2×2 four-pass|
|---|---:|---:|---:|---:|
|All noncancelling point additions|634,486|600,576|654,290|617,488|
|Active projective doublings|846|2,826|846|2,826|
|Vector-IFMA proxy|116,322,150|106,980,500|120,232,150|112,742,150|
|Elementary products|11,223,641|11,311,097|11,548,580|11,171,441|
|Scalar base-field inversions|6,607|4,574|6,746|4,732|
|Scalar Fq M|185,133|483,889|174,444|356,061|
|Scalar Fq S|336|2,198|344|2,170|
|Scalar Fq Fs|88,130|412,065|73,020|136,509|

Vector proxies fall 8.03%/6.23%; inversion calls fall 30.77%/29.85%. Elementary products change +0.7792%/−3.2657%. The positive work reductions are retained; scalar work and wider-band overhead also remain charged.

|Queue and geometry quantity|1×2 original|1×2 four-pass|Grouped 2×2 original|Grouped 2×2 four-pass|
|---|---:|---:|---:|---:|
|Ordinary stream arrivals|558,614|502,909|576,560|519,007|
|Free first bucket writes|40,960|141,485|40,960|142,721|
|Queued points sent to projective storage|1,178|16,424|376|2,374|
|Projective mixed updates after first assignment|749|7,826|16|186|
|Normalization points / finish joins|429|8,598|360|2,188|
|Batch-normalization calls|20|72|20|72|
|Affine + projective high-component maps|41,243|123,426|41,239|123,415|
|Streaming inversion batches|2,605|1,789|2,688|1,938|
|Tree/indexed inversion layers|3,935|2,562|3,991|2,571|
|Streaming scalar-tail pairs|148|528|152|504|
|Tree scalar-tail pairs|9,838|3,017|9,224|2,641|

The same 2,048 resident buckets do not imply the same arrival distribution or queue behavior. Filtering width-14 digits into consecutive bands changes the ordered stream. Primary overflow grows about 13.94× and normalization population about 20.04×; grouped 2×2 grows about 6.31×/6.08×. The original phase-boundary transforms also approach three times their original population. These are modeled structural effects, not observed native allocations, cache misses or a causal latency diagnosis.

The earlier model's abstract one-pass band-intake visits remain 37/11 times the original. The actual indexed-top builder scans its component input once for counts and once for order. Charging that second pass changes the relevant source input-loop geometry to 24n→76n: **722,136→2,286,764** visits for primary and **692,448→2,192,752** for grouped 2×2, a 3.1667× ratio. This still excludes decomposition, grouped preparation, tree input fetches, bucket copies and other memory operations; it is not byte traffic or bandwidth.

## Arithmetic accounting

Each modeled inverse bridge charges 5V+M+I, including a zero-pair tree layer if one occurs. None of these four real-data tree histograms has a zero-pair layer. The G2 prefix stream/tree charges 7V+5F per full eight-pair group; nonprefix tree layers below 32 pairs charge 8V+5F. Stream scalar tails cost 8M+2S+4Fs per pair; tree tails cost 7M+5Fs. Enabled original prefix arithmetic remains the merge path.

Source projective formulas are charged separately: mixed addition 7 Fq2 multiplications +4 squares, projective addition 11+5, doubling 2+5. A native scalar Fq2 multiplication lowers to two Fs; Fq2 square to two M. Per-Window batch normalization charges its 6n Fq2 multiplications +n squares, plus the norm/inverse/scaling boundary even for an empty population. Each live affine or projective high-component X map charges two M. Source counts are not a newly decoded instruction trace; compiler simplification, operand-specific inversion cost, linear operations and packing remain unpriced.

The larger weighted and merge populations offset much of the streaming saving. On primary, the stream's elementary products fall 8,887,045→5,938,877, while tree products rise 1,794,613→2,974,899, merge products 356,751→1,161,381 and projective products 91,512→765,252. Phi and normalization charges are included in the total separately. Partial counters alone would conceal this redistribution.

## Verification and disposition

All six new serialized Rust/xtask commands pass on their first executions: test build/run, count-model build/run and independent-reader build/run. Mathematical tools use pinned Rust 1.98.1, O3 and overflow checks. The 197 frozen pins include all 176 unchanged production sources, qualified split/grouping/Booth routines, the prior results, real inputs, compiler dependencies and canonical proof ledger.

The two new tests cover 128 queue/phase/band cases against a different state-map/VecDeque implementation, and all 513 batch sizes from 0 through 512 for tree/stream group, tail and bridge accounting. An empty normalization boundary is explicitly charged. Old native, decomposition, encoding, grouping and prover tests are inherited rather than rerun.

The offline reader independently recodes 235,764 component identities across both widths. It reconstructs the actual-input queue histograms with the tested state-map algorithm, indexed-layer histograms using ceil-division, and weighted layers using sets of occupied binary blocks rather than the producer's reducer. It also reconstructs transforms, normalization, projective/field totals, both selection decisions and conservation against the saved prior addition totals. Phi2 itself is shared qualified code; there is no new independent curve or machine-arithmetic oracle.

The reader checks its five preceding command/exit/stream receipts; publication checks all six and terminal workflow state, source pins and archive payload hashes. No old producer, successful test, capture, decoder or cryptographic verifier is replayed. No profile is claimed for a mathematical count experiment. There are zero new SDK requests/proofs; canonical verified-proof inventory remains 35,314.

This closes the fixed source-work screen without a native trial under its declared conditions. It does not prove that primary will be slower: fewer vector products/inversions could outweigh additional scalar work, or repeated scans could erase the saving. Establishing that requires independently justified native cost/caller evidence. Do not promote the favorable grouped population, rerun the same counts or relabel either failed screen as passed. A further band-tree, routing or queue proposal must first account for the prior [borrowed-tree SDK regression](../2026-11-07-borrowed-bucket-tree-pilot176/README.md), which remains closed.

## Accepted proof comparisons, inherited

All proof measurements below are inherited local 32-worker results. Supplied M5 references use different hardware. Latest primary 8.010122 ms and earlier 7.915095/7.928660 ms use the same accepted executable; no slower code version was adopted.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.010122|37 /4.619×|24 /2.996×|3.7|
|2×2|11.241463|59 /5.248×|30 /2.669×|5.9|
|4×2|15.933816|109 /6.841×|48 /3.012×|10.9|
|8×2|24.234311|189 /7.799×|74 /3.054×|18.9|
|12×2|33.855565|284 /8.389×|101 /2.983×|28.4|
|16×2|40.071563|335 /8.360×|124 /3.094×|33.5|
|24×2|65.917462|528 /8.010×|176 /2.670×|52.8|
|Merge24×1|65.305023|507 /7.764×|175 /2.680×|50.7|

Merge54 remains 162.240109 ms without an M5 reference. Original all-eight verified CPU 10× goal remains active and unmet, 0/8. No new native timing/profile, AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, RSS, minimum-RAM or concurrent-proof result is added.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [derived comparisons](artifacts/derived-comparison.json), [review](artifacts/review.json), [publication](artifacts/publication.json), [evidence archive](artifacts/evidence.json.gz), [checksums](SHA256SUMS). The UTF8 archive preserves source, histograms, plans/pins and command receipts. Raw scalar/key/witness/proof/public data, full ledgers, binaries/libraries and third-party full text are excluded.
