# Friendly-multiple Montgomery work screen

**Both fixed recipes close before native implementation.** A modulus multiple makes quotient digits cheap, but the larger modulus and required projection consume the saving. Canonical-state multiplication needs **110/160 modeled IFMAs** for ordinary/fused products, versus current **100/150**; widened-state multiplication needs **132/204**. Exact arithmetic checks pass. No native kernel, timing, proof or production change is added.

The idea chooses M=k·p with low radix-52 digit+1or−1. If the current low digit is t0, quotient digit q is −t0 modB for+1 and t0 for−1, B=2^52. Exact cancellation shifts52bits. Since p divides M, projecting (T+Q·M)/R modulo p gives the intended Montgomery field value. M is generally composite: it cannot replace the prime field for inversion or zero tests. This is an independently derived feasibility study, with no novelty claim or copied implementation.

|Prime|Low digit sign|Smallest positive multiplier k|M bits|Largest k fitting five limbs|
|---|---:|---:|---:|---:|
|Fq|1|3932390912793719|306|84|
|Fq|-1|571208714576777|303|84|
|Fr|1|3952108915130369|306|84|
|Fr|-1|551490712240127|303|84|

For each sign, k is the unique smallest positive solution modulo B; adding B only enlarges M. Neither sign for either prime fits five radix-52 limbs. The two signs/fields and two recipes were fixed before execution; there is no favorable-constant selection.

**Canonical-state R260:** keep inputs below2p and use five cheap cancellation rows against six-limb M. The resulting U is too wide to feed directly into another five-limb product. Every operation must project U back modulo p. The exact bound U<(k+1)p makes its projection quotient at most k, hence belowB. The ledger charges a one-digit quotient times five p limbs (ten IFMA halves), while optimistically pricing quotient estimation, tag conversion, normalization and bridges at zero. Omitting that mandatory projection would measure a different, invalid recurrence for five-limb state.

**Widened-state R312:** retain six-limb values below2M and perform six cancellation rows. Six-limb input products must also be charged. The exact bound (8M²+(R−1)M)/R<2M makes ordinary and fused outputs stable. Exports, field equality/zero tests and original-field inversions still need projection modulo p. Imports/exports and those bridges remain unpriced. No transformed point/key array is constructed.

|Explicit fixed tape|Baseline|Canonical R260|Widened R312|
|---|---:|---:|---:|
|Input limbs / cancellation rows|5 / 5|5 / 5|6 / 6|
|Ordinary raw-product IFMAs|50|50|72|
|Fused raw-product IFMAs|100|100|144|
|Cancellation plus quotient IFMAs|50|50|60|
|Mandatory per-product projection IFMAs|0|10|0, deferred to boundaries|
|Ordinary total|100|110|132|
|Fused total|150|160|204|

Both friendly low-digit cancellation and quotient selection receive zero IFMAs. All five higher digits of these four actual multiples are nonzero and nonunit. Counts explicitly enumerate the schoolbook product/cancellation/projection tape; they are not native instruction retirement, a universal lower bound, a complete dependency schedule or a proof that more IFMAs cannot run faster. The predeclared work-removal screen requires fewer modeled IFMAs for ordinary and fused products in both fields/signs. All eight recipe decisions fail. No native radix/estimator/table/subset rescue follows.

A widened eight-lane field value has six vector words versus five,384bytes versus320before compiler effects. This is logical per-value payload, not a frame bound, RSS, minimum RAM or measured prover allocation. Intermediate projection buffers, conversions, carry/ALU work, compiler spills and scheduling remain unpriced. Model BigUint allocations are offline checks. No table, expanded key/solver plan, production pool or architectural RAM cap is introduced.

The new BigUint recurrence passes **123,392 output checks and678,656 low-row cancellations** across both primes, signs and recipes. It checks the cheap digit recurrence, a full-R quotient from a separately constructed inverse and independent Arkworks field projections. Per context:64ordinary edge pairs,8,192fused corner tuples,6,144random-operation checks and1,024output-fed updates. Inclusive0/2p or0/2M, zero complements and both operation signs are covered. Counts include repeated values, not distinct proof inputs. The final canonical projection in the model uses exact division; it is not a proposed fast native divider.

A separately written same-agent u128 reader uses radix16, odd moduli3..31, both signs and two/three cancellation rows. It checks **122,140 instances /305,350 rows**, with brute-force full inverses rather than the producer's BigUint/Newton utility. It also recomposes all eight work ledgers and cohort counts, checks187pins and176unchanged production hashes. It does not replay the large Arkworks model, prove native unnormalized-digit headroom or qualify a changed inversion bridge. Finite arithmetic checks and algebra do not constitute formal native-code verification.

All four serialized Rust1.98.1/xtask commands succeed on first execution. The publisher checks the reader's own terminal receipt, all argv/exit/stream hashes, archive roundtrip and links. One unused Field-import warning is retained without a cleanup rebuild. No previous model, native suite, performance capture, solver/FFT extraction or proof is repeated.

This closes these two fixed work-reduction recipes. The useful lesson is to price the enlarged field and projection, rather than counting only the removed quotient multiplies. The earlier truncated/full-quotient, paired cancellation and parallel-carry results remain closed; this changes the modulus multiple and input/output contracts, not those schedules.

Accepted local32-worker full-proof observations and supplied historical M5 references remain unchanged:

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.829286|37 / 4.726×|24 / 3.065×|3.7|
|2x2|10.979335|59 / 5.374×|30 / 2.732×|5.9|
|4x2|15.803969|109 / 6.897×|48 / 3.037×|10.9|
|8x2|23.944017|189 / 7.893×|74 / 3.091×|18.9|
|12x2|33.703579|284 / 8.426×|101 / 2.997×|28.4|
|16x2|39.858478|335 / 8.405×|124 / 3.111×|33.5|
|24x2|66.186709|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.489902|507 / 7.742×|175 / 2.672×|50.7|

Primary remains **7.829286ms /133.50MiB**; earlier7.803404ms used the same executable. Primary16 remains9.192237ms; merge54 remains162.541995ms without a suppliedM5reference. Accepted SDK/source176 and canonical43,730verified proofs are unchanged. CPUAVX10×ALL8 remains **active and unmet,0/8**. No new AWS/Aeglos, SDK/Gnark proof, phone/ARM, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput result is added.

[Analysis](artifacts/analysis.json), [independent reader](artifacts/review.json), [decision](artifacts/decision.json), [fixed design](artifacts/design.md), [continuation/autoresearch loop](artifacts/next.md), [sources and exact receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Private root:/home/ubuntu/src/zopro-perf-work/friendly-multiple176.
