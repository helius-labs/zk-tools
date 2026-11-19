# Frequent coefficients and stable signed digits on 12×2

Eight public training transactions determined candidate offsets for rows outside the accepted 20,488. A complete coefficient qualified for consideration when it appeared in at least six inputs. A partial coefficient combined width-13 Booth digits shared by all eight inputs, including their incoming sign bits. Selection required an average saving of at least two nonzero digits on the training inputs after modular subtraction. The resulting file added 51 whole coefficients and zero partial coefficients, for an extra 1,836 bytes. The training model counted 885 additional nonzero digits removed per input.

The file was frozen before the original and eight held-out requests were timed. Both configurations ran the same accepted checkout binary on 32 Ryzen workers. Seven randomized matched blocks used three primes and eleven timed requests per process.

| Input | Stable file ms | Extended file ms | Paired change (wins) |
|---:|---:|---:|---:|
| 0 | 31.762 | 31.584 | -0.35% (5/7) |
| 57 | 31.578 | 31.783 | +1.07% (3/7) |
| 58 | 31.691 | 31.549 | -0.36% (4/7) |
| 59 | 31.549 | 31.539 | -0.39% (4/7) |
| 60 | 31.497 | 31.620 | +0.32% (3/7) |
| 61 | 31.668 | 31.632 | -0.58% (5/7) |
| 62 | 31.551 | 31.641 | +0.13% (3/7) |
| 63 | 31.528 | 31.338 | -0.60% (4/7) |
| 64 | 31.546 | 31.343 | -0.45% (4/7) |

Across the eight held-out inputs the median paired change was -0.37% and 5/8 inputs favored the extension. On the original request it was -0.35%. The file did not qualify for adoption. All 126 process exports verified with Gnark.

[Selection](artifacts/selection.json) · [Measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
