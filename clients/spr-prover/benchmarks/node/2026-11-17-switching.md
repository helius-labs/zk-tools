# Switching between loaded circuit shapes

Measured 2026-11-17 on Apple M5 Pro, Node v24.21.0, 18 threads. Alternating transfer 2→2 and 8→2; four isolated processes, 56 locally verified proofs. GPU cases require actual GPU execution.

Each process starts with an empty disk cache, preloads both shapes, then alternates proofs. The first proof for each shape is reported separately in the raw data; the table uses the next six samples per shape. The single-shape policy reads the decoded disk cache on each switch. The 512 MiB policy retains both provers in one worker.

| Backend | Resident budget | 2→2 median | 8→2 median | Key reloads | Peak process RSS |
| --- | ---: | ---: | ---: | ---: | ---: |
| CPU | Single shape | 152.1 ms | 448.7 ms | 14 | 2675 MiB |
| CPU | 512 MiB | 125.6 ms | 381.1 ms | 0 | 2394 MiB |
| GPU | Single shape | 112.4 ms | 257.9 ms | 14 | 2398 MiB |
| GPU | 512 MiB | 70.0 ms | 147.5 ms | 0 | 2315 MiB |

Timings include key selection/reload, witness generation, proving, local verification and worker messaging. These are switching latencies, not just the cryptographic proving step.

**Memory:** RSS is the process high-water mark over the entire case, including downloads and preloading, from `process.resourceUsage().maxRSS`. It includes CPU workers and allocator-retained memory; it does not fully account for external GPU allocations. The key-cache budget is an admission estimate, not a process-memory limit. Similar RSS here does not establish that caching is free on other shapes or hosts.

The resident cache eliminated all 14 reloads per case. It remains opt-in; larger shapes can evict other residents or run alone. These measurements cover this two-shape workload on one machine.

## Reproduce

```sh
npm ci
npm run build:prover
node scripts/bench-node-switching.mjs
```

Set `SPR_BENCH_OUTPUT` to choose the JSON output path. GPU mode requires a supported device and the optional Dawn binding (installed by this workspace).

[Raw samples and first-preloaded-proof timings](2026-11-17-switching.json).

Raw metadata records the base commit, pinned runtime, Dawn version and SHA-256 digests of the harness and compiled worker/controller used for this run.
