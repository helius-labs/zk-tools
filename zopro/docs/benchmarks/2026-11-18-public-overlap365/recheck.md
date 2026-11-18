# Independent 96-core comparison on 1×2

All five configurations ran in eleven new randomized blocks on the same C7a. Each fresh process timed 101 verified SDK calls after three warmups. The first trial and this confirmation use separate processes and proof exports. Peak RSS is the highest process high-water mark.

| Prover and setting | Median ms | CPU-ms | Peak MiB | Paired change vs Aeglos | Blocks faster than Aeglos | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|
| aeglos | 7.960 | 515.0 | 478 | — | — | 4.65× / 3.02× |
| release | 8.259 | 405.4 | 51 | +3.85% | 0/11 | 4.48× / 2.91× |
| off | 8.290 | 389.0 | 53 | +5.37% | 0/11 | 4.46× / 2.89× |
| late | 8.237 | 387.5 | 51 | +4.00% | 0/11 | 4.49× / 2.91× |
| early | 8.164 | 382.3 | 51 | +3.01% | 0/11 | 4.53× / 2.94× |

Early versus release measured -1.22% paired, with 10/11 lower process means.

Early versus off measured -1.51% paired, with 10/11 lower process means.

Early versus late measured -1.00% paired, with 10/11 lower process means.

Early versus aeglos measured +3.01% paired, with 0/11 lower process means.

[Protocol](artifacts/recheck-protocol.md) · [All process samples](artifacts/recheck.json.gz)
