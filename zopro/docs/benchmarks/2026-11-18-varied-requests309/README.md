# Inline grouping across varied transfer witnesses

On 32 logical Ryzen workers the G2 diversity check reduced median paired proof latency on each of eight new 1×2 requests. The median of those eight paired changes was −2.56%. Peak process RSS was lower on every 1×2 request. On eight 2×2 requests the median paired change was −0.42%; one variant showed a 12.7% increase across its three pairs.

Each variant changes the private blinding seed and redistributes value between two real outputs. The generator uses the pinned Go protocol to recompute the output blindings, both output commitments, private transaction hash and public input hash. Gnark computes the expected public witness before Rust proves the request. Regenerating the original seed and amounts reproduced the pinned public-input bytes for both shapes. The input notes and Merkle paths were shared within each shape.

| Shape | Variant | On / off / accepted ms | Paired vs off | Wins | Max RSS MiB on / off | M5 CPU / Metal speedup on |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 1 | 7.332 / 7.488 / 7.450 | -2.04% | 3/3 | 60 / 95 | 5.05× / 3.27× |
| 1x2 | 2 | 7.292 / 7.457 / 7.431 | -2.99% | 3/3 | 62 / 97 | 5.07× / 3.29× |
| 1x2 | 3 | 7.365 / 7.522 / 7.465 | -2.50% | 3/3 | 62 / 123 | 5.02× / 3.26× |
| 1x2 | 4 | 7.247 / 7.547 / 7.427 | -3.72% | 3/3 | 61 / 106 | 5.11× / 3.31× |
| 1x2 | 5 | 7.298 / 7.509 / 7.508 | -2.81% | 3/3 | 61 / 109 | 5.07× / 3.29× |
| 1x2 | 6 | 7.322 / 7.505 / 7.383 | -2.44% | 3/3 | 80 / 131 | 5.05× / 3.28× |
| 1x2 | 7 | 7.357 / 7.467 / 7.462 | -2.62% | 2/3 | 69 / 92 | 5.03× / 3.26× |
| 1x2 | 8 | 7.369 / 7.449 / 7.561 | -1.08% | 2/3 | 62 / 95 | 5.02× / 3.26× |
| 2x2 | 1 | 10.527 / 10.492 / 10.546 | +1.22% | 0/3 | 246 / 224 | 5.60× / 2.85× |
| 2x2 | 2 | 10.593 / 10.617 / 10.620 | -0.23% | 2/3 | 234 / 236 | 5.57× / 2.83× |
| 2x2 | 3 | 10.596 / 10.715 / 10.715 | -1.12% | 2/3 | 236 / 245 | 5.57× / 2.83× |
| 2x2 | 4 | 10.659 / 10.728 / 10.575 | -0.41% | 2/3 | 235 / 238 | 5.54× / 2.81× |
| 2x2 | 5 | 10.501 / 10.590 / 10.458 | -0.69% | 3/3 | 236 / 243 | 5.62× / 2.86× |
| 2x2 | 6 | 10.602 / 10.576 / 10.436 | -0.42% | 2/3 | 240 / 236 | 5.56× / 2.83× |
| 2x2 | 7 | 10.480 / 10.525 / 10.396 | -0.50% | 2/3 | 235 / 237 | 5.63× / 2.86× |
| 2x2 | 8 | 12.053 / 10.700 / 10.447 | +12.65% | 1/3 | 230 / 233 | 4.89× / 2.49× |

Each cell uses three fresh processes with three primes and 21 timed requests. All 144 exports verified externally and matched the precomputed public bytes. These measurements cover varied witnesses on the local machine; the independent C7a replication uses the original fixture.

[Generator](artifacts/generate.go) · [Request and public hashes](artifacts/variants.json) · [Raw timings](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
