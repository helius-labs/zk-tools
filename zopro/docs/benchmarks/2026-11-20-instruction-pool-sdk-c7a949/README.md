# A caller-owned instruction pool in the SDK

The SDK trial fell short of the matched requirement. The earlier qualified experimental solver pool remains the benchmark reference. On the original 4×2 request the opt-in took **14.641 ms** on 96-core C7a. The target for this request is **10.9 ms**.

The separate pool moves only general CPU witness instructions. On C7a it uses 16 cores in two L3 groups within the existing frontend mask. The 64 main and 32 frontend workers keep the earlier FFT and curve placements. The caller supplies the extra pool. The benchmark compares it with the frozen SDK, the same new build with the option absent, and the [byte-exact 16-core experiment](../2026-11-20-four-by-two-prep-placement-c7a946/README.md).

Twelve counterbalanced blocks per input prime three calls and time seventeen. The table reports medians of per-process means, plus paired changes. Peak MiB is maximum whole-process RSS.

| Input | Frozen ms | SDK off ms | SDK on ms | Prior solver16 ms | SDK on vs off | SDK on vs frozen | SDK on vs prior | On / off peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | 14.820 | 14.815 | 14.641 | 14.457 | -1.38% (9/12) | -1.57% (8/12) | +0.88% (6/12) | 222.9 / 219.8 |
| heldout | 14.887 | 14.881 | 14.463 | 14.495 | -3.52% (11/12) | -2.34% (12/12) | -0.36% (7/12) | 216.9 / 216.1 |

The byte-exact earlier candidate again cleared both controls. It measured 14.457 ms on the original request and 14.495 ms on held-out, winning 11 of 12 against frozen on each. The clean SDK had a lower paired median than frozen on both inputs and won eight of twelve on the original, one win short of the declared requirement.

All four native placement tests and all 103 independent proof verifications passed. The proof checks include six Ryzen exports, one generic x86-64 export, and 96 C7a exports. Both local builds also verified with the separate pool supplied directly through the SDK benchmark while the legacy frontend selector was absent. The temporary AWS instance and its bucket were removed.

## Eight supplied M5 references

Five of eight shapes reach 10× against M5 CPU. The previous report records the [complete benchmark lineage](../2026-11-20-four-by-two-prep-placement-c7a946/README.md#eight-supplied-m5-references).

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.736 | 37 / 24 | 4.78× / 3.10× |
| 2×2 | 11.898 | 59 / 30 | 4.96× / 2.52× |
| 4×2 | 14.457 | 109 / 48 | 7.54× / 3.32× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Portable proof](artifacts/portable-verified.json) · [AWS cleanup](artifacts/cleanup.json)
