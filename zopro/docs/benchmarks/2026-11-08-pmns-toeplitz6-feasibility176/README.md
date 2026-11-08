# Six-coefficient Toeplitz PMNS feasibility

The division-free coefficient identity passes exact arithmetic checks. It uses **18 products per six-coefficient Toeplitz multiplication**, versus 21 in the earlier local paired PMNS prototype. This establishes a different algebraic schedule, **not a native kernel or proving-time improvement**. The existing selected base-field polynomial needs a row correction and rises to 24 products. Neither existing parameter set supports unrestricted fused inputs with coefficient magnitudes up to `2ρ`; explicit counterexamples are preserved.

The next step is parameter and coefficient-bound qualification for a binomial base-field system. Production 176 and accepted SDK `6d83…` remain unchanged. No native benchmark is selected by this screen, and the verified all-eight CPU 10× objective remains unmet, **0/8**.

## Arithmetic and complete costs

Split the 6×6 Toeplitz matrix into four 3×3 blocks `[[A,B],[C,A]]`. Three smaller products give `P=A(u+v)`, `Q=(C−A)u`, `R=(B−A)v`; outputs are `P+R` and `P+Q`. Each 3×3 Toeplitz product uses six scalar products. The composed schedule has 18 coefficient products and 61 integer additions/subtractions before machine-word lowering. The [factor schedule](artifacts/factor-schedule.json) gives all 18 linear factors and output weights.

For `E=X⁶−λ`, polynomial multiplication modulo E is Toeplitz. The currently selected Fq parameters instead use `E=X⁶−X−1`; its multiplication matrix equals a Toeplitz matrix plus one six-product row correction. This is a local cost of that exact formula, not a lower bound on every trinomial algorithm. Existing Fr uses `E=X⁶−2`.

|Geometry|Coefficient products per stage|Nominal ordinary IFMAs|Nominal fused IFMAs|
|---|---:|---:|---:|
|Binomial Toeplitz|18|90|126|
|Selected Fq trinomial plus row correction|24|120|168|
|Accepted radix52 CIOS recipe|—|100|150|

Ordinary PMNS needs two full coefficient products and one low-only internal product, giving `5k`; fused AB±CD needs three full products plus one low-only product, giving `7k`. These optimistic counts exclude signed operands, possible 53-bit transformed inputs, carries, linear operations, constants, import/export, point formulas, memory traffic and register allocation. The 61 additions are mathematical integer operations, not 61 native instructions. A lower product count alone does not justify adoption; the [latest caller rejection](../2026-11-08-fused-scale-callers176/README.md) demonstrated that smaller compiled code can retire more instructions and run slower.

The representation retains six coefficient words per field versus five unpacked radix52 limbs: 384 versus 320 bytes per eight-lane value before compiler effects. The current compact mapped coordinate is 32 bytes; a naive six-u64 replacement would be 48 bytes. No such key, bucket or layout change is made here. Conversion and key-storage costs must be included before this can become a complete-caller candidate. No lookup table, point/key copy, retained solver plan, runtime buffer or pool is introduced.

## Exact checks and coefficient bounds

The new Rust model passes 66 independent Toeplitz basis pairs, 8,256 signed full-width matvec cases and 72 polynomial basis pairs. Using the exact prior Fq/Fr parameters, the new factored PMNS implementation checks **51,200 outputs** against dense polynomial convolution and BigUint field identities: 24,576 coefficient-corner outputs, 24,576 random outputs and 2,048 output-fed updates. Old native PMNS kernels, tests and timings are not replayed. These are synthetic arithmetic cases, not proofs.

Both old systems satisfy the conservative signed `ρ` input-box bound for ordinary and fused products. They fail the corresponding fused `2ρ` certificate. Among 4,096 signed `2ρ` fused corners per field, the new model finds 2 Fq and 480 Fr outputs outside the original signed `ρ` output box. A separate dense-BigInt extractor preserves explicit inputs, T, Q, M, N, exact divisions and field identities:

|Field|Old ρ|Example output coefficient|Corner index|
|---|---:|---:|---:|
|Fq|42,526,541,758,322|44,852,794,691,681|130|
|Fr|56,761,546,165,646|73,980,003,147,914|0|

These examples prove that the particular old output bound fails on some legal inputs to the proposed wider box. They do not invalidate PMNS or show that every narrower caller trajectory overflows. The [two explicit certificates](artifacts/range-witnesses.json) are synthetic public arithmetic data. The extractor inspects 131 Fq and one Fr corners using dense arithmetic; it does not replay the successful factored model population. Publication independently rechecks these two certificates with i128 coefficient arithmetic and direct polynomial evaluation.

## Fixed binomial geometry screen

For each exact BN254 prime, scan all 128 nonzero λ from −64 through 64. Sixth-root eligibility uses `λ^((p−1)/6)=1`. An obvious rational-square/cube flag excludes those immediate factor cases; absence of that flag is not an irreducibility proof or a PMNS parameter qualification.

Let B=2⁵² and S be the maximum absolute column sum of multiplication by a candidate M modulo `X⁶−λ`. A valid M has an odd, nonzero integer determinant because it is invertible modulo B. Evaluation at a root γ, with `M(γ)=0 mod p`, makes that determinant divisible by p. Hadamard therefore gives `S≥ceil(p^(1/6))`. For this binomial matrix the maximum absolute row sum equals the maximum column sum.

The fixed sufficient triangle-bound certificate for all signed inputs of magnitude ≤2ρ and a fused two-product operation is

`8wρ² + (B−1)S ≤ Bρ`, where `w=1+5|λ|`.

Its quadratic discriminant requires `32w(B−1)S≤B²`. Combining that with the determinant lower bound leaves only |λ|≤5 within the declared scan. This rules out this particular uniform certificate, **not actual arithmetic closure under every possible tighter invariant**.

|Field|Sixth-root eligible scan rows|Survive root, obvious-factor and necessary bound screens|
|---|---:|---|
|Fq|28/128|λ=−5, 2|
|Fr|32/128|λ=−4, −2, 2|

The prior local parameter search already saved six Fq binomial `λ=2` candidates; their existence is not a new parameter discovery. This screen qualifies none of them for the wider invariant. No new γ/M/N is generated. A usable system still needs actual constants, invertibility, a complete range proof, native signed-product lowering and conversion/point-consumer costs. Native work is deferred until those requirements hold.

## Research and review

The [PMNS paper](https://arxiv.org/html/2610.09767v1) discusses Toeplitz multiplication and the importance of non-multiply costs. The [authors' Toeplitz source](https://github.com/francoispalma/PMNS/blob/master/karatoep/toeplitzmacros.h), Git blob `0caad4cb1c779977cade7339c61847264858475d`, supplies standard two/three-way block identities. No mathematical novelty, imported implementation speedup or transfer of large-modulus benchmarks is claimed. No C code is compiled. The previous [dense multiword truncation screen](../2026-11-08-pmns-truncation-width176/README.md) remains closed; Toeplitz multiplication was outside that screen's exclusion.

An independent reader assembles the 18 bilinear factors separately and checks all **396 Toeplitz and 432 polynomial coefficient identities** over the integers. It reconstructs all 256 geometry rows using a different modular exponentiation method and BigUint discriminant/root checks. It checks source pins, command/log receipts, current source prime strings and the unchanged 37,023-proof inventory. It does not replay the model's finite PMNS population or provide a second independent field backend. The explicit counterexample checks are a separate supplemental qualification.

There are nine actual workflow command receipts: seven successes and two retained failures. The original model compile/run passes once. The first reader compilation fails on an ambiguous BigUint shift type before reader execution; adding explicit types and binding the new pinset fixes it. A reader block delimiter was corrected by source inspection before its first compilation. The first explicit-witness helper incorrectly assumes the all-positive `2ρ` input is an overflow example and fails its assertion. The corrected extractor uses the first actual signed-corner example and checks the absolute output bound. Old sources, plans, pins and receipts remain archived. The successful original model and reader are not rerun; helper compiler/preparer invocations are tool-transcript observations rather than additional workflow receipts.

No latency, profile, RSS, minimum-RAM, phone, huge-circuit, memory-pressure, concurrency, AWS, Aeglos, SDK request or Gnark export is added. The next experiment may construct constants for a surviving geometry under a declared invariant; this screen establishes neither a full PMNS field API nor a proving-speed gain.

## Accepted full-proof comparisons, unchanged

These local 32-worker observations are inherited from the [latest standard-SDK campaign](../2026-11-08-tree-policy-sdk176/README.md), compared with supplied M5 references on different hardware.

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

Merge54 remains 160.890372 ms without an M5 reference. The same accepted executable previously measured 7.915095 ms for 1×2; private 7.859756/7.869400 ms observations were not adopted. The latest 8.040649 ms does not establish an adopted regression or its cause. The full verified single-proof CPU AVX ≥10× M5 CPU objective remains active and unmet, 0/8.

[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [factor schedule](artifacts/factor-schedule.json), [explicit range examples](artifacts/range-witnesses.json), [design](artifacts/design.md), [research audit](artifacts/research-audit.md), [source/inputs/receipts](artifacts/evidence.json.gz), [checksums](SHA256SUMS). The archive includes the exact old parameter input because that earlier local report is not tracked in this branch. Reproduction requires adapting the recorded absolute paths to the extracted inputs and pinned dependencies. Full third-party source/paper bodies, binaries, libraries, keys, witnesses, proof payloads and unchanged full production sources are excluded.
