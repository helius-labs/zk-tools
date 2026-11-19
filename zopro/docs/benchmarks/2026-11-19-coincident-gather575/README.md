# Gathering both scalar vectors by local wire range

The ordinary solver collects A and B scalars in two parallel operations. The candidate partitions both sorted query lists at the same wire boundary and writes both outputs within each leaf. Each half owns disjoint output slices. A median split in the larger index list bounds recursion even when query sizes differ. This changes the two gathers after witness solving and before the Euler FFT. It reuses the existing Rayon pool and output vectors.

The equality test compares empty, identical, disjoint, overlapping and unequal query lists against two independent indexed reads at five task counts. Each Ryzen process primes three proofs and times eleven. The five shuffled blocks per shape and width compare two independent collectors with sixteen total leaves, a combined collector with sixteen or thirty-two leaves, a same-binary ordinary control, and a production release. Separate traced processes measure both gathers together. The included runner also supports 64 and 96 physical C7a cores; the table uses local Ryzen measurements.

| Shape | Workers | Gather policy | Release / selected ms | Paired vs release | Paired vs ordinary | Gather ms in traced runs | Trace effect | M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 16 | control | 43.718 / 43.638 | +0.23% (2/5) | +0.00% | 1.202 | +0.07% | 7.68× / 2.84× |
| 16x2 | 16 | separate16 | 43.718 / 43.599 | +0.04% (2/5) | -0.09% | 1.136 | +1.68% | 7.68× / 2.84× |
| 16x2 | 16 | shared16 | 43.718 / 43.359 | -0.66% (4/5) | -0.66% | 0.960 | +0.38% | 7.73× / 2.86× |
| 16x2 | 16 | shared32 | 43.718 / 43.856 | -0.28% (3/5) | +0.23% | 0.983 | -0.60% | 7.64× / 2.83× |
| 16x2 | 32 | control | 39.600 / 39.862 | -0.07% (3/5) | +0.00% | 1.358 | +1.07% | 8.40× / 3.11× |
| 16x2 | 32 | separate16 | 39.600 / 39.163 | -1.10% (3/5) | -1.67% | 1.211 | +0.17% | 8.55× / 3.17× |
| 16x2 | 32 | shared16 | 39.600 / 39.863 | +0.66% (1/5) | +0.88% | 1.023 | -1.64% | 8.40× / 3.11× |
| 16x2 | 32 | shared32 | 39.600 / 39.402 | +0.90% (2/5) | +0.52% | 1.175 | +4.49% | 8.50× / 3.15× |
| merge-24x1 | 16 | control | 68.577 / 68.641 | +0.21% (2/5) | +0.00% | 1.767 | -0.17% | 7.39× / 2.55× |
| merge-24x1 | 16 | separate16 | 68.577 / 68.246 | -0.48% (4/5) | -0.54% | 1.752 | -0.89% | 7.43× / 2.56× |
| merge-24x1 | 16 | shared16 | 68.577 / 68.000 | -0.84% (4/5) | -0.47% | 1.456 | +0.28% | 7.46× / 2.57× |
| merge-24x1 | 16 | shared32 | 68.577 / 67.583 | -1.45% (3/5) | -0.91% | 1.461 | +0.71% | 7.50× / 2.59× |
| merge-24x1 | 32 | control | 63.816 / 63.356 | -0.72% (4/5) | +0.00% | 1.923 | +0.43% | 8.00× / 2.76× |
| merge-24x1 | 32 | separate16 | 63.816 / 63.830 | +0.01% (2/5) | +1.10% | 1.802 | +1.41% | 7.94× / 2.74× |
| merge-24x1 | 32 | shared16 | 63.816 / 63.005 | -0.54% (3/5) | -0.92% | 1.514 | -0.59% | 8.05× / 2.78× |
| merge-24x1 | 32 | shared32 | 63.816 / 63.002 | -1.34% (4/5) | -0.59% | 1.499 | +0.49% | 8.05× / 2.78× |

All 180 exports verified independently. [Samples](artifacts/samples.json.gz) · [Source](artifacts/source.patch) · [Host runner](artifacts/run.py) · [Verification](artifacts/verification.json)
