# Where the 96-core 1×2 latency groups diverge

In three fresh C7a blocks the first consistent millisecond-scale difference appeared **between G2 query entry and the first bucket fill**. That interval averaged 0.18–0.19 ms on fast proofs and 1.48–1.69 ms on slow ones. Slow proofs also spent longer between the last fill returning and G2 returning, and between G2 returning and the canonical proof call finishing. The check for repeated scalars and the FFT tracked closely across both groups.

| Disjoint interval | Fast mean across requests, range of block means | Slow mean across requests, range of block means |
|---|---:|---:|
| G2 entry to first bucket fill | 0.18–0.19 ms | 1.48–1.69 ms |
| First fill starts through last fill returns | 3.46–3.48 ms | 3.48–3.68 ms |
| Last fill returns through G2 return | 0.55–0.56 ms | 1.49–1.72 ms |
| G2 return through canonical return | 0.02–0.03 ms | 0.56–0.76 ms |
| **Full verified proof** | **9.39–9.44 ms** | **12.50–12.64 ms** |

We classified requests using the previously observed thresholds of under 10 ms and at least 11 ms. The 453 traced 96-core proofs included 327 fast, ten intermediate and 116 slow. The 453 plain controls included 334, two and 117. The traced/plain ratios had medians of 1.016 at 96 cores and 1.023 at 32. Both passed the 5% observer limit. Every traced 96-core block retained both latency groups within the prespecified limits.

The G2 implementation splits canonical scalars into two signed components before launching the bucket fills. Running this split alongside the FFT after the diversity check succeeds would use the existing proof-local scalar buffer. That is a direct test of the first delay. The G2 entry-to-fill interval includes scheduling and synchronization. Its duration cannot be assigned wholly to scalar arithmetic. The second delay includes bucket reduction and joins, and other queries may still be active. Each of the twelve last-level cache groups hosted G2 entries in both fast and slow proofs. [Per-block results and CPU IDs](artifacts/classes.json.gz) · [Request-level intervals](artifacts/processes.json.gz)

| Plain 1×2 comparison | 32 cores | 96 cores | 96-core CPU ms/proof | 96-core peak RSS MiB |
|---|---:|---:|---:|---:|
| Experimental Zopro, both switches | 10.74 ms | 10.10 ms | 454 | 64.8 |
| Aeglos | 11.78 ms | 7.91 ms | 516 | 479.0 |

Each wall-time entry is the median of three uninstrumented process means. The 96-core candidate reaches 3.66× the supplied M5 CPU speed and 2.38× Metal on 1×2. The eight-shape 10× CPU goal remains open. [Earlier nine-shape panel](../2026-11-18-early-diversity257/README.md)

The first AWS preflight (261) registered probes at two locked counter increments. The timed certificate counters advanced at both core counts, but the two probe sites emitted no events on the AWS instance. All six completed exports passed independent host Gnark verification and entered the ledger. The revised observer (262) traced the containing scalar-check callback and passed a local smoke before launch. Each timed trace had one complete call per named stage and 22 complete G2 fills, checked by worker/thread and paired in SDK request order. The full run verified all 24 exported proofs on both hosts. Together with the revised local smoke the cumulative ledger reached 65,164. Both AWS instances terminated, with tagged EBS queries empty and temporary buckets removed. [Protocol](artifacts/protocol.md) · [Trace reader](artifacts/trace.rs) · [Verification](artifacts/verification.json)
