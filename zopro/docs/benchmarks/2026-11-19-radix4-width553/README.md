# Independent 12×2 confirmation of the four-input inverse FFT

The initial five-block experiment measured 28.713 ms on the original case. This new screen compares the frozen arithmetic binaries at 28, 30 and 32 workers on three public inputs. All masks retain every physical Ryzen core and distribute additional hardware threads equally between both L3 groups. Selection uses the lowest original-input candidate median. Seven new randomized blocks compare the selected candidate with ordinary arithmetic at the same width, the candidate at 32 workers and the frozen grouped build at 32. All modes use the identical prepared groups and direct scalar view. Each process primes three proofs and times eleven.

The original-input screen selected 28 workers.

| Request | Frozen 32 / ordinary selected / candidate 32 / candidate selected ms | Paired vs ordinary | Paired vs frozen | Selected peak MiB | Original vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 29.672 / 29.581 / 29.427 / 29.743 | +0.12% (3/7) | +0.50% (2/7) | 548.8 | 9.55× / 3.40× |
| 59 | 29.404 / 29.475 / 29.221 / 29.580 | +0.34% (1/7) | +0.52% (2/7) | 568.5 | — |
| 61 | 29.314 / 29.427 / 29.263 / 29.273 | -0.93% (6/7) | -0.06% (4/7) | 524.5 | — |
| 139 | 45.766 / 45.631 / 45.514 / 45.561 | -0.15% (4/7) | +0.34% (2/7) | 965.5 | — |
| 144 | 45.297 / 45.981 / 46.043 / 45.833 | +0.86% (3/7) | +1.18% (0/7) | 965.9 | — |

0/7 original selected-width process means reached 28.4 ms. All 212 exports verified independently. [Samples](artifacts/samples.json.gz) · [Design](artifacts/design.json) · [Selection](artifacts/selection.json) · [Verification](artifacts/verification.json)
