# Matching a second public scalar at a B2 wire

The existing frequent-offset rule uses a scalar origin per selected wire. The proposed extension prepares the point (v−origin)P for a second recurring public scalar v. On equality the prover can remove that residual scalar from the ordinary MSM and add the prepared point once. On any other scalar it uses the ordinary residual. A candidate is selected only when the second value occurs on at least two training requests and costs three or more nonzero signed width-12 digits. No witness data from the evaluated requests enters selection.

All matches are charged one additional point addition in the count below. Existing runtime grouping can share some of the modeled ordinary updates. The model excludes equality checks, reading indices and points, aggregation overhead, preparation and memory admission. Payload counts one affine G2 point, one scalar and one u32 index per record. The conditional path is exact for arbitrary field inputs.

| Shape | Held-out request | Selected records | Payload bytes | Actual matches | Ordinary residual digits | Modeled net updates avoided |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 2 | 8 | 1,312 | 4 | 558,271 | 84 (0.015%) |
| 1x2 | 4 | 8 | 1,312 | 4 | 558,257 | 84 (0.015%) |
| 1x2 | 6 | 8 | 1,312 | 0 | 558,415 | 0 (0.000%) |
| 1x2 | 8 | 8 | 1,312 | 8 | 558,331 | 168 (0.030%) |
| 2x2 | 34 | 23 | 3,772 | 8 | 979,269 | 168 (0.017%) |
| 2x2 | 36 | 23 | 3,772 | 15 | 979,407 | 315 (0.032%) |
| 2x2 | 38 | 23 | 3,772 | 0 | 979,495 | 0 (0.000%) |
| 2x2 | 40 | 23 | 3,772 | 23 | 979,501 | 483 (0.049%) |
| 12x2 | 0 | 142 | 23,288 | 139 | 5,078,583 | 2,907 (0.057%) |
| 12x2 | 57 | 142 | 23,288 | 103 | 5,084,157 | 2,163 (0.043%) |
| 12x2 | 59 | 142 | 23,288 | 48 | 5,085,192 | 1,008 (0.020%) |
| 12x2 | 64 | 142 | 23,288 | 142 | 5,082,042 | 2,969 (0.058%) |
| 12x2 | 141 | 142 | 23,288 | 7 | 5,103,215 | 147 (0.003%) |
| 12x2 | 142 | 142 | 23,288 | 0 | 5,103,075 | 0 (0.000%) |
| 12x2 | 143 | 142 | 23,288 | 0 | 5,102,300 | 0 (0.000%) |
| 12x2 | 144 | 142 | 23,288 | 10 | 5,103,890 | 209 (0.004%) |

All archived source exports passed independent proof verification again. This is an operation-count screen. [Counts and proof identities](artifacts/counts.json) · [Selection and evaluation](artifacts/screen.py)
