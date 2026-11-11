# Retained gather regression diagnostic

The closed 31-block latency screen remains failed. This separate diagnostic
asks where the enabled binary spends additional CPU work; it is not a rerun
of the adoption screen and cannot qualify the candidate for adoption.

Use exact OFF51f2fa06 and ONcbd87b66 SDKs, primary 1x2 at16 fixed workers
CPUs0-15, accepted environment, unchanged original driver. Three blocks;
rotate panels plain/stat/cycles and reverse odd blocks, alternate OFF/ON order.
Every process uses251timed requests after3primes. 18processes,4572SDKcalls,
4518timed samples,18fresh exports. Seed canonical51892; expected51910 only
after independent Gnark verification and bindings/receipt/freshness review.

Stat panel: grouped user cycles,instructions,cache misses,branch misses;
at least99.9%event running coverage. Cycles panel:16million fixed user-cycle
period, CPU/time/period/IP/symbol plus actual executable MMAP and PID/TID
ownership; at least1000samples, unknown<=5%, zero LOST. FIFO control enables
observers only around the measured SDK loop. All capture precedes decoding,
analysis and independent export verification. No owned work during recording.

Observer check: for each arm and each instrumented panel, median three
within-block mean-latency ratios to that arm's plain panel must lie within
factor1.05. This is a coarse descriptive representativeness check, not statistical
equivalence or proof of unbiased sampling. Preserve failures; do not rerun
unchanged panels. Report total instructions/cycles and symbol/category counts
without treating sample percentages as exclusive or removable wall time.

No source/build changes, performance rescue, new AWS resources or public push.
The wider ALL8 CPU10x objective remains active and unmet.
