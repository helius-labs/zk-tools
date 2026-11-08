# Tau9 G2 queue and tree census

**The fixed τ⁹ recipe is rejected before native implementation.** Its primary model has 1.98% fewer generic affine pairs than the earlier binary-ring model, but only 0.17% less vector-product volume, 5.70× as many inversion calls and 7.54× as many scalar-tail pairs. Its point-plane payload also grows. These counts do not provide a credible case for reversing the earlier binary-ring native regression. Production and accepted proof timings are unchanged.

## Fixed recipe and scope

This completes the queue/tree/source-operation screen permitted by the [preceding exact recoder and dense model](../2026-11-08-tau9-ring-work176/README.md). The geometry stays fixed: ρ=(1−ω)⁹=81+162ω, 18 positions, 3,281 representatives, three signed unit planes and two original point parts. There is no width, layout, flag or worker sweep and no native implementation or timing capture.

The new observer reuses the qualified arithmetic prefix and prior structural definitions byte-for-byte. It models the borrowed queue with batch200 and overflow199, signed indexed top tiles1024, grouped256 reductions, part/plane merges512 and weighted coefficient axes padded to128. Original and binary-ring counts are read from their earlier sealed output; their models and tests are not rerun. Raw2×2 remains diagnostic because production admits whole-scalar grouping there.

The population model assumes ordinary nonidentity, noncancelling sums. It does not execute native curve arithmetic or model equal-x cases, cancellations, allocator behavior, cache traffic or critical-path scheduling. Projective mixed additions, finish joins, normalization populations and weighted-tree postprocessing remain separately recorded. Recoding, normalization, projective arithmetic and ρ-Horner formulas are not priced. Their omission cannot turn the reported partial operation counts into total CPU costs.

## Results

These are structural counts, not proof or query timings. “Accepted” and “binary” are inherited counts expanded through the current source-operation formulas. The definitions differ from the preceding dense model.

|Primary1×2,30,089 rows|Accepted|Prior binary ring|τ⁹|
|---|---:|---:|---:|
|Generic affine pairs|632,442|572,574|561,256|
|Stream affine pairs|516,476|266,068|226,926|
|Indexed-top affine pairs|55,165|18,660|25,613|
|Other tree/merge affine pairs|60,801|287,846|308,717|
|Batch inversion calls|6,587|2,608|14,867|
|Scalar-tail pairs|9,986|3,598|27,128|
|Affine/projective unit maps counted separately|41,243|94,042|104,230|
|Stream projective mixed adds + finish joins|1,178|676|713|
|Stream normalization points|429|83|112|
|Vector-product proxy, millions|116.322150|104.454800|104.275700|

The primary problem is the thin final window. Indexed tiles reduce each signed bucket independently before combining tile partials. The τ⁹ top requires12,621 reduction/merge calls and23,365 scalar-tail pairs, versus250 calls and556 tails for the previous binary-ring top. Only2,248 of its25,613 top pairs occupy complete eight-lane groups. Independent tiny buckets therefore defeat the batching that the dense count assumed.

|Raw2×2,52,274 rows, diagnostic only|Accepted|Prior binary ring|τ⁹|
|---|---:|---:|---:|
|Generic affine pairs|1,082,397|955,437|930,456|
|Batch inversion calls|8,911|4,394|20,549|
|Scalar-tail pairs|13,237|3,845|43,944|
|Vector-product proxy, millions|198.460450|174.701450|171.032000|

For this diagnostic, τ⁹ has2.10% less vector-product volume than the prior binary model, but4.68× its inversion calls and11.43× its scalar-tail pairs. This does not qualify grouped2×2 runtime behavior.

## Source-operation accounting

V denotes an eight-lane Fq multiply, F an eight-lane fused two-product sum/difference, M/S a scalar Fq multiply/square, Fs a scalar fused two-product operation and I a scalar Fq inversion. For g complete groups and t tail pairs per batch:

- Borrowed-prefix streaming: `7g V +5g F +8t M +2t S +4t Fs`, plus the inverse bridge.
- Tree reductions with at least32 ordinary pairs use the same vector counts. Smaller nonprefix tree groups use `8g V +5g F`. Tree tails cost `7t M +5t Fs`.
- Each inverse bridge costs `5V +M +I`. The current `reduce_layer` source executes it even for a zero-ordinary-pair layer. Streaming skips empty batches. There are2 such tree layers for τ⁹ and31 for the prior binary model.
- Each affine unit map adds two scalar Fq multiplications, recorded separately. Axis-result and Horner maps remain separate.

|Primary source operations|Accepted|Prior binary ring|τ⁹|
|---|---:|---:|---:|
|V|579,669|511,133|542,012|
|F|389,035|355,610|333,830|
|M, excluding unit maps|76,637|27,918|204,897|
|S|296|248|268|
|Fs|49,782|17,866|135,506|
|I|6,587|2,608|14,867|
|Separate unit-map M|82,486|188,084|208,460|

The proxy is `100V +150F`, reflecting raw vector IFMA product volume in the current kernels. It excludes scalar expansion, inversion internals, packing, correction, shuffles, branches and memory operations. It is neither retired instruction count nor a cycle estimate. Scalar and vector operations are not assigned interchangeable costs. Despite fewer total affine pairs, the vector-product proxy is effectively unchanged against the already-slow binary method, with substantially more unpriced scalar inversion work.

The [earlier binary native trial](../2026-11-06-g2-joint-ring-native176/README.md) remains rejected: its primary query was60.24% slower than fresh control, with greater RSS and generic cache misses. That native result does not measure τ⁹. It is a material reference that a small partial-work gain cannot override. No end-to-end speed claim is made for this census.

## Working-memory accounting

|Retained payload term|Prior binary ring|τ⁹|
|---|---:|---:|
|Bucket slots per point part|8,193|9,843|
|All window/part point-array payload, bytes|39,850,752|45,356,544|
|Primary staged digit payload, bytes|1,143,382|1,083,204|
|All axis-result payload, bytes|629,888|596,736|

The point-array accounting covers all pending window parents, including suspended Rayon parents, independently of worker count. Its43.26MiB τ⁹ payload compares with38.00MiB for binary and5.50MiB for the original11×2×2,048-slot ordinary-window accounting. These are capacity terms for the proposed schedule, not a measured simultaneous allocation, mandatory algorithmic minimum, process RSS or minimum device RAM.

The output also records conservative indexed-top parent and streaming-leaf terms. The summed terms are explicitly incomplete: allocator rounding, vector headers, grouping/tree leaf maxima, runtime scratch retention and all key/CS/witness/prover memory are excluded. The τ⁹52,490-byte scalar map/representative figure additionally excludes a native axis-routing layout; the older binary metadata term includes its routing. Those metadata subtotals are not a matched complete-memory comparison. No complete admission bound or low-RAM guarantee is claimed, and no native allocation is attempted. Larger point planes remain a disadvantage independent of these missing terms. No point lookup table, key copy, retained solver plan, production pool or fixed synthetic RAM cap is added.

## Validation and decision

The first census compile failed because two sum assertions needed explicit u64 result types. The initial source, pin set and failed compiler receipt are preserved. The only correction adds those two type annotations. A new workflow directory compiles the corrected observer and runs the new census once; both commands succeed. No successful parent model, test population or census is replayed.

The separate reader initially counted all untyped-looking `sum()` calls, including a third call already typed as usize. That reader failure and its source are retained. A separate corrected reader verifies the two exact source substitutions and passes without rerunning the observer. It checks216 current and216 initial pins, both successful command receipts and the failed compile receipt, exact reused definitions, all36 window rows against the parent dense output, histogram/arrival conservation and six source-operation aggregates. It does not independently replay the complete point/token population or prove the source formulas' native cycle cost. [Complete counts](artifacts/analysis.json), [reader result](artifacts/review.json), [observer source](artifacts/census.rs).

**Close this fixed recipe; do not proceed to a native candidate.** Its necessary dense-count gate remains passed as recorded, but was insufficient. The fuller census exposes inversion and tail growth while the retained point planes grow. This is a design rejection, not a failed native speed measurement or a newly invented performance threshold. A different reducer or layout would require a distinct hypothesis and prior-history review, not promotion or selective rescue of this result.

The next investigation is the accepted indexed-top reducer itself: determine from existing profiles and history whether sharing inversion batches across independent small buckets can remove measured work without increasing retained point storage. This is an audit target, not a selected implementation or claimed breakthrough. The rejected τ⁹ geometry stays closed.

## Accepted CPU and Metal comparisons, inherited

Production176, accepted SDK6d83 and the35,171-proof inventory remain unchanged. There are zero new proof/query timings, SDK requests, profiles or proofs. The same accepted binary measured7.915/7.929/7.996ms locally for1×2 in earlier runs, with latest8.003ms. The cause of inter-run variation is unconfirmed; no adopted regression is established.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.002770|37 /4.623×|24 /2.999×|3.7|
|2×2|11.241463|59 /5.248×|30 /2.669×|5.9|
|4×2|15.933816|109 /6.841×|48 /3.012×|10.9|
|8×2|24.234311|189 /7.799×|74 /3.054×|18.9|
|12×2|33.855565|284 /8.389×|101 /2.983×|28.4|
|16×2|40.071563|335 /8.360×|124 /3.094×|33.5|
|24×2|65.917462|528 /8.010×|176 /2.670×|52.8|
|merge-24×1|65.305023|507 /7.764×|175 /2.680×|50.7|

Source: [preceding comparison](../2026-11-08-tau9-ring-work176/README.md). Seven nonprimary rows remain from the earlier full campaign. Merge54 remains162.240109ms with no supplied M5 reference. The verified all-eight CPU10× goal remains active and unmet, **0/8**. No new AWS/Aeglos, huge-circuit, mobile/ARM/TEE, memory-pressure/swap, minimum-RAM or concurrent-throughput qualification is added.

[Design](artifacts/design.md), [corrected plan](artifacts/plan-fixed.json), [pins](artifacts/pins-fixed.json), [exact source/receipt archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json) and [checksums](SHA256SUMS) preserve the completed experiment, including both tooling failures. All UTF-8 member hashes and the gzip roundtrip are checked. Private points/scalars/keys/witnesses/proofs, compiled libraries/binaries, unchanged full production source and full papers are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/tau9-ring-census176`.
