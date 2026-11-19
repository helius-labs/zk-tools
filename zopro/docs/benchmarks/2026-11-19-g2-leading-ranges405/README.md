# Independent range count for the leading G2 window

Regular G2 windows use streaming signed bucket updates. The narrow leading window uses an indexed reduction with its own parallel tasks. The candidate changes only that window’s input partition count and its position in the Rayon iteration order. After reduction it restores the original window order before the scalar fold. A conservative workspace charge covers additional memory and the original path handles denied preparation.

Three randomized blocks screened five policies under both two-range and four-range geometries. The selected policy for each shape was timed in seven fresh blocks on 32 Ryzen hardware threads. Each confirmation process timed 61 full SDK proofs.

| Shape | Regular / leading parts | Leading first | Standard release ms | Matched release ms | Disabled ms | Selected ms | Paired vs standard (wins) | Paired vs matched release (wins) | Paired vs disabled (wins) | Peak MiB standard / selected | M5 CPU / Metal gain |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 2 / 1 | False | 7.276 | 7.276 | 7.293 | 7.304 | +0.30% (3/7) | +0.30% (3/7) | +0.03% (3/7) | 66.2 / 65.2 | 5.07× / 3.29× |
| 2x2 | 2 / 2 | True | 10.165 | 10.165 | 10.237 | 10.096 | -0.52% (4/7) | -0.52% (4/7) | -1.04% (6/7) | 117.8 / 117.8 | 5.84× / 2.97× |

On 1×2, halving the leading range count lost four of seven pairs against both controls. On 2×2, submitting the leading window first won six pairs against the same executable with the feature disabled, and four against the integrated release. Neither setting met the five-win condition against every control. The integrated G2 scheduling policy remains in use. All 151 native core tests passed, including a generator-sum oracle covering asymmetric ranges and reordered windows.

Where the screen selected two regular ranges, the standard and matched release columns use the same control processes. The routing preflights verified all five policies at both geometries on both shapes. The zero-preparation fallback exported a verified proof.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Implementation](artifacts/source.patch.gz) · [Verification](artifacts/verification.json)
