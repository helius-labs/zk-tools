# FFT work available before the last solver band

No complete radix-32 column is available before the last band on 1×2, 2×2 or 4×2 in the modeled four-band schedules. Computing each A or B expression as soon as its dependencies are ready makes no complete radix-32 column available earlier. The existing radix-32 kernel therefore has no whole column to move into an earlier band under these schedules.

The reader uses the pinned 898 release images for 1×2 and 2×2 and the 854 seeded profile for 4×2. It replays the operation and stored-sum dependencies and applies the profiles’ band assignment. Padded rows are available from the start. A packet is four adjacent butterfly pairs with both A and B available. Within a radix-32 column, a subset can finish an early layer while later layers wait for the other input stripes.

| Shape and schedule | Layer 1 | Layer 2 | Layer 3 | Layer 4 | Layer 5 | All early packets as a share of inverse packets |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 packed | 2,840 | 1,584 | 420 | 0 | 0 | 7.88% |
| 2×2 packed | 6,192 | 4,192 | 548 | 0 | 0 | 8.34% |
| 4×2 packed | 12,389 | 11,098 | 5,812 | 0 | 0 | 10.52% |
| 4×2 unpacked | 13,408 | 12,022 | 7,660 | 984 | 0 | 12.23% |

The counts include every logical inverse layer in the denominator. They include unit roots and do not weight butterflies by multiplication cost. Forward transforms, quotient products, extra stores and dispatch are outside this count. The 4×2 packed scan recovers 89, 124 and 131 new radix-4 tiles in the first three bands, matching the route probes in the [earlier Ryzen proof trial](../2026-11-20-solver-radix32-resume-ryzen880/README.md). That trial measured small and inconsistent gains from moving those precomputed tiles earlier.

The separate-expression model assumes an A or B expression can be recomputed for free once its last external operand has been written. It retains both branches of an A- or B-side assignment when finding dependencies. It moves both operands earlier on 105, 132 and 197 real rows for 1×2, 2×2 and packed 4×2, respectively. The paired packet totals remain identical. Allowing each channel to run alone adds at most 3, 10 and 15 packets, respectively, before the final band. The observed limit applies to the selected outer bands. This model does not estimate timing or readiness within a band.

A separate calculation used the earlier solver-image parser and direct row-stride formulas to reproduce all 60 paired stage and tile records. It matched all 30 comparable tile records from the [earlier scan](../2026-11-20-solver-fft-readiness878/README.md). The separate-expression results use the new reader. This experiment ran offline with no new proof measurements. The published [M5 CPU and Metal scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) remains at **5 of 8** shapes at 10× M5 CPU.

[Reader](artifacts/frontier.rs) · [Input pins](artifacts/pins.json) · [1×2 counts](artifacts/1x2.csv) · [2×2 counts](artifacts/2x2.csv) · [4×2 counts](artifacts/4x2.csv) · [4×2 unpacked counts](artifacts/4x2-unpacked.csv) · [Verification](artifacts/verification.json)
