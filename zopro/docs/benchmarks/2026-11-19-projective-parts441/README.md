# Keeping internal curve results in projective coordinates

The ordinary five-part API serializes four G1 points and one G2 point. The randomized CPU prover immediately decodes those 384 bytes to assemble a proof. The candidate can carry the projective points directly into the same blinding and final serialization code. The former zero-A fallback still requests the full five parts and fresh nonzero proof. With fixed blinding factors, the arithmetic test compares the two encodings and their independent exponent oracle. All three shape probes recorded the direct path on every enabled request. Nine randomized blocks timed 51 complete SDK requests after three warmups.

| Shape | Release / encoded / projective ms | Paired vs release | Paired vs encoded | CPU-ms encoded / projective | Peak MiB encoded / projective | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.291 / 7.306 / 7.277 | +0.05% (4/9) | -0.25% (7/9) | 151.0 / 150.9 | 68.8 / 64.8 | 5.08× / 3.30× |
| 2x2 | 10.194 / 10.146 / 10.144 | -0.81% (6/9) | +0.01% (4/9) | 201.8 / 201.3 | 119.2 / 125.8 | 5.82× / 2.96× |
| 4x2 | 15.322 / 15.346 / 15.292 | -0.02% (5/9) | -0.35% (6/9) | 403.7 / 402.6 | 329.5 / 325.8 | 7.13× / 3.14× |

No shape won six of nine blocks against both controls. Bypassing the intermediate encoding gave a small advantage over the encoded path on 1×2, where it beat the release in only four blocks. The release keeps its existing proof assembly. All 87 exported proofs passed independent Gnark verification.

[Source patch and oracle](artifacts/source.patch) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Protocol](artifacts/protocol.md)
