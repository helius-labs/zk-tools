# Fixed-factor truncated multiplication: integer screen

**The new fixed-factor arithmetic passes its integer screen and selects SIMD qualification. No speedup is measured.** The source ledger uses **76 IFMA operations plus five ordinary 64-bit low multiplies**, versus 100 IFMAs in the current ordinary CIOS product, before accounting for carries, normalization, loads and register pressure. Production and accepted full-proof times are unchanged.

The [preceding joint X/delta experiment](../2026-11-09-joint-coordinate-delta-release176/README.md) remains rejected. This is a different algorithm: precondition a fixed canonical multiplier, retain only selected high and low limb products, and recover an exact bounded residue. It does not rearrange that rejected endpoint arithmetic.

## Exact arithmetic

Let B=2^52, R=B^5, normalized raw input 0≤x≤2p and fixed **ordinary canonical** factor 0≤w<p. Precompute Q=floor(wR/p). The output is raw x·w modulo p, so it preserves x's R256 or R260 Montgomery radix without a conversion. A Montgomery-encoded factor cannot be substituted for ordinary w.

Approximate the high product xQ/R by keeping full digit products with i+j≥5 and only high halves at i+j=4. The resulting integer H underestimates the true quotient. Its omitted contribution E is below 9: the independent digit bound has numerator `9R − 10B^4 + 1`. With the preconditioning error below x/R<1, this gives

    0 ≤ xw − Hp < 10p < R.

The low five digits of xw−Hp therefore recover that exact remainder. Their final carry is deliberately discarded modulo R. It can range from −9 to 8 because the truncated convolutions are not independently normalized; it is not merely a single subtraction borrow.

One final small quotient, k=floor(r4·mu/B) with mu=floor(B/(p4+1)), reduces the remainder below 2p. The separate reader checks the exact modulus-specific inequality that proves this range for both accepted primes. The five digit corrections use ordinary low multiplies; their products fit below 2^56. Signed carry bounds fit 64 bits. [Design](artifacts/design.md), [model results](artifacts/model-result.json), [separate review](artifacts/review.json).

This follows the fixed-operand idea described by [Shoup](https://www.shoup.net/ntb/ntb-v2_5.pdf) and [FLINT's fixed-operand documentation](https://flintlib.org/doc/ulong_extras/shoup.html), with background on truncated SIMD products from [Didier et al.](https://arxiv.org/abs/2410.18129). Their word-size and RSA implementations do not establish performance here. No external implementation body or reported speedup is imported. The five-limb bounds and recipe are derived locally.

## Evidence and retained failures

The unrestricted high-product test checks 1,048,576 digit-pattern pairs, independently verifies the omitted-product bound, and observes a maximum quotient deficit of 6. The fixed-factor model checks 9,522 outputs per prime, 19,044 in total: 18 input edges ×17 factor edges, 8,192 full-width random pairs and 1,024 output-fed iterations. It verifies exact high quotients, recovered full remainders, canonical residues, normalized digits and final ranges against full BigUint multiplication/division. These are arithmetic cases, not proofs.

The general bound allows a quotient deficit of 9; the observed maximum is 6 for both fields. Observed maximum absolute carry is 5 for Fq and 6 for Fr. Finite maxima supplement the general arguments rather than replace them. The separate reader checks actual accepted field moduli, reconstructs the product count and omitted-product formula, and checks 20 independent carry extrema without replaying the model populations.

Two failures remain in the evidence. The first build needed two explicit Rust loop-index types; no arithmetic executed before that repair. The first test execution then passed the unrestricted high test but failed an overly narrow diagnostic assertion on the discarded low-convolution carry. Correcting that assertion to the derived [−9,8] interval leaves the arithmetic, exact remainder equality, canonical/range checks and populations unchanged. Only the previously failed test is run again; the successful million-case high test is retained without replay. [Type repair](artifacts/repair.json), [carry-assertion repair](artifacts/repair-carry.json).

The review checks all five command receipts: three successes and two retained failures, together with 193 current bindings and all 176 unchanged production files. Both distinct mathematical tests now pass across the retained executions. This is not a clean first-pass suite, native SIMD validation, or a machine-code proof.

## Work, memory and next step

|Source work|IFMA operations|
|---|---:|
|Truncated high quotient|25|
|Low input × factor|25|
|Low quotient × modulus|25|
|Final small quotient|1|
|Total|76|

Five additional ordinary low multiplies, carry/mask operations and factor loads remain. The current CIOS reference has 100 IFMAs. No emitted code or cycle measurement exists yet; fewer IFMAs need not mean faster execution.

The provisional FFT application covers only the bottom seven scalar butterfly stages. Their root set contains at most 31 layer quads plus pair/single quads per direction. At 80 bytes per scalar factor and three directions, retaining all original roots would add at most **31,680 bytes** of factor payload, independent of circuit domain. This is a structural bound, not an application RAM cap, actual allocated capacity, stack/RSS peak or minimum RAM. There is no proposed whole-domain quotient table or large persistent lookup. Setup cost, admission and real native storage remain unqualified.

Next is one native implementation of this fixed recipe, independent raw/export field checks, generic guards and actual complete FFT-consumer code inspection. Precomputed-factor loads and extra live vectors may erase the arithmetic saving. Native component success would still require complete callers and all-eight plus merge54 SDK/Gnark/resource/portable/concurrency checks before adoption. The 24% IFMA-count reduction on eligible factors is not a twofold proof forecast. [Continuation](artifacts/next.md).

## Accepted CPU and Metal comparison, inherited

The following unchanged local Ryzen 9950X full-proof observations use 32 workers and common fixed placement. Supplied M5 CPU/Metal references use different hardware. None is a new measurement from this integer screen.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

Production176/SDK6d83 and the 40,154-verified-proof ledger remain unchanged. Goal active and unmet, 0/8. No new native field/core/prover qualification, timing, profile, SDK request, proof, Gnark export, AWS/Aeglos, mobile/ARM, huge-circuit, memory-pressure, swap, minimum-RAM or throughput result is added. No job or owned cloud resource remains live.

[Source and receipt archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
