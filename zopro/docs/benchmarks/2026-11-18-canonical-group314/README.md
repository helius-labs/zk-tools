# Canonical scalar grouping on 16 physical cores

All arms enable the G2 diversity check. The candidate also enables the same bounded bitmap on the canonical grouping path. The 1×2 census identified a grouping map that would be discarded for nine different requests. The latency panel uses the original fixtures.

| Shape | Integrated G2-only ms | Rebuilt G2-only ms | Both checks ms | Paired change | Wins / 7 | Disabled calibration | Peak MiB enabled / disabled |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.474 | 8.499 | 8.432 | -0.17% | 4 | +0.45% | 54 / 59 |
| 2x2 | 11.865 | 11.928 | 11.972 | -0.25% | 4 | +0.52% | 180 / 184 |

The primary 1×2 screen did not pass. Each of 42 fresh processes performed three warm-up proofs and 51 timed SDK requests. Every process exported a verified proof. The patch applies to the integrated source at f7b2ddcdf9.

## Query overlap in an earlier C7a trace

In the earlier frontend-limited C7a build the A query returned before G2 on 451 of 453 traced 1×2 requests at 64 workers and 451 of 453 at 96. Its median lead was 1.12 and 1.42 ms, respectively. Advancing A would rarely move the last of those two returns if G2 timing held fixed. Resource contention can still affect G2. This trace used a different executable from the local grouping experiment. [Original trace](../2026-11-18-window-tail294/README.md) · [Recomputed intervals](artifacts/historical-query-overlap.json)
