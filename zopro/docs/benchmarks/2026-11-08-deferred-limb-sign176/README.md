# Deferred sign compensation for limb cross-products

**The new local signed-product lowering passes exact algebra and native SIMD checks. Performance is unmeasured; production is unchanged.** It removes the sign-orientation dependency before IFMA while preserving the exact low/high product words of the earlier pairwise convolution. This qualifies a fixed candidate for a separate matched release benchmark, not a speedup or adoption.

The [preceding late-centered caller recipe](../2026-11-08-late-centered-callers176/README.md) remains rejected. That recipe changed signed field intermediates and modular corrections. This experiment changes only signed limb cross-product evaluation inside the pairwise convolution; its diagonal construction, Montgomery cancellation, final radix conversion and correction remain fixed.

## Identity and work

For B=2^52 and −B<x,y<B, let u=x mod B, v=y mod B, sx=[x<0] and sy=[y<0]. IFMA computes l=uv mod B and h=floor(uv/B). Then

```text
xy = l + B * (h - sx*y - sy*u).
```

Expanding x=u−sxB and y=v−syB proves the identity. Subtracting sx*y uses the signed y and includes the both-negative cross term without a separate B² correction. The final low/high words equal the Euclidean split of xy exactly. Therefore replacing every pair also preserves every unnormalized convolution coefficient, not just a residue modulo p.

The old path computes two differences, selects the sign of one, takes its absolute value, conditionally orients the other, and then multiplies. The new path sends the signed differences directly to IFMA, which consumes only their low 52 bits. Sign masks and the low-digit mask can be computed independently; two masked high-word corrections follow multiplication. Both source recipes have nine explicit vector/mask operations per pair, including two IFMAs. There is no claim of fewer per-pair instructions or measured cycles from that count. Shorter operand dependencies may be offset by the second high correction, compiler scheduling, register lifetime or spills.

The corrected high quotient lies strictly between −B and B. Intermediate correction magnitudes remain below 2B; conservative ordinary/fused convolution coefficient bounds are 16B/32B, below signed64 range. Existing signed-pair REDC contracts are retained by exact coefficient equality. This introduces no field representation change, lookup table, key/point copy, retained solver plan, new pool or RAM cap.

## Executed checks

A new checked-i128 model passes once: 86,367 exhaustive signed pairs at radices 2 through 128, 225 radix52 boundary pairs, 131,072 deterministic full-width signed pairs and 16,384 ordinary/fused five-limb convolutions. Pair words match both independent Euclidean division of xy and the old sign-oriented formula. Every convolution matches a separately organized schoolbook multiplication after carry normalization, and its coefficients exactly match the old recipe before normalization. All four sign quadrants are exercised. These counts include related cases and are not a population of cryptographic proofs.

A separately compiled reviewer checks 186 pins, both successful model receipts/streams, reconstructed counts and signed64 bounds. Its sparse-polynomial expansion gives an identically zero difference between the new identity and the independent expansion of xy. It reads the completed model; it does not rerun it.

Two new native tests then pass once, unfiltered:

|Native coverage|Checks|
|---|---:|
|Actual ISA signed limb products, low word and signed quotient|66,504|
|Field raw results, both primes and both radices|544,608|
|Independent canonical compact R256 export comparisons|544,608|

Each field/radix population uses 51 lazy boundary values, all 2,601 ordered boundary pairs with independent lane offsets, 2,048 full-width random vector tuples and 1,024 output-fed vector steps. All three operations—ordinary product, fused sum and fused difference—are checked at eight lanes. That is 5,673 vector rows and 136,152 outputs per population. Raw values remain normalized and strictly below 2p, match Arkworks arithmetic and independent BigInt modular products, and preserve the expected R256/R260 export scaling. The difference path includes the 2p−c complement boundary at c=0.

This native source copies 18 field originals: 17 exact and one append-only parent child declaration. Removing the suffix recovers the original parent; all 176 production identities remain unchanged. The investigation child preserves the prior pairwise diagonal/convolution/reduction body except for the pair-lowering replacement and method name. Private diagnostic constructors/accessors have explicit input bounds. Existing production methods and the generator stay unchanged. The old model and field/core/prover suites are inherited where unchanged, not replayed for bookkeeping.

The four-step native workflow completes first attempt: field build, new test link, two-test run and generic no-ASM/no-IFMA field compile. The qualifier checks 212 pins, all four successful command receipts and stream hashes, source recovery/delta, output counts and artifact identities. Native settings are Rust 1.98.1, -O, native CPU, one CGU and overflow checks; the test consumer adds fat LTO/debug assertions. Generic compilation checks child gating at the field crate; it is not full-prover portability or ARM execution. Inherited unused-import/dead-code warnings and a test naming warning remain without cleanup rebuilds.

## Emitted code and limits

The actual linked pair function is 88 bytes. Its [saved disassembly](artifacts/pair-codegen.txt) has two IFMAs consuming the raw input vectors, no vector absolute value, and merged sign/mask corrections. This confirms the intended operand scheduling in this consumer. It does not prove a lower cycle count.

All four actual field-test consumers are decoded once. They have zero vector absolute-value sites and 280/281 whole-function IFMA sites. Their bodies also include constructors, all three operations, shared expressions, BigInt/Arkworks oracles and checks. These counts are not standalone product counts, dynamic retirement, spill traffic or stack peaks. The unused standalone multiplication wrapper is eliminated; no decode or execution is claimed for it. A future benchmark needs its own actual final-consumer qualification.

No performance clock, complete point caller, production Window/core integration, SDK request or proof is added. There is no new allocation/RSS/minimum-RAM, concurrent-proof, mobile/ARM/TEE, pressure/swap, huge-circuit or AWS/Aeglos measurement. Correctness and favorable dependency structure alone do not establish the requested speedup.

## Next decision

Build matched release consumers with frozen accepted-field, fresh original, exact old pairwise and new pairwise controls. The old-pair control separates this new dependency change from the existing multiplication-count tradeoff. Keep Fq/Fr, R256/R260, ordinary one/four changing states, fused sum/difference and 1/16/32 workers. All fused operands must change, avoiding the constant-operand hoisting limitation in the earlier primitive harness.

Freeze all output/resource, calibration, nonregression/gain and automatic counter/profile-quality rules and independent readers before any preflight. Preserve all measurements and failures without a field/radix/worker/compiler/layout/host rescue. Primitive success would only select complete G1/G2 callers, then actual Window/core and verified SDK/all8+54/primary16/Gnark/portable/resource qualification before adoption. [Continuation details](artifacts/next.md).

The history audit preserves the [radix51 pairwise](../2026-10-23-pairwise-radix51-component/README.md), [exact FMA](../2026-10-23-exact-fma-rne-component/README.md), [symmetric-square proof](../2026-10-24-integrated-square-proof/README.md), [signed-pair global screen](../2026-11-06-signed-pair52-screen176/README.md) and [rank-five raw-sharing](../2026-11-08-rank5-cross-raw176/README.md) results as closed evidence. The [SQIsign paper](https://arxiv.org/html/2608.13948v2), sections 3.2–3.3, uses a different prime form and SIMD organizations already examined locally; its reported gains are not BN254 predictions. The selected asymmetric limb identity is derived here from the current source dataflow.

## Unchanged proof comparison

All following proof times are inherited from the [preceding report](../2026-11-08-late-centered-callers176/README.md): latest local primary plus seven explicitly older full-campaign rows. M5 CPU/Metal are supplied references on different hardware. No model or primitive quantity is substituted for proof latency.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.002770|37.0|24.0|4.623×|2.999×|3.7|
|2x2|11.241463|59.0|30.0|5.248×|2.669×|5.9|
|4x2|15.933816|109.0|48.0|6.841×|3.012×|10.9|
|8x2|24.234311|189.0|74.0|7.799×|3.054×|18.9|
|12x2|33.855565|284.0|101.0|8.389×|2.983×|28.4|
|16x2|40.071563|335.0|124.0|8.360×|3.094×|33.5|
|24x2|65.917462|528.0|176.0|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507.0|175.0|7.764×|2.680×|50.7|

Accepted local 1×2 remains **about 8 ms**: the same binary previously measured 7.915095/7.928660/7.995682 ms; latest 8.002770 ms. The cause of inter-run variation is unconfirmed; no code adoption/regression follows from this stage. Merge54 remains inherited 162.240109 ms without a supplied M5 reference. Production176, SDK6d83 and canonical35,171 stay unchanged. The verified all-eight CPU10× goal remains active and unmet, **0/8**.

[Design](artifacts/design.md), [exact model](artifacts/model-result.json), [independent model review](artifacts/model-review.json), [native qualification/codegen](artifacts/native-qualification.json), [pair assembly](artifacts/pair-codegen.txt), [source and evidence archive](artifacts/evidence.json.gz). Each archived UTF8 payload and gzip roundtrip is verified. Exact new source/tests/helpers, model/native plans/pins/receipts and all five saved disassemblies are retained. Unchanged full source copies, executables/libraries, keys/witness/proofs/private ledgers and full external papers/source bodies are excluded; compiled artifacts are hash-bound. Public prose trims trailing blank lines only; frozen archived bytes remain exact.
