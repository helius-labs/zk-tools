# Width-12 bitwise NAF work screen

No native implementation is selected. On the retained primary B scalars, conventional bitwise NAF reduces nonzero arrivals by **7.98%** versus all radix windows, but increases the declared complete dense generic-addition model **10.61%**, from634,405to701,734. These are operation counts for two explicit recipes, **not latency measurements or a lower bound on every NAF algorithm**. Production remains unchanged.

The candidate uses odd signed digits with12-bit zero spacing and up to127bit positions,1,024odd-magnitude buckets per position. The control uses balanced radix4096, eleven positions and2,048signed-magnitude buckets per position. Both use the original two-part root geometry and inherited exact G2 scalar decomposition. The primary fixture has30,089points and60,178components. It is hash-bound to the retained independently checked Arkworks fixture; no complete curve oracle is rerun. All actual component encodings reconstruct exactly in both recipes, with oddness/range/spacing checks for NAF. Another20,000boundary/random cases pass. Only scalar-recoding correctness is newly qualified.

This is conventional bitwise NAF. The [libsecp256k1 fixed-window implementation](https://github.com/bitcoin-core/secp256k1/blob/master/src/ecmult_impl.h) instead uses fixed blocks and a skew correction; its digit bounds, number of positions and bucket economics differ. This screen does not test or reject that implementation. [Width-w NAF literature](https://arxiv.org/abs/1110.0966) discusses recoding optimality; that does not establish whole-MSM or proof latency. No external implementation is copied.

| Model count | Radix ordinary only | Radix all11, hypothetical dense top | Bitwise NAF all127 |
|---|---:|---:|---:|
| Nonzero arrivals | 558614 | 613891 | 564892 |
| Occupied bucket first writes | 40960 | 41072 | 176121 |
| Shared slots merged across parts | 20480 | 20536 | 74269 |
| Dense weighted-fold model adds | 40940 | 41050 | 238694 |
| Total unweighted model adds | 579074 | 634405 | 701734 |

The model is arrivals minus occupied first writes, plus shared-part bucket merges, plus twice(maximum-used-slot−1)per active position for an eager dense weighted fold. It counts sign-independent occupied slots and ignores point cancellations, equal-x cases, queue/projective fallback, inversion/normalization, SIMD tails, codecs and scheduling. Odd weights also require a per-position doubling and final combination; those are omitted, favoring the candidate. Small integer model counters are temporary research storage, not retained runtime point tables.

The accepted implementation uses an optimized indexed top, represented by55,277terms here, not the hypothetical dense top. Ordinary arrivals558,614and first writes40,960agree with the retained [actual operation census](../2026-11-05-g2-operation-census176/README.md); arrivals after first writes include1,178projective arrivals rather than claiming every update takes the affine path. Complete native execution cost for the candidate is unmeasured. This model therefore declines a native trial of the fixed eager recipe; it does not exclude every sparse NAF reduction, predict a speed ratio, or justify selecting a width after seeing the counts.

## Preserved comparator correction

The successful helper's final selection boolean compared the candidate's full count to ordinary-only radix work. That omitted the indexed top and was insufficient for a complete-work comparison. All original counts/source/receipt and that boolean remain unchanged. An offline reviewer reconstructs the row sums and instead compares the two complete dense counterfactual recipes already present. Both decline native selection; no scalar scan, recoding validation, sample, curve oracle or successful command is repeated. [Scope correction](artifacts/interpretation.md). Neither comparator measures the accepted indexed-top cost.

All176production source hashes remain exact. The helper includes an exact pinned slice of the qualified scalar decomposition; its existing mathematical qualification is inherited. The two-step workflow compiles and counts once. The independent offline review checks source/input pins, both command/exit/stream hashes and every aggregate row, but does not perform a second scalar scan or complete curve oracle. Scalars, digits, per-point bucket ordinals and input point bytes stay private. No executable, proving key, witness/image or frozen library is published.

Zero new SDK requests, Gnark proof exports or performance measurements. Canonical proof ledger remains30,708. No AWS/Aeglos, phone/ARM/TEE, 250/4999, memory pressure/minimum-RAM or concurrent-proof result follows. The all-eight CPU10× goal remains active and unmet,0/8. The earlier [late-Y caller](../2026-11-05-g2-late-y176/README.md) remains closed at its measured regression.

## Accepted CPU and Metal comparisons, inherited

These are accepted32-worker local proof timings and supplied historical M5 references, not measurements of this recoder. [Accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json).

|Shape|Accepted ms|M5 CPU ms|M5 Metal ms|CPU/local|Metal/local|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37|24|4.667×|3.027×|3.7|
|2x2|11.151932|59|30|5.291×|2.690×|5.9|
|4x2|16.086386|109|48|6.776×|2.984×|10.9|
|8x2|24.179453|189|74|7.817×|3.060×|18.9|
|12x2|33.884455|284|101|8.381×|2.981×|28.4|
|16x2|40.077381|335|124|8.359×|3.094×|33.5|
|24x2|65.850777|528|176|8.018×|2.673×|52.8|
|merge-24x1|65.060937|507|175|7.793×|2.690×|50.7|

Merge54 remains161.883899ms with no supplied M5 reference.

[Design](artifacts/design.md), [raw counts](artifacts/analysis.json), [review](artifacts/review.json), [source](artifacts/screen.rs), [receipts](artifacts/receipts.json), [checksums](SHA256SUMS). Reconstruct the helper fromscreen.rsandsplit.rs, the pinned original fixture and exact tool/library argv in the two-step plan. Only aggregate counts are public. Receipt members verify exact UTF-8bytes, lengths andSHA-256before publication.
