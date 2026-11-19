# Fixed public offsets from two endomorphism scalar components

This screen uses the signed width-12 decomposition from the earlier 1×2 B2 study. Each candidate offset combines independently stable signed digits or frequent complete halves using the BN254 eigenvalue. For each input the scanner subtracts the chosen offset modulo the scalar field, decomposes that exact residual again and verifies scalar reconstruction. It compares ordinary coefficients, exact whole coefficients fixed on all training inputs, and whole coefficients present in at least three quarters of training inputs. A new offset for a varying row must save at least two nonzero training digits per witness on average. All chosen offsets are frozen before the held-out inputs are read for scoring. Five 1×2 inputs trained the first plan and four alternated 2×2 inputs trained the second. The 2×2 counts describe that same candidate G2 recoding; other runtime grouping decisions can change the executed work.

| Shape | Held-out request | Ordinary / fixed / frequent whole / combined nonzero digits | Additional saving over frequent whole as % of ordinary |
|---|---:|---:|---:|
| 1x2 | 5 | 614,349 / 592,802 / 558,422 / 558,422 | +0.000% |
| 1x2 | 6 | 614,364 / 592,815 / 558,436 / 558,436 | +0.000% |
| 1x2 | 7 | 614,452 / 592,903 / 558,523 / 558,523 | +0.000% |
| 1x2 | 8 | 614,182 / 592,707 / 558,239 / 558,239 | +0.000% |
| 2x2 | 34 | 1,065,593 / 979,269 / 979,269 / 979,269 | +0.000% |
| 2x2 | 36 | 1,065,635 / 979,407 / 979,407 / 979,407 | +0.000% |
| 2x2 | 38 | 1,065,783 / 979,495 / 979,495 / 979,495 | +0.000% |
| 2x2 | 40 | 1,065,553 / 979,501 / 979,501 / 979,501 | +0.000% |

The scanner also checks a synthetic family with one fixed half, both signs of recoded digits, and exact modular identities. All 17 archived request exports verified again. No new SDK timing is inferred from digit counts. [Scanner](artifacts/screen.py) · [Counts and fixture identities](artifacts/counts.json)
