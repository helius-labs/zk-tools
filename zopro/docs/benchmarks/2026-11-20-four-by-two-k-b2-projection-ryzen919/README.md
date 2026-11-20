# Sharing the K scalar classification with B2 on 4×2

Projecting B2 classes from the K classification increased paired prover latency by **4.77% / 4.78%** versus same-build ordinary and **4.02% / 4.35%** versus the same-scheduling control on the original and held-out inputs. The projected mode took **14.245 / 14.228 ms** and lost all eight comparisons to each control on both inputs.

The trial classifies K once and uses the solver's private wire indices to look up each B2 class. It checks the input scalars exactly and handles public B2 values through exact lookup. Both queries retain first-occurrence class ordering and stable member ordering. The shared classes are converted to canonical limbs once. Query buffers are reserved under the existing optional memory policy, and rejection takes independent classification. [Verified wire mapping](../2026-11-20-four-by-two-scalar-relations-ryzen918/README.md)

The `gate` control puts K and B2 in the same branch as the projected mode, concurrent with A and H. It retains independent classification. `ordinary` uses the original join tree in the candidate executable, and `frozen` uses the original seeded executable. Ryzen uses 24 main and eight frontend hardware threads. Each process primes three proofs and times seventeen. Eight counterbalanced blocks per input compare the four modes. The ordinary controls calibrate at −0.26% / +0.55% against frozen. Advancing requires lower paired medians and at least six wins out of eight against every control on both inputs. RSS is whole-process peak, including key loading.

| Input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Paired vs gate | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | frozen | 13.590 | +0.00% (0/8) | +0.27% (4/8) | -0.71% (6/8) | 213.2 |
| catalog | ordinary | 13.558 | -0.26% (4/8) | +0.00% (0/8) | -0.82% (5/8) | 216.2 |
| catalog | gate | 13.659 | +0.71% (2/8) | +0.83% (3/8) | +0.00% (0/8) | 218.8 |
| catalog | projected | 14.245 | +4.98% (0/8) | +4.77% (0/8) | +4.02% (0/8) | 244.2 |
| heldout | frozen | 13.490 | +0.00% (0/8) | -0.55% (7/8) | -0.96% (6/8) | 217.5 |
| heldout | ordinary | 13.589 | +0.55% (1/8) | +0.00% (0/8) | -0.38% (5/8) | 211.2 |
| heldout | gate | 13.696 | +0.97% (2/8) | +0.38% (3/8) | +0.00% (0/8) | 218.8 |
| heldout | projected | 14.228 | +5.13% (0/8) | +4.78% (0/8) | +4.35% (0/8) | 236.5 |

Separate clocked blocks show that shared preparation took **3.97 / 3.83 ms** before K and B2 began. The individual query spans shortened, but the later finish of the pair moved from **8.71 / 8.44 ms** in the gate control to **9.34 / 9.26 ms** under projection. These spans include waiting and nested worker work. The measurements establish the cost of this schedule and shared preparation together.

| Input | Mode | Preparation ms | A span ms | K span ms | H span ms | B2 span ms | K/B2 finish ms | A/H finish ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | ordinary | — | 7.495 | 8.121 | 8.529 | 8.517 | 8.522 | 8.534 |
| catalog | gate | 0.000 | 7.083 | 8.208 | 8.304 | 8.689 | 8.705 | 8.307 |
| catalog | projected | 3.968 | 7.382 | 5.338 | 8.327 | 4.775 | 9.344 | 8.330 |
| heldout | ordinary | — | 7.035 | 7.972 | 8.147 | 8.374 | 8.375 | 8.150 |
| heldout | gate | 0.000 | 7.486 | 8.055 | 7.995 | 8.293 | 8.436 | 8.406 |
| heldout | projected | 3.828 | 7.632 | 5.309 | 8.197 | 4.980 | 9.256 | 8.267 |

The native test checks first-occurrence equivalence on 96 synthetic inputs and rejects mismatched private values, out-of-range wires and unknown public values. Route probes confirm projection on both inputs and successful fallback at zero optional workspace. All **90** exported proofs independently verified. The candidate fell short of the advancement criteria.

The [recorded C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) remains five of eight at 10× M5 CPU, with CPU and Metal comparisons for all supplied shapes. AWS SSO expiration kept the pending 2×2 confirmation unrun.

[Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Aggregates](artifacts/summary.json) · [Trial source](artifacts/source.patch) · [Proof verification](artifacts/verification.json)
