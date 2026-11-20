# Packing one BN254 cyclotomic exponentiation on Ryzen

The [C7a 1×2 trace](../2026-11-19-tail-c7a726/README.md) separated about 0.23 ms from core return to proof-generation return, 0.29 ms from generation return to final exponentiation, and 0.32 ms for the final exponentiation on the original request. The last interval gives the scale of the work targeted here.

The three Fq4 squares in each cyclotomic square use the same AVX-512 IFMA instructions across six coordinates at a time. Two vectors carry 12 coordinates from one Fq12 result. They stay packed across consecutive squares. Fq12 products use the existing scalar arithmetic, with conversion at their boundaries. The packed binary ladder starts at the leading nonzero digit. The Miller loops, proof equation and final result comparison retain their current implementations. The portable build uses the existing scalar exponentiation.

Final exponentiation took 171.65 µs ordinary and 124.36 µs packed. The paired latency change was -27.43% with 8/8 wins. The component clock covers the actual final exponentiation, including its easy part and hard part, over 24 distinct inputs from the Miller loop. Each mode runs eight pinned processes on CPU 2. Each process uses 48 warmups and 192 timed calls, with output checks outside those clocks. The two modes produce identical canonical outputs for all 24 inputs. The native field suite passed 40 tests. The generic build with IFMA and assembly disabled also passed its field suite.

The full-proof comparison uses 24 main and eight frontend Ryzen logical CPUs. Ten matched blocks per input and shape compare the pinned release, same-build ordinary, and packed mode. Each process primes three proofs and times 23 on 1×2 or 17 on the other shapes. Advancement requires a lower paired median and eight wins against each Zopro control on both inputs. No shape cleared both Zopro controls on both inputs. All 180 exports independently verified. This experiment used Ryzen only. AWS authentication had expired before the run.

| Shape | Input | Mode | Full proof ms | Paired vs release | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.861 | +0.00% (0/10) | -0.68% (8/10) | 58.2 |
| 1x2 | catalog | ordinary | 6.902 | +0.68% (2/10) | +0.00% (0/10) | 58.5 |
| 1x2 | catalog | packed | 6.908 | +0.34% (4/10) | -0.19% (6/10) | 58.2 |
| 1x2 | heldout | release | 6.786 | +0.00% (0/10) | +0.15% (4/10) | 59.2 |
| 1x2 | heldout | ordinary | 6.788 | -0.15% (6/10) | +0.00% (0/10) | 58.2 |
| 1x2 | heldout | packed | 6.768 | -0.22% (6/10) | -0.29% (6/10) | 58.0 |
| 2x2 | catalog | release | 9.486 | +0.00% (0/10) | -0.83% (10/10) | 99.8 |
| 2x2 | catalog | ordinary | 9.557 | +0.83% (0/10) | +0.00% (0/10) | 95.8 |
| 2x2 | catalog | packed | 9.499 | +0.25% (2/10) | -0.92% (8/10) | 95.2 |
| 2x2 | heldout | release | 9.504 | +0.00% (0/10) | -0.77% (8/10) | 98.2 |
| 2x2 | heldout | ordinary | 9.565 | +0.78% (2/10) | +0.00% (0/10) | 99.2 |
| 2x2 | heldout | packed | 9.501 | +0.42% (4/10) | -0.58% (8/10) | 97.5 |
| 4x2 | catalog | release | 13.584 | +0.00% (0/10) | -0.15% (5/10) | 215.0 |
| 4x2 | catalog | ordinary | 13.586 | +0.15% (5/10) | +0.00% (0/10) | 223.0 |
| 4x2 | catalog | packed | 13.605 | +0.01% (5/10) | -0.04% (5/10) | 213.8 |
| 4x2 | heldout | release | 13.603 | +0.00% (0/10) | -0.22% (5/10) | 213.5 |
| 4x2 | heldout | ordinary | 13.617 | +0.22% (5/10) | +0.00% (0/10) | 239.0 |
| 4x2 | heldout | packed | 13.632 | +0.17% (4/10) | -0.20% (6/10) | 217.2 |

[Source](artifacts/source.patch) · [Component samples](artifacts/component.json.gz) · [SDK samples](artifacts/samples.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

## Eight supplied M5 references

The [recorded C7a profiles](../2026-11-20-typed-small-proof-c7a898/README.md) still reach 10× M5 CPU on **5/8** shapes.

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.858 | 37 / 24 | 4.71× / 3.05× |
| 2×2 | 12.029 | 59 / 30 | 4.90× / 2.49× |
| 4×2 | 14.726 | 109 / 48 | 7.40× / 3.26× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |
