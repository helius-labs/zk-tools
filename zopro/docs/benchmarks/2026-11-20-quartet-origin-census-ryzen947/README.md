# Poseidon state comparisons for prepared 4×2 quartets

A prepared quartet lets the K and B2 queries use one summed point when their four coefficients are equal. The prior [compaction prototype](../2026-11-20-compacted-quartets-ryzen937/README.md) checks the four coefficients at each quartet. This probe tests how often one comparison of complete Poseidon states can certify many such quartets.

The observer uses the original prepared quartet records and validates that each B2 index projects to its exact private K index. Where all four positions have a recognized origin at the same width and round coordinate, it checks all first-round state values. One result is shared across output coordinates from the same four spans. Every certified equality is compared with the actual solved coefficients. Two independent processes per input produced identical counts, and all four exported proofs passed independent Gnark verification.

| Input | Prepared quartets | Equal nonzero | Origin-compatible | Certified nonzero | Span tuples | Largest tuple | Fallbacks |
|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | 17,611 | 17,611 | 14,394 | 14,394 | 64 | 225 | 3,217 |
| heldout | 17,611 | 16,903 | 14,394 | 14,394 | 64 | 225 | 3,217 |

| Input | Direct scalar field comparisons | Full-state field comparisons | With fallback upper bound |
|---|---:|---:|---:|
| catalog | 52,833 | 576 | 10,227 |
| heldout | 52,329 | 576 | 10,227 |

An independent scan of the earlier coefficient captures reproduced the direct counts. The held-out input has 16,903 equal nonzero quartets, including six inside larger scalar classes. The earlier figure of 16,897 counted exact four-member classes.

The bound adds up to three direct equality comparisons for each fallback to the observed state comparisons. It excludes indexing, memory traffic, zero checks, point preparation and MSM arithmetic. The probe adds no full-proof timing or score credit.

[Counts and proof binding](artifacts/counts.json) · [Protocol](artifacts/protocol.json) · [Experimental observer](artifacts/source.patch.gz)
