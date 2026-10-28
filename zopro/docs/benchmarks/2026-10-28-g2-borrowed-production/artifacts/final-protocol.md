# Final production artifact capture

Fixed before capture: three immutable executable inodes (accepted, rebuilt control,
clean candidate), 15 complete rotated/reversed blocks, nine shapes at 32 workers
on CPUs 0-31 and primary 1x2 at 16 workers on CPUs 0-15. No executable copy,
layout/flag reroll, successful-capture repetition, exclusion, or sample extension.
Every SDK process primes three times; primary cells then run 151 requests, other
cells 31. Preflight uses one request for all nine shapes and three variants.
All 477 final exports must independently verify with the historical Gnark verifier;
SDK verification is included in each timed request. Loading/primes are excluded.
Proofs are fresh, randomized, single-request latency, not concurrent throughput.
No owned build, helper compilation, hashing/archive or other bench overlaps captures.
The host is not exclusively reserved; retain before/after process snapshots.

Analysis uses median process latency per block, paired ratios by block and ranks
4 and 12 of 15 for nominal 96.484375% median bounds (no familywise guarantee).
For all nine default32 cells, rebuilt-control/accepted calibration bounds must
fit [1/1.03,1.03]. Candidate versus each control requires upper <1 and at least
11/15 wins for primary32; other shapes require upper <=1.02. No minimum gain
floor. Physical16 results are descriptive in this final-artifact protocol, and
the original two failed16 checks remain false; accepting their uncertainty must
be an explicit engineering exception. Do not rerun merely to clear a failed bound.

Report CPU time, whole-process RSS median/max, all eight CPU and Metal comparisons,
all failed checks and original10x goal progress. Old resource evidence concerns the
experimental source only where the implementation and guards match. No new claim
about minimum RAM, smartphones, swap, giant circuits or AWS. Forced-high stress
timed out on accepted baseline; candidate under that condition is untested.

Clean-source adoption is conditional on this actual artifact, source audit and
explicit review of the retained16/stress limitations. This screen alone is not
the full goal, and no successful partial scope may replace that goal.
