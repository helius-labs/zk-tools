# Transaction sensitivity of 1×2 at 96 C7a cores

Four fresh blocks paired the production server profile, an identically built binary with function probes, and prepared Aeglos on six transactions. The four additional transactions are a private split (v25), a deposit (v26), a withdrawal (v27), and a data-bearing transaction with a zero second amount (v28). The prover key and its public coefficient file remained constant across requests. Each cell is a median of process means from fifteen warm proofs. Recorded probe entry and return pairs were matched per thread within each timed SDK request. Curve intervals can overlap. Inlined work may be outside the probes. All captures reported zero lost records.

| Transaction | Zopro / Aeglos ms | Profiling effect | Solver / FFT / G1 / G2 / curve union ms | Peak MiB |
|---|---:|---:|---:|---:|
| catalog | 8.034 / 8.230 | +9.40% | 1.650 / 0.825 / 3.467 / 4.747 / 4.754 | 44.5 / 457.9 |
| heldout | 11.114 / 8.295 | +7.67% | 1.623 / 0.836 / 4.635 / 5.661 / 5.690 | 39.3 / 460.5 |
| v25 | 11.124 / 8.155 | +8.16% | 1.654 / 0.835 / 4.815 / 5.262 / 5.428 | 39.2 / 457.1 |
| v26 | 11.191 / 8.228 | +7.42% | 1.641 / 0.810 / 4.976 / 5.212 / 5.621 | 39.9 / 455.6 |
| v27 | 11.183 / 8.122 | +8.28% | 1.612 / 0.819 / 4.816 / 5.438 / 5.501 | 37.8 / 462.3 |
| v28 | 11.118 / 8.042 | +9.41% | 1.667 / 0.847 / 4.881 / 5.236 / 5.360 | 42.3 / 453.7 |

The original and second transactions also ran on the same production binary with the public offset file omitted, or with optional scalar deduplication disabled. Each intervention was compared with the ordinary server profile.

| Transaction | Intervention | ms | Paired change vs server (wins) |
|---|---|---:|---:|
| catalog | no-offset | 11.424 | +42.34% (0/4) |
| catalog | direct | 9.939 | +24.03% (0/4) |
| heldout | no-offset | 11.219 | +0.94% (0/4) |
| heldout | direct | 9.467 | -14.72% (4/4) |

[Samples and per-request intervals](artifacts/aws-rows.json.gz) · [Probe offsets](artifacts/offsets.json) · [Proof verification](artifacts/verification.json)
