# Mapped B2 group sums and contiguous fallback
     
The experimental key retains every original B2 point, reorders grouped points contiguously, and adds a mapped prefix of group sums. The loader checks the wire permutation, boundaries and group sums on setup. The SDK reads solver wire IDs in mapped order. An optional sparse proof uses group representatives and exact residuals. Denying optional memory exercises a complete dense MSM on the reordered points. The ordinary and adaptive controls use the original image. Integrated adds adaptive A and K groups to the mapped layout. The 12×2 groups are fixed across requests and identical between the adaptive and integrated configurations.

All four configurations used 32 pinned Ryzen workers, four randomized blocks, three warmups and eleven timed proofs. Peak RSS includes key loading. The verification process used the original image. Forced dense and zero-allowance probes covered all three shapes. Corrupt sum, duplicate wire and invalid boundary fixtures were rejected during loading.

| Shape | Request | Ordinary / adaptive / mapped / integrated ms | Integrated vs adaptive | Peak MiB in same order |
|---|---|---:|---:|---:|
| 8x2 | Original | 23.693 / 23.669 / 23.295 / 23.268 | -0.87% (4/4) | 441.2 / 532.8 / 471.8 / 505.0 |
| 8x2 | 153 | 24.299 / 23.207 / 23.179 / 23.014 | -0.11% (2/4) | 427.8 / 494.5 / 497.8 / 505.0 |
| 12x2 | Original | 31.862 / 30.010 / 29.391 / 29.590 | -0.55% (2/4) | 579.3 / 531.0 / 512.5 / 510.1 |
| 12x2 | 59 | 31.187 / 29.157 / 29.446 / 29.394 | +0.91% (1/4) | 581.6 / 505.7 / 514.8 / 510.8 |
| 12x2 | 141 | 47.975 / 47.663 / 50.195 / 49.911 | +4.70% (0/4) | 989.9 / 962.4 / 973.4 / 940.4 |
| 24x2 | Original | 62.897 / 58.438 / 63.809 / 61.925 | +4.94% (0/4) | 1042.2 / 1106.5 / 1070.4 / 1121.2 |
| 24x2 | 97 | 63.557 / 57.128 / 62.707 / 60.091 | +5.19% (0/4) | 1016.5 / 1152.7 / 1041.0 / 1085.0 |
| 24x2 | 103 | 63.479 / 58.378 / 62.899 / 61.829 | +5.74% (0/4) | 1028.7 / 1161.7 / 1044.6 / 1108.1 |

[Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Key sizes](artifacts/images.json) · [Source changes](artifacts/incremental.patch)
