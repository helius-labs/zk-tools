# Post-screen diagnosis, no qualification rescue

The fixed latency screen is complete and failed. Preserve that decision. Use the
same executable copy0 and primary32 fixture for one warm counter panel (201 timed
queries) and one user-cycle profile (251 timed queries) per mode. Counter order
0/1/2, profile order2/1/0. Three primes and one postcheck remain oracle-checked.
This adds1380 complete queries, no SDK proof exports. No retiming or code change.

Question: did avoiding left-point staging lower complete-caller instructions or
cache-miss events, and where is the residual tree work sampled? Reuse the existing
perf FIFO gate and LOST decoder. Sampling period16million cycles,512mmap pages;
decode only after all six captures finish. Only poll the live workflow during
capture. Check each output/route, command and stream receipt, gate sequence,
sample PID/worker CPU bindings, event/period and zero LOST. Require100% running
counters, at least1000 samples/profile and at most5% unknown before interpreting
those panels. Absolute sampled-period coverage is not established. One panel per
mode supports no confidence interval, exclusive wall-time or memory-level claim.

Retain all observations, including failed quality checks. This diagnosis may
change the next research question, but cannot reverse the completed latency gate
or authorize adoption. No parameter sweep or extra run to obtain favorable data.
