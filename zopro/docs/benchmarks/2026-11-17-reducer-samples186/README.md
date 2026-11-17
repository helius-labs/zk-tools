# Retained shared-bucket component cycle samples

Sampling the exact retained component binary preserves timed-query behavior
within the prospective coarse guards, but **fails whole-process observer
qualification**. The process includes table setup, all three query arms and
profiler initialization/drain. It is not a warm-only proof or reducer profile.
No new optimization or production speedup follows from this capture.

Three reversed-order plain/profile pairs reuse the exact unmodified 184 binary
`4415b8c6709af611935de70201970c9b3863943b22b57dc10db9893098177b5b`.
Each executes both captured queries with current, serial and parallel arms,
three primes and nine timed calls per arm. All 432 component outputs self-check
against retained independent Arkworks results; no proof export is generated.

| Observer scope | Median profile/plain | Factor-1.05 guard |
|---|---:|---|
| Whole process, including profiler startup/drain | 2.197450 | Fail |
| 1x2 current timed queries | 0.995515 | Pass |
| 1x2 serial timed queries | 0.982326 | Pass |
| 1x2 parallel timed queries | 1.010145 | Pass |
| 2x2 current timed queries | 0.969918 | Pass |
| 2x2 serial timed queries | 0.965200 | Pass |
| 2x2 parallel timed queries | 0.979676 | Pass |

All three sample-quality checks pass: at least 1000 samples each, unknown symbols
below 5%, zero LOST records, fixed 16-million user-cycle period, and executable
MMAP/PID/TID ownership. Recorded CPUs remain within the 32-worker CPU set.
The checks do not establish instruction-period completeness or exclusive CPU.
The separate warm-query guards are retained; they do not override the failed
whole-process guard. No capture or threshold was changed after results.

The first profile has 6210 samples. About 44% fall in G2 point doubling, consistent
with substantial cold table-building activity. Indexed-tile worker code and the
G2 reduce_layer function are also visible. These whole-component counts mix setup
and query arms; they cannot be presented as production stage fractions, removable
wall time, or a ranking of causes for the C7a discrepancy. Raw symbols remain in
result.json rather than being relabeled as a warm-only profile.

Next use the existing FIFO-controlled perf boundary around a sufficiently long
warm-query panel, with table setup and profiler startup/drain outside that panel.
Retain this failed-scope capture. Do not repeat its unchanged whole-program
protocol, relax its threshold or select samples after seeing a preferred result.
No parameter sweep or new reducer implementation is selected here.

The source counter experiment's [independent limitations](../2026-11-17-reducer-census185/README.md)
also remain: its stage observer failed overhead qualification. Semantic geometry
and grouping counts are still useful evidence. Production, canonical56585 and
AWS state are unchanged. No public push is included. Private root:
`/home/ubuntu/src/zopro-perf-work/reducer-samples186`.
