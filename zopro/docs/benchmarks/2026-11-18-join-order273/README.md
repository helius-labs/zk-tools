# Reordering the G1 and G2 branches on 1×2

Starting the G2 branch on the calling worker improved median paired 16-thread full-proof latency by 3.34% across three local Ryzen blocks. It increased latency by 2.01% at 32 threads, with no wins there. Starting K/H on the G1 branch’s calling worker gave mixed results. Both same-binary controls calibrated to the exact experimental parent within a factor of 1.03.

| Threads | Order | Mean ms | P95 ms | CPU ms/proof | Peak MiB | Paired wall change vs OFF | Wins |
|---:|---|---:|---:|---:|---:|---:|---:|
| 16 | Exact 257 | 8.616 | 9.393 | 105 | 56 | -2.08% | 2/3 |
| 16 | Original (OFF) | 8.767 | 9.521 | 106 | 56 | +0.00% | 0/3 |
| 16 | K/H first | 8.833 | 9.546 | 107 | 73 | -0.93% | 2/3 |
| 16 | G2 first | 8.752 | 9.425 | 105 | 69 | -3.34% | 3/3 |
| 32 | Exact 257 | 7.536 | 7.961 | 167 | 63 | +1.26% | 1/3 |
| 32 | Original (OFF) | 7.442 | 7.901 | 166 | 64 | +0.00% | 0/3 |
| 32 | K/H first | 7.432 | 7.775 | 168 | 100 | +0.63% | 1/3 |
| 32 | G2 first | 7.594 | 9.066 | 168 | 98 | +2.01% | 0/3 |

The same three queries use the original Rayon pool, scalar buffers and curve routines. The flags explicitly select the left and right sides of two joins only for certified queries. All local configurations explicitly set both diagnostic flags to zero or one. Their shared native feature helper defaults to enabled on the IFMA target when a flag is unset. This prototype requires an explicit selection and has no production default.

The 1×2 panel used three primes and 31 timed requests per process. Four additional 2×2 processes checked the uncertified fallback. All 28 exported proofs verified, and the native tests and generic compile check passed. At 32 threads, the original schedule measured 7.442 ms or 4.97× the supplied M5 CPU speed and 3.22× Metal. The local comparison does not establish a C7a scheduling policy. [Paired blocks](artifacts/cells.json) · [Source changes](artifacts/changes.patch.gz) · [Verification](artifacts/verification.json)
