# Direct evaluation of wire-product assignments

The solver evaluates the existing stored sums, multiplies the two input wires, applies the output-wire coefficient and writes the original R1CS row. The square setting specializes equal wire IDs. The key image and per-key memory layout stay the same. Each setting uses five randomized full-proof processes.

| Shape | Workers | Mode | Median ms | Paired change vs off | Faster blocks | CPU-ms | Peak MiB |
|---|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 32 | retained | 7.767 | +0.23% | 2/5 | 152.9 | 64 |
| 1x2 | 32 | off | 7.748 | +0.00% | 0/5 | 153.1 | 63 |
| 1x2 | 32 | direct | 7.699 | -1.16% | 4/5 | 152.9 | 65 |
| 1x2 | 32 | square | 7.703 | +0.02% | 2/5 | 151.8 | 64 |
| 2x2 | 32 | retained | 10.792 | +1.08% | 0/5 | 204.2 | 114 |
| 2x2 | 32 | off | 10.665 | +0.00% | 0/5 | 202.1 | 116 |
| 2x2 | 32 | direct | 10.704 | +0.35% | 2/5 | 204.0 | 114 |
| 2x2 | 32 | square | 10.626 | -0.37% | 3/5 | 203.5 | 113 |
| 16x2 | 16 | retained | 44.747 | +0.32% | 2/5 | 657.0 | 620 |
| 16x2 | 16 | off | 44.742 | +0.00% | 0/5 | 655.1 | 620 |
| 16x2 | 16 | direct | 44.948 | +0.27% | 2/5 | 656.4 | 617 |
| 16x2 | 16 | square | 44.776 | -0.13% | 3/5 | 654.9 | 626 |
| 24x2 | 16 | retained | 72.407 | +0.65% | 2/5 | 1080.1 | 1059 |
| 24x2 | 16 | off | 72.779 | +0.00% | 0/5 | 1074.4 | 1033 |
| 24x2 | 16 | direct | 72.124 | -0.94% | 4/5 | 1075.9 | 1020 |
| 24x2 | 16 | square | 72.649 | -0.03% | 3/5 | 1075.9 | 975 |
| merge-24x1 | 16 | retained | 70.744 | -0.37% | 5/5 | 1030.5 | 935 |
| merge-24x1 | 16 | off | 71.173 | +0.00% | 0/5 | 1032.6 | 927 |
| merge-24x1 | 16 | direct | 70.955 | -0.96% | 4/5 | 1030.1 | 910 |
| merge-24x1 | 16 | square | 70.907 | -0.83% | 3/5 | 1032.6 | 937 |

All 104 exported proofs verified. Separate probes checked the number of direct wire-product writes against the independently measured interpreter census.

[Protocol](artifacts/protocol.md) · [Proof timings](artifacts/processes.json.gz) · [Routing](artifacts/routing.json) · [Verification](artifacts/verification.json)
