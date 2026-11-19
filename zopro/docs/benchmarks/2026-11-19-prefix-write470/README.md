# Writing only the Euler derivative prefix on 12×2

The solver needs 280,242 derivative rows in a 524,288-row domain. The candidate keeps the existing inverse transform and interior forward stages. The last radix-32 pass emits only the first 70,062 packed pairs including final SIMD padding and skips the unused upper subtractions. The subsequent pointwise product keeps its usual contiguous traversal. Truncating the initialized workspace changes its length and retains its allocated capacity through the final consumer.

An initial SDK preflight exposed a missing packed pair when the final consumer loaded eight lanes. The measured build rounds its output slice length to whole pairs of SIMD packs. A BN254 differential test compares a full final pass with seven requested prefix lengths and eight pointwise consumer lengths at each of two domain sizes, using cached and ordinary roots. The full SDK screen used the trained 12×2 wire file on all three arms at 32 Ryzen workers. Seven randomized matched blocks on the original, a private split, a data-bearing transfer and a withdrawal used three primes and eleven timed requests per process.

| Request | Release / disabled / prefix ms | Paired vs release | Paired vs disabled | Peak RSS disabled / prefix MiB |
|---:|---:|---:|---:|---:|
| 0 | 31.587 / 31.508 / 31.479 | +0.33% (3/7) | +0.50% (3/7) | 568.3 / 561.0 |
| 57 | 31.587 / 31.637 / 31.622 | +0.48% (3/7) | +0.11% (3/7) | 552.2 / 575.2 |
| 60 | 31.466 / 31.263 / 31.532 | +0.29% (3/7) | +0.33% (2/7) | 568.5 / 565.7 |
| 63 | 31.620 / 31.927 / 31.328 | -0.96% (5/7) | -1.82% (7/7) | 583.0 / 576.3 |

The candidate did not improve all four requests against both controls. The integrated build keeps the previous transform. Gnark independently verified all 84 exports.

The existing key fixes the constraint roots in natural order. The model for this layout found that every final lower output still requires its twiddle product. This trial measures the cost of skips in arithmetic and writeback in that layout. The truncated Fourier transform literature uses selected evaluation points in bit-reversed order. [Existing prefix-dependency study](../2026-11-07-fft-prefix-pruning176/README.md) · [Van der Hoeven and Schost, evaluation at TFT points](https://www.texmacs.org/joris/mmeval/mmeval.pdf)

[Patch](artifacts/source.patch) · [Processes](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
