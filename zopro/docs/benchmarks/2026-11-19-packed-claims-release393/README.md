# Clean solver task packing with task claiming on Ryzen

The native opt-in packing policy blends outer dependency depth and work count, then coalesces adjacent independent components up to 128 operations in each outer or refined band. The same executable runs the claiming-only, packing-only and combined arms. The release and earlier prototype have separate executable controls. Address encoding is active on 16×2 in all arms. Each timing is the median of per-process means for full warm SDK requests.

| Shape | Blocks | Release ms | Claim ms | Pack ms | Prototype ms | Combined ms | Paired vs release (wins) | Paired vs claim (wins) | Paired vs pack (wins) | Peak MiB release / combined | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 9 | 7.357 | 7.379 | 7.297 | 7.278 | 7.307 | -0.95% (6) | -0.98% (8) | -0.15% (6) | 67.5 / 67.0 | 5.06× / 3.28× |
| 2x2 | 15 | 10.374 | 10.362 | 10.237 | 10.177 | 10.248 | -1.01% (12) | -1.06% (12) | -0.67% (10) | 119.5 / 119.2 | 5.76× / 2.93× |
| 4x2 | 3 | 17.018 | 16.818 | 17.338 | 17.134 | 17.196 | +2.89% (0) | +2.31% (0) | +0.53% (1) | 207.8 / 203.5 | 6.34× / 2.79× |
| 8x2 | 3 | 26.484 | 26.895 | 26.690 | 26.378 | 26.517 | +0.66% (0) | -0.84% (2) | -1.00% (3) | 348.0 / 347.5 | 7.13× / 2.79× |
| 12x2 | 3 | 39.056 | 38.824 | 39.021 | 38.257 | 38.497 | -0.95% (2) | -0.39% (2) | -0.82% (3) | 497.0 / 513.5 | 7.38× / 2.62× |
| 16x2 | 5 | 44.776 | 44.917 | 44.743 | 44.765 | 44.784 | -0.31% (3) | -0.21% (4) | +0.09% (2) | 637.8 / 626.8 | 7.48× / 2.77× |
| 24x2 | 3 | 73.509 | 73.592 | 73.846 | 73.555 | 72.619 | -0.80% (3) | -1.46% (3) | -1.26% (3) | 928.7 / 931.1 | 7.27× / 2.42× |
| merge-24x1 | 3 | 72.439 | 71.195 | 72.319 | 71.837 | 72.005 | +0.33% (1) | +1.82% (1) | -0.43% (2) | 876.0 / 869.3 | 7.04× / 2.43× |

On 2×2 the combination took 10.248 ms and beat both the release (10.374 ms) and claiming alone (10.362 ms) in 12 of 15 paired blocks. Against packing alone it won 10 of 15, yet packing alone had a slightly lower standalone median at 10.237 ms. This failed one prespecified condition for promoting the combination. The native solver source stays at the 388 release. A separate study can test the individual effect of enabling packing on the existing claiming configuration across distinct requests.

The clean 2×2 test missed its prespecified comparison against the release and both individual mechanisms. The fallback with zero optional preparation allowance also exported a verified proof.

[Earlier factorial experiment](../2026-11-19-packed-claims391/README.md) · [Samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Source](artifacts/source.patch.gz) · [Verification](artifacts/verification.json)
