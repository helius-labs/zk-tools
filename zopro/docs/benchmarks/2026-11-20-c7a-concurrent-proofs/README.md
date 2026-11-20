# Concurrent proof requests on 96-core C7a

The benchmark ran 7,296 timed proofs across 80 groups. Each group uses warmed processes and one circuit key. Zopro and Aeglos ran on the same c7a.24xlarge against the same catalog request at each load.

Zopro had higher paired throughput in 10 of 10 load settings and lower peak group memory in 10 of 10. On 1×2, increasing Zopro from 12 to 24 workers raised median throughput by 2.7% while p95 service time rose from 35.6 to 68.2 ms.

| Shape | Concurrent workers | Proofs per burst | Zopro / Aeglos proofs/s | Zopro / Aeglos p95 service ms | Zopro / Aeglos burst ms | Zopro / Aeglos peak GiB |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 | 1 | 120 | 131.3 / 125.5 | 7.9 / 8.5 | 914.2 / 955.8 | 0.09 / 0.51 |
| 1×2 | 4 | 120 | 278.5 / 218.6 | 15.1 / 19.1 | 430.9 / 549.0 | 0.18 / 1.66 |
| 1×2 | 12 | 120 | 344.7 / 260.3 | 35.6 / 47.6 | 348.1 / 461.0 | 0.28 / 4.63 |
| 1×2 | 24 | 120 | 354.0 / 261.6 | 68.2 / 94.2 | 339.0 / 458.7 | 0.43 / 9.14 |
| 4×2 | 1 | 120 | 72.2 / 54.8 | 14.5 / 19.0 | 1663.1 / 2190.3 | 0.27 / 1.39 |
| 4×2 | 4 | 120 | 138.0 / 75.5 | 30.3 / 55.0 | 869.9 / 1589.7 | 0.74 / 5.12 |
| 4×2 | 12 | 120 | 157.3 / 79.7 | 77.8 / 154.0 | 762.9 / 1505.1 | 1.15 / 14.95 |
| merge-54×1 | 1 | 24 | 9.4 / 5.7 | 138.0 / 179.5 | 2554.7 / 4189.3 | 1.51 / 12.88 |
| merge-54×1 | 4 | 24 | 18.5 / 5.7 | 229.3 / 720.2 | 1296.4 / 4235.5 | 4.45 / 50.86 |
| merge-54×1 | 8 | 24 | 20.4 / 5.4 | 397.3 / 1485.6 | 1175.5 / 4429.6 | 7.27 / 103.32 |

## Scaling

| Shape | Engine | Highest measured throughput | Workers | Throughput vs one worker | CPU ms per proof, one worker → best | Peak GiB at best width |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | Zopro | 354.0 proofs/s | 24 | 2.70× | 320.4 → 223.1 | 0.43 |
| 1×2 | Aeglos | 261.6 proofs/s | 24 | 2.08× | 513.7 → 348.8 | 9.14 |
| 4×2 | Zopro | 157.3 proofs/s | 12 | 2.18× | 657.3 → 431.4 | 1.15 |
| 4×2 | Aeglos | 79.7 proofs/s | 12 | 1.46× | 1400.8 → 1177.9 | 14.95 |
| merge-54×1 | Zopro | 20.4 proofs/s | 8 | 2.17× | 6086.7 → 4043.8 | 7.27 |
| merge-54×1 | Aeglos | 5.7 proofs/s | 1 | 1.00× | 14998.3 → 14998.3 | 12.88 |

## Measurement

Each process primes three proofs and then waits at a common release gate. The burst clock ends when the last worker completes its assigned queue. Service latency includes proof generation, verification and the SDK output path. The service p95 excludes waiting behind earlier proofs in the same worker queue. The 1×2 and 4×2 bursts contain 120 requests. Merge-54×1 uses 24 requests and stops at eight workers to keep prewarmed-key memory within the instance budget.

Worker CPU masks are disjoint and partition the 96 physical cores. Single-worker Zopro runs use the recorded low-latency profiles. At higher concurrency the main and frontend widths scale with each worker’s CPU allotment. Aeglos uses its CPU backend with the same worker allotment and a prepared-cache hit for every timed proof. Each worker owns its own prover instance and prepared state. This models a persistent process pool and leaves shared in-process key preparation unmeasured. The maximum release skew between workers was 2.421 ms.

Four matched blocks reverse engine order. Throughput and latencies are medians across blocks. Peak memory is the highest Linux cgroup high-water mark across the four blocks, includes warmup, and counts memory charged to the worker group. The groups share the host file cache. Memory charges can depend on which process first faults a page, and setup preloads fixture files for checksum validation. The archive also includes sampled total worker RSS and per-worker process high-water marks. The group memory limit was 156 GiB with swap disabled. All groups completed within that limit.

Every timed proof passed the prover’s verification. Gnark independently reverified the final proof from each of the 568 worker processes. A first attempt ended after a verifier log line preceded the expected receipt. Its one proof was independently reverified and excluded from the timing table. Both temporary instances and their storage were removed.

[Single-proof latency and memory for all 41 shapes](../2026-11-20-c7a-41-shape-latency-memory/README.md) · [Summary JSON](artifacts/summary.json) · [CSV](artifacts/summary.csv) · [Block results](artifacts/rows.json.gz) · [Benchmark driver](artifacts/burst.py) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
