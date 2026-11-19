# Training fixed point pairs from shared signed digits

Scalar redundancy elimination exploits repeated nonzero digits between scalars in an MSM. This experiment asks whether public training requests can predict a small, reusable set of point pairs for future witnesses. Each selected pair would store P+Q or P−Q. Matching signed digits in an endomorphism component can use the corresponding point sum. An unequal digit retains the original two arrivals. Matching digits of zero save no arrivals. The method adds no online pair search. The original SRE work searches within each input. [Ku et al., Scalar Redundancy Elimination](https://arxiv.org/abs/2610.27452)

The screen uses the same frozen width-12 signed BN254 G2 decomposition and verified inputs as the partial-GLV study. Each candidate row must show at least three distinct training values. Equal or opposite whole-scalar training vectors contribute one representative. For each of the 22 digit positions, signed equality must hold across every training input and be nonzero in a majority. The last four matching partners in source order are considered. Candidate edges are ranked by training arrivals and chosen disjointly. Held-out whole-pair equalities are excluded from the partial saving. The denominator is the ordinary nonzero digit count before grouping. Other forms of existing runtime grouping, saved point construction, cache effects and latency are outside the model.

| Shape | Request | Eligible rows / prepared pairs | Partial arrivals saved / ordinary | Saved | Pairs with one or more / two or more matches | Matches excluded as whole-scalar pairs |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 5 | 25,341 / 1 | 0 / 614,349 | 0.000% | 0 / 0 | 22 |
| 1x2 | 6 | 25,341 / 1 | 0 / 614,364 | 0.000% | 0 / 0 | 22 |
| 1x2 | 7 | 25,341 / 1 | 0 / 614,452 | 0.000% | 0 / 0 | 22 |
| 1x2 | 8 | 25,341 / 1 | 0 / 614,182 | 0.000% | 0 / 0 | 22 |
| 2x2 | 34 | 27,248 / 8 | 21 / 1,065,593 | 0.002% | 2 / 1 | 0 |
| 2x2 | 36 | 27,248 / 8 | 21 / 1,065,635 | 0.002% | 2 / 1 | 0 |
| 2x2 | 38 | 27,248 / 8 | 22 / 1,065,783 | 0.002% | 4 / 1 | 0 |
| 2x2 | 40 | 27,248 / 8 | 21 / 1,065,553 | 0.002% | 2 / 1 | 0 |

Seventeen archived proof exports verified against the original key and public input again. [Counts and fixture identities](artifacts/counts.json) · [Screen](artifacts/screen.py) · [Signed decomposition](artifacts/zp_glv513.py)
