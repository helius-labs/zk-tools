# Combining public Euler origins and curve groups on 24×2

The offline conversion folds a fixed public reference into the K points, retains the original private slots and exports a proof verified with the original key. A streaming reader verifies that only the appended dictionary changes when the compact block layout replaces the flat dictionary. All Zopro settings use identical external wire offsets. The grouped modes prepare the same A/K/B2 indices against the actual loaded points. Five shuffled blocks use 32 pinned Ryzen workers, three warmups and eleven measured calls. The two additional public transfers are held out. The Aeglos arm uses its prepared key on all measured calls. An independent verifier checks the public outputs and proof of every process.

| Request | Release / grouped / blocked / combined / Aeglos ms | Combined vs grouped / Aeglos | Max combined / Aeglos RSS MiB | Combined vs M5 CPU / Metal |
|---|---:|---:|---:|---:|
| Original | 64.702 / 59.956 / 63.081 / 58.838 / 457.680 | +0.03% (2/5) / -87.13% (5/5) | 1093.7 / 6355.8 | 8.97× / 2.99× |
| 99 | 64.506 / 57.953 / 62.191 / 58.338 / 455.937 | -0.83% (3/5) / -87.24% (5/5) | 1082.7 / 6340.6 | — |
| 103 | 63.699 / 57.990 / 63.100 / 58.644 / 458.561 | +0.91% (1/5) / -87.22% (5/5) | 1121.5 / 6338.5 | — |

All 76 exports verified. [Complete process samples](artifacts/samples.json.gz) · [Conversion and key identities](artifacts/inputs.json) · [Verification](artifacts/verification.json)

Against the release, grouping alone reduced the median paired latency by 7.91%, 10.14% and 8.85% on the original, 99 and 103 respectively. The combined setting reduced it by 8.61%, 9.49% and 8.36%. Both settings beat the release in every block on all three inputs. The combined setting beat grouping alone in two of five pairs on the original. On that request the 10× CPU target is 52.8 ms, still 6.038 ms below the combined median.

The offline converter handled a domain of 1,048,576, retained all 544,587 private slots and added none. It reported 359.917 seconds for setup. Some development compilation and correctness checks shared the host during conversion. All timed comparisons began afterward. The flat image occupies 347,459,560 bytes and the block image 346,929,782 bytes.

## CPU consumption and stage timing

The CPU figures cover the 11 verified calls in each timed loop. Effective CPUs divide user plus system CPU time by elapsed loop time. Peak RSS includes key preparation, and minor faults count process page faults during timing. The table reports the median of five processes per cell.

| Reference request | CPU ms/proof | Effective CPUs out of 32 | Minor faults/proof | Median process peak MiB |
|---|---:|---:|---:|---:|
| release | 1880.0 | 29.1 | 5723 | 1022.2 |
| grouped | 1680.1 | 28.2 | 6304 | 1080.2 |
| blocked | 1816.1 | 28.9 | 5620 | 998.4 |
| both | 1639.0 | 27.9 | 5789 | 1069.8 |
| aeglos | 14281.9 | 31.2 | 1720 | 6347.9 |

Aeglos reports separate elapsed times for MSM, FFT and witness construction on each call. The remainder includes all other work in the verified request. Each column is a median of process means, so their sum can differ slightly from the reported overall median.

| Aeglos request | MSM ms | FFT ms | Witness ms | Remaining ms |
|---|---:|---:|---:|---:|
| Original | 391.98 | 52.41 | 11.15 | 2.15 |
| 99 | 390.93 | 52.15 | 10.24 | 2.28 |
| 103 | 392.76 | 52.39 | 11.13 | 2.30 |

[Per-process costs](artifacts/process-costs.json)
