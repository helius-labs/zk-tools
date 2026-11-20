# Scoped exclusive G2 overlap on 96-core C7a

The scoped schedule beat both Zopro controls in all twelve paired blocks on both inputs for 1×8 and 1×16. Paired median latency dropped 11.22–14.65% versus the pinned profile. Aeglos retained a 0.69–0.70 ms lead on 1×8 and a 1.48–1.62 ms lead on 1×16, based on raw medians.

The [exact C7a traces](../2026-11-20-exact-one-input-phase-c7a986/README.md) put G2 after the quotient for 1×8 and 1×16. The [scoped SDK candidate](../2026-11-20-scoped-g2-overlap-ryzen991/README.md) starts the existing G2 query on a distinct 48-core pool and joins it after the main 48 cores finish the quotient and G1. The same-build late profile keeps the ordinary order. The pinned baseline and prepared Aeglos use the exact shared requests and proving keys.

| Shape | Input | Pinned / late / scoped / Aeglos ms | Scoped paired vs pinned | Scoped paired vs late | Scoped paired vs Aeglos | Peak scoped / Aeglos MiB |
|---|---|---:|---:|---:|---:|---:|
| 1×8 | catalog | 13.201 / 13.230 / 11.256 / 10.564 | -14.33% (12/12) | -14.74% (12/12) | +6.92% (0/12) | 139.7 / 690.6 |
| 1×8 | heldout | 13.098 / 13.022 / 11.242 / 10.539 | -14.65% (12/12) | -14.02% (12/12) | +6.82% (0/12) | 142.0 / 687.7 |
| 1×16 | catalog | 15.559 / 15.585 / 13.799 / 12.179 | -11.22% (12/12) | -11.50% (12/12) | +13.37% (0/12) | 177.2 / 821.9 |
| 1×16 | heldout | 15.558 / 15.564 / 13.656 / 12.175 | -12.15% (12/12) | -12.49% (12/12) | +11.90% (0/12) | 167.6 / 822.5 |

| Shape | Input | Quotient / G2 overlap ms | G2 / G1 finish ms | Trace effect |
|---|---|---:|---:|---:|
| 1×8 | catalog | 1.184 | 7.422 / 7.128 | +3.03% |
| 1×8 | heldout | 1.179 | 7.215 / 6.858 | +1.27% |
| 1×16 | catalog | 1.216 | 8.603 / 7.989 | -5.08% |
| 1×16 | heldout | 1.215 | 9.007 / 8.057 | +1.96% |

The timing panel primes three proofs and times seventeen per process across twelve matched blocks. The separate clock panel uses three blocks, and its interval origin is G2 dispatch after witness preparation. Scratch denial selected the usual path. Every timed Aeglos proof hit the prepared cache, all 212 exports independently verified, and the temporary AWS resources were removed.

[Summary](artifacts/summary.json) · [Samples and route probes](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
