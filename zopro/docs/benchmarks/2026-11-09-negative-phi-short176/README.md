# Forward cubic map using a short negative multiplier

The new arithmetic and native correctness checks pass. The actual linked forward-map caller uses **72 IFMA sites versus 100** for ordinary SIMD, while preserving the original map and point order. This selects a separate complete-transform comparison. **No latency improvement or production adoption is established.**

The preceding [inverse-oriented query](../2026-11-09-inverse-phi-query176/README.md) remains closed with its five failed checks. This construction computes the original forward map directly, so it does not swap scalar components, change bucket streams or require a final query correction. It is a distinct arithmetic recipe, without an exception or rerun of that failed query.

## Identity and bounded representation

For the accepted cube-root constant β, `β² + β + 1 = 0`. Therefore `β = −γ mod p`, where γ=β²+1=p−β is the positive 191-bit integer `2203960485148121921418603742825762020974279258880205651967`. Both γ and `floor(γ·2^260/p)` occupy four radix-52 words. The [qualified inverse-map construction](../2026-11-09-inverse-phi-small176/README.md) motivates the short-factor layout; its exact factor and quotient are different and are not retested here. The [older full-width fixed-β kernel](../2026-10-31-beta-fixed176/README.md) used 84 IFMAs and remains rejected.

Let B=2^52, R=B^5 and c=floor(γR/p). The truncated high product supplies Q no greater than floor(ac/R), with truncation error below 8/B and quotient underestimate at most one. For inclusive normalized numerical R256 inputs 0≤a≤2p, the positive remainder r=aγ−Qp satisfies `0≤r<(1+a/R+8/B)p<2p`. The new signed low accumulators evaluate **2p+Qp−aγ**, reversing the product signs inside the calculation rather than negating a finished field output.

The result initially lies in (0,2p]. The excluded endpoint occurs only for numerical a=0: invertibility of γ forces a∈{0,p,2p} when r=0, and the strictly downward reciprocal excludes r=0 at the two nonzero multiples. A mask clears exactly zero input lanes. The final result has normalized radix-52 limbs and is strictly below 2p, including changing chains and zero. The existing export can retain its single subtraction of p. Inclusive lazy boundaries are tested through private raw vectors; no out-of-range public Fp is constructed.

Each low coefficient has at most eight product halves from either side and an initial doubled modulus digit. The conservative absolute bound 20B, including carry, is below 2^57 and therefore fits signed64. Arithmetic right shifts propagate negative carries. Low products are computed modulo R; their discarded top carry need not vanish. The observed maximum absolute carry is three. The range proof, finite examples and machine tests supplement one another; they are not a formal machine-code proof.

The source count is 24 quotient IFMAs, 24 low modulus IFMAs and 24 low factor IFMAs. The zero-input mask and all normalization, packing, export and stores remain part of the actual caller. The factor/quotient payload is **64 logical bytes**, independent of circuit size; emitted constants, broadcasts and compiler stack are additional. No per-key lookup table, retained key/witness/point/scalar copy, solver plan, input-sized array, production pool, artificial memory cap or global field dispatch change is added. Scalar tails retain accepted forward multiplication.

## Qualification

The new exact BigUint model checks:

- 6,920 BN254 raw outputs: 776 boundary inputs, 4,096 fixed random inputs and 256 eight-step output-fed chains. Each checks the quotient underestimate, exact signed low reconstruction, strict range, coefficient bounds and an independent Arkworks field result.
- All **1,048,575** inputs in inclusive [0,2p] for one declared toy radix-32 instance, checking direct integer quotients, remainders, zero handling and signed digit reconstruction.
- 128 independent Arkworks curve-map identities against multiplication by λ, including identity and threefold restoration.

Three focused native tests pass, with 35 unchanged field tests filtered and inherited. They check **36,928 field-lane outputs** against Arkworks, raw inclusive lazy boundaries, strict output bounds, zero exports and changing chains; **14,610 complete point outputs** across identities and lengths/tails through 2,049 points; and all **53 actual test workers** in groups of 1/4/16/32 with 512 KiB requested stacks. The complete transform agrees with forward λ and λ² after its first two applications, restores the original after three, preserves pointers/capacities and checks output curve equations. Threaded tests use standard threads, not a new production pool or a Rayon qualification claim.

The private snapshot contains 178 source members: 175 accepted originals remain exact, one original field parent has an appended native-gated module declaration, and two children hold the implementation and tests. Stripping the suffix recovers all 176 originals. Generated fields, templates and generator remain byte-exact. Test builds use native O3, fat LTO, one codegen unit, debug and overflow checks; the reusable release field library is built separately with embedded bitcode. No unchanged inverse/field/core/prover suite is replayed, and no generic/mobile runtime or full proof is qualified by this stage.

## Actual linked code

One coherent native O3/fat-LTO/single-CGU/debug consumer contains original scalar, original ordinary-SIMD and new short-factor forward transforms. All modes compute the same forward map. The source is adapted from the retained fixed-β driver, with its guarded fixed method replaced. **The consumer has not been invoked**, so no preflight, latency or profile has occurred.

|Linked transform|IFMA sites|Static instruction sites|Instruction bytes|RSP-operand sites|
|---|---:|---:|---:|---:|
|Original scalar|0|406|2,080|32|
|Ordinary SIMD|100|286|1,792|12|
|Short negative factor|72|282|1,792|6|

Counts cover each linked symbol through the next symbol, including terminal alignment NOPs. They are not dynamic instruction/spill counts, stack bounds or a cycle prediction. Their padding convention differs from older reports. The 28% IFMA-site reduction does not establish a 28% transform or proof gain; even the total static instruction count falls only slightly. No wrapper/layout/attribute sweep is performed.

## Review, preparation failures and next step

The first model compile needed an explicit usize bit-index annotation. The first native test compile needed the Arkworks Zero trait import. Both failed sources and compiler receipts remain; only those spans change. Neither failure executed arithmetic or tests. The corrected model, native qualification/library, consumer link/decode and result reader each execute once. At the end there are **ten successful workflow commands and two failed compiler receipts**. Manual preparation/helper compilations have tool-transcript evidence.

Review checks 380 unique current pins, all original-source recoveries, exact repair spans, native counts and command stream hashes. It independently reconstructs the factor/quotient, range inequality, coefficient geometry and linked function counts without replaying the successful model, tests or decoder. It is same-agent review using the accepted field; it does not independently reproduce every model/native case. Its nine visible successful receipts include its own completed compiler step; publication checks the final tenth receipt after review exits. [Review result](artifacts/review-result.json).

**Next: freeze and run a separate complete forward-transform protocol against both original-field and fresh ordinary SIMD controls**, with 1/16/32 workers, small/large arrays, fixed matched order, declared uncertainty/gain/nonregression/resource gates and automatic profiles. Reuse this qualified library and linked fresh caller. No arbitrary gain floor, unchanged rerun, favorable subset or retrospective threshold change is selected. All recording must finish before offline decoding/analysis. A passing transform would select a new original-order query experiment; full SDK, ALL8 plus Merge54, independent Gnark, resource/admission, portable and concurrent-proof qualification remain required before adoption.

The history review found prior local studies of Gauss/raw sharing, common-Z, tripling, bucket bands and scheduler changes; none is reopened. The [SQIsign paper, sections 3.2–3.3 and 4.3](https://arxiv.org/html/2608.13948v1) uses special-prime reduction and Montgomery differential/cubical preconditions that do not directly supply a BN254 arbitrary bucket-add kernel. The [memory-constrained MSM paper](https://eprint.iacr.org/2026/1316) studies adapted bucket counts, already represented in local earlier studies. Neither reported speedup is transferred here, and no external implementation or full text is redistributed. This factor identity is derived from current algebra, without a novelty claim.

## Accepted CPU and Metal comparison, inherited

These preceding accepted-library observations use fixed worker placement; that placement is not an adopted production default. Supplied M5 references use different hardware. Historical standard 1×2 **7.915095 ms** remains valid; latest standard **8.075143 ms** and fixed **7.869302 ms** retain their separate conditions. No accepted code regression, new proof record or speed change is established here.

|Shape|Accepted fixed ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.869302|37 /4.702×|24 /3.050×|3.7|
|2×2|10.984868|59 /5.371×|30 /2.731×|5.9|
|4×2|15.734200|109 /6.928×|48 /3.051×|10.9|
|8×2|23.929023|189 /7.898×|74 /3.092×|18.9|
|12×2|33.722646|284 /8.422×|101 /2.995×|28.4|
|16×2|39.776845|335 /8.422×|124 /3.117×|33.5|
|24×2|66.183856|528 /7.978×|176 /2.659×|52.8|
|Merge24×1|65.448733|507 /7.747×|175 /2.674×|50.7|

Source: [preceding full campaign](../2026-11-09-weighted-prefix-fixed-sdk176/README.md). Merge54 remains 162.489760 ms without an M5 reference. Production176/SDK6d83 and canonical **38,668** verified exports stay unchanged. Zero new SDK requests/Gnark exports, AWS resources, Aeglos runs, mobile/huge-circuit/pressure/swap/minimum-RAM or concurrent-proof results are added. The full all-eight verified CPU10× goal remains active and unmet, **0/8**.

[Design](artifacts/design.md), [model](artifacts/model.rs), [model result](artifacts/model-result.json), [native primitive](artifacts/negative_phi_short.rs), [native tests](artifacts/negative_phi_short_tests.rs), [source recovery](artifacts/source-audit.json), [result review](artifacts/review-result.json), [continuation loop](artifacts/next.md), [source/receipts/linked assembly](artifacts/evidence.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS). The archive maps relative UTF8 paths to bytes, hash and exact contents, with gzip roundtrip and source-byte checks. Executables/libraries, unchanged full source trees, keys/witnesses/proofs and the private proof inventory are excluded.
