# Sparse-inverse RNS conversion screen

An exact mixed-radix RNS construction works for both BN254 fields, but the specified conversion recipes do not show a complete advantage over current arithmetic. **No native kernel or SDK trial is selected, and no speedup is claimed.** Production remains unchanged; accepted local1x2 remains8.324853ms at134.50MiB peak RSS.

The [Selected RNS Bases paper](https://www.acsel-lab.com/arithmetic/arith19/papers/ARITH19_Bajard.pdf) uses sparse moduli and inverses to replace constant products with shifts/additions during mixed-radix conversion. Its operation proxy targets architectures with expensive multiplication. Here it is a search heuristic, not an AVX timing model. The construction, equations, search examples and discussion were inspected; the downloaded PDF remains local with a hash in the pins.

## Search and correctness

The declared family has50 distinct moduli of the form2^52-c, where c is0,1 or2^t±1 for1<=t<=25. All714520 coprime unordered five-sets were enumerated. A subset DP selects the best order for each set; an exact bounded pair search performs3992870 cross-pair checks. The selected pair has the global minimum declared additive proxy,510, within this family. It is not a global native-code optimum.

| Basis | Ordered offsets c |
|---|---|
| Left | 129,65537,4194305,0,33554433 |
| Right | 1025,131073,16777217,1,17 |

Both bases use five residue channels. Exact mixed-radix conversion removes the earlier CRT weights, quotient estimates and correction table, but requires four dependent digit rounds followed by four Horner rounds per direction. A multiplication requires both directions. These levels are not latency predictions.

Normal representatives are in[0,4p), with one linear operation permitted to produce inputs in[0,8p). The product, fused sum2 and difference2 with a field-zero64p² offset all return below3p. All results match independent BigInt Montgomery arithmetic and field interpretation; both residue bases reconstruct the same integer. [Bounds and cost assumptions](artifacts/bounds.md) give the exact conditions.

| Validation | Fq | Fr |
|---|---:|---:|
| Boundary reductions | 3888 | 3888 |
| Fixed random reductions | 12288 | 12288 |
| Changing chain steps | 1024 | 1024 |
| Total | 17200 | 17200 |

Another2048 full-width base-extension integers,1024 per direction, agree with independent residues under both word recipes. Three new tests pass once: NAF/inverses and modular word recipes; exhaustive small-integer conversions on near-52-bit moduli; and DP scores against all120 permutations on eight fixed synthetic cost matrices. No test or arithmetic run failed. A pre-execution design clarification changed the small-modulus test draft to small integers on the actual word-size assumptions; both drafts are retained. The build's unused-import warning is preserved.

## Arithmetic work

The model spells out signed-NAF word products and two pseudo-Mersenne folds. It also checks a hybrid using52bit Shoup multiplication: one IFMA, two low64 products and canonicalization. The hybrid chooses the smaller logical operation count for each constant. This is parameter selection, not a measured latency optimization.

| Specified product-body recipe | Fq sites | Fr sites |
|---|---:|---:|
| Sparse shift/add constants | 3797 | 3691 |
| Hybrid | 945 | 945 |
| Hybrid base conversions only | 666 | 666 |

The hybrid product has85 IFMA sites,140 low64 products and720 other logical arithmetic sites. Current ordinary CIOS has100 IFMA sites. Comparing only the IFMA counts would discard most of the new work. Fused hybrid sum2/difference2 have1153/1193 sites. All modeled costs remain identical across the checked input sets.

These are counts for the explicit recipes, not optimized compiler instructions, cycles or a lower bound on other RNS algorithms. Constant-specific simplifications may reduce them. Memory/register allocation, spills, binary-format conversion and curve integration remain excluded. This study does not justify porting this recipe to the prover; it does not establish that every RNS implementation would be slower.

The raw modulus/coefficient/offset arrays contain95 u64 words,760bytes, before Shoup factors, NAF descriptors and object layout. There are no proving-key or point lookup tables. The representation still has ten working residue planes versus five, and80 stored residue bytes per field versus32 currently. No proof RSS or minimum-RAM prediction follows from these sizes.

All172 production source hashes remain unchanged and are checked before/after and at closure. There are **zero new SDK requests, exports, verifier calls or timing/profile captures**; the proof ledger remains12524. No AWS, Aeglos, phone, TEE, large-circuit, concurrency or memory-pressure result is added. Further work should leave this unchanged two-base MRS recipe and address another mechanism supported by the current prover profile.

## Inherited CPU and Metal comparison

The [accepted all-eight report](../2026-10-27-clean-fft-adoption/README.md) remains authoritative, including its full-performance-gate failure and engineering exception. M5 figures are supplied historical references. The original all-eight CPU10x goal remains active and unmet,0/8.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 4.445x | 24 | 2.883x |
| 2x2 | 11.460675 | 59 | 5.148x | 30 | 2.618x |
| 4x2 | 15.854365 | 109 | 6.875x | 48 | 3.028x |
| 8x2 | 24.084626 | 189 | 7.847x | 74 | 3.072x |
| 12x2 | 33.693397 | 284 | 8.429x | 101 | 2.998x |
| 16x2 | 39.981636 | 335 | 8.379x | 124 | 3.101x |
| 24x2 | 66.010670 | 528 | 7.999x | 176 | 2.666x |
| Merge24x1 | 65.086624 | 507 | 7.790x | 175 | 2.689x |

Accepted merge54 remains161.341835ms without a supplied M5 reference.

[Design](artifacts/design.md), [exact model](artifacts/model.rs), [search results](artifacts/search.json), [arithmetic results](artifacts/analysis.json), [source/library/PDF hashes](artifacts/pins.json), [artifact hashes](artifact-index.json).
