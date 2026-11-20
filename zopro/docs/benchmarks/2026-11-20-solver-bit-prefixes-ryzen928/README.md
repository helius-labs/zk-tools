# Rebuilding bit-weighted solver prefixes

The packed path replaced term-by-term field arithmetic in **3,014 terms on 1×2** and **5,689 on 2×2**. The 1×2 instruction clock went from **0.725 / 0.728 to 0.717 / 0.720 ms** on the original and held-out inputs. Its paired full-proof change against same-build ordinary was **-0.26% / -0.17%**. The candidate fell short of the full-proof comparison on all three shapes. On 1×2 it was 0.06% and 0.34% slower than the frozen release on the two inputs.

The candidate recognizes runs of eight or more consecutive powers of two on wires written by existing bit hints. It packs positive or negative runs in either order into four limbs, converts to the field once and applies the original incoming prefix value. It splits at every stored prefix and non-chain dependency. Every original wire, row and stored prefix still reaches its usual consumer. A value other than zero or one takes the interpreter path for the run.

Preparation requires the existing definite-assignment check and optional memory admission. It keeps a bounded read-only index of the selected prefix runs and uses four local limbs per reconstruction. The mapped solver records and serialized key format are read as before. An early implementation considered tagging the sum index. The measured implementation uses the optional map so that full-width indices keep their original range.

| Shape | Packed spans per proof | Terms in spans | Terms in negative spans |
|---|---:|---:|---:|
| 1x2 | 28 | 3,014 | 760 |
| 2x2 | 50 | 5,689 | 1,520 |
| 4x2 | 95 | 11,052 | 3,040 |

The independent image reader matched every runtime plan and route count. The zero-optional-workspace probes selected the interpreter and verified their proofs. The tests cover both signs and directions, limb boundaries, long masks, non-bit fallback and boundaries where stored prefixes or dependencies split a run. All 32 enabled native solver tests passed. Gnark independently verified all 198 proof exports.

## Timed proofs

The Ryzen run uses 24 main and eight frontend hardware threads. Eight counterbalanced blocks for each shape and input compare the frozen release, same-build ordinary and packed variants. Each process primes three proofs and times seventeen. Advancing requires lower paired medians and at least six wins against both Zopro controls on both inputs. All six ordinary controls calibrated within 2%. Peak RSS covers the entire process. The proof samples use no route logging or solver clocks.

| Shape | Input | Profile | Proof ms | Paired vs release | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.928 | +0.00% (0/8) | -0.12% (5/8) | 57.8 |
| 1x2 | catalog | ordinary | 6.925 | +0.12% (3/8) | +0.00% (0/8) | 58.0 |
| 1x2 | catalog | packed | 6.915 | +0.06% (4/8) | -0.26% (6/8) | 57.8 |
| 1x2 | heldout | release | 6.780 | +0.00% (0/8) | -0.60% (5/8) | 58.0 |
| 1x2 | heldout | ordinary | 6.808 | +0.61% (3/8) | +0.00% (0/8) | 58.2 |
| 1x2 | heldout | packed | 6.816 | +0.34% (3/8) | -0.17% (4/8) | 57.0 |
| 2x2 | catalog | release | 9.488 | +0.00% (0/8) | -0.27% (6/8) | 95.0 |
| 2x2 | catalog | ordinary | 9.514 | +0.27% (2/8) | +0.00% (0/8) | 96.8 |
| 2x2 | catalog | packed | 9.473 | +0.15% (4/8) | -0.45% (5/8) | 97.8 |
| 2x2 | heldout | release | 9.515 | +0.00% (0/8) | +0.16% (2/8) | 98.5 |
| 2x2 | heldout | ordinary | 9.492 | -0.16% (6/8) | +0.00% (0/8) | 100.0 |
| 2x2 | heldout | packed | 9.462 | -0.36% (5/8) | -0.42% (5/8) | 95.0 |
| 4x2 | catalog | release | 13.777 | +0.00% (0/8) | +0.28% (4/8) | 211.2 |
| 4x2 | catalog | ordinary | 13.650 | -0.28% (4/8) | +0.00% (0/8) | 208.2 |
| 4x2 | catalog | packed | 13.623 | -0.67% (5/8) | -0.01% (4/8) | 206.5 |
| 4x2 | heldout | release | 13.678 | +0.00% (0/8) | +0.04% (4/8) | 210.2 |
| 4x2 | heldout | ordinary | 13.750 | -0.04% (4/8) | +0.00% (0/8) | 211.8 |
| 4x2 | heldout | packed | 13.679 | -0.18% (5/8) | -0.98% (7/8) | 214.8 |

## Solver clock

Separate three-block captures time instruction execution, with three primes and seven timed proofs per process.

| Shape | Input | Profile | Solver ms |
|---|---|---|---:|
| 1x2 | catalog | ordinary | 0.725 |
| 1x2 | catalog | packed | 0.717 |
| 1x2 | heldout | ordinary | 0.728 |
| 1x2 | heldout | packed | 0.720 |
| 2x2 | catalog | ordinary | 0.917 |
| 2x2 | catalog | packed | 0.923 |
| 2x2 | heldout | ordinary | 0.928 |
| 2x2 | heldout | packed | 0.912 |
| 4x2 | catalog | ordinary | 1.507 |
| 4x2 | catalog | packed | 1.525 |
| 4x2 | heldout | ordinary | 1.547 |
| 4x2 | heldout | packed | 1.584 |

The recorded C7a score remains **5/8** at 10× M5 CPU. CPU and Metal references are available for all eight shapes. AWS SSO had expired when checked. [C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings)

[Protocol](artifacts/protocol.json) · [Samples and route probes](artifacts/rows.json.gz) · [Image census](artifacts/image-census.json) · [Census reader](artifacts/bit-recomposition-census.py) · [Trial source](artifacts/source.patch) · [Verification](artifacts/verification.json)
