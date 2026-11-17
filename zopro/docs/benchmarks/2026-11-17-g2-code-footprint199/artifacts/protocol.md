# G2 code footprint199

Question: does the ~52KiB complete G2 symbol establish a hot instruction footprint
too large for C7a but small enough locally? First check actual cache inventories,
then distinguish IFMA-loop strongly connected components from scalar-tail/cold
code. Reuse198exact OFF kernel and its existing cycle recordings; no new native
build, proof, timing or AWS resource. This is static/offline analysis, not a new
performance candidate or a causal cache model.

Local sysfs and retained C7a hardware inventories are authoritative for sizes.
Both have32KiB L1I per physical core; local associativity is8ways/64sets/64B lines.
Reject a capacity-only host reversal explanation if capacities agree. Compute
full symbol and loop byte/cache-line sets, and annotate existing G2 samples by
symbol offset. The static union is not a measured resident working set: branch
frequencies,callees,data/instruction paths,replacement and temporal order matter.

Reuse existing CFG parsing/SCC helpers from the audited stack-lifetime study.
Direct conditional targets/fallthroughs count; known panic/allocation failure
calls are noreturn. Check byte coverage,IFMA conservation,exact instruction-start
sample mapping and recorder sample accounting. No instruction count is a uop
count or cache-miss counter. No new annotation/outlining/loop-bound sweep follows
solely from a function's size. Preparation/analysis allowance15min.
