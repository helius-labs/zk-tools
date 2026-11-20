# Running the 4×2 K query during the FFT on C7a

The [curve replay and timing](../2026-11-20-four-by-two-phase-c7a891/README.md) found that K and B2 finish close together on the seeded 4×2 profile. This trial starts the complete K query on the frontend before the main pool computes the FFT. A, H and B2 begin on main after the FFT and can overlap any remaining K work. The late-start control uses the same K workspace reservation, arithmetic and curve join after the FFT. The ordinary profile retains its existing embedded K placement.

Zopro uses 64 main and 32 frontend physical cores. Prepared Aeglos uses all 96. The seeded solver, quotient and SDK verification retain the frozen settings. Eight matched blocks per input prime three proofs and time thirteen. Early K advances with lower paired medians and at least six wins against the same-build ordinary, frozen, and late controls on each input. All 124 exports independently verified. Separate probes checked both routes and the fallback at zero optional workspace. The temporary instance and storage were removed.

The frozen seeded profile took 14.923 / 14.894 ms. Early K took 15.139 / 15.251 ms, with paired changes of +0.91% / +1.89% against ordinary and -0.70% / +0.42% against the late-start control. The clocks observed 2.134 / 2.126 ms of K/FFT overlap. Early K fell short of its matched latency comparisons.

| Input | Mode | Proof ms | Paired vs ordinary | Paired vs frozen | Paired vs late | Paired vs Aeglos | Peak MiB | Blocks at or below 10.9 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | release | 15.323 | +3.57% (0/8) | +3.28% (0/8) | — | -14.23% (8/8) | 211.7 | 0/8 |
| catalog | frozen | 14.923 | -0.28% (4/8) | +0.00% (0/8) | — | -16.12% (8/8) | 200.4 | 0/8 |
| catalog | ordinary | 15.013 | +0.00% (0/8) | +0.30% (4/8) | — | -16.44% (8/8) | 201.1 | 0/8 |
| catalog | early | 15.139 | +0.91% (2/8) | +0.98% (2/8) | -0.70% (6/8) | -15.31% (8/8) | 200.5 | 0/8 |
| catalog | late | 15.118 | +1.50% (3/8) | +1.08% (0/8) | — | -15.81% (8/8) | 206.5 | 0/8 |
| catalog | aeglos | 17.887 | +19.67% (0/8) | +19.23% (0/8) | — | +0.00% (0/8) | 1365.3 | 0/8 |
| heldout | release | 15.328 | +3.77% (0/8) | +2.90% (2/8) | — | -13.63% (8/8) | 215.4 | 0/8 |
| heldout | frozen | 14.894 | -0.14% (4/8) | +0.00% (0/8) | — | -16.76% (8/8) | 204.1 | 0/8 |
| heldout | ordinary | 14.967 | +0.00% (0/8) | +0.14% (4/8) | — | -16.84% (8/8) | 199.7 | 0/8 |
| heldout | early | 15.251 | +1.89% (1/8) | +2.32% (2/8) | +0.42% (3/8) | -15.06% (8/8) | 211.7 | 0/8 |
| heldout | late | 15.189 | +2.14% (0/8) | +1.58% (2/8) | — | -15.12% (8/8) | 199.6 | 0/8 |
| heldout | aeglos | 17.997 | +20.25% (0/8) | +20.14% (0/8) | — | +0.00% (0/8) | 1346.3 | 0/8 |

Three separate blocks measure FFT and curve spans, overlap, and per-pool scheduled CPU time across the timed requests. Ordinary curve time includes K. The early and late rows report the A, H and B2 stage on main separately from K, including any public correction. Pool CPU time includes solving, FFT, scheduling and worker spin.

| Input | Mode | Diagnostic proof ms | FFT ms | K ms | Curves ms | K/FFT overlap ms | K wait after curves ms | Main CPU-ms/proof | Front CPU-ms/proof |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | ordinary | 14.906 | 2.037 | 7.197 | 7.377 | 0.000 | 0.000 | 527.3 | 120.8 |
| catalog | early | 15.162 | 2.166 | 5.892 | 7.229 | 2.134 | 0.000 | 541.7 | 117.9 |
| catalog | late | 15.106 | 2.201 | 6.749 | 6.991 | 0.000 | 0.112 | 528.1 | 122.7 |
| heldout | ordinary | 14.800 | 2.044 | 7.131 | 7.246 | 0.000 | 0.000 | 519.2 | 121.1 |
| heldout | early | 15.159 | 2.173 | 5.736 | 7.277 | 2.126 | 0.000 | 542.2 | 115.9 |
| heldout | late | 15.420 | 2.177 | 6.844 | 7.326 | 0.000 | 0.085 | 542.2 | 124.4 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Placement source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**5/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 854 seed4-emit | 14.726 | 109 / 48 | 7.40× / 3.26× | +3.826 |
| 8×2 | 852 SDK seeded tasks and B2 vector conversion | 18.849 | 189 / 74 | 10.03× / 3.93× | -0.051 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
