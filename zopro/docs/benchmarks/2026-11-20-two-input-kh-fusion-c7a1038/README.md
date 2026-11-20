# K/H fusion for 2×4 on C7a

At fixed 64+32 worker placement, fusion fell short of a repeatable latency gain on both inputs. Primary peak process memory moved from 186.2 to 163.0 MiB.

K and H are two G1 sums that the CPU proof consumes as a combined result. The fused path uses one bucket set per window and preserves the separate path when scratch admission fails. This run keeps the SDK, proving key, request, and G2 preparation fixed and changes only K/H fusion and the listed main/G2 worker placement.

| Request | Main + G2 workers | Separate proof ms | Fused proof ms | Paired fusion change | Separate peak MiB | Fused peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | 64+32 | 11.311 | 11.978 | +6.09% (1/8) | 186.2 | 163.0 |
| catalog | 56+40 | 11.239 | 11.844 | +5.08% (0/8) | 179.5 | 152.6 |
| catalog | 48+48 | 11.973 | 12.082 | +0.82% (0/8) | 136.8 | 78.3 |
| heldout | 64+32 | 11.144 | 11.983 | +8.04% (0/8) | 186.8 | 154.7 |
| heldout | 56+40 | 11.273 | 11.807 | +4.67% (0/8) | 184.6 | 156.3 |
| heldout | 48+48 | 11.908 | 11.881 | -0.18% (4/8) | 133.7 | 83.5 |

## Same-host Aeglos comparison

| Request | Main + G2 workers | Zopro fused ms | Aeglos ms | Paired change | Zopro peak MiB | Aeglos peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | 64+32 | 11.978 | 12.076 | -1.06% (6/8) | 163.0 | 827.3 |
| catalog | 56+40 | 11.844 | 12.076 | -1.97% (7/8) | 152.6 | 827.3 |
| catalog | 48+48 | 12.082 | 12.076 | -0.97% (5/8) | 78.3 | 827.3 |
| heldout | 64+32 | 11.983 | 12.213 | -1.67% (8/8) | 154.7 | 820.6 |
| heldout | 56+40 | 11.807 | 12.213 | -3.53% (8/8) | 156.3 | 820.6 |
| heldout | 48+48 | 11.881 | 12.213 | -2.50% (8/8) | 83.5 | 820.6 |

The 48+48 layout brought fused peak memory to 78.3 and 83.5 MiB. At the same layout the separate path was 136.8 and 133.7 MiB. Both 48+48 alternatives were slower than the 64+32 control in paired timings. The latency scorecard keeps the earlier 64+32 profile.

Each timing process primes three proofs and times seventeen. A paired comparison qualifies with a lower median of block ratios and at least six of eight wins on each input. Timed Aeglos proofs all use prepared keys. The separate probes observed 56,762 K terms and 57,554 H terms at width 13. Gnark independently reverified all 124 C7a exports and four local route preflights. The temporary instance, storage, and bucket were removed.

[Samples and route checks](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
