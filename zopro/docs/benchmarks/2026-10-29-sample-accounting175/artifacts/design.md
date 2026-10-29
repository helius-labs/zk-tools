# Simultaneous sampling-period and event-count accounting

Declared before SDK captures, 29 October 2026. Previous goal turn is progress:
the requested consolidated report and twelve instruction profiles were published
at 3bfa11a8bd. Accepted source175/SDK20fde is unchanged; all eight CPU10x targets
remain unmet. This diagnostic resolves a measurement gap, not a new speedup.

The preceding separate profiles' period estimates fall below nonsampled PMU
counts. Their zero-loss/attribution gates remain passing; absolute period coverage
was not tested. This trial records the SAME architectural event with perf stat
and perf record simultaneously. It compares frequency199 versus fixed period
16,000,000, independently for instructions:u and cycles:u. Two counters fit the
PMU, but running coverage must still be >=99.9% with finite positive counts.

Three fixed rotated/reversed blocks x physical16 CPUs0-15 / logical32 CPUs0-31
x two events x two sampling modes =24 processes, each151 fresh verified requests
after3 primes. All24 captures precede ownership transfer, independent Gnark,
decode, loss/metadata audit and analysis. 3696 SDK requests,24 new164B exports,
ledger22210->22234. Exact pinned earlier independent preflights are inherited,
not rerun or recounted. Exact same accepted image/request/environment/ELF.

The SDK's unchanged FIFO enable/disable commands pass through a blocking relay
outside the measured process tree. It enables record then stat, waits for BOTH
acks before SDK proceeds, and disables record then stat before returning its ack.
This creates slightly different counter boundaries around blocked SDK control
waits; the diagnostic does not claim cycle-exact identical windows. Counts are
user-only. perf record launches perf stat which launches taskset/SDK, so stat
counts SDK and its workers; record additionally observes the perf-stat monitor.
Only PID(s) mapped to the exact SDK path enter SDK period totals. All other
sampled PID periods are retained and must be <=0.1% of total for qualification.
Only SDK samples are subject to the worker affinity check. The relay and perf
monitors are declared measurement machinery; no compiler, analysis/hash/archive
helper or independent CPU workload overlaps capture.

Attribution checks: exact event/mode/period/CPU/TIME metadata, SDK mapping and fork
records, one SDK PID, zero lost/throttle records, >=1000 SDK samples, <=5% unknown
SDK period. Retain all failures without rerolls or extension. Count-accounting
qualification additionally requires period/count ratio in [0.98,1.02]. This is
a prospective empirical tolerance, NOT a proof of per-symbol unbiased sampling
or a theoretical bound on unsampled final counter residue. Report each ratio,
all per-TID samples/periods and category shares, and descriptive ranges/medians
over3 panels. Fixed period and frequency are separate instrumented executions.
Do not turn their timing into a production result, calculate per-symbol CPI from
separate captures or transfer local SMT effects to physical-core C7a scaling.

Capability-only probes before this declaration found that perf -s sets
inherit_stat/read_format but /bin/true records no end-of-thread READ. Adding
:S is rejected by this perf in inherited per-thread mode. Those records/errors
remain local. The relay is an explicit alternate measurement design, not a
repair of or unchanged rerun of previous SDK captures. Its tests cover ack
ordering, round-trip commands, malformed ack and CSV parsing before SDK capture.

All helpers including analyzer/publisher compiled before captures. No source
build or active SDK uprobes. No field/core/prover test rerun for unchanged code.
Publish positive/negative/unqualified outcomes, commit and push. Preserve the
full goal and CPU/Metal comparisons. No AWS, mobile, memory-pressure, large-
circuit, minimum-RAM, concurrent-throughput or adoption qualification.
