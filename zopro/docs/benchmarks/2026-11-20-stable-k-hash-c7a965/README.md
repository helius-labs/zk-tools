# Stable parallel K scalar classification on C7a 4×2

Eight shards cut about 0.22 ms from K hash classification. That was less than half the saving observed on Ryzen. The original proof was slower and the held-out proof was faster against the same-build control. Sixteen and 32 shards also fell short of the full-proof requirement.

The test used the [exact Ryzen executable and classifier](../2026-11-20-stable-k-hash-ryzen964/README.md) on a c7a.24xlarge. The 64 main and 32 frontend workers occupy disjoint physical cores. K runs on the frontend and the solver uses 16 of those cores. Only K uses the alternate classifier. Each input has twelve paired blocks of seventeen timed proofs after three warm-ups. Separate phase clocks use six paired blocks of thirteen measured proofs.

| Input | Hash layout | Full proof ms | Paired vs ordinary | Paired vs frozen | Paired K hash change, ms | Paired K query change, ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | frozen | 14.558 | — | — | — | — | 214.7 |
| catalog | ordinary | 14.588 | — | +0.42% (5/12) | — | — | 216.1 |
| catalog | 8 shards | 14.728 | +1.78% (3/12) | +0.88% (5/12) | −0.213 | −0.043 | 220.6 |
| catalog | 16 shards | 14.612 | +0.34% (4/12) | +0.93% (3/12) | +0.041 | +0.042 | 231.2 |
| catalog | 32 shards | 14.665 | +0.66% (5/12) | +0.42% (5/12) | −0.055 | +0.040 | 223.7 |
| heldout | frozen | 14.539 | — | — | — | — | 214.0 |
| heldout | ordinary | 14.636 | — | +0.71% (5/12) | — | — | 217.8 |
| heldout | 8 shards | 14.506 | −1.07% (9/12) | −0.03% (6/12) | −0.224 | −0.126 | 209.5 |
| heldout | 16 shards | 14.649 | +0.06% (6/12) | +0.61% (3/12) | +0.053 | +0.165 | 220.7 |
| heldout | 32 shards | 14.600 | −0.08% (6/12) | +0.23% (5/12) | −0.057 | +0.050 | 222.9 |

The ordinary K hash interval was 2.832 and 2.822 ms. In the ordinary traces, K or B2 finished last in 70 of 78 original proofs and 69 of 78 held-out proofs. The clocks include scheduling and allocation. The group interval covers partitioning, per-shard hashing and global class numbering. These measurements leave open whether shared cache-line writes explain the weak scaling across C7a shards.

The paired ordinary control calibrated within 0.72% of frozen. The largest paired observer drift was 2.19%. Route checks compared the complete K class data for all 97,075 inputs, and the zero-scratch route used ordinary classification. All 178 exported proofs independently verified after retrieval. The temporary instance and storage were removed. [Eight-shape M5 CPU and Metal scorecard](../2026-11-20-curve-query-timing-c7a961/README.md#eight-supplied-m5-examples)

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Cleanup](artifacts/cleanup.json)
