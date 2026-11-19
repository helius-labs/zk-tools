# C7a harness for the four query return times

This harness compares the production build with the four-return diagnostic on the same original 16×2 and merge-24×1 requests. Seven randomized blocks use 64 and 96 physical cores with fixed affinity and the same main-pool solver at each width. Each process primes three proofs and times eleven. The runner records wall latency, CPU time, peak RSS and the A, K, H, B2 and join timestamps, so that the tracing effect can be evaluated against the plain run. Each exported proof is checked against the original verification material.

The packaged runner completed its eight local smoke processes at 16 and 32 Ryzen workers. All eight exported proofs verified. The 64/96-core cloud measurements are pending. The smoke run supplies no evidence about speed at those widths.

[Runner](artifacts/run.py) · [Binary, key and request hashes](artifacts/config.json) · [Local verification](artifacts/smoke.json)
