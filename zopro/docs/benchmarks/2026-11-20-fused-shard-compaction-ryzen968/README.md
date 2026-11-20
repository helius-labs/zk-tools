# Fused shard classification and point compaction on Ryzen 4×2

Fusing K shortened preparation and point compaction by 0.37–0.40 ms. Its MSM windows took about 0.29–0.30 ms longer, leaving full-proof latency near parity. Fusing B2 made the full proof 4.24–4.42% slower than the ordinary path.

The trial uses the [same scalar partition as the preceding K classifier](../2026-11-20-k-shard-output-ryzen966/README.md), with sixteen shards. Each shard identifies exact nonzero scalar classes in original member order and reduces the corresponding affine points before returning. The caller concatenates the compact points and canonical scalar words, then invokes the existing window MSM. Global class order follows shard order. The original classifier and compactor remain available when the scratch reservation is denied.

The additional conservative scratch forecast is 40.94 MiB for K and 65.08 MiB for B2. These are reservation amounts sized for worst-case class survival during construction. The extra charge drops after compact input assembly. In the measured joint arm, whole-process peak RSS was 228.0 and 223.5 MiB, compared with 209.3 and 215.0 MiB for ordinary.

| Input | Mode | Full proof ms | Paired vs ordinary | Paired vs frozen |
|---|---|---:|---:|---:|
| catalog | frozen | 14.506 | — | — |
| catalog | ordinary | 14.504 | — | −0.01% (4/8) |
| catalog | K | 14.585 | +0.37% (2/8) | +0.56% (2/8) |
| catalog | B2 | 15.108 | +4.24% (0/8) | +4.58% (0/8) |
| catalog | K and B2 | 15.132 | +4.21% (0/8) | +4.45% (0/8) |
| heldout | frozen | 14.550 | — | — |
| heldout | ordinary | 14.539 | — | −0.56% (5/8) |
| heldout | K | 14.536 | +0.08% (3/8) | +0.12% (4/8) |
| heldout | B2 | 15.202 | +4.42% (0/8) | +4.89% (0/8) |
| heldout | K and B2 | 15.189 | +4.81% (0/8) | +4.15% (0/8) |

The runner uses 24 main, eight frontend and four solver workers. The unclocked profiles have eight paired blocks per input with three warm-ups and seventeen timed proofs. Four separate clocked blocks per input time thirteen proofs. The paired clock drift stayed within 1.89%.

| Input | Single fused query | Paired preparation and compaction change, ms | Paired window change, ms | Paired total query change, ms |
|---|---|---:|---:|---:|
| catalog | K | −0.404 | +0.301 | −0.071 |
| heldout | K | −0.369 | +0.289 | −0.055 |
| catalog | B2 | +0.947 | −0.273 | +0.672 |
| heldout | B2 | +0.975 | −0.224 | +0.734 |

The busiest of sixteen shards carries 6,789–7,478 nonzero points across the four query and input combinations. The corresponding means are 5,213–5,421. Those counts were independently reproduced from the [pinned scalar captures](../2026-11-20-four-by-two-scalar-relations-ryzen918/README.md) and agree with the trial's class totals.

Two native tests passed. Route probes compared every class's membership and the full curve point against the ordinary query, including both curves running together. Probes at zero experimental scratch verified the serial fallback. All 122 proof exports independently verified. [C7a scorecard](../2026-11-20-curve-query-timing-c7a961/README.md#eight-supplied-m5-examples)

[Results](artifacts/summary.json) · [Shard counts](artifacts/distribution.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz)
