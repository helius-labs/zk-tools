# Per-prover overlap of public-input verification

The Linux x86-64 SDK exposes an opt-in setting on the loaded prover. For keys with at most sixteen public inputs, the existing worker pool computes a fresh public Miller term during proof generation. The two proof-dependent terms run when the proof is ready. Each SDK request checks the full pairing equation. The setting defaults to disabled.

| Host / stage | Shape | Main workers | Release ms | Clean disabled ms | Clean enabled ms | Paired vs release | Wins / pairs | Paired vs disabled | Wins / pairs | Enabled peak MiB | M5 CPU / Metal gain |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| C7a | 1x2 | 64 | 8.227 | 8.256 | 8.159 | -1.13% | 4/5 | -1.48% | 4/5 | 49.8 | 4.53× / 2.94× |
| C7a | 1x2 | 96 | 8.264 | 8.261 | 8.133 | -1.58% | 7/7 | -1.32% | 7/7 | 48.3 | 4.55× / 2.95× |
| C7a | 2x2 | 48 | 13.015 | 13.002 | 13.050 | +0.43% | 2/5 | -0.60% | 3/5 | 102.3 | 4.52× / 2.30× |
| Ryzen confirmation | 1x2 | 32 | 8.519 | 7.637 | 7.605 | -8.56% | 6/7 | -0.30% | 4/7 | 86.5 | 4.87× / 3.16× |
| Ryzen screen | 12x2 | 16 | 38.442 | 38.769 | 40.694 | +7.08% | 0/5 | +4.96% | 1/5 | 521.5 | 6.98× / 2.48× |
| Ryzen screen | 16x2 | 16 | 45.856 | 45.484 | 49.495 | +2.97% | 1/5 | +8.82% | 0/5 | 616.4 | 6.77× / 2.51× |
| Ryzen screen | 1x2 | 32 | 7.668 | 7.714 | 7.581 | -0.54% | 4/5 | -1.72% | 4/5 | 63.5 | 4.88× / 3.17× |
| Ryzen screen | 24x2 | 16 | 73.349 | 74.664 | 77.998 | +6.34% | 1/5 | +2.07% | 1/5 | 835.7 | 6.77× / 2.26× |
| Ryzen screen | 2x2 | 32 | 10.731 | 10.671 | 10.912 | +2.50% | 1/5 | +3.43% | 2/5 | 173.2 | 5.41× / 2.75× |
| Ryzen screen | 4x2 | 16 | 16.940 | 16.932 | 16.898 | -0.25% | 3/5 | +0.00% | 2/5 | 199.5 | 6.45× / 2.84× |
| Ryzen screen | 8x2 | 16 | 26.720 | 26.796 | 26.944 | +0.31% | 2/5 | +0.28% | 2/5 | 344.5 | 7.01× / 2.75× |
| Ryzen screen | merge-24x1 | 16 | 71.904 | 71.176 | 77.529 | +7.80% | 0/5 | +8.93% | 0/5 | 764.4 | 6.54× / 2.26× |

On 96 cores the earlier experimental verification build measured 8.118 ms. Aeglos measured 7.962 ms and 471.2 MiB peak RSS. The clean enabled setting was +2.17% versus Aeglos in paired process means, with 1/7 wins.

On C7a 1×2 at 96 cores, enabling the option beat both the release and the same-binary disabled control in seven of seven paired blocks. Peak RSS was 48.3 MiB for both the release and the enabled candidate. The local confirmation shows a much smaller change against the same-binary control than against the release. The first local screening block partially overlapped independent proof verification for the preceding experiment. The seven-block confirmation and every C7a block ran separately from that local screening work.

Validation included 146 native core tests. The verifier tests exercised valid proofs, modified inputs and proof points, concurrent distinct requests, mismatched input lengths and failed generation on one- and four-worker pools. A generic x86-64 release check built the SDK, and two generic prover tests passed. The integrated tree also passed the verifier regression test and a Linux build check with the wgpu feature. Each process exported a fresh proof and public witness for independent Gnark verification. The benchmark driver set the option on the loaded prover. Production uses the measured prover implementation, with formatting changes confined to its new test module.

[Local request timings](artifacts/local.json.gz) · [AWS request timings](artifacts/aws.json.gz) · [Source change](artifacts/source.patch.gz) · [Protocol](artifacts/protocol.md)
