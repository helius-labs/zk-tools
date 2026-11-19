# Public B2 offsets in verified 1×2 proofs

The isolated binary used 1,168 fixed public coefficients from five training fixtures. It checked the B2 point-set digest and derived the correction point from the immutable key at first use. A per-key reservation covered the offsets and correction. Each proof uses the exact modular difference at every selected position and adds the correction point. Four held-out inputs used the same main pool of 32 pinned workers and eight frontend workers. Seven randomized matched blocks timed 31 full SDK proofs after three warmups.

| Input | Release / disabled / residual ms | Paired vs release | Paired vs disabled | CPU-ms disabled / residual | Peak MiB disabled / residual |
|---:|---:|---:|---:|---:|---:|
| 5 | 7.068 / 7.075 / 7.038 | -0.03% (4/7) | -0.93% (6/7) | 143.9 / 142.8 | 72.0 / 62.2 |
| 6 | 7.056 / 7.020 / 7.031 | -0.55% (5/7) | +0.53% (3/7) | 142.8 / 142.6 | 61.8 / 63.5 |
| 7 | 7.057 / 7.048 / 7.027 | +0.04% (3/7) | -0.21% (4/7) | 143.7 / 143.2 | 63.2 / 61.5 |
| 8 | 7.096 / 7.109 / 6.994 | -1.76% (7/7) | -0.87% (7/7) | 144.1 / 142.4 | 64.5 / 62.2 |

The median paired change across witnesses was -0.29% versus release (3/4 faster) and -0.54% versus the identical disabled binary (3/4 faster). The one-time setup probe took 135.9 ms. This candidate embeds a 42,048-byte fixture-specific offset vector in the diagnostic executable. Zero optional workspace selected the ordinary path. All 87 exports passed independent Gnark verification.

The candidate met the warm latency criterion on this key. It remains experimental. The next check is first-proof cost with batched correction setup and replication on new inputs. To reproduce the prototype, copy `artifacts/public_offsets.bin` to `crates/zopro-core/src/public_b2_offset.bin` in the patched zopro workspace.

[Observed samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Source patch](artifacts/source.patch) · [Coefficient selection](../2026-11-19-g2-public-offset453/README.md)
