# Scalar grouping admission across inputs, worker counts and public offsets

The diagnostic records the actual return of the common scalar grouping function alongside the number of distinct nonzero values. It covers both the ordinary and diversity-checked callers. It holds one key and three public transactions fixed while toggling the public wire offsets file. Runs with 16 workers used four frontend workers. Runs with 96 workers used sixteen frontend workers and intentionally oversubscribed the same local sixteen physical cores to exercise width-dependent code paths. No timings are compared. The diagnostic allocates a separate hash set after each decision to count uniqueness. Its memory use is specific to this test. The independent verifier checked all twelve proof exports.

| Input | Main workers | Offsets | Query length | Zeros | Distinct nonzero | Admitted classes | Calls per proof |
|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | 16 | 1 | 21,006 | 2,551 | 17,234 | 0 | 1 |
| catalog | 16 | 1 | 30,089 | 2,768 | 25,816 | 0 | 1 |
| catalog | 96 | 1 | 21,006 | 2,551 | 17,234 | 0 | 1 |
| catalog | 96 | 1 | 30,089 | 2,768 | 25,816 | 0 | 1 |
| catalog | 16 | 0 | 21,006 | 1,733 | 17,454 | 0 | 1 |
| catalog | 16 | 0 | 30,089 | 1,600 | 26,142 | 0 | 1 |
| catalog | 96 | 0 | 21,006 | 1,733 | 17,454 | 0 | 1 |
| catalog | 96 | 0 | 30,089 | 1,600 | 26,142 | 0 | 1 |
| heldout | 16 | 1 | 21,006 | 2,371 | 17,800 | 0 | 1 |
| heldout | 16 | 1 | 30,089 | 2,585 | 26,650 | 0 | 1 |
| heldout | 96 | 1 | 21,006 | 2,371 | 17,800 | 0 | 1 |
| heldout | 96 | 1 | 30,089 | 2,585 | 26,650 | 0 | 1 |
| heldout | 16 | 0 | 21,006 | 1,604 | 18,082 | 0 | 1 |
| heldout | 16 | 0 | 30,089 | 1,476 | 27,060 | 0 | 1 |
| heldout | 96 | 0 | 21,006 | 1,604 | 18,082 | 0 | 1 |
| heldout | 96 | 0 | 30,089 | 1,476 | 27,060 | 0 | 1 |
| v27 | 16 | 1 | 21,006 | 2,385 | 17,800 | 0 | 1 |
| v27 | 16 | 1 | 30,089 | 2,599 | 26,650 | 0 | 1 |
| v27 | 96 | 1 | 21,006 | 2,385 | 17,800 | 0 | 1 |
| v27 | 96 | 1 | 30,089 | 2,599 | 26,650 | 0 | 1 |
| v27 | 16 | 0 | 21,006 | 1,612 | 18,082 | 0 | 1 |
| v27 | 16 | 0 | 30,089 | 1,484 | 27,060 | 0 | 1 |
| v27 | 96 | 0 | 21,006 | 1,612 | 18,082 | 0 | 1 |
| v27 | 96 | 0 | 30,089 | 1,484 | 27,060 | 0 | 1 |

[Counts](artifacts/counts.json) · [Source](artifacts/source.patch) · [Proof verification](artifacts/verification.json)
