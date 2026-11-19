# Preconverted Euler keys for 12×2 transfers

The four-fixture public modal reference needs 739,416 bytes on disk and 878,372 bytes for its decoded dictionary and indices. A reference taken from a single generated fixture removes slightly more nonzero terms in the earlier screen but needs 3,550,308 decoded bytes. Each dictionary independently reconstructed the corresponding full reference byte for byte. The converter used the smaller dictionary and retained the existing K slots.

Five shuffled blocks on 32 Ryzen workers compare ordinary and mapped images in the same SDK. Both load the qualified external wire profile. The original request and four held-out transfers use identical input order across arms in each block. Each input primes three requests and times fifteen. Proof and public input exports were verified against the ordinary image.

| Input | Ordinary / mapped ms | Paired difference | Median process peak MiB, ordinary / mapped | M5 CPU / Metal |
|---|---:|---:|---:|---:|
| Original | 31.329 / 30.513 | -2.20% (5/5) | 1062.2 / 1056.7 | 9.31× / 3.31× |
| 53 | 31.246 / 30.865 | -2.30% (4/5) | 614.2 / 630.9 | — |
| 56 | 31.595 / 30.950 | -2.04% (3/5) | 1043.0 / 1003.4 | — |
| 141 | 47.138 / 47.278 | -0.10% (3/5) | 1045.1 / 1030.6 | — |
| 142 | 46.484 / 46.674 | +0.65% (2/5) | 1173.0 / 1161.2 | — |

The supplied CPU target is 28.4 ms on the original input. All 51 exports passed verification, including the conversion process. [Samples](artifacts/samples.json.gz) · [Key identities](artifacts/image.json) · [Dictionary sizes](artifacts/dictionaries.json) · [Verification](artifacts/verification.json)
