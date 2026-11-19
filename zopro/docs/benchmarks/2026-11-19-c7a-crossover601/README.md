# Where the 96-core C7a comparison changes winner

In the archived full catalog, Aeglos had lower latency on every one of the 19 transfers with one through five inputs at 96 physical cores. Zopro had lower latency on every one of the 16 transfers with eight or more inputs. The six-input cases split: 6×2 favored Zopro, while 6×4 and 6×8 had inconclusive intervals. The three merges fall outside this transfer count. At 32 cores Zopro led on every transfer and merge. These statements apply to the measured executables and requests. A newer focused 1×2 candidate subsequently narrowed the AWS gap. [Newer 1×2 run](../2026-11-18-overlap-c7a369/README.md)

Among transfer shapes with opposite interval verdicts and the same FFT domain, 5×16 and 6×2 are closest in constraint count (4.60% apart). The following arithmetic means include all 63 warmed calls per engine and shape at 96 cores. Moving from 6×2 to 5×16 cost Zopro 3.299 ms and Aeglos 0.342 ms, of which 0.247 ms fell within Aeglos’s MSM. Both ran in a 262,144-point FFT domain. A matched phase profile on these two keys can distinguish extra solver, transformation and curve work from a change in domain length. These whole-proof differences alone do not localize the Zopro cost.

| Transfer | Constraints | B2 points | FFT domain | Zopro ms | Aeglos ms | Aeglos MSM ms |
|---|---:|---:|---:|---:|---:|---:|
| 5x16 | 150,746 | 145,951 | 262,144 | 30.008 | 27.865 | 19.572 |
| 6x2 | 144,114 | 139,420 | 262,144 | 26.709 | 27.523 | 19.325 |
| 6x4 | 147,858 | 143,058 | 262,144 | 27.470 | 27.466 | 19.329 |
| 6x8 | 156,594 | 151,558 | 262,144 | 28.903 | 28.257 | 20.069 |

For each input count with multiple output widths, compare its largest output width with its smallest within the same recording block. The change is the median of nine ratios of Zopro’s latency growth to Aeglos’s latency growth, expressed as a percentage relative to one. Positive values mean increasing output width cost Zopro proportionally more. The last column counts positive blocks. Verdicts use the published marginal paired interval for each individual shape. Input counts 32, 40, 48 and 49 each had one output width and favored Zopro.

| Inputs | Output widths | Zopro / Aeglos / inconclusive | Relative change, smallest to largest width | Positive blocks |
|---:|---|---:|---:|---:|
| 1 | 2, 4, 8, 16 | 0 / 4 / 0 | +11.08% | 9/9 |
| 2 | 2, 4, 8, 16 | 0 / 4 / 0 | -1.60% | 3/9 |
| 3 | 2, 4, 8 | 0 / 3 / 0 | +1.80% | 7/9 |
| 4 | 2, 4, 8, 16 | 0 / 4 / 0 | +5.02% | 8/9 |
| 5 | 2, 4, 8, 16 | 0 / 4 / 0 | -3.10% | 0/9 |
| 6 | 2, 4, 8 | 1 / 0 / 2 | +5.22% | 9/9 |
| 8 | 2, 4, 8, 16 | 4 / 0 / 0 | +4.65% | 7/9 |
| 12 | 2, 4, 8 | 3 / 0 / 0 | +0.25% | 6/9 |
| 16 | 2, 4, 8 | 3 / 0 / 0 | +0.74% | 6/9 |
| 24 | 2, 4 | 2 / 0 / 0 | +1.73% | 7/9 |

The reader reconstructs both endpoint ratios of each published interval and the median for all 82 comparisons from 1,476 timed processes. It checks complete process means against the separately aggregated stage data. Key dimensions come from the 41 original Euler key headers. Each header digest and the image identity pinned by the archived runner are in the results. This analysis uses previously verified proofs and adds no new AWS timing. [Reader](artifacts/crossover.py) · [Per-shape data](artifacts/results.json) · [Full catalog](../2026-11-18-full-catalog-c7a244/README.md)

A separate full-file SHA-256 pass on all 41 local images matched the archived catalog configuration. The 5×16 and 6×2 requests also produced different numbers of distinct scalars in the same curve-query code. [Verified group counts](../2026-11-19-crossover-groups603/README.md)
