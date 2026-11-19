# Combining solver task packing and task claiming on Ryzen

Task packing merges adjacent independent components within blended outer dependency bands. Task claiming assigns the longest remaining job to an available worker. The four factorial arms share one SDK executable. The release and packing prototype use two separately qualified executables. Address encoding is disabled throughout. Every process used the same request and exported an independently verified proof.

| Shape | Pairs | Release ms | Prototype ms | Both off ms | Claim ms | Pack ms | Combined ms | Paired vs release (wins) | Paired vs claim (wins) | Paired vs pack (wins) | Peak MiB release / combined | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 9 | 7.351 | 7.294 | 7.615 | 7.340 | 7.303 | 7.336 | -0.21% (6) | +0.36% (4) | +0.55% (3) | 65.5 / 72.2 | 5.04× / 3.27× |
| 2x2 | 9 | 10.324 | 10.247 | 10.655 | 10.239 | 10.328 | 10.169 | -1.04% (8) | -0.48% (6) | -0.21% (6) | 126.8 / 120.2 | 5.80× / 2.95× |
| 4x2 | 3 | 16.953 | 17.145 | 16.943 | 16.987 | 17.254 | 17.228 | +1.25% (0) | +1.42% (0) | +0.31% (1) | 208.5 / 202.2 | 6.33× / 2.79× |
| 8x2 | 3 | 26.500 | 26.365 | 26.872 | 26.559 | 26.431 | 26.316 | -0.58% (2) | -0.91% (2) | -0.44% (3) | 351.0 / 362.8 | 7.18× / 2.81× |
| 12x2 | 3 | 38.463 | 39.212 | 38.954 | 38.992 | 38.935 | 39.340 | -0.53% (2) | +0.65% (1) | +0.97% (1) | 500.0 / 512.0 | 7.22× / 2.57× |
| 16x2 | 5 | 45.575 | 45.432 | 45.419 | 45.437 | 45.318 | 45.825 | +0.55% (2) | +0.85% (2) | +0.93% (1) | 627.5 / 641.0 | 7.31× / 2.71× |
| 24x2 | 3 | 74.019 | 73.650 | 73.285 | 73.194 | 72.810 | 73.053 | -1.90% (2) | -1.38% (2) | -0.31% (2) | 978.2 / 930.0 | 7.23× / 2.41× |
| merge-24x1 | 3 | 71.339 | 72.864 | 71.167 | 72.218 | 72.793 | 71.843 | +2.18% (1) | +1.11% (1) | +0.34% (1) | 911.8 / 862.2 | 7.06× / 2.44× |

On 2×2, the combination beat the release in eight of nine pairs and each constituent alone in six of nine. On 1×2, packing alone beat the combination in six of nine pairs. On 16×2, the combination was slower than the release, packing alone and claiming alone. The next qualification focuses on the 2×2 result. All 9,109 timed requests used the full SDK, and all 229 exported proofs passed independent Gnark verification. [Packing implementation](../2026-11-18-packed-waves377/README.md) · [Claiming implementation](../2026-11-18-wave-claims-release379/README.md) · [Executable and source hashes](artifacts/source.json)

The prespecified primary gate requires six of nine wins and lower paired and standalone medians against the release, claiming alone and packing alone. 1x2 missed, 2x2 passed. The zero optional preparation run used the fallback and exported a verified proof.

[Protocol](artifacts/protocol.md) · [All processes](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
