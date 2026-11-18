# Keeping proof generation on the SDK caller

In the caller variant one scoped Rayon job computes the fresh public Miller term. Generation stays on the invoking thread. The same binary also provides worker-join early, late and disabled controls. A separate comparison uses the original early prototype. Native correctness tests check thread identity, valid proofs, altered public inputs and proof points, generation errors and concurrent callers.

| Stage | Shape | Release ms | Joined early ms | Caller ms | Paired caller vs release | Faster blocks | Paired caller vs joined | Faster blocks | Caller peak MiB | M5 CPU / Metal speedup |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| screen | 1x2 | 7.648 | 7.627 | 7.703 | -0.34% | 4/7 | +0.36% | 3/7 | 68 | 4.80× / 3.12× |
| screen | 2x2 | 10.714 | 10.779 | 10.703 | -0.48% | 5/7 | -0.45% | 5/7 | 113 | 5.51× / 2.80× |
| screen | 4x2 | 17.040 | 16.962 | 16.945 | -0.45% | 6/7 | +0.04% | 3/7 | 218 | 6.43× / 2.83× |
| screen | 16x2 | 45.335 | 47.895 | 45.867 | +0.18% | 3/7 | -4.26% | 7/7 | 631 | 7.30× / 2.70× |
| confirm | 1x2 | 7.634 | 7.590 | 7.630 | +0.01% | 4/9 | +0.22% | 3/9 | 71 | 4.85× / 3.15× |
| confirm | 16x2 | 45.667 | 47.927 | 45.469 | -0.38% | 6/9 | -4.83% | 9/9 | 658 | 7.37× / 2.73× |

[Source](artifacts/source.patch.gz) · [Timing samples](artifacts/local.json.gz) · [Protocol](artifacts/protocol.md)
