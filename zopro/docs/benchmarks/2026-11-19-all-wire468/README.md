# Fixed public coefficients outside the B2 query on 12×2

Across eight public training inputs, the full solved vector had 275,342 original wires and 87 additional private slots for the folded key. The existing public file already covers 20,488 stable nonzero coefficients shared with B2. The additional search found 1,238 stable nonzero coefficients, all feeding both A and private K. None came from a folded slot. A coefficient qualified only if its exact nonzero field value matched across all eight inputs.

| Positions | Stable nonzero rows | Nonzero width-12 signed digits in the stable coefficients |
|---|---:|---:|
| B2 positions | 20,488 | 409,607 |
| Additional shared A/K positions | 1,238 | 2,189 |

The extra rows averaged 1.77 signed digits each. Extending the file format would add at least 44,568 index-and-coefficient bytes and target those digits in two G1 queries. The counts describe coefficient sparsity. They exclude point operations, zero handling and scheduling. This screen provides no full-proof latency estimate. The existing format was retained.

The diagnostic accepted only the eight public training fixtures, captured the expanded solver output once per process and evaluated rows in blocks of 4,096. All eight exported proofs passed independent Gnark verification.

[Coverage](artifacts/coverage.json) · [Verification](artifacts/verification.json)
