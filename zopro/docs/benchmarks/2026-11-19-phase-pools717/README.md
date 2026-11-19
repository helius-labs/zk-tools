# Separate solver and FFT pool alongside a disjoint G2 pool

For these three small keys the equal 8+8 research layout runs the frontend stages on the eight main cores and places the exclusive G2 query on eight different physical cores. This trial gives the solver and FFT a separate smaller Rayon pool pinned to two or four of the main cores. The trial compares it with both the earlier executable and the same binary without the extra pool. A further control creates and pins four workers and keeps the pool idle. CPU masks were checked for each process. No point tables are added.

Six fresh paired blocks on each of two transactions used sixteen physical Ryzen cores. Every process performed three warmups and thirteen complete verified requests. The independent verifier in the benchmark workflow checked each of the 180 exported proofs against the expected public input. A candidate qualifies for a C7a follow-up when it has a lower paired median and wins at least four blocks against each of the three controls on both inputs. Relative percentages use paired differences.

| Shape | Input | Frontend | Active / same binary / earlier / idle four ms | vs same binary | vs earlier | vs idle four |
|---|---|---|---:|---:|---:|---:|
| 1x4 | catalog | phase4 | 11.377 / 10.291 / 10.275 / 10.284 | +10.01% (0/6) | +10.82% (0/6) | +10.41% (0/6) |
| 1x4 | catalog | phase2 | 13.380 / 10.291 / 10.275 / 10.284 | +29.39% (0/6) | +29.97% (0/6) | +29.84% (0/6) |
| 1x4 | heldout | phase4 | 11.420 / 10.316 / 10.201 / 10.219 | +11.07% (0/6) | +11.99% (0/6) | +11.91% (0/6) |
| 1x4 | heldout | phase2 | 13.414 / 10.316 / 10.201 / 10.219 | +30.98% (0/6) | +31.41% (0/6) | +30.84% (0/6) |
| 1x8 | catalog | phase4 | 13.257 / 12.143 / 12.075 / 12.090 | +9.82% (0/6) | +10.27% (0/6) | +9.48% (0/6) |
| 1x8 | catalog | phase2 | 15.399 / 12.143 / 12.075 / 12.090 | +27.38% (0/6) | +28.06% (0/6) | +27.69% (0/6) |
| 1x8 | heldout | phase4 | 13.308 / 12.085 / 12.049 / 12.095 | +9.84% (0/6) | +10.12% (0/6) | +10.17% (1/6) |
| 1x8 | heldout | phase2 | 15.485 / 12.085 / 12.049 / 12.095 | +27.73% (0/6) | +28.59% (0/6) | +28.27% (0/6) |
| 1x16 | catalog | phase4 | 16.773 / 15.200 / 15.202 / 15.308 | +10.12% (0/6) | +10.48% (0/6) | +9.91% (0/6) |
| 1x16 | catalog | phase2 | 19.582 / 15.200 / 15.202 / 15.308 | +28.71% (0/6) | +29.06% (0/6) | +28.32% (0/6) |
| 1x16 | heldout | phase4 | 16.748 / 15.269 / 15.184 / 15.316 | +9.68% (0/6) | +10.21% (0/6) | +9.53% (1/6) |
| 1x16 | heldout | phase2 | 19.631 / 15.269 / 15.184 / 15.316 | +28.49% (0/6) | +28.84% (0/6) | +28.28% (0/6) |

[Qualification by shape](artifacts/eligible.json) · [Source](artifacts/source.patch) · [Measurements](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
