# Explicit four-range G2 selection

The integrated source reproduced the measured G2 configurations locally. The enabled 32-worker path made 48 range calls per proof. With the override disabled, at 16 workers, or with zero optional memory, the probes counted 22 calls. All 143 native tests passed. The previous C7a timing applies to the qualified experimental binary.

| Shape | Workers | Mode | Median ms | Peak RSS MiB |
|---|---:|---|---:|---:|
| 1x2 | 16 | production | 8.797 | 59 |
| 1x2 | 16 | off | 8.876 | 59 |
| 1x2 | 16 | four | 8.866 | 59 |
| 1x2 | 16 | qualified-four | 8.789 | 60 |
| 1x2 | 32 | production | 7.738 | 62 |
| 1x2 | 32 | off | 7.708 | 64 |
| 1x2 | 32 | four | 7.857 | 66 |
| 1x2 | 32 | qualified-four | 7.872 | 64 |
| 2x2 | 16 | production | 12.158 | 108 |
| 2x2 | 16 | off | 12.173 | 106 |
| 2x2 | 16 | four | 12.172 | 106 |
| 2x2 | 16 | qualified-four | 12.169 | 108 |
| 2x2 | 32 | production | 10.699 | 113 |
| 2x2 | 32 | off | 10.769 | 112 |
| 2x2 | 32 | four | 11.061 | 116 |
| 2x2 | 32 | qualified-four | 10.983 | 113 |

All 85 distinct exported proofs verified against their pinned public witnesses.

[Protocol](artifacts/protocol.md) · [Measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
