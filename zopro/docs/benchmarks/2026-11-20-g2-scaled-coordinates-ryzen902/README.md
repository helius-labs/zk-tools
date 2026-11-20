# Scaled G2 coordinate finishing on Ryzen

The borrowed G2 stream computes affine additions with eight-lane norm inversion. This experiment keeps the original point storage, denominator records, inversion, scalar tails and exceptional cases. It scales the reverse seed once per batch, giving all slopes in that reverse pass a common factor. A uniform-radix product returns one sixteenth of the ordinary host-radix product. The two layouts use that relation to evaluate the same affine coordinates.

- `four` multiplies the slope by four, so the uniform-radix square contributes the ordinary X coordinate directly. It multiplies the X difference by four before computing Y.
- `sixteen` multiplies the slope by sixteen, so the uniform-radix product for Y is directly correct. It divides the square by sixteen modulo the field prime while the result is still packed.

No digit buffer, retained point representation or key table is added. The field test compares 32,768 divided lanes with scalar field results. Three native borrowed-kernel tests cover the new layouts and the original signed path, including full and partial groups, reused scratch and changing signs. Route probes cover both scaling modes on every input and the ordinary fallback when the borrowed prefix path is disabled.

All 207 exported proofs independently verified. Ryzen uses 24 main and eight frontend logical CPUs. On 4×2 every timed mode uses the grouped Phi option so that the borrowed route is comparable. Eight blocks on each input time 17 proofs after three primes. Advancement requires a lower paired median and six wins against both Zopro controls on both inputs. AWS authentication was expired before this trial.

- 1×2 paired changes against ordinary were `four` +0.12% / -0.10% and `sixteen` +0.25% / -0.34%. Neither profile qualified.
- 2×2 paired changes against ordinary were `four` +0.32% / -0.08% and `sixteen` +0.43% / -0.27%. Neither profile qualified.
- 4×2 paired changes against ordinary were `four` +0.14% / +0.68% and `sixteen` +0.26% / +0.62%. Neither profile qualified.

| Shape | Input | Profile | Proof ms | Paired vs release | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.866 | +0.00% (0/8) | -0.32% (5/8) | 58.5 |
| 1x2 | catalog | ordinary | 6.851 | +0.32% (3/8) | +0.00% (0/8) | 58.2 |
| 1x2 | catalog | four | 6.881 | +0.48% (2/8) | +0.12% (3/8) | 58.0 |
| 1x2 | catalog | sixteen | 6.860 | -0.08% (4/8) | +0.25% (3/8) | 58.0 |
| 1x2 | heldout | release | 6.799 | +0.00% (0/8) | +0.26% (3/8) | 58.0 |
| 1x2 | heldout | ordinary | 6.797 | -0.26% (5/8) | +0.00% (0/8) | 57.5 |
| 1x2 | heldout | four | 6.790 | +0.21% (4/8) | -0.10% (4/8) | 58.2 |
| 1x2 | heldout | sixteen | 6.779 | -0.10% (4/8) | -0.34% (5/8) | 58.0 |
| 2x2 | catalog | release | 9.491 | +0.00% (0/8) | +0.22% (4/8) | 95.8 |
| 2x2 | catalog | ordinary | 9.479 | -0.22% (4/8) | +0.00% (0/8) | 96.2 |
| 2x2 | catalog | four | 9.515 | +0.36% (2/8) | +0.32% (2/8) | 94.2 |
| 2x2 | catalog | sixteen | 9.485 | -0.02% (4/8) | +0.43% (2/8) | 95.0 |
| 2x2 | heldout | release | 9.520 | +0.00% (0/8) | -0.21% (5/8) | 95.0 |
| 2x2 | heldout | ordinary | 9.504 | +0.21% (3/8) | +0.00% (0/8) | 95.0 |
| 2x2 | heldout | four | 9.499 | -0.22% (4/8) | -0.08% (4/8) | 93.8 |
| 2x2 | heldout | sixteen | 9.484 | -0.34% (6/8) | -0.27% (6/8) | 94.8 |
| 4x2 | catalog | release | 13.312 | +0.00% (0/8) | +0.73% (2/8) | 211.2 |
| 4x2 | catalog | ordinary | 13.217 | -0.72% (6/8) | +0.00% (0/8) | 219.2 |
| 4x2 | catalog | four | 13.215 | -0.80% (6/8) | +0.14% (3/8) | 210.8 |
| 4x2 | catalog | sixteen | 13.243 | -0.58% (6/8) | +0.26% (2/8) | 213.5 |
| 4x2 | heldout | release | 13.168 | +0.00% (0/8) | -0.56% (5/8) | 218.2 |
| 4x2 | heldout | ordinary | 13.231 | +0.56% (3/8) | +0.00% (0/8) | 225.8 |
| 4x2 | heldout | four | 13.323 | +1.28% (1/8) | +0.68% (2/8) | 221.2 |
| 4x2 | heldout | sixteen | 13.299 | +0.93% (2/8) | +0.62% (3/8) | 216.0 |

[Source](artifacts/source.patch) · [SDK samples and route traces](artifacts/samples.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

The [recorded C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md) reaches 10× M5 CPU on **5/8** original requests.

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
