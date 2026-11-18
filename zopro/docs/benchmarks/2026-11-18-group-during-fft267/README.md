# Moving scalar grouping into the FFT on 24×2

On a fresh c7a.24xlarge, accepted Zopro measured **52.62 ms at 48 cores**, the fastest median among the five tested worker counts. This is **10.03× the supplied M5 CPU speed and 3.34× Metal**. The selected width met the numerical target on this instance. 4 of seven process means met 52.8 ms. The width comparison was exploratory.

The candidate groups the A, K and B2 witness scalars alongside the FFT. Each query keeps its existing memory reservation and starts curve arithmetic after the FFT. An unproductive or denied grouping uses the raw path. The earlier full-overlap candidate also ran witness curve arithmetic alongside the FFT.

## AWS comparison

| Cores | Engine / schedule | Wall ms | CPU ms/proof | Peak MiB | Blocks ≤52.8 ms | M5 CPU | M5 Metal |
|---:|---|---:|---:|---:|---:|---:|---:|
| 32 | Accepted Zopro | 60.68 | 1754 | 1261 | 0/7 | 8.70× | 2.90× |
| 32 | Same-binary OFF | 59.59 | 1730 | 1287 | 0/7 | 8.86× | 2.95× |
| 32 | Candidate ON | 73.60 | 1800 | 1190 | 0/7 | 7.17× | 2.39× |
| 32 | Full witness overlap (266) | 73.88 | 1794 | 1280 | 0/7 | 7.15× | 2.38× |
| 32 | Aeglos | 172.69 | 5208 | 6373 | 0/7 | 3.06× | 1.02× |
| 48 | Accepted Zopro | 52.62 | 2078 | 1184 | 4/7 | 10.03× | 3.34× |
| 48 | Candidate ON | 68.51 | 2186 | 1261 | 0/7 | 7.71× | 2.57× |
| 48 | Full witness overlap (266) | 67.36 | 2159 | 1267 | 0/7 | 7.84× | 2.61× |
| 64 | Accepted Zopro | 53.44 | 2555 | 1257 | 2/7 | 9.88× | 3.29× |
| 64 | Candidate ON | 67.42 | 2566 | 1227 | 0/7 | 7.83× | 2.61× |
| 64 | Full witness overlap (266) | 66.16 | 2614 | 1328 | 0/7 | 7.98× | 2.66× |
| 80 | Accepted Zopro | 54.56 | 3075 | 1259 | 0/7 | 9.68× | 3.23× |
| 80 | Candidate ON | 68.24 | 3091 | 1197 | 0/7 | 7.74× | 2.58× |
| 80 | Full witness overlap (266) | 67.03 | 3191 | 1304 | 0/7 | 7.88× | 2.63× |
| 96 | Accepted Zopro | 56.98 | 3753 | 1262 | 0/7 | 9.27× | 3.09× |
| 96 | Same-binary OFF | 56.70 | 3612 | 1226 | 0/7 | 9.31× | 3.10× |
| 96 | Candidate ON | 71.07 | 3745 | 1261 | 0/7 | 7.43× | 2.48× |
| 96 | Full witness overlap (266) | 68.68 | 3867 | 1288 | 0/7 | 7.69× | 2.56× |
| 96 | Aeglos | 86.18 | 7338 | 6787 | 0/7 | 6.13× | 2.04× |

At 32 cores the median paired grouping-on/off change was +22.40% in latency and +3.24% in CPU time. Grouping overlap won 0/7 blocks. OFF/accepted calibration was 0.993 against a 1.03 multiplicative tolerance. At 96 cores the median paired grouping-on/off change was +24.75% in latency and +4.25% in CPU time. Grouping overlap won 0/7 blocks. OFF/accepted calibration was 1.008 against a 1.03 multiplicative tolerance.

Each median uses seven uninstrumented process means, 21 timed proofs and three primes per process. Peak RSS is the maximum over the seven processes and their preflight. All 2,812 timed requests used the pinned key and witness. One proof per process was exported. All 152 exports verified on AWS and again after retrieval. The EC2 instance terminated, the tagged EBS query returned zero volumes and the temporary bucket was removed. [Protocol](artifacts/protocol.md) · [Block means at every width](artifacts/core-counts.json) · [Paired ratios](artifacts/analysis.json) · [Verification](artifacts/verification.json)

## Local shape coverage

The Ryzen 9950X used 16 pinned workers, one process per setting and shape, three primes and seven timed proofs. All 18 exports verified. The candidate engaged on seven of the nine shapes. The peak is lifetime process RSS. The local screen measures route coverage and gives an exploratory latency comparison.

| Shape | Routed / 7 | OFF ms | ON ms | Peak OFF / ON MiB | M5 CPU / ON | M5 Metal / ON |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 0/7 | 8.38 | 8.63 | 66 / 62 | 4.29× | 2.78× |
| 2x2 | 0/7 | 12.10 | 12.00 | 142 / 142 | 4.92× | 2.50× |
| 4x2 | 7/7 | 16.79 | 17.47 | 196 / 219 | 6.24× | 2.75× |
| 8x2 | 7/7 | 26.72 | 28.97 | 329 / 383 | 6.52× | 2.55× |
| 12x2 | 7/7 | 38.76 | 43.14 | 486 / 581 | 6.58× | 2.34× |
| 16x2 | 7/7 | 46.37 | 52.10 | 558 / 683 | 6.43× | 2.38× |
| 24x2 | 7/7 | 74.06 | 88.95 | 890 / 880 | 5.94× | 1.98× |
| merge-24x1 | 7/7 | 76.19 | 83.13 | 793 / 822 | 6.10× | 2.11× |
| merge-54x1 | 7/7 | 172.03 | 197.75 | 1514 / 1644 | — | — |

The source passed the native check and a separate generic build check. The local trace in experiment 268 further measures the effect on the FFT and H query. [Local exports](artifacts/local-review.json) · [Source changes](artifacts/changes.patch.gz)
