# Four-probe CPU timelines on C7a

With fewer probes, the observed order held. On 1×2 held-out, G2 finished after the observed G1 call in all 66 traces, by a median of 1.38 ms. On 2×2, at least 1 ms remained between G2 return and core return in 62 of 66 primary traces and 64 of 66 held-out traces. The remaining work can include curve operations outside the observed canonical G1 call.

The four-probe pass added 3.13–5.14% paired proof latency. The broader probe set added 9.43–28.81%. The observed order is an optimization lead. Plain timings supply the latency comparison.

| Shape | Request | Plain ms | Traced ms | Paired trace cost | G2 after observed G1 ms | G2 return to core return ms | G2 finishes after G1 |
|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | 7.722 | 8.135 | +5.14% | +0.172 | 0.296 | 46/66 |
| 1×2 | heldout | 7.727 | 7.949 | +3.13% | +1.378 | 0.358 | 66/66 |
| 2×2 | catalog | 10.331 | 10.677 | +3.58% | -0.357 | 1.425 | 13/66 |
| 2×2 | heldout | 10.375 | 10.711 | +3.37% | -0.473 | 1.386 | 8/66 |

The paired run uses two inputs per shape, six blocks and eleven timed proofs per process. Plain and traced binaries are byte-identical on different inodes. Gnark independently reverified all 48 exports. All 264 warm traces had complete event pairs and no lost records. The temporary C7a instance and storage were removed.

[Full trace comparison](../2026-11-20-current-cpu-critical-path-c7a1032/README.md) · [All eight M5 comparisons](../2026-11-20-geometry-placement-c7a1027/README.md#all-eight-supplied-m5-references) · [Summary](artifacts/summary.json) · [Samples and spans](artifacts/rows.json.gz)
