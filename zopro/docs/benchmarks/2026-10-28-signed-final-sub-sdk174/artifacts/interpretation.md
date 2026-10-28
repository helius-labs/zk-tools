# SDK decision

Reject this integration. Primary32 ON/OFF paired latency regresses1.2901%,
interval[1.009507413,1.015769551],0/15 wins; ON/accepted regresses1.1640%,
interval[1.007473848,1.014595149],0/15 wins. FreshOFF/accepted calibration passes.
Both declared primary gain checks fail; the other five required checks pass.
Keep the component's genuine1.39–2.00% gains and the SDK regression separately.
No candidate adoption, threshold change, extension or unchanged reroll.

Primary32 CPU ms/request falls187.553->186.694 (freshOFF->ON) while median process
latency rises8.146555->8.248926ms. The sampled counter run also has lower cycles
(854.519->852.346million) and instructions(1453.407->1449.709million) perrequest,
but these are single diagnostic observations, not latency confidence intervals.
Fewer total instructions/CPU and a faster one-thread caller are insufficient
to establish lower parallel single-proof latency. The data do not isolate
critical-path placement, scheduling, code layout, contention or cache causes.
Do not infer that RAM capacity, allocation or Rust language choice explains it.

Primary16 is descriptive and noisy: ON/OFF ratio1.011349[.990304,1.064028],5wins;
ON/accepted .997667[.961741,1.056244],8wins. Larger nonregression checks pass,
but their paired point estimates are mixed:16x2 ON/OFF1.000824/ON-A1.000868;
54 ON/OFF1.003598/ON-A.999768. Preserve the small positive54/accepted estimate
and uncertain physical16 estimate, without using them to override primary32.

Peak RSS is mixed. Primary32 median/max: OFF130.00/133.00MiB, ON130.50/134.75,
accepted131.25/135.50.54 ON maximum1860.96MiB exceeds OFF1843.36 and accepted1818.01,
although its median is lower. No added source-level heap storage is not a
process-peak or minimum-RAM guarantee. No pressure/phone/giant qualification.

All18696 SDK requests verify internally;204 fresh164-byte exports verify with
independent Gnark and match public-input bindings;408 archive members; prior
ledger15553->15757, no duplicate hashes. Fifteen blocks and all diagnostics
finish once. Six4-event PMU groups run100%;6profiles have43434samples,onePIDeach,
zeroLost, FIFO excludesload/primes. Profiles are descriptive, not timed-gate inputs.

One preparation assertion failed before capture: the freshness predecessor
census has a2x2 cell instead of physical16 primary. Corrected ONLY the casefile
source, leaving source/libraries/driver unchanged; recordprepare-failure.json.
The caller-stage missing-sha2-link repair is already recorded in the component
report and is not a new SDK capture failure. No successful test or capture repeats.

This closes the tested streaming-x/y scope. Do not blindly widen the same helper
to other callsites or select a worker count after the result. The next useful
iteration should reach the actual primary32 proof path earlier, before paying
for larger-shape diagnostics: one-thread caller throughput does not reproduce
the parallel regression here. Any new mechanism still needs a history audit and
its own declared condition. This is a next-method requirement, not a qualified
new optimization or an excuse to stop the original all-eight goal.

Production174 and acceptedSDK81033 remain unchanged. The contemporaneous accepted
primary8.161132ms is a fresh control observation, not a new all-eight production
qualification. The inherited qualified primary8.366931ms/all8CPU+Metal table
remains authoritative. Goalactive/unmet0of8, not blocked. No owned AWS resources.
