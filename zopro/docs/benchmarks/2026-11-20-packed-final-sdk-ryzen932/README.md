# Packed final exponentiation through the CPU SDK

The per-prover candidate measured **6.859 / 6.781 / 6.722 ms** for verified 1×2 proofs on Ryzen. The paired changes were **-0.63% / +0.56% / -0.71%** against the same-build ordinary route and **-0.93% / +1.06% / -0.15%** against the frozen release, in original, held-out and v27 order. The candidate fell short of the paired requirement on at least one input. The SDK patch is attached for reproduction. The confirmed C7a latency and M5 score still come from the earlier cloud runs. AWS SSO expired before this study.

The candidate packs the three cyclotomic coordinate pairs of one pairing accumulator into AVX-512 IFMA lanes. It uses stack storage and the existing scalar Fq12 products between squarings. The option is disabled when the key is loaded. It changes the final exponentiation of the proof verification after either ordinary or overlapped Miller computation. Unsupported builds use the original scalar arithmetic. The candidate uses the [earlier packed arithmetic](../2026-11-20-pairing-factorial-ryzen924/README.md) without changing the Miller loop.

Each case uses eight counterbalanced blocks, 24 main and eight frontend hardware threads, three warmups and seventeen timing samples per process. The reported latency is the median of the eight process means. Paired changes use the median of the eight same-block ratios. The frozen control is SDK 851. The earlier trial is executable 923 with only packed exponentiation enabled.

Same-build ordinary calibrated against the frozen release at **-0.14% / +0.89% / +0.55%**, with a 2% absolute bound per input. The candidate needs a negative paired median and six wins on every input against both controls. It won **6/8 / 3/8 / 7/8** against ordinary and **7/8 / 1/8 / 4/8** against the frozen release. The earlier trial executable also cleared both controls on the original input only in this rerun.

| Input | Mode | Proof ms | Paired vs release | Paired vs ordinary | Paired vs earlier trial | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | release | 6.886 | +0.00% (0/8) | +0.14% (2/8) | +0.99% (1/8) | 58.5 |
| catalog | ordinary | 6.877 | -0.14% (6/8) | +0.00% (0/8) | +0.89% (1/8) | 58.8 |
| catalog | packed | 6.859 | -0.93% (7/8) | -0.63% (6/8) | +0.84% (3/8) | 58.0 |
| catalog | trial | 6.813 | -0.98% (7/8) | -0.88% (7/8) | +0.00% (0/8) | 58.0 |
| heldout | release | 6.705 | +0.00% (0/8) | -0.89% (6/8) | -0.24% (5/8) | 59.0 |
| heldout | ordinary | 6.747 | +0.89% (2/8) | +0.00% (0/8) | +0.76% (2/8) | 58.2 |
| heldout | packed | 6.781 | +1.06% (1/8) | +0.56% (3/8) | +1.07% (1/8) | 58.2 |
| heldout | trial | 6.732 | +0.24% (3/8) | -0.76% (6/8) | +0.00% (0/8) | 58.5 |
| v27 | release | 6.728 | +0.00% (0/8) | -0.55% (7/8) | -0.24% (5/8) | 57.8 |
| v27 | ordinary | 6.792 | +0.55% (1/8) | +0.00% (0/8) | +0.67% (2/8) | 58.0 |
| v27 | packed | 6.722 | -0.15% (4/8) | -0.71% (7/8) | -0.10% (4/8) | 58.5 |
| v27 | trial | 6.751 | +0.24% (3/8) | -0.66% (6/8) | +0.00% (0/8) | 58.5 |

All 40 BN254 tests passed. The core verification test accepted the original proof and rejected altered public input with both verification entry points, at one and four workers. It also checked altered proof rejection through overlapped verification. The generic x86-64 example built without the packed symbol and exported a valid proof with the option requested. Gnark independently verified all 97 process exports, including that fallback. The ledger now has 151842 distinct verified exports.

[Build and patch hashes](artifacts/built.json) · [Protocol](artifacts/protocol.json) · [Timing samples](artifacts/rows.json.gz) · [Pinned workflow](artifacts/workflow.json.gz) · [Verification ledger digest](artifacts/verification.json)
