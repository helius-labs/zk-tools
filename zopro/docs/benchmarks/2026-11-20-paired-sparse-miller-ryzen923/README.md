# Pairing sparse lines in the BN254 Miller loop

The native IFMA variant reduced isolated one-pair Miller time from **100.40 to 92.03 µs**, with a **-8.30%** paired change. The scalar variant took **101.39 µs**. On 1×2, the combined profile reached **6.854 / 6.755 ms** on the original and held-out inputs, a paired improvement of **0.85% / 0.91%** versus same-build ordinary. It was the only profile that cleared both controls on both inputs.

Each nonzero signed Miller digit uses a doubling line and an addition line. The trial multiplies these two sparse factors before updating the Fq12 accumulator. It uses 23 Fq2 multiplications per paired step instead of 26. The two final lines use the same product. The multiplication keeps the original tower basis and uses fixed local scratch. The IFMA variant batches five or six independent Fq2 products at each multiplication boundary. The combined profile uses the scalar paired lines and the previously measured packed final exponentiation. [Sparse tower products in Gnark Crypto](https://github.com/Consensys-Incorporated/gnark-crypto/blob/v0.21.0/ecc/bn254/internal/fptower/e12_pairing.go#L124-L190) · [Packed final exponentiation](../2026-11-20-single-cyclotomic-ryzen900/README.md)

| Mode | Median µs | Paired change vs ordinary | Wins |
|---|---:|---:|---:|
| ordinary | 100.40 | +0.00% | 0/8 |
| paired | 101.39 | +1.09% | 0/8 |
| ifma | 92.03 | -8.30% | 8/8 |

The component uses one pinned Ryzen thread and eight counterbalanced blocks. Each process primes 48 calls and measures 192 over 24 G1 and prepared G2 pairs. Every output is compared with the ordinary multi-pair entry, with an identity second pair. Each result is a median of process means.

## Full proofs

Proofs use the pinned four-epoch solver and the seeded 4×2 profile, with 24 main and eight frontend hardware threads. Each process primes three proofs and measures seventeen. Eight counterbalanced blocks compare the frozen release, same-build ordinary, scalar paired lines, IFMA paired lines, and combined profile. Advancing requires a lower paired median and at least six wins out of eight against both Zopro controls on both inputs. All six ordinary controls calibrated within 2%.

| Shape | Ordinary ms, original / held-out | Scalar vs ordinary | IFMA vs ordinary | Combined vs ordinary |
|---|---:|---:|---:|---:|
| 1x2 | 6.925 / 6.824 | -0.06% / +0.00% | -0.53% / -0.14% | -0.85% / -0.91% |
| 2x2 | 9.509 / 9.542 | +0.48% / +0.54% | +0.11% / -0.12% | -0.45% / -0.17% |
| 4x2 | 13.691 / 13.659 | +0.13% / -0.13% | -0.56% / -0.18% | -0.49% / -0.17% |

<details>
<summary>Matched proof timing and whole-process peak RSS</summary>

| Shape | Input | Mode | Proof ms | Paired vs release | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.964 | +0.00% (0/8) | -0.05% (4/8) | 60.5 |
| 1x2 | catalog | ordinary | 6.925 | +0.05% (4/8) | +0.00% (0/8) | 58.0 |
| 1x2 | catalog | paired | 6.928 | -0.49% (5/8) | -0.06% (4/8) | 57.8 |
| 1x2 | catalog | ifma | 6.889 | -0.67% (6/8) | -0.53% (8/8) | 58.0 |
| 1x2 | catalog | combined | 6.854 | -1.05% (6/8) | -0.85% (6/8) | 58.2 |
| 1x2 | heldout | release | 6.818 | +0.00% (0/8) | +0.00% (4/8) | 58.0 |
| 1x2 | heldout | ordinary | 6.824 | -0.00% (4/8) | +0.00% (0/8) | 58.2 |
| 1x2 | heldout | paired | 6.829 | +0.34% (3/8) | +0.00% (4/8) | 58.5 |
| 1x2 | heldout | ifma | 6.823 | -0.11% (4/8) | -0.14% (5/8) | 58.2 |
| 1x2 | heldout | combined | 6.755 | -0.84% (6/8) | -0.91% (6/8) | 58.8 |
| 2x2 | catalog | release | 9.482 | +0.00% (0/8) | -0.42% (6/8) | 94.5 |
| 2x2 | catalog | ordinary | 9.509 | +0.42% (2/8) | +0.00% (0/8) | 98.0 |
| 2x2 | catalog | paired | 9.542 | +0.92% (1/8) | +0.48% (3/8) | 98.0 |
| 2x2 | catalog | ifma | 9.547 | +0.50% (0/8) | +0.11% (4/8) | 96.0 |
| 2x2 | catalog | combined | 9.442 | -0.06% (5/8) | -0.45% (6/8) | 94.8 |
| 2x2 | heldout | release | 9.496 | +0.00% (0/8) | -0.40% (6/8) | 98.5 |
| 2x2 | heldout | ordinary | 9.542 | +0.40% (2/8) | +0.00% (0/8) | 94.0 |
| 2x2 | heldout | paired | 9.545 | +0.69% (2/8) | +0.54% (2/8) | 101.0 |
| 2x2 | heldout | ifma | 9.524 | +0.40% (2/8) | -0.12% (5/8) | 97.0 |
| 2x2 | heldout | combined | 9.502 | +0.31% (2/8) | -0.17% (5/8) | 95.0 |
| 4x2 | catalog | release | 13.749 | +0.00% (0/8) | -0.00% (4/8) | 214.8 |
| 4x2 | catalog | ordinary | 13.691 | +0.00% (4/8) | +0.00% (0/8) | 220.8 |
| 4x2 | catalog | paired | 13.649 | -0.52% (6/8) | +0.13% (3/8) | 204.2 |
| 4x2 | catalog | ifma | 13.619 | -0.88% (7/8) | -0.56% (5/8) | 215.8 |
| 4x2 | catalog | combined | 13.648 | -1.02% (6/8) | -0.49% (5/8) | 223.2 |
| 4x2 | heldout | release | 13.729 | +0.00% (0/8) | +0.77% (3/8) | 213.8 |
| 4x2 | heldout | ordinary | 13.659 | -0.77% (5/8) | +0.00% (0/8) | 214.8 |
| 4x2 | heldout | paired | 13.662 | +0.40% (3/8) | -0.13% (5/8) | 210.2 |
| 4x2 | heldout | ifma | 13.661 | +0.11% (4/8) | -0.18% (5/8) | 219.5 |
| 4x2 | heldout | combined | 13.625 | -0.62% (5/8) | -0.17% (6/8) | 217.0 |

</details>

The first build's IFMA feature check selected the ordinary fallback, so its IFMA arm measures that fallback. The scalar trial in that build took 118.79 µs against 125.82 µs ordinary. Its advantage did not repeat in the corrected build. Route tests caught the missing native path, and exact-output tests caught incorrect subtraction of lazy field values. The corrected build tests that native routing is reachable and matches packed Fq2 products, paired line products, and complete Miller outputs. All 44 native BN254 tests passed. Gnark independently verified all 240 corrected-run and 240 initial-run proof exports against the pinned original keys and public inputs.

The recorded AWS score remains **5/8** at 10× M5 CPU. The scorecard includes the supplied CPU and Metal references for all eight shapes. AWS SSO had expired, leaving the pending 2×2 confirmation unrun. [C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings)

[Protocol](artifacts/protocol.json) · [Full-proof samples](artifacts/rows.json.gz) · [Component samples](artifacts/component-rows.json.gz) · [Summary](artifacts/summary.json) · [Trial source](artifacts/source.patch) · [Verification](artifacts/verification.json) · [Initial fallback run](artifacts/initial-fallback/summary.json)
