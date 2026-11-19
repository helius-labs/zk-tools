# External public wire offsets in fresh 1×2 proofs

The experimental SDK accepts a borrowed coefficient file when preparing an existing CPU key. Its 48-byte header identifies the version, ordered B2 point digest, base count and number of records. Each 36-byte record holds a B2 index and a canonical public field element. Setup computes the three correction points from the loaded key, then releases the file. Warm proofs subtract the offsets before the canonical gather and add the corresponding points to A, K and B2. The executable contains no fixture coefficients.

Eight newly generated public witnesses used the 1×2 circuit on the Ryzen 9950X with 32 logical workers. Each process warmed up three requests and timed 31 complete proofs. Seven randomized matched blocks included the exact release, this executable without a file, and the embedded prototype from experiment 460.

| Input | Release / disabled / embedded / external ms | vs release | vs disabled | vs embedded | Peak RSS disabled / external MiB |
|---:|---:|---:|---:|---:|---:|
| 17 | 7.075 / 7.074 / 6.934 / 6.965 | -1.74% (7/7) | -1.43% (7/7) | +0.43% (1/7) | 61.5 / 64.0 |
| 18 | 7.097 / 7.075 / 7.010 / 6.965 | -2.25% (7/7) | -1.50% (7/7) | -0.50% (6/7) | 61.2 / 66.5 |
| 19 | 7.077 / 7.080 / 6.959 / 6.937 | -1.84% (6/7) | -2.07% (7/7) | +0.66% (3/7) | 62.5 / 65.0 |
| 20 | 7.052 / 7.070 / 6.951 / 6.975 | -1.42% (6/7) | -1.50% (6/7) | +0.01% (3/7) | 63.2 / 64.2 |
| 21 | 7.077 / 7.073 / 6.949 / 6.968 | -1.53% (7/7) | -1.83% (7/7) | -0.06% (4/7) | 61.0 / 63.8 |
| 22 | 7.059 / 7.034 / 6.986 / 6.975 | -0.65% (7/7) | -1.82% (6/7) | +0.03% (3/7) | 63.2 / 63.0 |
| 23 | 7.069 / 7.077 / 6.968 / 6.956 | -1.36% (6/7) | -1.52% (6/7) | -0.17% (4/7) | 61.5 / 62.5 |
| 24 | 7.062 / 7.062 / 6.958 / 6.978 | -1.10% (7/7) | -1.24% (6/7) | +0.34% (1/7) | 61.2 / 63.8 |

The external file changed median paired warm latency by -1.47% against release (8/8), -1.51% against the disabled control (8/8), and +0.02% against the embedded prototype (3/8). The external path met the prespecified criterion for both primary controls. Eager preparation, including the B2 digest check, took a median 5.20 ms per loaded key. The 42,096-byte file was admitted with a 1 MiB optional budget. A zero budget fell back to the ordinary path.

Ten malformed or foreign test cases were rejected. A separate three-row file made for a 2×2 key exercised both separate and combined K/H modes. Both returned independently verified proofs. Across timing and route checks, 231 exports passed Gnark verification.

[Protocol](artifacts/protocol.md) · [Public coefficient file](artifacts/public.wo) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Input validation](artifacts/validation.json) · [Prototype source](artifacts/source.patch)
