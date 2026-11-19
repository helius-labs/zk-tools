# Collecting grouped A-query scalars directly from the solved witness

For each admitted request the experimental prover reads A-group members through the existing wire index, builds canonical representatives and exact residuals, and evaluates them alongside the ordinary curve queries. This avoids materializing the full A vector and moves representative selection and residual collection ahead of the FFT. Admission uses the same sample and mismatch criteria as the grouped dense path. Scattered requests can take the dense fallback. The block dictionary, early row centering, K and B2 processing, CPU mask and public corrections stay common across the three settings.

Five diagnostic processes compare each admitted A result to a complete dense A MSM before timing, or record dense fallback. Seven shuffled timing blocks each use three warmups and 21 complete SDK calls per process with 30 pinned Ryzen workers. All 110 exported proofs verified against the original key and expected public output.

| Request | Diagnostic path | Frozen / control / indexed ms | Paired indexed vs control / frozen | Max control / indexed RSS MiB | Indexed vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|
| Original | indexed | 28.648 / 28.581 / 29.239 | +2.39% (1/7) / +2.06% (0/7) | 606.0 / 626.5 | 9.71× / 3.45× |
| 57 | indexed | 28.787 / 28.615 / 29.273 | +1.25% (0/7) / +1.08% (1/7) | 622.8 / 612.0 | — |
| 61 | indexed | 28.449 / 28.536 / 29.210 | +2.52% (0/7) / +2.81% (0/7) | 629.5 / 608.5 | — |
| 139 | fallback | 44.633 / 45.050 / 44.954 | -0.46% (4/7) / +0.61% (1/7) | 1113.6 / 1096.3 | — |
| 143 | fallback | 45.449 / 45.018 / 45.147 | +0.33% (3/7) / -0.34% (5/7) | 1168.5 / 1129.5 | — |

[Samples and audits](artifacts/samples.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
