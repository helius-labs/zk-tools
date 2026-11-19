# G1 and K/H work during the ordinary G2 range launches

On 1×2, all fills started within a 2.210 ms interval. During that interval an average of 15.52 scheduled threads were inside the K/H feed. On 2×2 the interval was 0.006 ms and that feed had zero calls. Both shapes executed 22 G2 fills. This identifies competing work during the 1×2 launch interval. The new window-grouping experiment measures whether giving G2 earlier access to workers helps the full proof.

The two modes use byte-identical ordinary SDK binaries on separate inodes. Each shape ran in four alternating process pairs with 17 timed proofs after three warmups. All 16 independent exports verified. Traces contained one G2 query and 22 range fills for every measured proof. The tracer reported zero lost or throttled records. The plain and observed latency columns summarize process means. Phase times are medians within each process followed by medians across the four processes.

| Shape | Plain ms | Observed ms | Paired traced vs plain | G2 entry to return ms | Entry to first fill ms | First to last fill start ms | Final fill end to G2 return ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.058 | 7.309 | +3.11% | 4.377 | 0.077 | 2.210 | 0.251 |
| 2x2 | 9.603 | 9.841 | +1.96% | 2.467 | 0.073 | 0.006 | 0.313 |

## Scheduled threads in each observed function

For each function, add up the time its threads were inside its probes and scheduled on CPU during the given interval. Divide by interval length to get the mean number of threads. Values describe only the named functions. Kernel probe overhead can affect task order. The launch interval runs from the first to the final fill start. The second interval extends from that final start to the final fill return.

| Shape | Interval | G2 fill closures | Balanced K/H feed | Ordinary borrowed G1 range closures |
|---|---|---:|---:|---:|
| 1x2 | launch | 10.87 | 15.52 | 1.24 |
| 1x2 | fills after launch | 9.30 | 0.00 | 7.11 |
| 2x2 | launch | 10.46 | 0.00 | 0.50 |
| 2x2 | fills after launch | 19.12 | 0.00 | 0.00 |

[Protocol](protocol.md) · [Symbols and binary pin](artifacts/symbols.json) · [Per-query analysis](artifacts/query-intervals.json.gz) · [Dispatch counts](artifacts/counts.json) · [Process samples](artifacts/runs.json.gz) · [Verifier receipts](artifacts/verification.json) · [Timestamped trace events](artifacts/events.json.gz) · [K/H window-grouping experiment](../2026-11-19-kh-window-groups645/protocol.md)

## How the 22 fills started

Times below are relative to the first range start. Each cell gives the median of the four process medians. A range can finish before all ranges get a worker.

| Input or mode | Start of fill 6 ms | Fill 11 ms | Fill 17 ms | Fill 22 ms | First fill return ms | Fills started before first return |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 0.002 | 0.004 | 1.944 | 2.210 | 1.775 | 11.0 |
| 2x2 | 0.002 | 0.003 | 0.005 | 0.006 | 1.375 | 22.0 |

[Sorted start times for each proof](artifacts/launch-ranks.json.gz)

The plain and traced arms used byte-identical files with different inodes. Their latency difference includes any executable backing effect. A separate three-arm comparison runs the very same copy with probes inactive and active. [Matched-copy protocol](../2026-11-19-probe-controls649/protocol.md)
