# Isolating the general solver interpreter on an existing worker pool

The experimental executable places just the instruction execution of the general solver on the supplied Rayon pool. Allocation, witness gathering, FFT and elliptic-curve work use their existing placement. Idle and active configurations create the same number of pinned auxiliary threads on the main affinity mask. Local tests use 16 physical Ryzen cores, eight solver epochs and two H ranges. Each setting has five randomized processes of eleven requests. The first execution inside the evaluator reports the actual team width.

| Shape | Mode | Verified ms | CPU-ms/proof | Peak MiB | Observed interpreter workers |
|---|---|---:|---:|---:|---:|
| 16x2 | release | 45.559 | 676.0 | 598 | unobserved |
| 16x2 | off | 45.697 | 675.6 | 615 | 16 |
| 16x2 | idle12 | 45.521 | 674.5 | 624 | 16 |
| 16x2 | active12 | 47.710 | 660.2 | 572 | 12 |
| 16x2 | active8 | 47.252 | 648.2 | 574 | 8 |
| merge-24x1 | release | 71.572 | 1051.7 | 868 | unobserved |
| merge-24x1 | off | 72.635 | 1058.4 | 904 | 16 |
| merge-24x1 | idle12 | 72.399 | 1053.4 | 857 | 16 |
| merge-24x1 | active12 | 72.517 | 1035.5 | 831 | 12 |
| merge-24x1 | active8 | 73.466 | 1030.9 | 834 | 8 |

All 143 native tests passed and all 50 exported SDK proofs verified against their pinned public witnesses. The candidate is an experiment; the integrated two-range H release supplies the production control.

[Measurements](artifacts/processes.json.gz) · [Source change](artifacts/source.patch) · [Verification](artifacts/verification.json)
