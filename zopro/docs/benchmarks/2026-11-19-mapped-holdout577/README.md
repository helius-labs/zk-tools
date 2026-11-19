# Preconverted merge keys on independent requests

The same preconverted merge-24×1 image is reused on additional public fixtures. Each request and public input digest is fixed before measurement. Requests whose public digest appeared in discovery, the earlier evaluation, or another selected cell are excluded. Ordinary and mapped processes in each block see identical input orders. Each input primes three proofs before its eleven measured SDK requests. Three randomized local blocks compare sixteen physical Ryzen cores and all thirty-two hardware threads. The host runner defaults to a separate 64/96 physical-core C7a comparison; all timings here are local.

| Request | Ryzen workers | Ordinary / mapped ms | Paired difference |
|---|---:|---:|---:|
| adj110 | 16 | 71.152 / 62.835 | -11.69% (3/3) |
| adj110 | 32 | 68.716 / 64.254 | -6.49% (3/3) |
| adj113 | 16 | 69.569 / 56.425 | -18.46% (3/3) |
| adj113 | 32 | 65.011 / 59.521 | -7.72% (3/3) |
| adj118 | 16 | 72.089 / 62.614 | -13.62% (3/3) |
| adj118 | 32 | 68.759 / 64.755 | -6.64% (3/3) |
| adj120 | 16 | 68.524 / 62.512 | -8.60% (3/3) |
| adj120 | 32 | 64.653 / 62.558 | -2.18% (2/3) |
| mix121 | 16 | 70.852 / 66.925 | -6.17% (3/3) |
| mix121 | 32 | 70.268 / 65.774 | -6.40% (2/3) |
| mix123 | 16 | 69.001 / 63.381 | -8.14% (3/3) |
| mix123 | 32 | 64.773 / 61.627 | -5.05% (3/3) |
| mix125 | 16 | 73.702 / 71.543 | -3.67% (3/3) |
| mix125 | 32 | 71.011 / 69.570 | -1.55% (3/3) |
| mix128 | 16 | 74.325 / 74.474 | +0.31% (1/3) |
| mix128 | 32 | 71.539 / 71.721 | -0.25% (2/3) |
| mix132 | 16 | 71.088 / 67.914 | -4.96% (3/3) |
| mix132 | 32 | 69.932 / 67.145 | -5.18% (2/3) |
| mix136 | 16 | 75.853 / 76.776 | +1.22% (1/3) |
| mix136 | 32 | 73.873 / 73.292 | -0.79% (2/3) |
| deep127 | 16 | 75.213 / 74.856 | -0.12% (3/3) |
| deep127 | 32 | 73.486 / 72.063 | -1.94% (3/3) |

All 132 proof exports verified using the original key. [Measurements](artifacts/samples.json.gz) · [Input and binary identities](artifacts/config.json) · [Host runner](artifacts/run.py) · [Verification](artifacts/verification.json)

## Results by worker count

For each randomized pair of processes, the first column of results takes the median latency change over the distinct public requests in that pair. The three differences are separate block observations. Requests measured in the same process share a loaded key and process history. Peak RSS covers the complete stream of requests in a process.

| Ryzen workers | Median paired change per block, % | Mapped wins per block | Inputs with a faster three-block median | Median whole-process peak MiB ordinary / mapped |
|---|---:|---:|---:|---:|
| 16 | -8.16 / -4.96 / -6.17 | 11 / 9 / 9 | 9/11 | 1136.4 / 1119.2 |
| 32 | -2.18 / -2.03 / -5.27 | 9 / 8 / 11 | 11/11 | 1412.4 / 1489.6 |

[Paired block and request differences](artifacts/widths.json)
