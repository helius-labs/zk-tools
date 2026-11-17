# Shared-bucket reducer census

The shared-table prototype preserves approximately the same leading digit
contribution count as the current phi-decomposed query. Its indexed reduction
still performs thousands of tree layers. On 2x2 it also misses the current path's
scalar grouping. These findings explain why adding table memory does not by
itself establish a faster complete query; they do not assign exact wall-time cost.

| Captured B2 query | 1x2 | 2x2 |
|---|---:|---:|
| Input points | 30,089 | 52,274 |
| Distinct nonzero scalars | 26,142 | 28,852 |
| Current grouping result | Declined | 28,852 groups |
| Shifted nonzero digit contributions | 609,901 | 1,058,011 |
| Occupied signed buckets | 4,096 | 4,096 |
| Largest bucket | 9,986 | 17,283 |
| Indexed first-stage tile tasks | 4,118 | 4,136 |
| Packed jobs | 634 | 1,141 |
| Modeled first-stage tree layers | 5,121 | 10,113 |
| Measured total tree-layer entries | 5,189 | 10,182 |
| Initial point-copy payload bytes | 78,067,328 | 135,425,408 |

The extra measured layers include partial merging and final weighted reduction.
Counts are not cycle samples or removable latency. Native scratch points occupy
128 bytes, the same size as affine G2 coordinates: FqLimbs wraps the existing Fq
on this architecture. This is a copy into mutable reduction storage, not a native
radix-52 point expansion. Payload bytes are not measured DRAM traffic.

Both 12-bit, two-component phi and full-scalar shifted layouts have an upper
geometry of about 22 digit contributions per point. The shared bucket topology
reduces bucket-set/recombination work, while adding routing and shifted accesses.
Actual scalar and bucket counts matter more than table size alone. Current 1x2
rejects grouping at its existing cutoff; 2x2 accepts it. Thus 2x2's current query
already reduces repeated scalars, whereas the prototype routes the original
scalar list. That is part of the prototype's real algorithmic disadvantage.
No performance comparison is retroactively removed or relabeled as a win.

An offline reader checks retained fixture hashes, recodes actual signed digits
and models unchanged tile packing. A separate native diagnostic verifies routed
lengths, representation size, actual grouping decisions, one indexed-reducer
entry per shared query, and output hashes against retained independent Arkworks
full-query results. Both native cross-checks pass. Exact per-layer exceptional
pair counts and exclusive arithmetic costs are not established by this model.

## Observer did not qualify

A separate four-stage diagnostic measures forward preparation, chain inversion,
backward coordinates and compaction with thread-local counters. Three paired
plain/observed queries per case use the same fresh diagnostic binary, after three
plain primes, at 32 fixed workers. All twelve timed outputs match the oracle.

Observed/plain median ratios are **1.084107** for 1x2 and **1.093637** for 2x2,
failing the prospective factor-1.05 observer guards. Consequently the retained
stage timings describe the instrumented build only. They are not qualified as
exact attribution to the parent or production. No threshold changed, no query
was dropped and the capture was not repeated. Timing shares do not select the
next optimization. A lower-overhead sampler is needed for timing attribution.
Layer-entry counts remain valid semantic observations of that execution.

The next useful question is whether independent inversion chains can share work
without introducing enough synchronization or scratch traffic to erase the gain.
Review the existing bounded G2 batching and older tree records first. No larger
tile sweep, new precomputation implementation or full-SDK integration is selected
from these failed phase timings. The broader CPU10x goal remains unmet.

The earlier [serial](../2026-11-17-server-shifted183/README.md) and
[parallel](../2026-11-17-server-routing184/README.md) reports are corrected to state
that final affine normalization and oracle checks occur outside component timers;
weighted projective reduction is inside. These are component observations, not
verified full-proof latency. The correction changes no captured sample or ratio.

Private root: `/home/ubuntu/src/zopro-perf-work/reducer-census185`. No production
change, AWS resource or new proof export. Canonical56585 is unchanged. Source,
model, all raw receipts, observer protocol and failed guards are retained.
This report is committed locally; publication is not requested by this experiment.
