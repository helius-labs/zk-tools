# Joint sensitivity to A, K/H and B2 in verified 1×2 proofs

The experiment removes selected online curve sums from repeated calls on a fixed public synthetic request. The A and combined K/H branches reuse a point derived from the same key only after checking both scalar slices for equality. The B2 branch uses the full-vector equality test from experiment 458. A probe primed with the first input then switched to a second public witness and confirmed that all three comparisons fell back to full computation. The solver, FFT, scalar preparation, randomization, serialization and verification still execute. The isolated diagnostic stores scalar copies for equality checks and is specific to these public fixtures.

Seven randomized matched blocks on a Ryzen 9950X used 32 logical workers, eight frontend workers and 51 timed calls after three warmups. Times are medians of process means. Each reduction can affect concurrent work and the worker schedule.

| Reused sums | Full SDK ms | Paired vs disabled | Wins / 7 | CPU-ms/proof | Peak RSS MiB |
|---|---:|---:|---:|---:|---:|
| release | 7.071 | -8.56% | 7 | 142.8 | 66.5 |
| off | 7.729 | +0.00% | — | 153.7 | 61.5 |
| A | 7.246 | -5.92% | 7 | 130.5 | 59.2 |
| KH | 5.471 | -29.15% | 7 | 103.3 | 61.2 |
| B2 | 4.774 | -38.23% | 7 | 83.6 | 58.8 |
| A+KH | 5.061 | -34.55% | 7 | 79.7 | 58.5 |
| A+B2 | 4.338 | -43.93% | 7 | 60.0 | 55.2 |
| KH+B2 | 3.085 | -60.03% | 7 | 33.4 | 58.2 |
| A+KH+B2 | 1.839 | -76.16% | 7 | 10.0 | 57.8 |

The disabled diagnostic measured 7.729 ms against 7.071 ms for the release. Differences between diagnostic modes use the same binary. This baseline shift limits extrapolation to production.

The 1×2 goal is 3.700 ms. With all three curve sums reused, the other proof work and input checks took 1.839 ms in this diagnostic. Timings for these artificial interventions apply to identical public scalar inputs. All 67 exported proofs passed independent Gnark verification.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Incremental diagnostic patch](artifacts/changes.patch) · [Fixed fixture identities](artifacts/fixture-pins.json) · [B2-only experiment](../2026-11-19-b2-sensitivity458/README.md)
