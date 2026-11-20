# Prepared G2 groups on 4×2 and 8×2 on C7a

Prepared G2 groups reduce the base count from 95,537 to 44,060 on 4×2 and from 183,347 to 63,474 on 8×2 before exact scalar mismatch corrections. The trial uses the current SDK with standard keys and the pinned worker placement. All tested inputs exercised the group route. With a zero optional preparation budget they exercised the fallback and produced valid proofs.

Paired warm latency changed by +10.55% to +15.75% versus the same-build control. Peak warm RSS changed by +27.0 to +46.5 MiB. Both shapes fell short of the two-input paired latency criterion. Process CPU time per proof rose by 2.4–7.7% across the four cases. The experimental route stays isolated.

| Shape | Request | Frozen / control / prepared warm ms | Prepared vs control | Prepared vs frozen | Control / prepared cold ms | Control / prepared peak warm MiB |
|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | 14.071 / 14.069 / 16.277 | +15.75% (0/8) | +15.82% (0/8) | 143.0 / 150.3 | 194.5 / 241.0 |
| 4×2 | heldout | 14.091 / 14.004 / 16.222 | +15.73% (0/8) | +15.07% (0/8) | 143.0 / 150.7 | 203.5 / 232.6 |
| 8×2 | catalog | 18.825 / 18.710 / 21.529 | +14.83% (0/8) | +13.79% (0/8) | 192.6 / 203.8 | 355.7 / 386.0 |
| 8×2 | heldout | 18.987 / 19.157 / 21.366 | +10.55% (0/8) | +12.43% (0/8) | 193.8 / 203.2 | 361.3 / 388.2 |

One c7a.24xlarge ran 72 main and 24 frontend workers, with the pinned dedicated solver masks. Eight order-balanced blocks measure thirteen warm proofs per process after three primes. Warm time is the median of process means. Cold time starts at the entry of each fresh process and includes preparation and the first verified response. Processes share the host file cache. Peak RSS is the maximum whole-process peak across blocks. Qualification requires a lower paired median and at least seven paired wins against both controls on both requests, with same-build control calibration within 2% of the frozen binary.

## Supplied M5 references

| Shape | Catalog prepared ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 4×2 | 16.277 | 109 / 48 | 6.70× / 2.95× |
| 8×2 | 21.529 | 189 / 74 | 8.78× / 3.44× |

Gnark independently reverified all 208 first and final proof exports after retrieval. The temporary instance, volume, and bucket were removed.

[Summary](artifacts/summary.json) · [Samples and route checks](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Ryzen comparison](../2026-11-20-middle-prepared-g2-ryzen1051/README.md) · [All eight M5 reference shapes](../2026-11-20-c7a-three-provers/README.md#supplied-m5-references)
