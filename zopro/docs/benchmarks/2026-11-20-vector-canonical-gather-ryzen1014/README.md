# Eight-lane canonical witness conversion on Ryzen

This experiment replaces scalar Montgomery conversion with groups of eight inside the existing fused wire gather. The batch kernel uses the existing mixed-radix IFMA Montgomery product with integer limb one. Each leaf converts its remaining wires with the scalar kernel. Point inputs and proof layout are unchanged.

Three native gather tests passed, including equality of converted wires and both output vectors across vector and Rayon leaf boundaries. The SDK benchmark uses one binary, fixed 16+16 placement, two requests per shape, and six paired blocks of thirteen proofs after three primes. Separate route runs measure gather time.

| Shape | Request | Ordinary / vector proof ms | Paired change | Ordinary / vector gather µs | Peak ordinary / vector MiB |
|---|---|---:|---:|---:|---:|
| 1×2 | catalog | 6.767 / 6.719 | -0.47% (4/6) | 81.9 / 72.4 | 56.8 / 57.5 |
| 1×2 | heldout | 6.564 / 6.546 | -0.03% (3/6) | 81.0 / 80.1 | 58.0 / 58.5 |
| 1×4 | catalog | 8.128 / 8.040 | -1.17% (5/6) | 72.6 / 66.6 | 98.8 / 101.2 |
| 1×4 | heldout | 8.084 / 8.079 | -0.15% (5/6) | 73.5 / 67.6 | 103.8 / 104.5 |
| 1×8 | catalog | 9.618 / 9.622 | -0.00% (3/6) | 100.6 / 95.8 | 120.5 / 118.8 |
| 1×8 | heldout | 9.678 / 9.624 | -0.60% (5/6) | 92.0 / 88.8 | 121.2 / 117.8 |
| 1×16 | catalog | 12.453 / 12.505 | +0.77% (1/6) | 127.2 / 126.9 | 146.9 / 145.0 |
| 1×16 | heldout | 12.476 / 12.444 | -0.45% (5/6) | 131.9 / 126.0 | 145.2 / 145.6 |
| 2×2 | catalog | 9.880 / 9.905 | +0.07% (3/6) | 109.5 / 104.3 | 90.0 / 88.8 |
| 2×2 | heldout | 9.941 / 9.901 | -0.67% (4/6) | 113.9 / 108.3 | 88.5 / 87.5 |

The largest measured gather reduction was about 10 µs. The 1×4 arms won five of six paired blocks on each request, with a 0.15% paired gain on the held-out request. The SDK keeps the existing gather. All 140 exported proofs were verified independently.

[Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
