# Signed G2 scalar groups across different input coins and Merkle trees

The discovery request supplies groups of row positions with equal or opposite nonzero scalars. Eight new 1×2 requests use different input coin amounts and blindings, output amounts and seeds, state leaf indices, 24 populated sibling subtrees, and independently populated nullifier trees. The signed groups come from the original discovery request in experiment 416. A group qualifies only if every row still matches its nominated representative with the original sign. On any mismatch the model keeps all original rows. An arbitrary witness can therefore be checked before using a prepared point sum. The row-by-row reconstruction matched the input scalar vector on every request.

| Request | Valid groups | Fallback groups | Rows removable | Same / opposite sign rows removed | Nonzero window updates avoided |
|---|---:|---:|---:|---:|---:|
| 0 | 926 | 0 | 2350 | 2338 / 12 | 39,575 / 613,891 (6.45%) |
| 1 | 554 | 372 | 862 | 852 / 10 | 18,835 / 614,253 (3.07%) |
| 2 | 554 | 372 | 862 | 852 / 10 | 18,835 / 614,206 (3.07%) |
| 3 | 554 | 372 | 862 | 852 / 10 | 18,834 / 614,368 (3.07%) |
| 4 | 554 | 372 | 862 | 852 / 10 | 18,835 / 614,200 (3.07%) |
| 5 | 554 | 372 | 862 | 852 / 10 | 18,835 / 614,349 (3.07%) |
| 6 | 554 | 372 | 862 | 852 / 10 | 18,835 / 614,364 (3.07%) |
| 7 | 554 | 372 | 862 | 852 / 10 | 18,835 / 614,452 (3.07%) |
| 8 | 554 | 372 | 862 | 852 / 10 | 18,834 / 614,182 (3.07%) |

The discovery plan has 926 groups and 2,350 nonrepresentative rows. A layout with one 128-byte affine sum per group, one 32-bit signed row index per member, and 32-bit offsets would retain 135,340 bytes before allocator overhead. The estimate excludes any changed query layout, validation scratch and preparation workspace.

The columns count signed nonzero window contributions before bucket collisions, scheduling and reductions. This is a feasibility measurement. Each new request independently verified through Gnark against the new public witness. A debugger sampled the unmodified executable at the G2 input on all three warmups and the timed proof. The scalar and base digests were identical across the four observations for each request, and all nine point arrays matched. No traced wall times are compared. The captured scalars and individual group membership remain local.

2,298 of 30,089 scalar positions had exactly the same value in all nine requests. Of 554 groups valid throughout, 270 had a representative whose value actually changed. These fixtures extend the input variation while preserving the same circuit, owner configuration and transfer shape. A circuit-derived group or a validation on every use would be needed for a correct implementation. No witness-derived cache was installed.

[Aggregate results](artifacts/counts.json) · [Verification](artifacts/verification.json) · [Fixture generator](artifacts/tree418_requests_test.go) · [Original witness panel](../2026-11-19-g2-varied-groups416/README.md)
