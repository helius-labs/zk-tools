# Packed final exponentiation on C7a 1×2 proofs

Packed final exponentiation took **7.752 ms** for the original 1×2 request. Frozen Zopro took **7.824 ms** and prepared Aeglos **8.212 ms**. The candidate cleared both controls on all three inputs. On the original M5 reference input, the candidate is **4.77× CPU** and **3.10× Metal**.

The exact packed exponentiation build uses ordinary Miller arithmetic and the validated 80+16 Zopro placement. Prepared Aeglos uses 96 workers and all timed calls hit its cache. The earlier [Ryzen experiment](../2026-11-20-pairing-factorial-ryzen924/README.md) isolates the same arithmetic path.

Each input uses twelve counterbalanced timing blocks. All 144 exports verified on the runner and again after retrieval. The shared c7a.24xlarge terminated after the last panel, its bucket was deleted, and the final check found zero tagged volumes or active benchmark instances.

| Input | Mode | Proof ms | Paired vs frozen | Peak MiB |
|---|---|---:|---:|---:|
| catalog | release | 7.824 | +0.00% (0/12) | 44.6 |
| catalog | ordinary | 7.869 | +0.66% (4/12) | 43.0 |
| catalog | final | 7.752 | -0.81% (11/12) | 40.8 |
| catalog | aeglos | 8.212 | +5.02% (0/12) | 464.1 |
| heldout | release | 7.812 | +0.00% (0/12) | 42.3 |
| heldout | ordinary | 7.803 | -0.37% (9/12) | 40.0 |
| heldout | final | 7.719 | -1.45% (11/12) | 42.3 |
| heldout | aeglos | 8.148 | +3.62% (0/12) | 459.1 |
| v27 | release | 7.806 | +0.00% (0/12) | 43.8 |
| v27 | ordinary | 7.817 | +0.25% (3/12) | 42.2 |
| v27 | final | 7.742 | -0.85% (10/12) | 41.5 |
| v27 | aeglos | 8.169 | +4.64% (0/12) | 463.8 |

## Matched packed candidate comparisons

| Input | Ordinary vs release | Packed vs ordinary | Packed vs Aeglos |
|---|---:|---:|---:|
| catalog | +0.66% (4/12) | -1.32% (10/12) | -5.59% (12/12) |
| heldout | -0.37% (9/12) | -0.96% (11/12) | -5.12% (12/12) |
| v27 | +0.25% (3/12) | -1.08% (12/12) | -5.31% (12/12) |

[Timing and proof verification](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Cleanup receipt](artifacts/cleanup.json)

## Eight supplied M5 references

The first two rows use the qualified C7a candidates from this run and the [Euler FFT confirmation](../2026-11-20-euler-ordinary-layout-c7a909/README.md). The other rows use the [previously verified profiles](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings). Five of eight shapes reach the 10× M5 CPU target.

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.752 | 37 / 24 | 4.77× / 3.10× |
| 2×2 | 11.926 | 59 / 30 | 4.95× / 2.52× |
| 4×2 | 14.726 | 109 / 48 | 7.40× / 3.26× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |
