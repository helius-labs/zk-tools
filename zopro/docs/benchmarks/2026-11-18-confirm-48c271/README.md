# Independent 48-core confirmation on 24×2

Accepted Zopro at 48 physical cores measured **52.731 ms** on a second c7a.24xlarge. **5/7** fresh process means were at or below 52.8 ms. This met the prespecified confirmation criterion for 10× the supplied 528 ms M5 CPU timing. The measured ratios were **10.013× M5 CPU and 3.338× M5 Metal**. The 48-core peak process RSS was 1331 MiB.

| Engine and workers | Wall ms | CPU ms/proof | Peak MiB | Blocks ≤52.8 ms | M5 CPU | M5 Metal |
|---|---:|---:|---:|---:|---:|---:|
| Zopro, 48 cores (primary) | 52.731 | 2083 | 1331 | 5/7 | 10.013× | 3.338× |
| Zopro, 64 cores | 52.881 | 2513 | 1329 | 3/7 | 9.985× | 3.328× |
| Zopro, 96 cores | 56.166 | 3711 | 1342 | 0/7 | 9.401× | 3.134× |
| Aeglos, 48 cores | 127.899 | 5668 | 6642 | 0/7 | 4.128× | 1.376× |

The first host measured 52.623 ms at 48 cores, with four of seven blocks at the target. That width was selected from a five-width screen. Here the binary and primary width were fixed before launch. Each main block used 41 timed requests after three primes. Latency and CPU are medians of the seven process means. Peak RSS includes key loading and preflight. The 48-core primary block means, in execution order, were 53.374, 52.943, 52.297, 52.517, 52.731, 52.145, 52.738 ms. The 64- and 96-core controls are secondary. [First host and local eight-shape comparison](../2026-11-18-group-during-fft267/README.md)

The two implementations used the same fixture and expected public witness. Each of the 32 processes exported a fresh proof that passed Gnark in the cloud and on retrieval. This includes 1,148 main timed requests and four single-request preflights. The instance terminated, tagged EBS volumes were absent, and the temporary bucket was removed. [Protocol](artifacts/protocol.md) · [Results by block](artifacts/analysis.json) · [Verification](artifacts/verification.json)
