# Moving the linear part of the Euler product into key points

The prover uses the field vector F(a,b) = (Da) ⊙ b + a ⊙ (Db) in the folded H query, where D is the implementation's domain-scaled Euler derivative and ⊙ is coordinatewise multiplication. An arbitrary pair of **public**, fixed row vectors (u,v) defines residuals (a−u,b−v). When both residual inputs vanish at a row, that row's residual H scalar vanishes even though D is dense.

Let Z be the original vector of H query points. Define row-point vectors

```
Ra = (Dv) ⊙ Z + Dᵀ(v ⊙ Z)
Rb = (Du) ⊙ Z + Dᵀ(u ⊙ Z)
```

For arbitrary field vectors a and b, the original commitment equals

```
⟨Z, F(a−u,b−v)⟩ + ⟨Ra,a⟩ + ⟨Rb,b⟩ − ⟨Z,F(u,v)⟩ .
```

With a = A·w and b = B·w, the two middle terms have one public point coefficient per wire given by AᵀRa + BᵀRb. Public inputs, the constant wire, private wires and folded solver slots must each keep their correct contribution when applying this change to a real key. The standalone Arkworks test builds this correction over BN254 G1 and compares the final group point with a direct coefficient-weighted DFT for 60 cases, including zero witnesses and padded rows. It checks the negative constant term whenever nonzero.

The SDK uses four generated public requests per shape for discovery. The first policy takes both row values from the first request. The modal policy selects a pair only if that exact pair appears on at least three of the four requests, defaulting to zero otherwise. Both policies are frozen before the original and other requests are evaluated. The scalar observer runs the actual native Euler transform twice and returns the ordinary output to the prover. A zero-origin control must reproduce the ordinary counts. Each process exports an independently verified original proof. This screen does not apply the modified G1 key to a complete proof.

| Shape | Evaluation request | Rows | Ordinary nonzero H scalars | First / modal residual nonzero | Removed vs ordinary, first / modal |
|---|---:|---:|---:|---:|---:|
| 1x2 | Original | 30,922 | 30,531 | 27,754 / 27,932 | +9.10% / +8.51% |
| 1x2 | 29 | 30,922 | 30,542 | 26,419 / 26,665 | +13.50% / +12.69% |
| 1x2 | 32 | 30,922 | 30,535 | 26,481 / 26,655 | +13.28% / +12.71% |
| 2x2 | Original | 53,874 | 53,150 | 45,937 / 46,345 | +13.57% / +12.80% |
| 2x2 | 37 | 53,874 | 53,157 | 45,937 / 46,358 | +13.58% / +12.79% |
| 2x2 | 40 | 53,874 | 53,150 | 46,014 / 46,385 | +13.43% / +12.73% |
| 4x2 | Original | 98,674 | 97,202 | 87,453 / 88,197 | +10.03% / +9.26% |
| 4x2 | 37 | 98,674 | 97,282 | 84,572 / 85,245 | +13.07% / +12.37% |
| 4x2 | 40 | 98,674 | 97,262 | 84,527 / 85,232 | +13.09% / +12.37% |
| 8x2 | Original | 189,618 | 186,650 | 167,122 / 168,515 | +10.46% / +9.72% |
| 8x2 | 37 | 189,618 | 186,776 | 161,924 / 163,418 | +13.31% / +12.51% |
| 8x2 | 40 | 189,618 | 186,760 | 162,037 / 163,551 | +13.24% / +12.43% |
| 12x2 | Original | 280,242 | 275,888 | 239,487 / 241,632 | +13.19% / +12.42% |
| 12x2 | 53 | 280,242 | 275,884 | 239,394 / 241,817 | +13.23% / +12.35% |
| 12x2 | 56 | 280,242 | 275,875 | 239,517 / 241,836 | +13.18% / +12.34% |
| 12x2 | 141 | 280,242 | 277,659 | 247,564 / 249,911 | +10.84% / +9.99% |
| 12x2 | 142 | 280,242 | 277,653 | 247,875 / 249,982 | +10.72% / +9.97% |
| 16x2 | Original | 371,122 | 364,950 | 325,835 / 328,663 | +10.72% / +9.94% |
| 16x2 | 69 | 371,122 | 365,172 | 316,912 / 319,686 | +13.22% / +12.46% |
| 16x2 | 72 | 371,122 | 365,164 | 316,862 / 319,671 | +13.23% / +12.46% |
| 24x2 | Original | 554,226 | 544,454 | 485,764 / 489,942 | +10.78% / +10.01% |
| 24x2 | 93 | 554,226 | 544,857 | 471,973 / 476,833 | +13.38% / +12.48% |
| 24x2 | 96 | 554,226 | 544,869 | 472,675 / 476,920 | +13.25% / +12.47% |
| merge-24x1 | Original | 525,759 | 517,758 | 240,646 / 254,483 | +53.52% / +50.85% |
| merge-24x1 | 109 | 525,759 | 518,029 | 146,431 / 150,605 | +71.73% / +70.93% |
| merge-24x1 | 112 | 525,759 | 518,040 | 250,433 / 254,450 | +51.66% / +50.88% |
| merge-24x1 | 122 | 525,759 | 521,708 | 310,098 / 322,773 | +40.56% / +38.13% |
| merge-24x1 | 124 | 525,759 | 521,622 | 353,492 / 357,526 | +32.23% / +31.46% |

These counts exclude the work of subtracting origins, handling new nonzero linear key coefficients, and preparing or storing the correction. The uncompressed diagnostic row file uses 64 bytes per active row plus an eight-byte header. A full transformed key would also require group-valued transpose operations during preparation. A lower count of H terms establishes neither a latency gain nor a memory advantage. No per-proof state is learned from evaluation inputs.

All 96 exported original proofs passed independent verification. The row origin files stay in the local fixture workspace. [Counts and fixture hashes](artifacts/rows.json) · [Native observer](artifacts/source.patch) · [Independent group oracle](artifacts/point_check.rs) · [Verification](artifacts/verification.json)
