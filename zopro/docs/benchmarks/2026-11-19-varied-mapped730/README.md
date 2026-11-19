# Curve-route differences between two 12×2 witnesses

The experimental mapped prover ran with 30 workers on 16 Ryzen physical cores. Each input has four paired blocks with an uninstrumented process and a separately probed process running byte-identical executables. Every process warmed three calls and timed fifteen verified requests. An independent verifier passed all sixteen exported proofs against the original key. No probe reported lost events.

| Input | Plain ms | Profiling effect | Solver / curve driver / grouped G1 probe / grouped G2 plan probe / exclusive G2 probe / G1 union / G2 union / both curves union ms | Peak MiB |
|---|---:|---:|---:|---:|
| catalog | 28.493 | +1.87% | 2.672 / 14.561 / 14.257 / 14.083 / 10.744 / 14.401 / 14.083 / 14.542 | 554.7 |
| heldout | 45.075 | +1.66% | 2.691 / 31.501 / 0.034 / 30.364 / 28.097 / 31.393 / 30.364 / 31.482 | 1028.5 |

The curve driver spans 14.561 ms on the original transaction and 31.501 ms on the second. The probed prepared G1 query spends 14.257 ms and 0.034 ms respectively. This pattern supports checking where the prepared group route declines. Raw point computations run outside that probe. The G2 plan probe covers a concurrent region and can encompass other work. Each named interval is the time in the union of matching calls within the request. Combining columns would double count overlapping work. No observed calls entered the probed FFT wrapper in these two routes.

[Intervals and launch configuration](artifacts/rows.json.gz) · [Exact symbols](artifacts/symbols.json) · [Verification](artifacts/verification.json) · [12×2 SMT experiment](../2026-11-19-mixed-smt729/README.md)
