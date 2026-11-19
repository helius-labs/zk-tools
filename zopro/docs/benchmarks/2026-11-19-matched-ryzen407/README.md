# Zopro and Aeglos verified proof latency on Ryzen

Both provers processed the same requests, with the same 32-CPU affinity mask on a 16-core Ryzen 9950X. Aeglos used 32 native workers and GOMAXPROCS=32. Zopro used 32 pinned main workers, with eight additional frontend workers on the same mask for the two smallest shapes. Each process primed three requests. All timed Aeglos requests reported a cache hit. The full request includes witness preparation, proof generation and verification. Peak RSS covers key loading, preparation and requests over the process lifetime. Five randomized matched blocks underlie each row.

| Shape | Zopro ms | Aeglos ms | Paired Zopro change | Lower Zopro latency / 5 | CPU-ms Zopro / Aeglos | Peak MiB Zopro / Aeglos | Zopro speedup vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.342 | 12.855 | -43.28% | 5 | 152.1 / 377.5 | 63.0 / 424.1 | 5.04× / 3.27× |
| 2x2 | 10.049 | 20.843 | -51.64% | 5 | 200.5 / 619.6 | 114.0 / 734.0 | 5.87× / 2.99× |
| 4x2 | 15.919 | 38.227 | -58.50% | 5 | 417.0 / 1166.0 | 239.5 / 1262.6 | 6.85× / 3.02× |
| 8x2 | 24.098 | 86.792 | -72.20% | 5 | 667.8 / 2673.8 | 400.0 / 2287.6 | 7.84× / 3.07× |
| 12x2 | 33.836 | 183.952 | -81.61% | 5 | 942.1 / 5786.4 | 566.0 / 3413.1 | 8.39× / 2.98× |
| 16x2 | 40.728 | 240.256 | -83.05% | 5 | 1169.8 / 7526.9 | 688.2 / 4311.8 | 8.23× / 3.04× |
| 24x2 | 69.213 | 460.215 | -84.94% | 5 | 2027.4 / 14431.2 | 1061.8 / 6336.7 | 7.63× / 2.54× |
| merge-24x1 | 67.071 | 445.857 | -84.89% | 5 | 1932.5 / 13889.6 | 1004.5 / 6140.6 | 7.56× / 2.61× |
| merge-54x1 | 173.643 | 1002.484 | -82.75% | 5 | 4861.4 / 31511.8 | 1659.1 / 12541.3 | — |

Zopro had lower median latency on all nine shapes and lower maximum process RSS on each. The 12×2 row keeps compact Poseidon outputs disabled to match the prespecified scorecard. A separate confirmation with that option enabled measured 33.002 ms. [12×2 configuration](../2026-11-19-medium-h-ryzen406/README.md)

The previous 96-physical-core C7a comparison measured 8.158 ms for Zopro and 8.081 ms for Aeglos on 1×2. It used an earlier Zopro build. [C7a comparison](../2026-11-18-claims-c7a374/README.md)

The merge-54×1 comparison has no supplied M5 timing. Each final process export passed independent Gnark verification against the same expected public-input hash for its shape. This comparison uses the retained Aeglos CPU revision recorded with the artifacts. Performance on C7a requires its own matched measurements.

[Protocol](artifacts/protocol.md) · [Aeglos stage timings](artifacts/aeglos-stages.json) · [Process samples](artifacts/processes.json.gz) · [Verification and build identities](artifacts/verification.json)
