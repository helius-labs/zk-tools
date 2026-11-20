# Prepared G2 groups on 4×2 and 8×2

The current 12×2 trial executable also accepts the archived single-level G2 group plans for 4×2 and 8×2. Setup derives point sums from the pinned standard key. Each proof checks the actual scalar coefficients and corrects any mismatches. Separate route probes confirmed 95,537 terms reduced to 44,060 on 4×2 and 183,347 to 63,474 on 8×2 before corrections. All four route checks with a zero preparation budget took the fallback.

Paired full-proof latency changed by -7.61% to -5.12% against the same-build control. Peak RSS changed by +3.8 to +42.0 MiB. The candidate met the paired criterion on 4×2 and 8×2. C7a confirmation is pending.

| Shape | Request | Frozen / control / prepared ms | Prepared vs control | Prepared vs frozen | Control / prepared peak MiB |
|---|---|---:|---:|---:|---:|
| 4×2 | catalog | 14.983 / 15.168 / 14.321 | -5.53% (6/6) | -4.37% (6/6) | 218.5 / 241.2 |
| 4×2 | heldout | 15.011 / 15.128 / 14.239 | -5.12% (6/6) | -5.15% (6/6) | 221.5 / 225.2 |
| 8×2 | catalog | 24.626 / 24.655 / 23.283 | -6.02% (6/6) | -6.16% (6/6) | 381.2 / 413.8 |
| 8×2 | heldout | 24.828 / 24.944 / 22.999 | -7.61% (6/6) | -7.42% (6/6) | 365.5 / 407.5 |

## 4×2 with public wire offsets disabled

These additional arms hold the offset setting fixed. The main table uses the pinned C7a configuration.

| Request | Control ms | Prepared ms | Paired change |
|---|---:|---:|---:|
| catalog | 15.095 | 14.385 | -5.93% (6/6) |
| heldout | 15.368 | 14.523 | -5.56% (6/6) |

Six order-balanced Ryzen 9950X blocks use 24 main workers, eight frontend workers and a four-worker solver pool. Each process warms three proofs and measures thirteen. Latency is the median of process means and memory is peak process RSS across blocks. Qualification requires at least five of six lower paired times and a lower paired median against each control on both requests. The same-build control must be within 2% of the frozen build. Gnark independently reverified all 106 exported proofs.

[Summary](artifacts/summary.json) · [Block samples and route checks](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial source](../2026-11-20-standard-image-nested-b2-ryzen1049/README.md) · [Original group plans](../2026-11-19-middle-groups502/README.md)
