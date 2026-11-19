# Choosing an equivalent endomorphism split for each G2 scalar

Two GLV components represent the same scalar after translation by a vector in the BN254 endomorphism lattice. This screen enumerates all representations whose two magnitudes fit a given bit limit, using the inverse of the two-by-two lattice matrix to bound both integer indices. Its determinant equals the scalar field modulus. Eleven signed radix-4096 windows suffice throughout. Each scalar selects the representation with the fewest nonzero digits, breaking ties by the smaller absolute top digit. The selected signed integers are reconstructed from those digits and verified against the original scalar.

Each shape uses four independently verified public-fixture exports and 256 wire positions sampled without replacement per request with recorded seeds. The original 126-bit split supplies the baseline at every width. The count gives optimistic savings on sampled scalar digits. It omits search, wider top-window weighting, point grouping, the sign encoding and backend changes required for wider components. Every choice is per request and requires no stored curve points. Work on small-factor scalar recoding and window clustering provided related background. [Hrabar, University of Waterloo](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403)

| Shape | Component magnitude bits | Ordinary / best nonzero digits | Removed | Rows with fewer digits | Largest original / selected top absolute digit | Mean legal representations per row |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 126 | 21,073 / 21,073 | 0.000% | 0/1024 | 56 / 56 | 1.3 |
| 1x2 | 127 | 21,073 / 21,060 | 0.062% | 13/1024 | 56 / 126 | 5.5 |
| 1x2 | 128 | 21,073 / 21,021 | 0.247% | 52/1024 | 56 / 254 | 21.3 |
| 1x2 | 130 | 21,073 / 20,457 | 2.923% | 613/1024 | 56 / 1024 | 339.6 |
| 2x2 | 126 | 21,135 / 21,131 | 0.019% | 4/1024 | 56 / 61 | 1.3 |
| 2x2 | 127 | 21,135 / 21,115 | 0.095% | 20/1024 | 56 / 126 | 5.4 |
| 2x2 | 128 | 21,135 / 21,074 | 0.289% | 61/1024 | 56 / 252 | 21.4 |
| 2x2 | 130 | 21,135 / 20,504 | 2.986% | 629/1024 | 56 / 1023 | 339.3 |

[Per-request counts and proof identities](artifacts/counts.json) · [Integer search and reconstruction](artifacts/screen.py)
