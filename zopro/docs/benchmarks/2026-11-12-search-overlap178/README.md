# C7a search residence and pending publications

In all 54 retained instrumented C7a graphs, most victim-search thread residence
occurs with no recorded published-but-not-started job. At 96 workers the median
whole-request fraction is **75.85%**, versus **79.45%** at 32. Only **0.33%** and
**1.62%**, respectively, overlap at least one pending job per worker.
This narrows the next question toward work exposure and search participation,
rather than another victim-discovery data structure. It establishes no speedup.

## Joint measurement

The previous reader measured search residence and pending jobs separately. This
offline Rust reader integrates their overlap over every main ON graph in the
[November 10 capture](../2026-11-10-task-interval-c7a176/README.md): 27 graphs per
worker count. No graph is selected or excluded. Values below are medians across
graphs; search-thread milliseconds sum elapsed search intervals across workers,
not exclusive CPU time or removable proof latency.

|Phase|Search-thread ms, 32 / 96|Search residence with no pending jobs, 32 / 96|
|---|---:|---:|
|Whole request|19.69 / 330.62|79.45% / 75.85%|
|Solver|3.01 / 54.24|86.66% / 74.39%|
|Scalar preparation|2.01 / 38.60|72.60% / 68.29%|
|FFT|5.53 / 66.71|76.17% / 82.52%|
|MSM queries|8.00 / 153.82|79.89% / 75.21%|
|Verification|0.16 / 0.98|98.43% / 98.91%|

Pending intervals begin at publication completion and end at the execution
marker. A second calculation begins at publication entry. Whole-request
no-pending fractions remain 79.16% / 75.74% with that wider proxy. Every graph
and phase has a majority under both definitions. Medians of phases need not sum
to the whole-request median. Full ranges and integer integrals are retained.

All 18 raw-file hashes match the earlier job-family analysis. Job generations
match unambiguously; search intervals do not overlap on a thread. All 270 prior
phase marginals reproduce. Endpoint sweeps match direct marginal sums and 729
independent discrete-time models. The xtask reader succeeds once, producing 648
joint rows in 1.779 seconds; preparation and analysis fit the 15-minute allowance.

## Decision and limits

Next examine whether phase-specific participation on the existing pool can
reduce interference during sparse work without delaying later parallel work.
First distinguish that intervention from the closed yield/pause, victim bitmap,
coarse proof-graph and separate-worker-group experiments. A universal idle-policy
retune or another pool is not justified. A useful test must include wake/transition
costs and uninstrumented proof latency, especially on C7a.

These are historical, perturbed traces: the 96-worker observer overhead check
failed at ON/OFF 1.095, and scalar phi grain has since been adopted. Pending
publications are not exact queue occupancy; jobs may already be claimed before
their execution marker. No pending publication does not exclude inline work,
publication in progress or future dependent work. Searching threads can be
descheduled. Consequently these results neither prove starvation nor estimate
an attainable saving. They do not establish current-source behavior.

Production, the 54,927 verified-export ledger, and the unmet all-eight 10× goal
remain unchanged. No benchmark, proof, AWS resource or publication is added.
[Accepted CPU/Metal comparisons](../2026-11-12-bundle-adoption178/README.md)
remain the reference. Sources, input pins, full results and xtask receipts are
under [artifacts](artifacts/); executable and historical raw traces remain at
`/home/ubuntu/src/zopro-perf-work/search-overlap178` and the referenced capture root.
