# Reusing a G2 input point across eight MSM windows

Each regular window processes a signed digit for the same input point. A proposed eight-lane update would add that point to eight different window buckets, broadcasting its coordinates and applying a separate sign per lane. The census uses all 30,089 original B-query entries from the retained 1×2 request. It checks every digit against the production Booth extraction and reconstructs each signed endomorphism component. The short top window retains its separate reduction and is excluded from the denominator.

The model groups the first eight regular windows and preserves input order inside each partition. A packet requires eight nonzero digits and eight previously visited buckets. Pending work flushes on a destination conflict, an ineligible row, a component boundary or a full batch. Arithmetic exceptions, identity buckets after cancellation and projective queue handling can change coverage in a real scheduler.

| Width | Parts | Max packets per batch | Regular updates | Updates in full packets | Share of all regular updates | Median packets per flush | Packets in batches of at least 16 |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 12 | 2 | 8 | 558,614 | 356,536 | 63.8% | 8 | 0 |
| 12 | 2 | 16 | 558,614 | 356,536 | 63.8% | 6 | 24,864 |
| 12 | 2 | 32 | 558,614 | 356,536 | 63.8% | 5 | 28,264 |
| 11 | 4 | 8 | 614,254 | 355,208 | 57.8% | 7 | 0 |
| 11 | 4 | 16 | 614,254 | 355,208 | 57.8% | 6 | 16,688 |
| 11 | 4 | 32 | 614,254 | 355,208 | 57.8% | 5 | 19,547 |

At width 12 with two point partitions, the model can put 356,536 of 558,614 regular updates into same-point packets. At a cap of 16 packets, the median flush contains six packets. Smaller inversion batches and fewer independent window tasks could offset cheaper coordinate loads. These counts motivate measuring the full affine update on already formed packets before changing the scheduler. The census adds no key table and supplies no proof-latency measurement.

[Raw counts and input hash](artifacts/counts.json) · [Rust census and synthetic tests](artifacts/census.rs)
