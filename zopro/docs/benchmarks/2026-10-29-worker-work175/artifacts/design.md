# Retired work across worker counts

Declared before capture, 29 October 2026. Previous goal turn is progress:
the exact accepted SDK stage diagnostic completed and was pushed. Its parallel
probe-overhead checks failed and remain failed; do not treat those intervals as
an unperturbed critical path. All eight CPU10x M5 targets remain unmet. Accepted
source175/SDK20fde is unchanged and CPU/Metal comparisons remain authoritative.

This separately declared PMU diagnostic asks how much retired instruction work
changes with worker count, instead of equating summed CPU time with useful work.
Use the exact frozen accepted SDK, primary1x2 image/request/environment, no active
uprobes and no source/library rebuild. One worker pins CPU2,16workers physical
CPUs0-15,32workers logicalCPUs0-31. Worker-dependent arithmetic partition/window
policy is part of the measured behavior; a counter difference does not isolate
scheduler overhead from algorithm geometry or different SIMD occupancy.

Three fixed rotated/reversed blocks, each of three topologies. Every process
times31fresh, internally verified requests after3primes. FIFO-gated grouped
cycles:u,instructions:u,cache-misses:u,branch-misses:u exclude loading/primes,
output and independent verification. All9captures finish before ownership,
all9Gnark checks and offline parsing.306SDKrequests,9fresh164-byte exports.
Reuse the preceding3independently verified topology preflights only after exact
binary/image/request/environment/affinity/receipt checks. Do not rerun them.

Each required counter must be finite/nonnegative with positive runtime and
at least99.9%running coverage in every retained process to qualify that panel.
Unsupported/multiplexed panels are retained as unqualified, not resampled.
Report all three per-topology observations and descriptive medians/ranges.
This is not a statistical performance/adoption gate or a replacement latency
qualification. No exclusions, rerolls, threshold changes or selected blocks.

Instructions are architectural retired instructions, not field-operation counts
or micro-ops. IPC is instructions/cycles. Generic cache misses are not memory
bandwidth. User cycles divided by user CPU time can indicate effective cycle
rate, with clock/accounting limitations. None isolates a hardware cause. SMT,
CPU clocks, OS placement and worker-policy differences remain possible factors.

All helpers/analyzers/publishers compiled before capture. No owned compiler,
helper/hash/archive/analysis or second CPU workload overlaps captures. Rust and
xtask, no new Python or agents. All9new proofs must verify independently, match
image/public hashes and be distinct from prior22189 before ledger advances22198.
No AWS resources, key tables, memory-pressure/mobile/minimum-RAM claim, source
adoption or completion claim. Commit and push the completed result.
