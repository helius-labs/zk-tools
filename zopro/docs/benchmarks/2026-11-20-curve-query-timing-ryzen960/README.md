# K and B2 query timing on Ryzen 4×2

When B2 is replaced with an exact replay, the still-active K query finishes 1.64–1.66 ms sooner in the paired runs. K's window processing and return account for 1.52–1.54 ms of that reduction. Removing K reduces the still-active B2 query by 0.44–0.55 ms.

The observer keeps the [4×2 solver placement and query scheduling](../2026-11-20-four-by-two-prep-placement-c7a946/README.md). The Ryzen profile uses 24 main, eight frontend and four solver logical CPUs. The frontend CPUs share physical cores with main-pool SMT siblings. In each diagnostic arm one or both queries return a primed result only when the original base address and all field scalars match. A and H run throughout.

| Input | Active query | All active ms | Peer replayed ms | Paired change ms | Paired scalar prep change | Paired point compaction change | Paired windows and return change |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | K | 8.213 | 6.586 | −1.636 | −0.045 | −0.032 | −1.523 |
| heldout | K | 8.189 | 6.547 | −1.662 | −0.056 | −0.035 | −1.538 |
| catalog | B2 | 8.671 | 8.276 | −0.443 | −0.067 | −0.204 | −0.201 |
| heldout | B2 | 8.724 | 8.143 | −0.548 | −0.162 | −0.083 | −0.167 |

The query clocks use four paired blocks of thirteen timed proofs per input. They measure from a common origin before the curve join, preserve K and B2 worker placement, and include the original pool dispatch. Active point counts agree across arms. The window interval includes the final reduction and return from the query.

| Replayed queries | Predicted saving with retained query times fixed, ms | Measured curve-stage saving, ms |
|---|---:|---:|
| K | 0.000 / 0.000 | 0.547 / 0.545 |
| B2 | 0.068 / 0.081 | 2.276 / 2.223 |
| K and B2 | 0.270 / 0.216 | 3.900 / 3.927 |

Each pair is catalog and heldout. The prediction removes the original endpoints of replayed queries and substitutes the measured replay durations at their original starts. Retained query times and the ordinary join overhead stay fixed. The measured changes show that removing work changes the durations of the remaining queries. They do not isolate a single cause, such as SMT sharing, cache contention or Rayon scheduling.

In the eight unclocked blocks, ordinary times were 14.747 and 14.748 ms, within 1.36% of frozen in paired medians. Tracing shifted the paired medians from −0.59% to +1.48% across the four modes. The native observer test passed and Gnark independently verified all 120 exports. Replay timings are diagnostic and the [C7a scorecard](../2026-11-20-poseidon-handoff-c7a959/README.md#eight-supplied-m5-examples) is unchanged.

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz)
