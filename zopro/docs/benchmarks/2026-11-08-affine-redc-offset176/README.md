# Deferred normalization of affine coordinate offsets

The new field primitives pass exact arithmetic and native SIMD qualification. Matched optimized wrappers have fewer instruction sites for all five operations while retaining the same IFMA counts. This selects complete point-caller qualification and benchmarking. **No latency improvement or production adoption is established.**

The mechanism folds canonical coordinate subtractions into the product's existing final carry normalization. The accepted R256 CIOS kernel exposes five unnormalized result coefficients after radix conversion. The candidate adds a fixed p or 2p bias and subtracts coordinate limbs there, then propagates signed carries once and corrects the range. It does not change the multiplication/cancellation prefix, input coordinates, keys or runtime buffers.

This differs from the [older multiply-subtract trial](../2026-10-22-universal-mul-sub-sdk/README.md), which normalized 2p−c and shifted it into the accumulator before final cancellation. It also differs from [typed uncarried X-sum subtraction](../2026-11-08-linear-carry-fusion176/README.md): this operation acts before product-output normalization. Those earlier results remain unchanged and are not retimed.

## Arithmetic and native qualification

For U=REDC(ab), U<2p. For fused U=REDC(ab+cd), including the accepted negative complement, U<3p. The five biased intermediates are:

|Operation|Intermediate before carry normalization|Range|
|---|---|---|
|mul − x|U+p−x|[0,3p)|
|mul − x − y|U+2p−x−y|[0,4p)|
|2·mul − x − y|2U+2p−x−y|[0,6p)|
|sum2 − x|U+p−x|[0,4p)|
|difference2 − x|U+p−x|[0,4p)|

Offsets x,y must be canonical, below p; product operands are normalized lazy values at most 2p. Negative coefficient carries require arithmetic shifts. One 2p correction suffices except for the doubled operation, which first corrects by 4p. Every returned limb is normalized and the complete value is below 2p. A p bias may change the lazy representative; canonical exports must agree. The [range argument](artifacts/bounds.md) also covers signed64 bounds and why no wider transient enters a public arithmetic API.

The checked exact interpreter passes **77,440 outputs and 387,200 cancellation steps**, across both BN254 primes, five operations, 72 edge values, every ordered edge pair, 2,048 deterministic full-width cases and 512 output-fed steps per operation. Whole-integer REDC, residues, signed carries, correction paths and bounds are checked independently of the limb recurrence.

Two new unfiltered native tests pass **619,520 lane outputs**, with the same number of canonical comparisons against Arkworks and the original sequences. Both primes and all five operations include lazy boundaries, inclusive 2p operands, zero/canonical offsets, random eight-lane inputs and output-fed chains. No invalid canonical Fp inputs are constructed. These are field checks, not cryptographic proofs or complete curve/prefix/exception qualification.

The first field build failed because the new final tuple constructor omitted PhantomData. A separate fixed snapshot changes only that marker. The failed source, stderr and exit remain archived. The corrected four-step native workflow passes; it does not rerun the mathematical model or unchanged field/core/prover suites. The generic field compile checks feature gating, not ARM execution or complete-prover portability. The inherited unused-import warning remains.

## Emitted work, not timing

One optimized variable-input consumer retains and checks 20 wrappers: original and new operations, both primes. One saved symbol listing determines all addresses; each body is decoded once. Fq counts follow; Fr has the same instruction/byte counts.

|Operation|Instruction sites, old → new|Bytes, old → new|IFMAs, old / new|
|---|---:|---:|---:|
|mul − canonical offset|238 → 235|1511 → 1491|100 / 100|
|mul − canonical sum|260 → 245|1652 → 1558|100 / 100|
|2·mul − canonical sum|296 → 282|1900 → 1803|100 / 100|
|sum2 − canonical offset|324 → 295|2065 → 1863|150 / 150|
|difference2 − canonical offset|343 → 315|2195 → 2013|150 / 150|

All 20 bodies have zero calls. Full per-body stack-operand/shift counts and hashes are in [qualification.json](artifacts/qualification.json). These are static instruction sites, not dynamic retirement, spills, stack peaks, cycles, proof latency or minimum RAM. Register lifetimes, branch frequencies and caller inlining can change the outcome.

A separate reader checks 575 pins, 29 successful workflow commands and the retained failed build, output counts, source recovery and every saved body count. BigInt/Arkworks arithmetic checks execute in the model/native tests; the reader audits their evidence rather than rerunning those qualifications.

## Next stage and memory

The qualified field library is ready for a coherent private core integration. G1 can store incoming BX in place of the existing X-sum record field, removing sum construction while retaining five 320-byte fields. It must retain only one of those fields, with an explicit role/name. G2 direct prefix paths already pack canonical coordinate offsets. No point/key copy, new record field, lookup table, solver plan, heap buffer, production pool or fixed RAM cap is proposed. Actual caller capacities, stack behavior and proving RSS still need qualification.

The canonical-offset contract excludes lazy generic-tree intermediates; global lane helpers must stay unchanged. Complete G1/G2 callers, tails, signs, exceptions, scratch reuse, small-stack workers and generic core guards precede timing. A frozen matched caller experiment with automatic counters/profiles must precede full SDK/all-eight plus merge54/primary16/Gnark/resource qualification. The private field child also needs generator integration before any production adoption. [Exact continuation](artifacts/next.md) binds the library and source paths.

Accepted production176/SDK6d83 and the canonical **36,477** verified proofs remain unchanged. This experiment adds no SDK request, Gnark export, full-proof timing, AWS/Aeglos comparison, huge/mobile/pressure/swap or throughput result.

## CPU and Metal comparisons, unchanged

Inherited latest accepted local 32-worker full-proof observations from the [default-carry comparison](../2026-11-08-uniform-carry-default-performance183/README.md), with supplied M5 references on different hardware:

|Shape|Accepted Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.018438|37 / 4.614×|24 / 2.993×|3.7|
|2×2|11.199751|59 / 5.268×|30 / 2.679×|5.9|
|4×2|15.957517|109 / 6.831×|48 / 3.008×|10.9|
|8×2|24.250900|189 / 7.794×|74 / 3.051×|18.9|
|12×2|33.735233|284 / 8.418×|101 / 2.994×|28.4|
|16×2|40.165709|335 / 8.340×|124 / 3.087×|33.5|
|24×2|65.802777|528 / 8.024×|176 / 2.675×|52.8|
|merge24|64.926690|507 / 7.809×|175 / 2.695×|50.7|

The full verified all-eight CPU10× objective remains active and unmet, **0/8**. Merge54 remains 162.442299 ms without a supplied M5 reference. Earlier accepted 1×2 observations include 7.915095 ms; private experiments include 7.859756 and 7.869400 ms. Sub-8 ms was already measured; cross-campaign absolute variation does not establish an adopted regression or new record.

[Qualification and code counts](artifacts/qualification.json), [exact model result](artifacts/model-result.json), [design](artifacts/design.md), [range argument](artifacts/bounds.md), [qualified native child](artifacts/affine_offset.rs), [native tests](artifacts/native-tests.rs), [matched wrappers](artifacts/codegen.rs), [model](artifacts/model.rs), [reader](artifacts/qualify.rs), [source recovery](artifacts/source-audit.json), [compile correction](artifacts/native-repair.json), [source/plans/pins/raw receipts and decodes](artifacts/sources-and-receipts.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS).

Every archived UTF8 member hash and the gzip roundtrip pass. Unchanged full source snapshots, executables/libraries, key/image/witness/proof bytes and the private proof ledger are excluded; relevant identities remain pinned. Manual helper compiler invocations do not have complete xtask compiler receipts, and no such claim is made.
