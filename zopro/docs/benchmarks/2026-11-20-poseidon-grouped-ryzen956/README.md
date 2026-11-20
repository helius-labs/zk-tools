# Shared Poseidon rounds in the 4×2 solver on Ryzen

Sharing Poseidon rounds recovered most of the latency added by grouping tasks. The shared executor still ran slower than the ordinary solver on both inputs.

The prototype bundles the seven disjoint groups from the [schedule census](../2026-11-20-poseidon-frontier-ryzen955/README.md). It runs each task's original prefix, checks all three first-round state elements, then computes one common tail on a match and writes every original destination. Differing states use independent tails. Prepared metadata accounts for 4,440 bytes under a 16 KiB admission charge. The shared executor uses the existing solution buffers for every write.

The timing runs use the frozen 4×2 placement build with 24 main, eight frontend and four solver workers on Ryzen. Each arm has eight counterbalanced blocks per input with three warm-up and seventeen timed proofs per process. The separate-tail control groups the same tasks and computes every tail independently.

| Input | Frozen ms | Ordinary ms | Separate tails ms | Shared tails ms | Shared vs separate | Shared vs ordinary | Shared vs frozen |
|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | 14.485 | 14.839 | 15.890 | 14.926 | −6.01% (8/8) | +0.78% (2/8) | +3.17% (0/8) |
| heldout | 14.512 | 14.804 | 15.904 | 14.995 | −5.96% (8/8) | +1.07% (0/8) | +3.66% (0/8) |

The catalog ordinary control was 2.08% slower than frozen, just outside the 2% calibration band. The held-out control was 1.98% slower. Shared tails lost to frozen in all eight blocks on each input. Separate phase probes put solving at 3.628 and 3.624 ms with separate tails, 2.641 and 2.680 ms with shared tails, and 2.482 and 2.564 ms with ordinary execution. Shared peak process RSS was 217 and 216 MiB.

Five native sparse-Poseidon tests passed. The route probes matched all 129,130 wire and retained-slot entries and all 98,674 entries in each materialized A and B buffer against the ordinary solver. This folded proof profile omits the C buffer. The aligned route shared all 64 tails on each input. A deliberately skewed route shared 59 of 63, took the independent path for the four mismatches, and still matched every materialized entry. Gnark independently verified all 86 exported proofs.

The prototype remains experimental. This trial used Ryzen, so the [C7a scorecard](../2026-11-20-eager-h-canonical-c7a954/README.md#eight-supplied-m5-references) remains unchanged.

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz)
