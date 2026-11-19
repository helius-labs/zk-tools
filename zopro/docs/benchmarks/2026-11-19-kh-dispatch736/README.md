# Scheduling individual K/H windows before G1 A

The candidate places each balanced K/H window directly in a FIFO scope before queuing A. One arm queues B2 ahead of the windows. The disabled and enabled arms use the same window arithmetic and proof assembly. The raw variants bypass A and B2 grouping, with their own ordinary scheduler control.

The C7a.24xlarge run covered three transactions at 96 main workers with a sixteen-worker frontend. Every cell summarizes four process means, each with three warm-ups and fifteen verified requests. Twelve local proof exports covered activation and the fallback with sixteen workers. The verifier independently checked sixty AWS exports and all twelve local exports.

| Input | Setting | Full proof ms | Paired change vs corresponding control | Peak MiB |
|---|---|---:|---:|---:|
| catalog | off | 8.063 | +0.00% (0/4) | 44.6 |
| catalog | off-raw | 9.926 | +0.00% (0/4) | 42.9 |
| catalog | kh | 8.945 | +10.83% (0/4) | 52.9 |
| catalog | kh-raw | 8.470 | -14.78% (4/4) | 46.8 |
| catalog | b2-raw | 8.443 | -15.12% (4/4) | 45.2 |
| heldout | off | 11.222 | +0.00% (0/4) | 44.6 |
| heldout | off-raw | 9.812 | +0.00% (0/4) | 46.0 |
| heldout | kh | 8.811 | -21.46% (4/4) | 43.0 |
| heldout | kh-raw | 8.408 | -14.10% (4/4) | 43.0 |
| heldout | b2-raw | 8.370 | -14.52% (4/4) | 46.1 |
| v27 | off | 11.148 | +0.00% (0/4) | 43.1 |
| v27 | off-raw | 9.709 | +0.00% (0/4) | 38.7 |
| v27 | kh | 8.766 | -21.38% (4/4) | 44.6 |
| v27 | kh-raw | 8.410 | -13.40% (4/4) | 42.3 |
| v27 | b2-raw | 8.394 | -13.58% (4/4) | 43.0 |

[Raw data](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source patch](artifacts/source.patch) · [Proof verification](artifacts/verification.json)
