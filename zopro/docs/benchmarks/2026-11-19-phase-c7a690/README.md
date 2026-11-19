# C7a solver, FFT and curve intervals at two physical widths

Four blocks paired the ordinary release with a byte-identical separately instrumented binary at each width. Three warmups preceded fifteen measured proofs. Linux function entry and return events were matched by thread inside the SDK request. Each phase cell is a median of process means. G1 and G2 measure the union of observed evaluation intervals for each curve. The overlap is their intersection. Inlined kernels can fall outside the probe coverage. The phases can run concurrently. Plain full-proof timing is the basis for scaling comparisons. All 32 exported proofs verified twice. Trace captures reported zero lost records.

| Shape | Physical cores | Plain ms | Trace overhead | Solver ms | FFT ms | Observed G1 ms | Observed G2 ms | G1/G2 overlap ms | Plain peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x16 | 32 | 17.380 | +4.48% | 2.885 | 1.543 | 10.248 | 11.840 | 10.085 | 253.1 |
| 1x16 | 96 | 19.670 | +8.03% | 4.176 | 2.079 | 9.103 | 12.801 | 9.097 | 244.2 |
| 5x16 | 32 | 27.696 | +2.30% | 4.738 | 3.831 | 17.001 | 11.705 | 10.993 | 434.0 |
| 5x16 | 96 | 26.887 | +1.92% | 6.303 | 3.660 | 13.567 | 7.420 | 6.811 | 411.3 |

At 1×16 the ordinary proof slowed by 2.290 ms at 96 cores. In the traced runs G1 shortened by 1.145 ms, but solver, FFT and G2 lengthened by 1.291, 0.536 and 0.961 ms respectively. At 5×16 the solver also lengthened, while observed G2 dropped from 11.705 to 7.420 ms. The different curve response is a reason to test these two shapes separately.

[Probe offsets](artifacts/offsets.json)

[Configuration](artifacts/workflow.json.gz) · [Pinned inputs](artifacts/pins.json) · [Measurements](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
