# Direct allocation for the Euler FFT column cache on C7a

The direct builder reserves the actual requested root storage under the existing 64 MiB preparation ceiling. It fills a fixed allocation per FFT level. The control requests 128 MiB on both tested domains and falls back to the ordinary layer layout. The C7a route probes built 39.96 MiB of root payload per process for the direct variant. A zero preparation budget declined all cache allocations and completed valid proofs.

Both shapes fell short of the paired latency criterion. On 12×2, the same-build catalog control also exceeded the 2% calibration range. The direct build lost against the frozen binary on both 12×2 inputs. Peak RSS increased by 44.5–53.1 MiB against the same-build controls. The candidate stays isolated.

| Shape | Input | Frozen / control / direct warm ms | Direct vs control | Direct vs frozen | Control / direct cold ms | Control / direct peak warm MiB |
|---|---|---:|---:|---:|---:|---:|
| 12×2 | catalog | 29.370 / 29.895 / 29.659 | -0.86% (6/8) | +1.26% (1/8) | 312.8 / 316.8 | 542.3 / 586.8 |
| 12×2 | heldout | 29.446 / 29.573 / 29.611 | +0.53% (2/8) | +0.40% (2/8) | 313.7 / 317.7 | 539.4 / 588.9 |
| 16×2 | catalog | 31.735 / 31.328 / 31.138 | -0.20% (6/8) | -1.37% (6/8) | 403.7 / 404.1 | 664.8 / 717.9 |
| 16×2 | heldout | 31.179 / 31.570 / 31.222 | -0.82% (5/8) | -0.62% (5/8) | 404.8 / 403.2 | 680.5 / 728.9 |

One c7a.24xlarge with 96 physical cores ran the same 64 main and 32 frontend worker settings as the three-prover benchmark. Eight order-balanced blocks time thirteen warm proofs after three primes. Warm values are the medians of the block means. Cold values are the medians of first verified responses from fresh processes with a shared host file cache. Peak RSS is the maximum whole-process peak in the warm panel. First-response RSS is recorded in the summary artifact. The candidate qualifies with a lower paired median and at least seven paired wins against each control on both inputs, with same-build control calibration within 2% of the frozen binary.

## Supplied M5 references

| Shape | 10× M5 CPU / Metal target ms | Direct catalog speedup CPU / Metal | Direct held-out speedup CPU / Metal |
|---|---:|---:|---:|
| 12×2 | 28.4 / 10.1 | 9.58× / 3.41× | 9.59× / 3.41× |
| 16×2 | 33.5 / 12.4 | 10.76× / 3.98× | 10.73× / 3.97× |

The broader C7a report covers all eight supplied reference shapes. The two shapes above used the same requests, images, and frozen Zopro executable in this run. Gnark independently reverified all 216 first and warm proof exports after retrieval. The temporary EC2 instance, EBS volume, and S3 bucket were removed.

[Samples and route probes](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Local trial and source patch](../2026-11-20-fft-bounded-column-cache-ryzen1046/README.md) · [All eight M5 references](../2026-11-20-c7a-three-provers/README.md#supplied-m5-references)
