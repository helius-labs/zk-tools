# Published-job/search overlap, retrospective diagnostic

Question: in all 54 retained C7a primary task graphs, does recorded victim-search
residence coincide mainly with already-published jobs that have not started?
This is a new joint measurement; the previous reader reported separate means.

Use every ON main graph (27 at each of 32/96 workers), phases 1 through 6.
Reconstruct job generations from publication and execution intervals, requiring
unambiguous nonoverlapping generations and exact previous phase counts/areas.
Bound the publication uncertainty with two proxies: publication completion to
execution start, and publication start to execution start. These are not exact
queue occupancy: a claimed job may not yet have entered its execution marker.
Union search intervals per thread; reject overlap rather than double-count it.
Sweep endpoints and integrate joint occupancy: no pending jobs, 1..N-1 jobs,
and at least N jobs; additionally record any-search wall time and search-thread
nanoseconds. Verify marginal areas against direct sums and the prior reader.
Test endpoint clipping/ties/overlap with an independent discrete-time oracle.

If most search residence overlaps no pending jobs in both proxies, prioritize
work exposure/search participation rather than faster victim discovery. If most
overlaps substantial pending work, investigate discovery/claim-to-start delays.
Mixed outcomes require phase-specific hypotheses. These are descriptive choices,
not adoption gates. No confidence interval or inference about uninstrumented
critical-path time, idle CPU, attainable gains, or current source is claimed.

Source: November 10 task-interval C7a captures. Observer overhead at 96 workers
failed qualification (ON/OFF 1.095); scalar phi grain has since been adopted.
Do not rerun proofs or benchmarks. No new production code or AWS resources.
Preparation/analysis allowance: 15 minutes excluding prior history review.
Reuse xtask for one offline reader, retain input pins and raw numerical output.
