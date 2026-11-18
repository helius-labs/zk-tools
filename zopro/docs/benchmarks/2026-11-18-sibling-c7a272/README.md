# Measuring K/H execution on the G2 worker on C7a

On a fresh c7a.24xlarge, the uninstrumented experimental 1×2 prover averaged 9.968 ms at 96 cores (median of three process means), versus 7.723 ms for Aeglos. Adding K/H feed probes and per-task switch recording increased median paired latency by 13.7%. It moved 223 of 453 traced requests into the 10–11 ms band, compared with two in the uninstrumented controls. The observer exceeded the 5% overhead limit, and all three traced blocks exceeded the allowed middle-class fraction. We cannot use these measurements to assign a cause to the ordinary two latency groups.

| Cores | Plain Zopro ms | Aeglos ms | Traced/plain | Plain fast / middle / slow | Traced fast / middle / slow |
|---:|---:|---:|---:|---|---|
| 32 | 10.709 | 11.825 | 1.138 | 10 / 315 / 128 | 0 / 2 / 451 |
| 96 | 9.968 | 7.723 | 1.137 | 337 / 2 / 114 | 55 / 223 / 175 |

The experiment tracked the G2 thread before the first bucket and after the last bucket. After G2 returned it tracked the canonical-query thread. We intersected its scheduled runtime with actual K/H feed intervals. All 906 main traced requests contained 40 K/H feed calls and 22 G2 bucket calls. The reader checked complete intervals and worker affinity. Among the 175 observed slow 96-core requests, 62 had a majority of their largest phase excess in on-CPU K/H feed work and 71 had a majority off-CPU. Both mechanisms occur in the instrumented program. These counts cannot estimate their prevalence in uninstrumented requests. The remaining requests had a different phase, residual work or mixed attribution. [Per-block calibration](artifacts/panels.json) · [Attributed 96-core requests](artifacts/attribution.json.gz)

The plain 96-core experimental prover reached 3.71× the supplied M5 CPU and 2.41× M5 Metal speeds. The run generated 2,724 timed requests and 24 exported proofs. All exports verified on AWS and after retrieval. The instance terminated, no tagged EBS volumes remained and the temporary bucket was deleted. [Process measurements](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
