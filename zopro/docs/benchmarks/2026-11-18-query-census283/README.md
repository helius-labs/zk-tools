# Identifying the last raw curve query on C7a

On merge24, H finished last among the four raw calls in 80 of 93 proofs at 48 workers and 92 of 93 at 64. On 16×2 at 48 workers the last return was split among H, the grouped G1 calls and G2.

| Shape | Workers | H last | Grouped G1 last | G2 last | Mean H call ms | Mean time H continued after other raw calls, ms |
|---|---:|---:|---:|---:|---:|---:|
| merge-24x1 | 48 | 80 | 2 | 11 | 28.85 | 1.84 |
| merge-24x1 | 64 | 92 | 1 | 0 | 30.31 | 4.27 |
| 16x2 | 48 | 35 | 15 | 43 | 21.83 | 0.50 |
| 16x2 | 64 | 66 | 8 | 19 | 22.19 | 1.87 |

Each row uses the three qualified 31-proof AWS traces from [experiment 279](../2026-11-18-general-width279/README.md). The final column counts zero when another query returned later. It estimates how much time a faster H could remove with other raw call times held fixed. Contention, grouping, joins, and verification can also change elapsed time.

The accepted executable’s G1 raw entry is at ELF address 0x1d33d0. A return to 0x4ae364 follows the direct MSM call on Montgomery H in the Rayon closure. The other two G1 calls return to 0x1d3315 in the scalar grouping implementation. The decoder subtracts each process’s relocation and matches returns and entries by thread. It requires exactly one H, two grouped G1 and one G2 call per proof and validates the existing trace output hashes. Both grouped G1 calls have the same return site, so the AWS counts combine them.

A local debugger inspected the actual point and scalar arrays at that entry for the same executable and pinned fixtures. Merge24 H had 525,759 bases, 8,001 zero scalars and 517,758 distinct nonzero scalar words. The 16×2 H call had 371,122 bases, 6,172 zero scalars and 364,950 distinct nonzero words. Both had zero identity bases. Grouping repeated nonzero H values therefore offers no savings on either input. These are full input counts from the first call of each distinct length. No debugger timings enter the latency evidence. Both resulting exports passed independent Gnark verification. [Call intervals](artifacts/intervals.json.gz) · [Input counts](artifacts/counts.json) · [Reader](artifacts/callers.py) · [Verification](artifacts/verification.json)
