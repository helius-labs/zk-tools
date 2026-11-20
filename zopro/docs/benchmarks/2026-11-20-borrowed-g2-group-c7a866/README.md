# Borrowing first-layer G2 operands on 96 C7a cores

The [Ryzen dispatch study](../2026-11-20-borrowed-g2-group-dispatch-ryzen865/README.md) found substantially shorter 2×2 point reduction and a small full-proof signal. This follow-up uses the same binary on C7a. Zopro uses 64 main and 32 frontend physical cores. Prepared Aeglos uses all 96 physical cores. Both Zopro variants use the same scalar grouping and medium fused K/H settings.

Ten matched blocks per input each prime three proofs and time thirteen. All 100 exports independently verified. Separate probes check the borrowed route and fallback at zero optional workspace. The temporary instance and storage were removed.

The fastest trial median on the original input was 12.253 ms with `borrow`. It took 12.371 ms on the held-out input. No configuration improved both controls consistently across both inputs.

| Input | Profile | Proof ms | Matched control | Paired vs control | Paired vs release | Paired vs Aeglos | Peak MiB | Blocks at or below 5.9 ms |
|---|---|---:|---|---:|---:|---:|---:|---:|---:|
| catalog | release | 12.029 | ordinary | -0.10% (5/10) | +0.00% (0/10) | +1.37% (1/10) | 73.1 | 0/10 |
| catalog | ordinary | 12.157 | ordinary | +0.00% (0/10) | +0.10% (5/10) | +1.67% (0/10) | 72.9 | 0/10 |
| catalog | borrow | 12.253 | ordinary | +0.88% (0/10) | +1.80% (1/10) | +3.25% (0/10) | 73.7 | 0/10 |
| catalog | aeglos | 11.890 | ordinary | -1.64% (10/10) | -1.36% (9/10) | +0.00% (0/10) | 808.0 | 0/10 |
| heldout | release | 12.191 | ordinary | +0.08% (5/10) | +0.00% (0/10) | +2.52% (0/10) | 74.5 | 0/10 |
| heldout | ordinary | 12.233 | ordinary | +0.00% (0/10) | -0.08% (5/10) | +2.92% (0/10) | 82.7 | 0/10 |
| heldout | borrow | 12.371 | ordinary | +0.63% (3/10) | +1.49% (2/10) | +3.98% (0/10) | 75.9 | 0/10 |
| heldout | aeglos | 11.901 | ordinary | -2.84% (10/10) | -2.45% (10/10) | +0.00% (0/10) | 807.6 | 0/10 |

## G2 class point reduction timing

Four matched blocks on each input separately time the reduction of G2 points within scalar classes. Clocked proofs have an unclocked companion in the same block.

| Input | Profile | Reduction ms | Full proof with clock ms | Paired clock effect |
|---|---|---:|---:|---:|
| catalog | ordinary | 0.528 | 12.125 | -0.16% (2/4) |
| catalog | borrow | 0.671 | 12.371 | +0.99% (0/4) |
| heldout | ordinary | 0.555 | 12.318 | -0.93% (3/4) |
| heldout | borrow | 0.698 | 12.506 | +1.11% (1/4) |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [SDK source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**5/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 854 seed4-emit | 14.726 | 109 / 48 | 7.40× / 3.26× | +3.826 |
| 8×2 | 852 SDK seeded tasks and B2 vector conversion | 18.849 | 189 / 74 | 10.03× / 3.93× | -0.051 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
