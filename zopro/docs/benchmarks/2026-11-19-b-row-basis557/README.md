# Choosing a B-query basis from existing constraint evaluations

A B-side constraint row containing a wire used nowhere else on the B side supplies a pivot. Replacing the pivot scalar xᵢ by the row value z = f Σcⱼxⱼ can change digit costs even when the number of scalar positions is unchanged. Replace its point by Pᵢ/(fcᵢ) and subtract fcⱼ times that point from each other base in the row. Shared nonpivot bases receive every correction. All changes depend only on the key and the frozen selected rows. A solver already producing the B evaluation could obtain the new scalar from that value and a fixed origin adjustment.

Four public training inputs select at most one pivot per row and either the raw or pivot-normalized evaluation. Selection requires two fewer signed width-12 digits per training request on average relative to its existing frequent origin. Four separate requests per key test this frozen choice. A deterministic field-linear probe checks the full dot product after all simultaneous substitutions on each held-out input. A synthetic control includes two pivots sharing a corrected base and both normalization choices. This model excludes offline point changes, different runtime grouping and the cost of obtaining the selected row evaluations.

| Shape | Held-out request | Eligible / selected rows | Affected key points | Ordinary residual digits | Net digits saved |
|---|---:|---:|---:|---:|---:|
| 1x2 | 2 | 200 / 1 | 10 | 558,271 | 20 (0.004%) |
| 1x2 | 4 | 200 / 1 | 10 | 558,257 | 20 (0.004%) |
| 1x2 | 6 | 200 / 1 | 10 | 558,415 | 20 (0.004%) |
| 1x2 | 8 | 200 / 1 | 10 | 558,331 | 20 (0.004%) |
| 2x2 | 34 | 361 / 1 | 10 | 979,269 | 20 (0.002%) |
| 2x2 | 36 | 361 / 1 | 10 | 979,407 | 20 (0.002%) |
| 2x2 | 38 | 361 / 1 | 10 | 979,495 | 20 (0.002%) |
| 2x2 | 40 | 361 / 1 | 10 | 979,501 | 20 (0.002%) |

All 16 archived source proofs verified independently again. [Counts and proof identities](artifacts/results.json) · [Selection and algebra](artifacts/screen.py)

The checked-in reader accepts the original key image and hash, plus four training and four held-out scalar-vector filenames in request order. Each file holds the canonical 32-byte little-endian scalars. Run `python3 artifacts/screen.py --help` for the arguments.
