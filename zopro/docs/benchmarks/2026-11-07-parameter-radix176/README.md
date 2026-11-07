# Curve-parameter radix Montgomery screen

The exact arithmetic screen passes, but this explicit radix-u recipe is **closed before native implementation**. Eliminating the cancellation inverse multiply does not establish a complete work reduction: wider digit products and nonbinary normalization introduce substantial work. Production source 176 and accepted SDK `6d83` stay unchanged. This is a work model, with no new timing or proof.

For u=4965661367192848881, the actual primes satisfy
`Fq=36u^4+36u^3+24u^2+6u+1` and
`Fr=36u^4+36u^3+18u^2+6u+1`.
Their constant coefficient is 1. In radix u, choose `m=-c_i mod u`, add `m*p`, divide the low coefficient exactly by u and carry. Five rounds use Montgomery `R=u^5`, a 311-bit integer. The small modulus coefficients permit shifts and adds. This differs from the prior [six-coefficient PMNS](../2026-10-23-pmns-field-component/README.md) and [quadratic representation](../2026-10-28-quadratic-fq-screen/README.md) recipes.

The input invariant is `[0,2p)`. Its first four digits have at most 63 bits and its last digit is at most 72, requiring 7 bits. Fused difference uses the nonnegative complement `2p-c`, including `2p` when `c=0`; the same digit bounds still hold. Unsigned 128-bit convolution accumulators suffice under conservative ordinary/fused bounds of 127/128 bits. Since `p+8p^2/R<2p`, all three modeled operations retain lazy closure. Integration would also require import/export and conversion between Montgomery factors; those costs are not measured here.

## Literal product and reduction work

Split a 63-bit digit into 52+11 bits. A full partial product wider than 52 bits needs both IFMA halves. The separate mixed recipe uses VPMULLQ when that partial product fits 64 bits. These are explicit source-operation recipes, without compiler or CPU cost estimates.

|Quantity|Ordinary|Fused sum/difference|
|---|---:|---:|
|IFMA-only raw convolution|137|274|
|Mixed raw convolution IFMAs|32|64|
|Mixed raw convolution VPMULLQs|65|130|
|Distinct coefficient quotient/remainder positions|9|9|
|Small modulus-coefficient products|20|20|
|Accepted complete-product IFMAs|100|150|

The IFMA-only raw count is `16*7+8*3+1`. The mixed count is 32 IFMAs and `16*3+8*2+1` wide low products. IFMA and VPMULLQ are different instructions; the mixed counts are **not** an equivalent-IFMA score or a speed measurement. Nine quotient/remainder positions comprise five cancellation positions and four final carry positions. This optimistic structural count permits quotient reuse within a position; it is not a count of evaluated scalar divisions in the model or a native instruction count. No vector divider for 128-bit coefficients by u, or reciprocal schedule, has been implemented or priced. Linear operations, carries, packing, conversions, register pressure and spills remain additional.

The all-IFMA recipe already exceeds the accepted complete-product count before reduction. The mixed recipe has no specified inexpensive complete normalization, so it does not pass the frozen continuation condition. No native kernel, radix/width sweep or favorable operation subset is selected. This closes this exact construction, not all nonbinary representations or an optimization lower bound.

Five normalized u-digits would use 40 bytes per stored coordinate versus the existing 32-byte compact field representation. A literal ten-coefficient u128 convolution represents 20 vector words, or 1,280 bytes for eight lanes, before other operands and carry scratch. These are layout payloads, not simultaneous live-register bounds, actual allocation, RSS or minimum RAM. No table, key copy, solver plan or production pool is introduced.

## Checks

Each prime passes 11,568 operations: 36 boundary values crossed into 1,296 tuples, 2,048 fixed full-width random tuples and 512 changing-chain tuples, with ordinary product, fused sum and fused difference checked for every tuple. Exact coefficient convolution and Montgomery output residues match independent BigUint integer products and extended-Euclid radix inverses. Each output is normalized and below `2p`. The conservative coefficient bounds and lazy-closure inequality are checked separately.

An exhaustive radix-4/modulus-97 control checks all 37,636 pairs from `[0,194)`. This is new arithmetic evidence, not a native SIMD or cryptographic proof qualification. The build, new model and review succeed on their first execution; no existing successful tests, profiles, recordings or verifiers are rerun.

Review checks 182 pins, all 176 unchanged production identities, both polynomial identities, case counts, literal work totals and the model receipt. Publication also checks both successful command receipts and output hashes, complete workflow state and every archive member against its source. Source, frozen design, plans, pins, outputs and receipts are preserved in the [UTF-8 evidence archive](artifacts/evidence.tar.gz). The report received a prose edit after generation. No executable, library, key, witness or proof payload is published.

## Accepted full-proof comparisons, inherited

The latest [matched full SDK campaign](../2026-11-07-fft-column-prep-sdk176/README.md) remains authoritative. These are local Ryzen 9950X/32-worker accepted proof observations, compared with supplied M5 references on different hardware. They are not measurements of this model. All eight CPU 10× requirements remain unmet.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10x limit ms|
|---|---:|---:|---:|---:|
|1x2|8.013551|37 /4.617x|24 /2.995x|3.7|
|2x2|11.254041|59 /5.243x|30 /2.666x|5.9|
|4x2|15.963789|109 /6.828x|48 /3.007x|10.9|
|8x2|24.269224|189 /7.788x|74 /3.049x|18.9|
|12x2|33.884727|284 /8.381x|101 /2.981x|28.4|
|16x2|40.199590|335 /8.333x|124 /3.085x|33.5|
|24x2|65.690345|528 /8.038x|176 /2.679x|52.8|
|merge24|65.555031|507 /7.734x|175 /2.670x|50.7|

Merge54 remains 161.737128 ms without an M5 reference. Earlier 1×2 sub-8 ms observations used the same accepted binary; the cause of inter-run variation is unestablished. Canonical proof ledger remains 33,949; this model adds zero SDK requests or proofs. No new AWS/Aeglos, ARM/mobile, huge-circuit, pressure/swap, peak-RAM or minimum-RAM result is claimed. The original all-eight 10× goal stays active.

[Model result](artifacts/analysis.json), [review](artifacts/review.json), [frozen design](artifacts/design.md), [checksums](SHA256SUMS).
