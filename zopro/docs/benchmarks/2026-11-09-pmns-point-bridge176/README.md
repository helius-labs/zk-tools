# PMNS point ranges and compact-coordinate conversion

**This bridge family is closed before native implementation.** Its best certified coordinate import costs 102 nominal IFMAs. Even an idealized G2 update with free reuse of converted X coordinates and no bucket-normalization cost reaches 1,668 nominal IFMAs, versus the current 1,450. Retaining compact key and bucket coordinates gives 2,944 before further sign/carry/linear-operation costs. These are source-operation ledgers, **not measured slowdowns or lower bounds on every PMNS implementation**.

The [preceding Toeplitz screen](../2026-11-08-pmns-toeplitz6-feasibility176/README.md) established a different coefficient identity. This study checks the coordinate bridge and point ranges that identity would need. No native benchmark is selected. Production 176, accepted SDK `6d83…` and the canonical 37,023-proof inventory remain unchanged. The verified all-eight CPU 10× objective remains active and unmet, 0/8.

## Source paths and operation budget

The actual G1 full group imports four coordinate vectors, stores its packed records, then exports two updated coordinates. Both reserve/write and push constructors have the same count. Its six ordinary products include numerator scaling, prefix, slope, inverse update, square and Y finishing.

The signed borrowed G2 prefix imports bucket X and key X; its reverse pass imports bucket X/Y and key X/Y. Each complex coordinate has two base-field components: **12 base-field vector imports and four exports per eight-point group**. Its arithmetic is seven ordinary and five fused two-product operations. Alternative branches for the sign handling are counted once at runtime, not twice from their textual occurrences.

The inherited binomial PMNS recipe has nominal 90/126 IFMAs per ordinary/fused operation, versus the current 100/150. Arithmetic alone would therefore save 60 G1 or 190 G2 IFMAs per group. Every converter in this declared family uses at least one PMNS internal reduction, priced optimistically as 18 low-half plus 36 full-product-half instructions: 54 IFMAs before forming the input linear combination. Importing even the four distinct G2 incoming key components costs at least `4×54=216`, exceeding the 190 saving. G1's two incoming components similarly cost at least 108 against 60 saved. This bound concerns this fixed conversion recipe.

|Hypothetical full-group source ledger|G1 IFMAs|G2 IFMAs|
|---|---:|---:|
|Current arithmetic, bridge unchanged|600|1,450|
|PMNS arithmetic only, all other costs omitted|540|1,260|
|PMNS plus ideal distinct-key imports, all bucket costs omitted|744|1,668|
|PMNS plus imports in the actual retained-bucket loop, bucket costs omitted|744|1,872|
|PMNS with current compact coordinates, imports and exports|1,178|2,944|

These PMNS rows assume a hypothetically range-safe product kernel at the inherited primitive counts. They omit additional normalization, sign compensation, potentially wide transformed operands, carries, additions, constants, memory traffic, allocation, inverse bridges, scalar tails and dispatch. Retained PMNS buckets and free X reuse are deliberately favorable accounting cases, not implemented layouts or qualified architectures. The compact case already loses its product-volume advantage before those costs.

## Exact conversion family

Use all six existing Fq parameter candidates with `E=X⁶−2`. Their roots were already saved by the old local search; no root or parameter discovery is claimed. Raw p/γ decimal tokens are parsed exactly rather than through JSON floating-point values, with quote/escape guards for the pinned input format.

Write B=2⁵² and H=2²⁵⁶. The host coordinate is a compact R256 word value v, representing `v/H mod p`; a PMNS polynomial A represents `A(γ)/B`. For k-bit input digits and r internal reductions, fixed small polynomials satisfy

`Cᵢ(γ) = 2^(ki) H⁻¹ B^(r+1) mod p`.

The converter forms `T=Σ digitᵢ Cᵢ` and reduces r times. Thus its result satisfies `A(γ)H=vB mod p`. Small power-of-two constant polynomials use shifts in the count. The fixed study includes digit widths 43–52 and one/two reductions: **120 configurations**. Fixed field constants are small; this is not a key-sized cache or lookup table.

|Input digit bits|One reduction: nominal IFMAs / certified candidates|Two reductions: nominal IFMAs / certified candidates|
|---|---:|---:|
|43–48|114 / 6 each|168 / 6 each|
|49|114 / 6|180 / 6|
|50|126 / 6|168 / 6|
|51|102 / 6|156 / 6|
|52|102 / 0|156 / 6|

Overall, **114/120** configurations have a conservative per-coefficient `ρ` output certificate. The 51-bit, one-reduction case costs 102 for all six candidates and is certified. The six 52-bit, one-reduction cases fail that certificate; three also produce 20 finite outputs above ρ. The other three have no sampled overrun, which does not repair the failed bound. Every configuration still preserves its field identity and round trip; no row is excluded.

For export, fixed constants are `γʲ H² B⁻¹ mod p`. A signed six-term constant dot product is followed by literal 52/52/52/52/48-bit Montgomery reduction. Its source recipe has 60 raw product halves and 55 reduction IFMAs, or 115 nominal IFMAs. This retains the five quotient-generation products; counting only the modulus products would underprice it. For coefficients bounded by 8ρ, the signed pre-reduction range gives a result strictly between −p and 2p, so one correction on either side suffices. Sign/carry lowering remains unimplemented and unpriced.

Per configuration, 16 boundaries, 256 deterministic full-width inputs and 64 output-fed cases give **40,320 imports and 40,320 round-trip exports**. Another 64 signed corners and 128 random exports per parameter bring export checks to **41,472**. Import evaluations agree with the independent field equation; literal export rows agree with the closed Montgomery quotient and direct polynomial evaluation. These are synthetic arithmetic cases, not SDK proofs. Successful prior PMNS/Toeplitz populations are not rerun.

## Point bounds and normalization

The interval calculation conditionally assumes each PMNS product returns coefficients bounded by ρ and leaves additions/subtractions unnormalized. It follows the actual Fq2 square: the real component is one product, while the imaginary component doubles another product. After the first G2 update, the resulting upper bounds are X=(3ρ,4ρ), Y=(2ρ,2ρ). The finishing complex product combines asymmetric offsets 4ρ and 5ρ, giving a raw bound `9wρ²`, rather than the preceding generic two-input `8wρ²` envelope.

|Prior updates without coordinate normalization|X input bounds / ρ|Norm raw coefficient / wρ²|Numerator coefficient / wρ²|Y-product coefficient / wρ²|
|---:|---|---:|---:|---:|
|0|1, 1|8|8|9|
|1|3, 4|41|27|19|
|n|1+2n, 1+3n|8+20n+13n²|8+14n+5n²|9+10n|

The reader derives these polynomial coefficients independently and checks all 65 recorded n=0…64 rows. They are conservative conditional certificates, **not executed valid-curve trajectories or proof that every actual point addition overflows**.

The certificate `Cwρ²+(B−1)S≤Bρ` requires discriminant `4Cw(B−1)S≤B²`. Using the inherited determinant bound `S≥ceil(p^(1/6))`, the first G2 packet excludes the prior Fq λ=−5 geometry under this certificate; λ=2 remains only a necessary geometry candidate. All six actual existing Fq binomial M matrices fail the first-packet certificate for any ρ. Retained unnormalized buckets also fail the second-update norm certificate for all five inherited necessary geometries. Fr rows in that artifact are arithmetic diagnostics, not scalar-field G2 implementations.

A complete implementation would therefore need a different proven bound or explicit normalization, in addition to its representation bridge. Neither cost is hidden in setup or excluded from a claimed proving-time gain. The fixed converter is already unfavorable without charging them, so no new M search, native kernel, key format or retained-coordinate rescue is selected here.

## Research and independent review

The [authors' conversion generator](https://github.com/arithPMNS/low_memory_efficient_PMNS/blob/main/C_codes_generator/gen_file_useful_functs.sage), Git blob `e4c4933bf78c43563eb80b995c1f58c1f32a9750`, emits digit/constant-polynomial accumulation followed by internal reduction. This study adapts that family to R256 input and checks export separately. No C, Sage or Python code is executed. The ePrint 2025/467 PDF returned HTTP403 and was not read; its full algorithm is not cited as inspected evidence.

The independent reader checks 222 pins, both successful model receipts, the retained compiler failure, all 120 ledgers, 696 constant-polynomial identities, 36 export constants, closed-form coefficient bounds, source paths, symbolic recurrences and the six existing-parameter range decisions. It uses Fermat inverses/direct polynomial evaluation instead of the producer's Euclid/Horner organization and a closed geometric bound instead of iterated floors. It does not replay the finite conversion/export populations or independently implement a native field backend. Shared BigInt arithmetic and the stated interval assumptions remain limitations.

Five workflow command receipts are retained: four successes and one failed initial compilation. The failure compares `&BigInt` with `BigInt` in an assertion; removing the redundant borrow repairs only the assertion type before any model execution. The corrected model and independent reader each run once. Manual helper compiler/preparation/publication calls are tool-transcript observations, not extra workflow receipts. No successful prior math/native suite, decoder, profile, verifier or capture is replayed.

No runtime key/point copy, new representation layout, table, solver plan, buffer or pool is added. A naive six-u64 coordinate would occupy 48 bytes versus the current 32; that is a prospective payload comparison, not a measured RAM regression. The selected import constants would occupy at most 288 bytes plus 240 bytes of export constants, excluding code/broadcast representation and other field constants. No RSS, minimum-RAM, pressure, phone, huge-circuit, concurrency, AWS, Aeglos, latency, profile, SDK request or Gnark result is added.

## Accepted full-proof comparison, unchanged

Inherited local 32-worker observations from the [latest standard-SDK campaign](../2026-11-08-tree-policy-sdk176/README.md), against supplied historical references on different hardware:

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1×2|8.040649|37 / 4.602×|24 / 2.985×|3.7|
|2×2|11.149097|59 / 5.292×|30 / 2.691×|5.9|
|4×2|16.069415|109 / 6.783×|48 / 2.987×|10.9|
|8×2|24.131178|189 / 7.832×|74 / 3.067×|18.9|
|12×2|33.940628|284 / 8.368×|101 / 2.976×|28.4|
|16×2|40.164083|335 / 8.341×|124 / 3.087×|33.5|
|24×2|66.187704|528 / 7.977×|176 / 2.659×|52.8|
|merge24×1|65.361571|507 / 7.757×|175 / 2.677×|50.7|

Merge54 remains 160.890372 ms without an M5 reference. The same accepted executable previously measured 7.915095 ms for 1×2; private 7.859756/7.869400 ms observations were not adopted. Later 8.040649 ms does not establish an adopted regression or its cause. No shape meets the full CPU 10× requirement; the goal remains active.

[Analysis](artifacts/analysis.json), [constants](artifacts/constants.json), [source accounting](artifacts/source-context.json), [point bounds](artifacts/point-bounds.json), [existing-parameter range decisions](artifacts/parameter-range-review.json), [independent review](artifacts/review.json), [design](artifacts/design.md), [source/inputs/receipts](artifacts/evidence.json.gz), [checksums](SHA256SUMS). All UTF8 member hashes and gzip roundtrip are checked. Full third-party source/paper, executables/libraries, keys/witnesses/proofs, raw recordings and unchanged production sources are excluded. Exact old parameter bytes are archived because that earlier local experiment is untracked; reproduction requires adapting recorded absolute paths to the extracted inputs and pinned tools.
