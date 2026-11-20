# K/H fusion for 1×8 on C7a

At fixed 32+64 worker placement, K/H fusion lowered paired latency by 3.47% on the primary input and 3.51% on the held-out input. Primary peak process memory moved from 139.7 to 78.3 MiB.

K and H are two G1 sums that the CPU proof consumes as a combined result. The fused path uses one bucket set per window and preserves the separate path when scratch admission fails. This run keeps the SDK, proving key, request, and G2 preparation fixed and changes only K/H fusion and the listed main/G2 worker placement.

| Request | Main + G2 workers | Separate proof ms | Fused proof ms | Paired fusion change | Separate peak MiB | Fused peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | 32+64 | 10.388 | 10.012 | -3.47% (8/8) | 139.7 | 78.3 |
| catalog | 40+56 | 10.608 | 9.931 | -6.45% (8/8) | 145.7 | 68.5 |
| catalog | 40+40 | 11.005 | 10.848 | -1.16% (8/8) | 143.5 | 75.3 |
| heldout | 32+64 | 10.370 | 9.984 | -3.51% (8/8) | 143.5 | 74.6 |
| heldout | 40+56 | 10.666 | 9.884 | -7.06% (8/8) | 135.9 | 67.7 |
| heldout | 40+40 | 11.017 | 10.748 | -2.07% (7/8) | 146.5 | 73.8 |

## Same-host Aeglos comparison

| Request | Main + G2 workers | Zopro fused ms | Aeglos ms | Paired change | Zopro peak MiB | Aeglos peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | 32+64 | 10.012 | 10.748 | -7.20% (8/8) | 78.3 | 691.0 |
| catalog | 40+56 | 9.931 | 10.748 | -7.92% (8/8) | 68.5 | 691.0 |
| catalog | 40+40 | 10.848 | 10.748 | +0.26% (3/8) | 75.3 | 691.0 |
| heldout | 32+64 | 9.984 | 10.715 | -8.71% (8/8) | 74.6 | 689.5 |
| heldout | 40+56 | 9.884 | 10.715 | -7.80% (8/8) | 67.7 | 689.5 |
| heldout | 40+40 | 10.748 | 10.715 | +0.51% (3/8) | 73.8 | 689.5 |

The 40+56 fused profile recorded the lowest peak memory and lowest raw medians. It won three of eight primary blocks and four of eight held-out blocks against the 32+64 fused profile, so the scorecard uses the fixed-placement 32+64 result.

Each timing process primes three proofs and times seventeen. A paired comparison qualifies with a lower median of block ratios and at least six of eight wins on each input. Timed Aeglos proofs all use prepared keys. The separate probes observed 42,763 K terms and 43,162 H terms at width 13. Gnark independently reverified all 124 C7a exports and four local route preflights. The temporary instance, storage, and bucket were removed.

[Samples and route checks](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
