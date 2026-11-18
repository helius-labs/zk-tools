# Measuring the G2 scalar split on a 96-core C7a

The lightweight observer’s median ON/OFF latency ratio was 0.993. The paired OFF/exact-parent ratio was 0.994. 7/7 observed 96-core blocks preserved the fast and slow classes within the specified fractions. The primary data meet the declared conditions for attribution to this interval.

Among 268 slow observed 96-core proofs, **137** spent over 1 ms more in the G2 scalar split than the fast median of their own block. Of these, 93 ran a K/H feed on the coordinating thread during decomposition. Using the prespecified majority-of-excess rule, 93 were dominated by on-CPU K/H feed work, 30 by off-CPU time, 14 by other scheduled work, and 0 had mixed or unresolved attribution. The remaining 131 slow proofs had no such long split. These 137 splits contributed a total of 439.29 ms above their within-block fast medians, or 0.416 ms averaged over all 1,057 observed requests. Recovering that entire excess with every other interval fixed would still leave most of the gap to Aeglos. This is a counterfactual estimate.

| Cores | Engine / observer | Wall ms | CPU ms | Peak MiB | Fast / middle / slow | M5 CPU | M5 Metal |
|---:|---|---:|---:|---:|---:|---:|---:|
| 32 | Exact 257 | 10.707 | 231 | 69 | 15 / 763 / 279 | 3.46× | 2.24× |
| 32 | Same binary, OFF | 10.717 | 230 | 66 | 5 / 764 / 288 | 3.45× | 2.24× |
| 32 | Same binary, ON | 10.679 | 231 | 71 | 13 / 782 / 262 | 3.46× | 2.25× |
| 96 | Exact 257 | 10.073 | 452 | 72 | 765 / 5 / 287 | 3.67× | 2.38× |
| 96 | Same binary, OFF | 9.997 | 452 | 77 | 773 / 8 / 276 | 3.70× | 2.40× |
| 96 | Same binary, ON | 9.920 | 451 | 74 | 778 / 11 / 268 | 3.73× | 2.42× |
| 96 | Accepted Zopro | 12.349 | 693 | 122 | 0 / 0 / 1057 | 3.00× | 1.94× |
| 96 | Aeglos | 7.733 | 514 | 476 | 1057 / 0 / 0 | 4.78× | 3.10× |

Seven independent uninstrumented or internally observed processes per arm ran 151 timed requests after three primes. Each observed proof took clock readings around the G2 scalar decomposition. Additional clock reads were confined to K/H feeds on the same coordinating thread inside that interval. The residual scheduled time includes scalar arithmetic, other callbacks, reduction and scheduling within the measured interval. The data do not cover the later bucket and final joins. [Request measurements](artifacts/requests.json.gz) · [Blocks and calibration](artifacts/analysis.json)

All 64 fresh exports verified in Gnark on AWS and on retrieval. The instance terminated, no tagged EBS volumes remained and the temporary S3 bucket was removed. [Local observer and source](../2026-11-18-split-clock275/README.md) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
