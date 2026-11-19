# Search near a worker before crossing its L3 boundary

This private Rayon 1.13.0 experiment changes which peer deque a worker probes first when it runs out of local work. Both the local domain and the remaining domains are scanned from randomized starting positions. Every other worker is eligible exactly once per complete sweep. The existing steal operations, retry behavior, injected work and sleep protocol remain in use. A registry with a matching worker count allocates one ordered worker vector and one range per worker during setup. Each proof uses the original queues and arithmetic. The sixteen physical Ryzen cores expose two L3 domains and 32 hardware threads.

Frozen is the independently built, integrated SDK. Off, local and shuffled are one executable. Local uses the fixed main-worker mapping reported by Linux. Shuffled assigns alternating worker numbers to two artificial groups of the same size. Independent diagnostic executions confirmed that the real grouping was installed and that each main worker was pinned to the expected CPU. The two 5×16 cases use the qualified packed FFT and grouped G2 settings in every arm. The xtask runs counterbalanced full proofs, with three primes and thirteen timed calls, followed by separate nineteen-call CPU profiles. Every exported proof verifies against the original key and expected public inputs.

| Input | Mode | Median warm ms | Paired vs off (wins) | Paired vs frozen (wins) | Paired vs shuffled (wins) | CPU-ms/proof | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x16 | frozen | 12.970 | +0.49% (2/6) | +0.00% (0/6) | -0.54% (3/6) | 308.7 | 258.5 |
| 1x16 | off | 12.850 | +0.00% (0/6) | -0.49% (4/6) | -2.12% (5/6) | 306.5 | 254.8 |
| 1x16 | shuffled | 13.106 | +2.17% (1/6) | +0.56% (3/6) | +0.00% (0/6) | 312.5 | 264.5 |
| 1x16 | local | 12.769 | -0.52% (3/6) | -0.96% (3/6) | -2.35% (5/6) | 305.8 | 272.0 |
| 5x16 | frozen | 22.651 | +0.17% (2/6) | +0.00% (0/6) | -1.77% (5/6) | 608.1 | 433.5 |
| 5x16 | off | 22.674 | +0.00% (0/6) | -0.17% (4/6) | -1.35% (5/6) | 612.8 | 442.0 |
| 5x16 | shuffled | 23.064 | +1.38% (1/6) | +1.80% (1/6) | +0.00% (0/6) | 618.4 | 440.8 |
| 5x16 | local | 23.003 | +0.20% (3/6) | +0.77% (3/6) | -0.55% (3/6) | 613.2 | 439.8 |
| 6x2 | frozen | 20.920 | +0.10% (2/4) | +0.00% (0/4) | -0.76% (3/4) | 571.6 | 369.8 |
| 6x2 | off | 20.899 | +0.00% (0/4) | -0.10% (2/4) | -1.45% (3/4) | 567.8 | 388.5 |
| 6x2 | shuffled | 21.169 | +1.48% (1/4) | +0.77% (1/4) | +0.00% (0/4) | 579.8 | 374.0 |
| 6x2 | local | 20.768 | -1.48% (3/4) | -0.72% (3/4) | -1.91% (4/4) | 571.1 | 375.2 |
| 1x2 | frozen | 7.208 | +2.17% (1/6) | +0.00% (0/6) | +2.07% (1/6) | 146.0 | 60.0 |
| 1x2 | off | 7.055 | +0.00% (0/6) | -2.12% (5/6) | -0.27% (4/6) | 145.5 | 60.2 |
| 1x2 | shuffled | 7.106 | +0.27% (2/6) | -2.03% (5/6) | +0.00% (0/6) | 146.2 | 63.8 |
| 1x2 | local | 7.115 | +0.52% (2/6) | -1.29% (5/6) | +0.40% (2/6) | 145.7 | 60.5 |
| 5x16-1702 | frozen | 22.997 | +1.55% (1/4) | +0.00% (0/4) | +0.88% (2/4) | 615.3 | 451.2 |
| 5x16-1702 | off | 22.949 | +0.00% (0/4) | -1.50% (3/4) | -0.10% (2/4) | 616.5 | 439.2 |
| 5x16-1702 | shuffled | 22.815 | +0.11% (2/4) | -0.86% (2/4) | +0.00% (0/4) | 620.5 | 445.8 |
| 5x16-1702 | local | 22.656 | -0.95% (3/4) | -1.69% (4/4) | -0.65% (3/4) | 613.2 | 436.5 |

On 1×16, real cache domains beat shuffled domains in 5/6 blocks and split 3/6 against the ordinary search order in the same executable. On original 5×16, the paired change was +0.20% against disabled. On 6×2, the local order saved 1.48% against disabled and 0.72% against frozen, winning 3/4 on both. On 1×2 the same-binary disabled control already beat frozen in 5/6 blocks. Real-domain search lost 4/6 against disabled on that input. Alternating the order of queue probes did not produce a consistent reduction in CPU-ms. Keep the stock scheduler for the release.

A short tooling compilation overlapped the frozen and disabled processes in held-out 5×16 block zero. All observations are retained. Omitting that entire block leaves three pairs with median changes of −0.92% versus disabled and −1.04% versus frozen. The original catalog 5×16 run preceded the overlap. The candidate passed all 102 Rayon library tests, and all 129 exported proofs independently verified.

[Runtime patch and traversal test](artifacts/registry.patch) · [Build and test](artifacts/build.json) · [Worker domains](artifacts/markers.json) · [Configurations](artifacts/configs.json.gz) · [Measurements](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
