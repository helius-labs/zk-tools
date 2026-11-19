# Phase timings at the 96-core transfer crossover

The archived 5×16 and 6×2 comparisons had opposite winners at 96 cores despite using the same FFT domain. This experiment runs both shapes and the 4×2 and 8×2 M5 reference shapes using the *same archived executables, keys and requests* on Ryzen. Three modes measure ordinary Zopro, warm Aeglos and a byte-identical Zopro copy with Linux uprobes. Each process warms three times and measures eleven calls. Five randomized blocks use identical CPU masks. The Aeglos engine must report a prepared-key hit on every measured call. The independent verifier uses the original key and expected public output for each export.

An SDK control FIFO restricts trace events to the measured requests. Solver entry/return and FFT entry/return partition each verified SDK call into five adjacent intervals. Each interval sum must match the full trace duration. Compare trace and plain timings within each shape and worker count. Where the paired median difference exceeds 5%, describe intervals as observations of the instrumented build. The combined curve and final proof work after the FFT share one elapsed interval. The two shapes were selected from an older cloud capture. Local results apply to the Ryzen machine; repeat the same protocol to attribute time on C7a.

The four-worker smoke processed two measured calls and verified one export before qualification. To reproduce from the existing full catalog bundle, run the following from this report directory with a fresh output path. Linux perf and tracefs access are required for the traced mode.

```sh
python3 artifacts/profile.py /path/to/full-c7a244/bundle /path/to/phase-output --workers 32
```

For the AWS comparison, the same runner accepts `--workers 32 64 96`. [Profiler](artifacts/profile.py) · [Executable identities and probe sites](artifacts/pins.json) · [Smoke receipt](artifacts/smoke.json)
