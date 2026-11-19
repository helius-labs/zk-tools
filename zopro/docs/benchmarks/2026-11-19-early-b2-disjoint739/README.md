# Overlapping B2 with the quotient FFT in the dedicated-pool profile

The experimental 1×2 route starts B2 when the witness and scalar gathers finish. The frontend runs the quotient FFT while the main pool executes B2. When the FFT returns, G1 A and K/H can begin while B2 is still running. The existing memory forecast covers fields and curve workspace. The route requires that forecast and an attached smaller pool. Each proof computes B2 from its own witness.

Ryzen used 24 main and eight frontend hardware threads with disjoint CPU IDs. Some threads are SMT siblings. This run covers schedule correctness and latency on that layout. The original and held-out inputs each used six blocks with three warmups and seventeen timed proofs. Six additional proof exports checked routing with grouping enabled or disabled and with the frontend detached. The independent verifier checked all 54 exports.

| Input | Mode | Full proof ms | Paired vs disabled | Paired vs accepted | Peak MiB |
|---|---|---:|---:|---:|---:|
| catalog | accepted | 6.887 | +0.96% (2/6) | +0.00% (0/6) | 58.2 |
| catalog | off | 6.873 | +0.00% (0/6) | -0.95% (4/6) | 58.5 |
| catalog | early | 6.844 | +0.21% (2/6) | -0.59% (5/6) | 61.5 |
| catalog | grouped | 6.940 | +1.16% (0/6) | +0.48% (2/6) | 65.8 |
| heldout | accepted | 6.745 | +0.48% (1/6) | +0.00% (0/6) | 58.2 |
| heldout | off | 6.706 | +0.00% (0/6) | -0.48% (5/6) | 61.0 |
| heldout | early | 6.597 | -1.46% (5/6) | -1.78% (5/6) | 62.8 |
| heldout | grouped | 6.654 | -0.74% (5/6) | -1.39% (5/6) | 67.2 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source patch](artifacts/source.patch) · [Proof verification](artifacts/verification.json) · [Dedicated C7a profile](../2026-11-19-raw-a-api737/README.md)
