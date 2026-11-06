# Deferring inverse-root canonicalization: raw range screen

**Range screen passed; no codec or performance result yet.** The existing scalar Montgomery multiplier can accept the proposed lazy root limbs for BN254 Fq/Fr and still return a canonical result. This permits testing canonicalization after the root product, while the [half-root export route](../2026-11-06-inverse-half-roots176/README.md) remains closed.

The raw scalar range screen passes and selects a separate native SIMD codec implementation, not timing or adoption. For each BN254 prime, the exact condition 4p<2^256 permits multiplying two raw values up to 2p with the unchanged scalar Montgomery multiplier and returning a canonical result after its existing single conditional subtraction.

The proof uses nonnegative interleaved CIOS state T_i<a+p. Each pre-division row is below 2^64(a+p)≤3p·2^64<2^320, so its five-word accumulator cannot overflow. After four rows, T_4<p+ab/2^256≤p+4p²/2^256<2p. The model checks exact divisibility, state/carry bounds and the final rational inequality using BigUint, alongside an independent modular-product reference. The current native multiplier drops its cancelled word and uses one reduce_once; no assembly is changed.

Both unchanged portable and actual BMI2/ADX multiplier functions match the independent reference on 11,308 distinct raw operand pairs per prime, 22,616 field-tagged cases total. Each field has 7,136 cases with at least one noncanonical input and 802 zero outputs. All 90,464 CIOS rows satisfy the checked bounds. Observed row/state sizes reach 320/256 bits and remain within the strict analytic limits. Endpoints include 0, p and 2p; the intended SIMD source domain is strictly below 2p, and actual inverse roots remain nonzero under the existing checked-batch contract.

One normal native Rust harness includes byte-exact accepted portable.rs and x86_64.rs and executes only their original multiplier functions. The accepted field library supplies public modulus/inverse constants; it is not rebuilt. Input pairs are raw arrays, and no unreduced Fp object is constructed. Source, harness executable and helper-library hashes are frozen before the one successful Rust/xtask execution. No earlier field/core suite, extraction, native candidate or timing run is repeated.

The normalized radix52 root codec, public field method and core integration are not implemented here. The range pass does not establish their correctness, register use, instruction savings, performance or portable-device behavior. Those require fresh qualification of the changed boundary. In particular, mixed R260 values must retain the original conversion/fallback; a raw R256 interpretation would be wrong there.

The half-root export experiment remains closed: its original premise was disproved in optimized code. This screen instead permits deferring canonicalization across the scalar product, which could remove the actual vector reduction. No saved work or latency is claimed until that candidate's final consumer is checked and measured. Accepted production176/SDK6d83, inherited local proof times/RSS and proof ledger 30,846 remain unchanged. The full all-eight CPU10× objective stays active and unmet.


## Exact arithmetic scope

Let R=2^256, B=2^64 and 0≤a,b≤2p. Both primes satisfy 4p<R. Interleaved CIOS starts T₀=0 and computes Tᵢ₊₁=(Tᵢ+a·bᵢ+mᵢp)/B, where 0≤mᵢ<B makes the numerator divisible by B. If Tᵢ<a+p, the numerator is less than B(a+p)≤3pB<2^320 and Tᵢ₊₁<a+p<R. After four rows, T₄<p+ab/R≤p+4p²/R<2p, so the existing single subtraction returns a canonical residue. Canonicalizing either input before multiplication gives the same residue because input values remain congruent modulo p.

The harness checks the rational final bound without rounding it downward. It compares the exact row model and both actual existing multipliers against an independent BigUint modular product times R inverse. Raw input arrays, including p and 2p boundaries, never become Fp objects. The intended SIMD roots remain strictly below 2p; the scalar screen tests the inclusive boundary too. Mixed R260 cannot use raw R256 interpretation and retains its conversion/fallback.

|Field|Distinct pairs|Noncanonical input pairs|Zero results|CIOS rows|Maximum observed row-sum bits|Maximum observed state bits|
|---|---:|---:|---:|---:|---:|---:|
|Fq|11308|7136|802|45232|320|256|
|Fr|11308|7136|802|45232|320|256|

Counts are distinct within each field; 22,616 is the sum of the two field-tagged populations. Tested pairs cover the Cartesian endpoint set, power/limb boundaries and 8,192 deterministic random pairs per prime, with duplicates removed. The analytic bounds are strict below 2^320 for a row sum and below R for a state; an observed bit length of 320 or 256 fits those bounds. These checks are not a formal instruction-level proof of every CPU flag transition.

## Source and workflow

A normal native Rust harness includes the byte-exact accepted [portable multiplier](../../../crates/zopro-bn254/src/portable.rs#L77) and [BMI2/ADX multiplier](../../../crates/zopro-bn254/src/x86_64.rs#L66), including the latter's existing single final reduce_once. No multiplier, field-crate build, assembly backend or prior field/core test suite is added or rerun. The accepted field library supplies modulus/inverse constants. The [field invariant](../../../crates/zopro-bn254/src/fp.rs#L322) continues to require canonical Fp objects; this screen does not weaken that API.

Source, compiled harness, accepted field and explicit helper-library hashes are frozen before the one successful Rust/xtask execution. The publisher rechecks 14 pins, the command/exit/stream bindings and both field/row totals without repeating the native harness. The harness compiled and ran on its first attempts. No profile, SDK request, solver/FFT extraction, proof, Gnark check or AWS resource was added.

Earlier [direct-chain inversion](../2026-10-24-direct-chain-inverse/README.md) already removes the outer SIMD/host bridges. [Whole-array word export](../2026-11-06-word-export176/README.md), [direct Euclidean finish](../2026-10-28-inverse-finish-sdk/README.md) and [scaled-R260 G2](../2026-10-28-scaled-r260-native/README.md) retain their own results. This screen changes the admissible multiplication boundary; it does not reopen those implementations. History/keyword inspection is not an exhaustive claim about every old temporary source.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.




## Continuation and artifacts

The [first unfinished step](artifacts/next.md) is the actual normalized-lazy SIMD root-product codec and its independent output/range checks, followed by final-consumer inspection and prospective complete-caller evidence. Scalar range checks alone do not establish the codec, emitted work reduction or proof speed. All eight verified CPU10× targets remain active and unmet.

[Analysis](artifacts/analysis.json), [design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [harness](artifacts/check.rs), [pre-execution freeze](artifacts/freeze.rs), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [helpers and receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Full executable/library blobs remain local. No private key, image, witness, point/scalar cache or proof bytes are included. The accepted source files are linked and hash-bound rather than duplicated as a new backend.
