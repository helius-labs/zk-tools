# Opt-in solver task claiming on Ryzen

The candidate bounds the number of workers claiming tasks in each dependency band. Each claimant takes the next unprocessed task from one atomic cursor. The cursor lives on the stack during execution. The option retains the existing task vectors and scratch budget. The benchmark enables the candidate with ZOPRO_SOLVE_TASK_CLAIMS=1. The disabled column uses the same binary with the option unset.

| Shape | Workers | Release ms | Disabled ms | Claiming ms | Paired vs release | Wins / pairs | Paired vs disabled | Wins / pairs | Peak MiB release / claiming | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 16 | 8.744 | 8.796 | 8.484 | -2.69% | 8/9 | -2.95% | 8/9 | 60.8 / 60.2 | 4.36× / 2.83× |
| 1x2 | 32 | 7.613 | 7.627 | 7.289 | -3.63% | 8/9 | -4.02% | 8/9 | 67.5 / 65.8 | 5.08× / 3.29× |
| 2x2 | 32 | 10.776 | 10.765 | 10.272 | -4.42% | 3/3 | -4.50% | 3/3 | 110.5 / 113.0 | 5.74× / 2.92× |
| 4x2 | 16 | 16.875 | 16.782 | 16.761 | -0.44% | 2/3 | -0.23% | 2/3 | 208.8 / 216.2 | 6.50× / 2.86× |
| 8x2 | 16 | 26.898 | 26.735 | 26.738 | -1.67% | 3/3 | -0.62% | 2/3 | 358.5 / 370.0 | 7.07× / 2.77× |
| 12x2 | 16 | 38.449 | 38.703 | 38.488 | +0.59% | 1/3 | -0.42% | 2/3 | 496.5 / 518.0 | 7.38× / 2.62× |
| 16x2 | 16 | 46.360 | 45.585 | 45.217 | -2.65% | 2/3 | -0.47% | 2/3 | 611.2 / 612.0 | 7.41× / 2.74× |
| 24x2 | 16 | 73.151 | 74.062 | 73.250 | +0.14% | 1/3 | -1.10% | 2/3 | 963.5 / 962.5 | 7.21× / 2.40× |
| merge-24x1 | 16 | 71.524 | 72.169 | 71.245 | -0.39% | 2/3 | -1.28% | 3/3 | 871.1 / 868.5 | 7.12× / 2.46× |

The integrated solver now exposes this policy as a native option, disabled by default. On 1×2 the candidate beat both the previous release and the same binary with claiming disabled in eight of nine independent blocks at each worker count. The other shapes had only three blocks. The earlier claims prototype did not demonstrate a repeatable C7a gain, and the clean executable has only been qualified on Ryzen for latency. [Matched C7a study](../2026-11-18-claims-c7a374/README.md)

The exact original prototype measured 8.501 ms at 16 workers.

The exact original prototype measured 7.339 ms at 32 workers.

Aeglos with 32 native workers and GOMAXPROCS=32 measured 12.897 ms on 1×2, with 433.2 MiB peak RSS across the nine processes. Each prover used the same request, CPU mask, and independent proof verifier.

[Raw samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Source](artifacts/source.patch.gz)

The native suite passed 147 tests. The portable x86-64 build with default features disabled passed all 106 tests with one test thread. Its default concurrent test run had three allocator-policy test failures. A concurrent run of the allocator tests on the previous baseline also produced three failures. These tests inspect process-wide allocation policy. A zero-optional-preparation-budget request with task claiming enabled exported a verified proof using the fallback. [Portable checks](artifacts/portable-validation.json) · [Fallback proof](artifacts/zero-preparation.json)
