# Full merge proofs with a packed public Euler origin

Each SDK process loads the original merge-24×1 key once and proves five requests on 32 Ryzen workers. The middle process prepares replacement G1 coefficients and uses them for all five inputs. The public origin occupies 542,093 bytes with 8 bits per row. Proofs read the packed identifiers directly. The two ordinary processes bracket the transformed one with the same input order. An attempted trailing control inherited the origin setting and was stopped during setup. The replacement control cleared the setting explicitly and was required to emit no conversion event. Each input primes three requests before its 21 measured proofs. A single converted process measures feasibility across inputs and does not establish repeatable latency.

Conversion took 117.03 seconds on this host and added 170,797 retained private solver slots. Peak RSS includes the conversion's temporary group-valued arrays and earlier requests within each process. A production offline converter would be needed to avoid this setup cost on devices. The palette width was selected using the earlier structural screen, preferring the smallest encoding retaining 95% of removed H terms on all five inputs, with eight bits as the fallback.

| Request | Ordinary / packed / second ordinary ms | Peak MiB in the same order | Packed vs M5 CPU / Metal |
|---|---:|---:|---:|
| Original | 63.240 / 68.108 / 64.730 | 1411.4 / 1707.2 / 1389.1 | 7.44× / 2.57× |
| 109 | 65.994 / 70.139 / 66.028 | 1409.6 / 1689.4 / 1385.6 | — |
| 112 | 67.385 / 71.412 / 67.827 | 1187.0 / 1509.0 / 1181.1 | — |
| 122 | 66.339 / 71.117 / 66.855 | 1248.6 / 1571.5 / 1282.5 | — |
| 124 | 68.211 / 73.086 / 68.144 | 1346.4 / 1658.5 / 1334.9 | — |

All 15 exports verified with the original verification key and independently derived public input hashes. [Runtime samples](artifacts/samples.json) · [Source](artifacts/source.patch) · [Palette and SDK](artifacts/profile.json) · [Verification](artifacts/verification.json)
