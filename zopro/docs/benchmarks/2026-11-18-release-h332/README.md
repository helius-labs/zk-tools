# Opt-in mixed H windows on the integrated prover

The integrated binary selects seven 14-bit and twelve 13-bit unsigned windows for the dense Montgomery H query when enabled and admitted. A guard preserves the diagnostic signed path. Measurements below use 16 physical Ryzen cores and the actual integrated build. Three randomized process blocks per configuration each timed eleven proofs. The external C7a qualification applies to the isolated candidate.

| Shape | Qualified candidate ms | Integrated off ms | Integrated on ms | Paired on vs qualified | Peak MiB on | M5 CPU / Metal, on |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.509 | 8.628 | 8.494 | -0.18% | 71 | 4.36× / 2.83× |
| 2x2 | 11.944 | 12.012 | 11.965 | +0.76% | 160 | 4.93× / 2.51× |
| 4x2 | 17.038 | 16.997 | 16.929 | -0.64% | 207 | 6.44× / 2.84× |
| 8x2 | 27.001 | 26.788 | 26.614 | -0.61% | 354 | 7.10× / 2.78× |
| 12x2 | 37.824 | 39.245 | 37.898 | -0.07% | 520 | 7.49× / 2.67× |
| 16x2 | 45.058 | 46.944 | 44.981 | +1.51% | 596 | 7.45× / 2.76× |
| 24x2 | 71.574 | 75.692 | 72.248 | +0.99% | 943 | 7.31× / 2.44× |
| merge-24x1 | 71.171 | 73.458 | 70.437 | -0.74% | 902 | 7.20× / 2.48× |

The native core suite passed 143 tests and the generic build compiled with IFMA disabled. Entry probes confirmed the mixed-window route on enabled 24×2 and the ordinary route on disabled 24×2, ineligible 8×2 and zero optional memory. All 76 exported proofs verified.

[Process measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [C7a measurement of the isolated candidate](../2026-11-18-width-repeat327/README.md)
