# Server precomputation: shared G2 bucket set

The first Rust all-window shared-bucket component is slower on this Ryzen host.
It precomputes every radix-4096 shift, routes signed contributions into one
bucket set, then uses the existing indexed and weighted reductions. This differs
from the closed shift64 quarter-window trials. No production server API or
complete memory-budget guarantee is introduced.

| Query | Workers | Current query ms | Shared buckets ms | Paired ratio | Wins |
|---|---:|---:|---:|---:|---:|
| 1x2 | 16 | 3.648917 | 9.849463 | 2.727307 | 0/9 |
| 1x2 | 32 | 3.313815 | 9.276804 | 2.832186 | 0/9 |
| 2x2 | 16 | 4.559133 | 17.051126 | 3.736833 | 0/9 |
| 2x2 | 32 | 4.237206 | 16.006307 | 3.846558 | 0/9 |

Three primes per mode precede nine alternating pairs in one native binary.
Timers include word copying, recoding, counting, scattering and weighted
projective reduction. Final affine normalization and oracle checking occur
after timing. These are G2 components, not full-proof timings.
The 1x2 table retains 84,730,624 bytes and takes 135.55 ms setup at 32 workers;
2x2 retains 147,203,584 bytes and takes 236.16 ms. Whole-process peak RSS is
204892 KiB across all contexts. CPU includes setup/all cases; per-context CPU
was not measured. Routing/reducer scratch is additional to table payload.

Every result matches a retained independent Arkworks full-query oracle, with
fixture hashes checked. Edge cases cover identity, duplicate, inverse and
full-width signed scalars; four OS callers share an immutable table. Both
correctness tests pass. Initial result parsing missed a libtest-prefixed first
line; the reader was fixed and the retained capture parsed without rerunning.
No new proofs; canonical56585 remains. The table allowance rejects insufficient
capacity before allocation, but does not enforce total setup/concurrent memory.

This exact serial-routing implementation fails its screen. Source inspection
shows two serial scans of 22 x n contributions before parallel reduction. Next
test parallel routing against this control and measure routing/reduction spans;
no width sweep. This local result does not close server precomputation or C7a.
No production change, AWS resource or public push. Private root:
`/home/ubuntu/src/zopro-perf-work/server-shifted183`.
