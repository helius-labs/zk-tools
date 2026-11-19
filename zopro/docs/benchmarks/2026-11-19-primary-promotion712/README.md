# Isolating the 1×2 configuration and testing scalar classes on 1×8 and 1×16

At 1×2 the pairwise four-range reducer runs alongside three independent controls: the identical binary with pairwise reduction disabled, the earlier production server configuration and prepared Aeglos. At 1×8 and 1×16, fresh AWS measurements compare the curve-specific scalar classifier against the same binary disabled, the incumbent identified by the prior AWS screen, and prepared Aeglos. The two class scopes also ran in each block. The scope shown was selected before AWS timing using the geometric mean of its local per-transaction medians. All memory reservations and actual worker masks are retained.

Eight counterbalanced blocks per transaction ran on the same c7a.24xlarge. Every process had access to the same 96 physical cores. Each 1×2 process timed 23 verified proofs after three warmups. The two other shapes timed 17. Four shorter processes independently checked classifier dispatch before the measured comparisons. Every one of the 228 exported proofs passed the remote verifier and separate local reverification.

| Shape | Input | Candidate | Candidate / same binary off / earlier Zopro / Aeglos ms | vs off | vs earlier | vs Aeglos | Peak MiB candidate / earlier / Aeglos |
|---|---|---|---:|---:|---:|---:|---:|
| 1x2 | catalog | pair-on | 8.016 / 8.048 / 8.060 / 8.199 | -0.45% (6/8) | -0.65% (6/8) | -2.38% (6/8) | 46.8 / 48.2 / 462.3 |
| 1x2 | heldout | pair-on | 11.122 / 11.157 / 11.117 / 8.215 | -0.34% (6/8) | -0.20% (6/8) | +35.37% (0/8) | 43.1 / 42.2 / 467.0 |
| 1x8 | catalog | b2 | 13.719 / 13.929 / 13.144 / 10.676 | -2.64% (5/8) | +4.09% (0/8) | +29.58% (0/8) | 90.9 / 130.8 / 691.5 |
| 1x8 | heldout | b2 | 13.711 / 13.851 / 13.126 / 10.657 | -1.27% (5/8) | +4.60% (0/8) | +28.49% (0/8) | 84.9 / 130.8 / 687.9 |
| 1x16 | catalog | shared | 16.653 / 16.874 / 15.487 / 12.488 | -2.08% (5/8) | +7.23% (0/8) | +33.00% (0/8) | 111.2 / 165.2 / 820.4 |
| 1x16 | heldout | shared | 16.595 / 16.561 / 15.584 / 12.353 | -0.53% (4/8) | +6.72% (0/8) | +33.60% (0/8) | 112.7 / 171.9 / 818.8 |

[Complete observations](artifacts/rows.json.gz) · [Source and local qualification](../2026-11-19-curve-class-scope713/README.md) · [Pairwise G2 reduction](../2026-11-19-pair-merge697/README.md) · [Independent proof checks](artifacts/verification.json)
