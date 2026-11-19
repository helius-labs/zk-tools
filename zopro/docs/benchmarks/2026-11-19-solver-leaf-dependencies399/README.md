# Critical paths through prepared solver tasks

The existing solver waits at outer epoch boundaries and at inner wave boundaries within each parent task. The table compares that schedule with releasing whole parent tasks once their actual input-producing parents finish, then with releasing individual leaves on their own dependencies. Each operation costs one time unit. The model charges no time for dispatch, synchronization or memory access. Counts apply before Poseidon encoding.

| Shape | Packing | Leaves | Work | Current span | Parent-ready span | Leaf-ready span | Leaf span reduction | Direct edges after transitive reduction |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | off | 810 | 372731 | 13096 | 11426 | 10536 | 19.5% | 638 |
| 16x2 | on | 599 | 372731 | 14045 | 10804 | 9963 | 29.1% | 560 |
| 1x2 | off | 97 | 31061 | 11500 | 9963 | 9963 | 13.4% | 58 |
| 1x2 | on | 69 | 31061 | 11419 | 9963 | 9963 | 12.8% | 57 |
| 2x2 | off | 129 | 54083 | 12931 | 11261 | 10381 | 19.7% | 80 |
| 2x2 | on | 89 | 54083 | 11936 | 10266 | 9963 | 16.5% | 73 |
| 4x2 | off | 242 | 99035 | 12145 | 10299 | 9963 | 18.0% | 177 |
| 4x2 | on | 171 | 99035 | 15003 | 13119 | 12229 | 18.5% | 155 |
| 8x2 | off | 435 | 190331 | 13196 | 10684 | 9963 | 24.5% | 334 |
| 8x2 | on | 278 | 190331 | 12301 | 9963 | 9963 | 19.0% | 258 |
| 12x2 | off | 630 | 281371 | 12727 | 11056 | 10164 | 20.1% | 493 |
| 12x2 | on | 431 | 281371 | 12303 | 10131 | 9963 | 19.0% | 422 |
| 24x2 | off | 1235 | 556987 | 14661 | 12172 | 11280 | 23.1% | 990 |
| 24x2 | on | 910 | 556987 | 13029 | 11025 | 10845 | 16.8% | 855 |
| merge-24x1 | off | 1378 | 526435 | 34834 | 27403 | 27397 | 21.3% | 1392 |
| merge-24x1 | on | 786 | 526435 | 39426 | 32238 | 30640 | 22.3% | 866 |

## Finite-worker model on 16×2

A ready task with the longest remaining dependent chain takes priority. A running task occupies one worker until it finishes. This produces one feasible schedule at each width. The unlimited-worker spans above give lower bounds under the same unit-cost assumption.

| Packing | Workers | Current schedule | Leaf-ready schedule |
|---|---:|---:|---:|
| off | 16 | 26208 | 23296 |
| off | 64 | 13096 | 10536 |
| off | 96 | 13096 | 10536 |
| on | 16 | 26179 | 23308 |
| on | 64 | 14045 | 9963 |
| on | 96 | 14045 | 9963 |

The 16×2 model reaches its unlimited-worker span by 64 workers in both schedules. With packing enabled, removing artificial joins changes that span from 14,045 to 9,963 operations. Merely increasing the width to 96 leaves it unchanged. This clears the 15% threshold for testing a lower-overhead schedule on actual proofs. A second Rust implementation reproduced all 48 unlimited-worker spans. Both implementations model equal operation costs, so the reductions do not predict milliseconds saved.

[Independent Rust span reader](artifacts/spans.rs) · [Protocol](artifacts/protocol.md) · [Diagnostic](artifacts/diagnostic.rs) · [Captured DAGs](artifacts/graphs.json.gz) · [Model results](artifacts/models.json) · [Verified exports](artifacts/verification.json) · [Earlier C7a execution timings](../2026-11-18-counted-ready-c7a248/README.md)
