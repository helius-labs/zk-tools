# Two inverse FFT layers with shared radix-52 sums

Two pairs of radix-2 DIF layers in each inverse radix-32 column use a four-input butterfly. For four normalized inputs below 2p it forms the two uncarried limb sums L and R, then carries L+R and 4p+L−R once each. The first is below 8p and is reduced by conditional subtractions of 4p and 2p. The second goes directly into R260 Montgomery multiplication by the next root. Its reduced product is below p+16p²/2²⁶⁰ < 2p. The upper pair of outputs follows the existing twiddle multiplication. Forward column transforms retain their existing arithmetic. No additional key tables or heap workspace are required.

The native field test checks both outputs and limb bounds on unreduced inputs, including values near p and 2p. Three transform and coefficient tests cover root layouts, prefix lengths and actual Euler polynomials. Five randomized blocks compare the frozen grouped build, a new ordinary build and the new four-input path on all eight original requests and two additional public requests. Each process uses 32 Ryzen workers, three warmups and eleven timed proofs, or 31 on the two smallest shapes. The 12×2 and 24×2 profiles use fixed prepared groups across all arms.

| Shape | Request | Frozen / ordinary / candidate ms | Paired vs ordinary | Paired vs frozen | Peak MiB ordinary / candidate | Candidate vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | Original | 7.096 / 7.102 / 7.103 | -0.10% (3/5) | -0.21% (3/5) | 64.2 / 67.0 | 5.21× / 3.38× |
| 2x2 | Original | 9.711 / 9.675 / 9.592 | -0.86% (4/5) | -1.21% (5/5) | 114.0 / 116.0 | 6.15× / 3.13× |
| 4x2 | Original | 15.879 / 15.514 / 15.667 | +0.20% (2/5) | -1.40% (3/5) | 250.0 / 258.2 | 6.96× / 3.06× |
| 8x2 | Original | 24.356 / 24.576 / 24.070 | -2.06% (3/5) | -0.99% (3/5) | 427.2 / 428.5 | 7.85× / 3.07× |
| 12x2 | Original | 29.963 / 29.616 / 28.713 | -3.44% (4/5) | -4.17% (4/5) | 539.3 / 558.4 | 9.89× / 3.52× |
| 16x2 | Original | 39.424 / 39.745 / 39.462 | -1.45% (3/5) | -1.60% (3/5) | 663.2 / 671.3 | 8.49× / 3.14× |
| 24x2 | Original | 60.037 / 59.843 / 60.666 | +0.33% (2/5) | +1.05% (1/5) | 1105.9 / 1100.3 | 8.70× / 2.90× |
| merge-24x1 | Original | 63.918 / 64.029 / 64.618 | +0.35% (1/5) | +1.12% (1/5) | 1015.8 / 1049.7 | 7.85× / 2.71× |
| 12x2 | 142 | 46.413 / 45.600 / 46.322 | +1.58% (2/5) | -0.67% (3/5) | 1022.9 / 1006.6 | — |
| 24x2 | 103 | 58.208 / 57.918 / 58.754 | +1.25% (1/5) | +0.67% (1/5) | 1094.1 / 1104.6 | — |

All 150 process exports verified. [Samples](artifacts/timings.json.gz) · [Source and oracle](artifacts/source.patch) · [Verification](artifacts/verification.json)
