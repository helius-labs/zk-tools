# Matched scaled G2 norm and numerator

The mathematical construction passes and selects native qualification. Using the existing uniform-R260 fused kernel on raw R256 operands computes a fused result **divided by 16**. Dividing both the G2 norm and numerator by 16 leaves the slope unchanged. This may remove terminal mixed-radix packing from three fused operations per full group without coordinate conversions, extra products or larger records. **No native speed or adoption is established.**

## Construction and bounds

Write `H=2^256` and `R=2^260=16H`. Uniform REDC of `T=ab+cd`, interpreted as an ordinary R256 field result, gives `(AB+CD)/16`. Complement difference similarly gives `(AB−CD)/16`. Inputs in `[0,2p]` give `T≤8p²`; REDC returns strictly below `p+8p²/R<2p`. Existing normalized-digit bounds remain sufficient.

The proposed wrappers construct the uniform kernel's input directly from raw limbs, execute its existing fused arithmetic and wrap raw output back under the R256 type. They must not call R260 import/export or multiply by inverse16 at runtime. The method computes a scaled field value; point/key storage representation stays unchanged.

For `n=norm(dx)` and `u=dy·conj(dx)`, use `n'=n/16` and `u'=u/16`. The existing prefix/inverse recurrence gives `1/n'`, so `u'/n'=u/n`. Accumulated prefix powers cancel through the ordinary R256 inverse bridge automatically. Scalar tails retain their original norms and numerators; their prefix factors cancel as before. Final coordinate square/multiply/subtract remain original, including final-Y fused products.

Only owned and signed-borrowed G2 prefix streaming are proposed targets. Generic Fq2 inversion helpers, generic trees and G1 are not interchangeable users of this scaled primitive. [Full derivation and scope](artifacts/bounds.md).

This differs from the earlier coordinate-scaled R260 backend, which imported/exported coordinates and changed wider-domain arithmetic, and from shared partial Montgomery reduction, shared-CIOS outputs and scaled/doubled square formulas. The existing uniform kernel is selected because its factor cancels here, not as a radix parameter sweep. No new table, proving-key copy, retained plan, pool, buffer, field representation, C or assembly implementation is proposed.

## Exact model

One Rust/xtask compile and model execution pass on their first executions:

|New coverage|Fq|Fr|
|---|---:|---:|
|Inclusive raw corners|4,802|4,802|
|Full-width random outputs|4,096|4,096|
|Limb/radix-boundary outputs|84|84|
|Changing-chain outputs|1,024|1,024|
|Total raw fused outputs|10,006|10,006|

BigUint calculates exact R260 REDC representatives and checks divisibility/output bounds. Separate Arkworks field expressions check their logical R256 values against fused sums/differences divided by 16. Inclusive `p`/`2p` raw encodings are interpreted mathematically, without constructing invalid canonical field objects.

The complete prefix/tail model checks **21,717 candidate point outputs** across **162 fresh cases**: 18 lengths from 0 through 511, three sign modes and three successive updates. Both scale1 and scale1/16 agree with independent complete Arkworks projective addition, including untouched bucket slots. Six explicit identity, equal-x/opposite, duplicate-index and bounds guards decline before mutation. This is a finite-point model, not production fallback qualification.

Variable source work stays **7 ordinary vector multiplies plus 5 fused-two-product operations**, nominal IFMA proxy **1,450**. Three fused calls change their terminal reduction. No additional input/output codec or record payload is required by the algebra. Emitted instruction count, calls, stack/register allocation and actual whole-proof latency remain unmeasured.

An offline reviewer checks all 176 accepted source hashes, 79 original model pins, seven source anchors, two exact command/exit/stream receipts and coverage accounting. It does not replay the model or supply a second independent derivation/REDC implementation. Its compile and execution pass. [Results](artifacts/analysis.json), [review](artifacts/review.json), [model](artifacts/model.rs), [prospective protocol](artifacts/design.md).

## Upstream and next stage

Authenticated read-only GitHub CLI confirms Aeglos `alex/cpu-latency` still points to [0290613](https://github.com/helius-labs/aeglos/tree/0290613e8eeca17a4ce88f6f2d726e8e06a5385d), the measured reference, after the GitHub connector could not access that repository. No unchanged Aeglos capture is selected. The uniform-radix source prompted this local derivation; no Aeglos point kernel or reported speedup is copied.

Next: implement the raw scaled wrappers in an isolated snapshot, qualify native field and coupled core behavior, and inspect actual optimized consumers for packing/correction removal and stack cost. Only qualified source selects complete SDK correctness and a fixed primary timing pilot, followed by all-eight plus merge54 qualification if positive. This model alone establishes neither a speedup nor a twofold path.

Production/source176 and accepted SDK remain unchanged. Accepted local 32-worker **1×2 remains 7.915095 ms**, versus supplied M5 CPU 37 ms and Metal 24 ms; [inherited full comparisons](../2026-11-07-g2-real-norm-z176/README.md). The verified CPU10× objective remains active and unmet on all eight shapes. No proof or timing is created; the canonical ledger stays **31,543**. No new AWS, Aeglos performance, mobile, huge-circuit, pressure or minimum-RAM result is claimed.

Sources, plans, pins and exact model/review streams accompany this report. Binaries, libraries, proving material and private scalar payloads stay local. Private root: `/home/ubuntu/src/zopro-perf-work/scaled-norm-numerator176`.
