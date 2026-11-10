# Sorted G2 component gaps: work screen

**The fixed Brent–Kung scan recipe is closed before native implementation.** The transform passes actual curve-result checks and shortens both components from 126 to at most 115 bits, reducing eleven width-12 windows to ten. Once suffix-point scans are charged, modeled point additions increase **2.61% for 1×2** and **7.79% for 2×2**, before sorting and other native overhead. Both required point-work checks fail. No speedup, slowdown, adoption or full-proof timing is measured.

After the original scalar-grouping decision and exact signed Phi split, each component sorts nonzero, nonidentity terms by magnitude, with original-index ties. For sorted magnitudes mᵢ and signed points Qᵢ, use dⱼ=mⱼ−mⱼ₋₁, m₋₁=0, and Sⱼ=Σᵢ≥ⱼ Qᵢ. Then ΣmᵢQᵢ=ΣdⱼSⱼ. Zero gaps and identity suffix outputs are omitted. The high-component endomorphism remains linear. This is a global sorted-gap/suffix transform, distinct from prior input-order difference bases and disjoint-neighbor Bos–Coster updates; no earlier negative result is reopened.

The fixed proposed native scan is an in-place Brent–Kung inclusive scan of reversed signed points, with power-of-two upsweep and downsweep corrections. Width remains 12 and parts remain two; window count follows the actual maximum gap bitlength plus Booth carry. Planned per-level batched affine updates would use existing pool/scratch. That native implementation does not exist. Arkworks projective scan storage here is an offline oracle, not the proposed affine layout or a memory measurement.

|Work screen|1×2 original|1×2 gaps + scan|2×2 original|2×2 gaps + scan|
|---|---:|---:|---:|---:|
|Windows|11|10|11|10|
|Nonzero/nonidentity digit arrivals|613891|519691|633695|572960|
|Suffix-scan nonidentity joins|0|112758|0|115330|
|Common generic grouping joins|0|0|20659|20659|
|Complete modeled point-add calls|634486|651020|674949|727518|
|Weighted/Horner active doublings|846|763|846|763|

Primary removes 94,200 digit arrivals but needs 112,758 scan joins; 2×2 removes 60,735 arrivals and needs 115,330 joins. The ledger also includes generic fill/part merges, occupied-bucket weighted reduction, Horner and unchanged grouping work. Generic fill/merge/weighting assumes noncancellation; known identity inputs and actual derived identity outputs are filtered. It does not equate native costs of affine, projective, exceptional or doubling paths. Sorting, queue behavior, inversions/batching, normalization, packing, allocations, scheduling and cache traffic remain unpriced. These totals are neither a universal lower bound nor measured latency.

Two of six prospective necessary checks fail: strictly fewer complete modeled additions in each shape. Both weighted/Horner doubling checks and checked-payload arithmetic checks pass. Scan equal-point/doubling costs are not separately priced. No minimum gain floor, favorable shape, changed scan family, width, partitions, threshold or native rescue is selected. A passing screen would only permit native correctness/resource qualification and later full ALL8+54 SDK/Gnark validation. The full goal is not reduced to these two early-rejection fixtures.

|Proposed new logical native payload|1×2|2×2|
|---|---:|---:|
|New point/sort/gap/index/sort-scratch payload bytes|12182832|12460392|
|Unchanged mutable scalar words bytes|962848|923264|
|Checked sum bytes|13145680|13383656|

The new payload model uses 128 bytes per component scan point, 32-byte signed sort records, 16-byte gaps, eight-byte indices and a second full 32-byte sort-record plane. It assumes in-place compaction into the scan array. Headers, allocator capacity, scratch, suspended actor frames and actual overlap remain for native qualification. This is not an allocation bound, RSS, minimum RAM, phone qualification or architectural RAM cap. There is no persistent key table, expanded solver plan or new pool. Any eventual native design would need admission and original fallback before allocation.

Actual correctness uses the two retained B-query fixtures: 30,089 raw 1×2 terms (grouping declines), and 52,274 raw 2×2 terms, grouped to 28,852 before transformation. All 58,941 inherited Phi splits reconstruct in independent Arkworks Fr. **114,089 actual scan outputs** match independent sequential suffix sums. Four new component Arkworks MSMs combine to the two retained independent complete-query oracle hashes. The old oracle producer, solver and FFT extraction are not rerun. This validates the transformation on actual curve inputs, not production AVX code or new proofs.

Two new tests cover 175 ragged signed integer scan cases and 288 telescoping group identities. A separate same-agent reader uses a different recursive weighted recurrence to reproduce all 42 window-work rows and six decisions. A second scalar reader independently checks **2,396,202 bitwise digits** and **227,832 exact coefficient reconstructions**, plus sort/sign hashes, gaps, maximum bit bounds and all 42 occupancy masks. It shares the qualified Phi helper and inherits the zero-identity facts from the actual curve execution; it does not replay the scans/MSMs or independently verify Arkworks arithmetic.

All eight serialized Rust/xtask commands succeed on first execution: six model/test/review commands and two follow-up scalar-audit commands. The extra audit checks the newly shortened-window carry boundary and scalar occupancy ledger without repeating successful curve work or changing scope/gates. Closing review checks the reader's own terminal receipt and all eight command/exit/stream hashes. Original 202 pins and the scalar audit's 207 pins match; all 176 accepted production source files remain unchanged. The original weighted counter is reused and checked by the new recursive reader. No new native field/core suite, native kernel, latency observation or performance profile is added.

Continue with a distinct bulk work or dependency reduction. Source percentages and reduced scalar length must include the work required to obtain them; here scan construction consumes the saving before native overhead is considered. Accepted SDK and canonical **43,730** verified-proof inventory are unchanged. The CPU AVX 10× goal remains active and unmet, **0/8**. The following accepted local 32-worker full-proof observations and supplied historical M5 CPU/Metal references are inherited from the previous SDK campaign.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.829286|37.0 / 4.726×|24.0 / 3.065×|3.7|
|2x2|10.979335|59.0 / 5.374×|30.0 / 2.732×|5.9|
|4x2|15.803969|109.0 / 6.897×|48.0 / 3.037×|10.9|
|8x2|23.944017|189.0 / 7.893×|74.0 / 3.091×|18.9|
|12x2|33.703579|284.0 / 8.426×|101.0 / 2.997×|28.4|
|16x2|39.858478|335.0 / 8.405×|124.0 / 3.111×|33.5|
|24x2|66.186709|528.0 / 7.977×|176.0 / 2.659×|52.8|
|merge-24x1|65.489902|507.0 / 7.742×|175.0 / 2.672×|50.7|

Accepted primary remains **7.829286 ms / 133.50 MiB** at 32 workers and 9.192237 ms at 16. Earlier 7.803404 ms used the same accepted executable in another campaign. Merge54 remains 162.541995 ms without a supplied M5 reference. No new SDK requests, proof exports, Gnark verification, AWS/Aeglos, phone/ARM, large-circuit, pressure/swap, minimum-RAM or concurrent-throughput result is added.

[Analysis](artifacts/analysis.json), [recursive work review](artifacts/review.json), [independent scalar/digit audit](artifacts/scalar-audit.json), [fixed design](artifacts/design.md), [decision](artifacts/decision.json), [continuation and autoresearch loop](artifacts/next.md), [sources and exact receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Private root: /home/ubuntu/src/zopro-perf-work/sorted-gap-scan176.
