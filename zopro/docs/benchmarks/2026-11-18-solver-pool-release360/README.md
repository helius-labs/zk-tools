# General witness execution on a caller-owned pool

The optional Linux x86-64 path uses the shared frontend pool for the instruction-execution section of a CPU general solve. It requires an attached pool with fewer threads than the current Rayon pool and the explicit setting `ZOPRO_CPU_SOLVER_FRONTEND=1`. Instruction workers join before the caller uses the output buffers. Allocation, gather and subsequent FFT and curve work keep their original placement.

On C7a merge24 at 96 physical cores, the integrated option took 51.183 ms against 54.202 ms with the same supplied pool idle and 52.913 ms on the earlier release with no auxiliary pool. The paired change versus idle was -5.44% in 7/7 faster blocks. The earlier 64-core release took 49.085 ms. Adoption is opt-in. [Full AWS results](aws.md)

## Local checks

All 144 native core tests passed. A concurrency test shares a pool and a plan across four callers, checks error propagation and verifies the resulting assignment and all row buffers. A generic x86-64 core and SDK release check also passed. Each of the eight shapes ran in three randomized blocks through the real SDK.

| Shape | Prior ms | Disabled ms | Enabled ms | Enabled peak MiB |
|---|---:|---:|---:|---:|
| 1x2 | 7.950 | 7.660 | 7.829 | 69 |
| 2x2 | 10.800 | 10.753 | 10.679 | 120 |
| 4x2 | 16.961 | 16.795 | 17.514 | 208 |
| 8x2 | 26.735 | 26.486 | 27.765 | 346 |
| 12x2 | 38.926 | 39.079 | 40.424 | 493 |
| 16x2 | 45.055 | 45.317 | 46.979 | 556 |
| 24x2 | 73.673 | 73.277 | 75.924 | 884 |
| merge-24x1 | 71.653 | 71.364 | 72.994 | 798 |

## Observed instruction execution

Linux uprobes on the release executable recorded threads executing solver tasks. The selected compiled symbol is preserved with each observation. Thread classification uses the process ID and the measured main-worker thread IDs. Each process contained only the caller, the pinned main workers and the explicitly attached frontend workers. The harness turns a request for an equal-sized frontend into zero attached workers, so the two zero-attachment probes exercise the no-pool fallback. Each probe run exported a separately verified proof.

| Shape | Main / attached | Option | Observed thread classes |
|---|---:|---:|---|
| 16x2 | 16 / 12 | 0 | main-worker |
| 16x2 | 16 / 12 | 1 | frontend-worker |
| 16x2 | 16 / 0 | 1 | main-worker |
| 16x2 | 16 / 0 | 1 | main-worker |
| 1x2 | 32 / 8 | 0 | frontend-worker |
| 1x2 | 32 / 8 | 1 | frontend-worker |

[Local timings](artifacts/local.json.gz) · [Route observations](artifacts/routes.json) · [Verification](artifacts/verification.json)

[All-shape C7a results](aws.md)
