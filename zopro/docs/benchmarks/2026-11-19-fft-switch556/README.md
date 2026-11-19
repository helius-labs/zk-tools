# One executable for the two inverse FFT arithmetic paths

The benchmark switches the inverse column algorithm in one process-wide setting. Both arms use the same executable and memory layout on disk. Frozen separately compiled executables from the first screen provide a second comparison. Another pair of processes times the complete Euler product on each verified request. A drop guard covers early and ordinary returns and captures the timestamp before emitting it. Five randomized blocks at 32 Ryzen workers use identical public inputs per pair. The timer effect is measured against the matching untraced switch setting. Each timed process warms three proofs and measures eleven calls, or 31 for 1×2.

| Shape | Request | Single binary ordinary / paired layers ms | Paired change | Frozen binary paired change | Euler ordinary / paired layers ms | Timer effect ordinary / paired layers | Original vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x2 | Original | 7.082 / 7.183 | +1.01% (1/5) | +0.69% (2/5) | 0.556 / 0.555 | +0.18% / -0.70% | 5.15× / 3.34× |
| 12x2 | Original | 29.225 / 29.191 | +0.51% (1/5) | -1.46% (5/5) | 8.685 / 8.710 | -0.31% / -0.44% | 9.73× / 3.46× |
| 24x2 | Original | 59.299 / 58.701 | -0.85% (4/5) | +0.77% (2/5) | 20.196 / 20.432 | -0.20% / +0.12% | 8.99× / 3.00× |
| 12x2 | 58 | 29.239 / 29.140 | +0.14% (2/5) | +1.03% (0/5) | 8.802 / 8.924 | +0.76% / +2.02% | — |
| 12x2 | 142 | 45.866 / 45.557 | -0.17% (3/5) | +2.51% (1/5) | 9.052 / 8.789 | +1.26% / -0.87% | — |

All 150 process exports verified independently. [Raw process and Euler times](artifacts/samples.json.gz) · [Runtime switch and timing source](artifacts/source.patch) · [Verification](artifacts/verification.json)
