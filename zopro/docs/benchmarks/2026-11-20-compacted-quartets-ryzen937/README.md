# Validating quartet sums before scalar classification

The early joint route took **13.359 / 13.356 ms** on the original and held-out Ryzen 4×2 requests. It improved the frozen production control by **-2.12% (8/8) / -2.16% (8/8)**. Against the [earlier joint preparation build](../2026-11-20-joint-prepared-quartets-ryzen936/README.md) its paired change was **-0.09% (4/8) / +0.11% (4/8)**. The early B2-only mode also fell short against its matching earlier build. The experimental patch stays out of the SDK.

The trial compares all four coefficients of each prepared quartet before hashing. When all four are equal and nonzero, it submits one synthetic index for the exact point sum. The ordinary classifier still merges this entry with any other row of the same scalar. Restoring raw indices gave the ordinary path on mismatches. The original input used 17,611 accepted quartets per query. The held-out input used 16,903, including six quartets per query whose scalar classes contained additional rows. Route counts matched the archived coefficient model for every prime and exported proof.

| Input | Query | Ordinary nonzero inputs | After compaction | Saved |
|---|---|---:|---:|---:|
| catalog | K | 86,732 | 33,899 | 52,833 |
| catalog | B2 | 86,163 | 33,330 | 52,833 |
| heldout | K | 83,995 | 33,286 | 50,709 |
| heldout | B2 | 83,409 | 32,700 | 50,709 |

The mode reuses the same prepared sums and adds two bytes of optional temporary markers per raw coefficient. Nine grouping tests passed. The route checks covered zero optional budget and a wrong K plan with B2 still active. Gnark independently verified all 142 exported proofs.

## Full proof timing

The same Ryzen 9950X placement uses 24 main and eight frontend hardware threads. Each case uses eight counterbalanced blocks, with three primes and 17 timed calls per process. Latencies are medians of process means. Ordinary calibrated at **+0.20% (3/8) / +0.19% (3/8)**. The late mode performs ordinary scalar classification before consulting the cache. The prior mode is the exact 936 executable and matching preparation option. An early mode needs a lower paired median and at least six wins on each input against frozen, ordinary, same-build late, and the matching exact prior mode.

| Input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Paired vs matched late | Paired vs matched prior | CPU-ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | frozen | 13.612 | +0.00% (0/8) | -0.19% (5/8) | — | — | 292.2 | 208.2 |
| catalog | prior-b2 | 13.398 | -1.59% (7/8) | -1.75% (7/8) | — | — | 289.3 | 215.5 |
| catalog | prior-both | 13.409 | -1.34% (8/8) | -2.26% (8/8) | — | — | 285.4 | 225.0 |
| catalog | ordinary | 13.670 | +0.20% (3/8) | +0.00% (0/8) | — | — | 292.1 | 213.5 |
| catalog | late-b2 | 13.527 | -0.49% (7/8) | -1.12% (5/8) | — | — | 289.6 | 223.2 |
| catalog | late-both | 13.432 | -1.69% (7/8) | -2.14% (8/8) | — | — | 285.3 | 219.0 |
| catalog | early-b2 | 13.414 | -1.33% (8/8) | -1.69% (8/8) | -0.93% (6/8) | +0.30% (3/8) | 287.6 | 224.8 |
| catalog | early-both | 13.359 | -2.12% (8/8) | -2.09% (8/8) | -0.44% (5/8) | -0.09% (4/8) | 284.0 | 222.8 |
| heldout | frozen | 13.659 | +0.00% (0/8) | -0.19% (5/8) | — | — | 291.9 | 229.5 |
| heldout | prior-b2 | 13.470 | -1.18% (7/8) | -0.92% (7/8) | — | — | 289.8 | 249.8 |
| heldout | prior-both | 13.398 | -1.49% (8/8) | -1.59% (8/8) | — | — | 285.1 | 217.5 |
| heldout | ordinary | 13.642 | +0.19% (3/8) | +0.00% (0/8) | — | — | 291.8 | 217.5 |
| heldout | late-b2 | 13.426 | -1.61% (8/8) | -1.48% (8/8) | — | — | 288.2 | 215.8 |
| heldout | late-both | 13.408 | -1.64% (8/8) | -1.57% (7/8) | — | — | 286.1 | 222.2 |
| heldout | early-b2 | 13.427 | -1.94% (8/8) | -1.58% (8/8) | -0.00% (4/8) | -0.33% (5/8) | 288.0 | 213.5 |
| heldout | early-both | 13.356 | -2.16% (8/8) | -1.86% (8/8) | -0.39% (6/8) | +0.11% (4/8) | 284.3 | 220.5 |

## C7a confirmation

The prepared archive pins the exact 936 B2-only and combined modes that cleared the Ryzen controls. It compares them with the frozen and same-build ordinary modes on 64 main and 32 frontend physical cores, with twelve matched blocks per input. The 12 route preflight exports ran locally and verified with Gnark. AWS SSO expired and the C7a run is pending. The verified score remains [five of eight shapes at 10× M5 CPU](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings).

[Reproducible patch](artifacts/source.patch) · [Route and timing protocol](artifacts/protocol.json) · [Samples and proof verification](artifacts/rows.json.gz) · [C7a archive receipt](artifacts/c7a-prepared.json) · [Local preflight](artifacts/c7a-preflight.json)
