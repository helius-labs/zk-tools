# Ordinary release G2 fill launches with G1 arithmetic probes

On 1×2 the G2 range starts spread across 2.515 ms. On 2×2 they spread across 0.007 ms. The active G1 streaming routine ran thousands of times per request. Its entry and return probes increased proof latency by 12–15%, which limits how closely the phase durations represent an uninstrumented proof. The lighter follow-up traces query phases.

The two modes use byte-identical ordinary SDK binaries on separate inodes. Each shape ran in four alternating process pairs with 17 timed proofs after three warmups. All 16 independent exports verified. Traces contained one G2 query and 22 range fills for every measured proof. The tracer reported zero lost or throttled records. The plain and observed latency columns summarize process means. Phase times are medians within each process followed by medians across the four processes.

| Shape | Plain ms | Observed ms | Paired observer change | G2 entry to return ms | Entry to first fill ms | First to last fill start ms | Final fill end to G2 return ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.047 | 7.902 | +12.03% | 4.697 | 0.080 | 2.515 | 0.257 |
| 2x2 | 9.424 | 10.847 | +14.74% | 2.982 | 0.049 | 0.007 | 0.318 |

## Scheduled threads in each observed function

For each function, add up the time its threads were inside its probes and scheduled on CPU during the given interval. Divide by interval length to get the mean number of threads. Values describe only the named functions. Kernel probe overhead can affect task order. The launch interval runs from the first to the final fill start. The second interval extends from that final start to the final fill return.

| Shape | Interval | G2 fill closures | G1 arithmetic routine |
|---|---|---:|---:|
| 1x2 | launch | 10.44 | 12.22 |
| 1x2 | fills after launch | 8.19 | 4.49 |
| 2x2 | launch | 10.29 | 0.00 |
| 2x2 | fills after launch | 18.97 | 0.00 |

[Protocol](protocol.md) · [Symbols and binary pin](artifacts/symbols.json) · [Per-query analysis](artifacts/query-intervals.json.gz) · [Dispatch counts](artifacts/counts.json) · [Process samples](artifacts/runs.json.gz) · [Verifier receipts](artifacts/verification.json) · [Follow-up with phase probes](../2026-11-19-competing-feed644/README.md)
