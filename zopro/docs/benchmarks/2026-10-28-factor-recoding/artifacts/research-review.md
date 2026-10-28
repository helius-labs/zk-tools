# Source and history review

Primary source: Oleksandr Hrabar, *On the Performance of Pippenger's algorithm for Multi-Scalar Multiplication*, University of Waterloo, 2026.

- Landing page: https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403
- PDF: https://uwspace.uwaterloo.ca/bitstreams/697c043d-622c-43a8-bab0-90ce10b68c4a/download
- Selected material read: chapter 4, including algorithms 3 and 4 and the 5/3/2 priority rule; selected clustering sections and abstract. This is not a claim to have read or reproduced the entire thesis.
- `thesis-sha256.txt` records the locally fetched PDF. No PDF or extracted full text is included in this report.

Our BN254 screen uses the accepted native overlapping Booth recoder rather than assuming the thesis's curve, implementation, window bounds or operation counts transfer unchanged. The exact tests use independently implemented integer and Arkworks group references. The synthetic small-scalar fixture intentionally tests a case in which order-complement recoding can expand magnitude; it is not presented as an actual prover distribution.

The preceding queue-drain idea was set aside after the history review found explicit preserved negative scanning and early-drain results in `2026-10-22-universal-msm-diagnostics/README.md` and the retained queue audit. `2026-10-25-shared-point-window-queue/README.md` separately records negative bounded shared-window queue results. No queue implementation or repeat benchmark was performed in this study.

The prior `2026-10-21-ryzen-research/global-window-search-floor/README.md` rejected a serialized all-window-pair search: even its search-only optimistic floor exceeded the retained native G2 MSM. That does not reject every clustering method, but prevents treating unchanged global pair search as an untested next step. This study does not rerun that search, point-tuple tables, large precomputation or prior paired/triple affine arithmetic kernels.

Signed centering is a standard comparison in this screen, not a novelty claim. The report corpus search found no exact matching report name or phrase, which is not an exhaustive source-history proof. A native implementation must first audit existing signed-magnitude and top-window behavior and inspect real scalar distributions. No universal, native, full-proof or AWS advantage is established here.
