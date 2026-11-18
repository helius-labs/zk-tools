# C7a scaling: current-build CPU diagnostics

The current build again slows down on 1×2 when going from 32 to 96 physical
cores: **10.897 → 12.503 ms**, while summed CPU time rises **3.02×**.
Merge54 improves **142.991 → 124.575 ms**. No production change is adopted.

The 96-core 1×2 cycle profile passes its observer check and places approximately
56% of samples in Crossbeam epoch functions, compared with approximately 18%
of retired-instruction samples. This supports investigating coordination stalls.
It does **not** establish a recoverable wall-time fraction or a critical path.

## Plain verified requests

Fresh c7a.24xlarge, 96 physical cores, SMT disabled, fixed worker affinity.
Three rotated/reversed blocks use three independent executable backings.
Each process primes three requests and times 251 requests for 1×2 or 17 for
merge54. Times are medians of three process means; CPU is summed user+system
time per request. RSS is the maximum process-lifetime high-water mark among
these plain captures, including loading and priming; it is not minimum RAM.

|Circuit|Engine|Cores|Request ms|CPU ms/request|Peak RSS MiB|
|---|---|---:|---:|---:|---:|
|1×2|Zopro|32|10.896800|230.042|118.81|
|1×2|Zopro|96|12.502850|697.923|115.70|
|1×2|Aeglos|32|11.988469|327.187|436.16|
|1×2|Aeglos|96|7.860529|515.701|479.84|
|merge54|Zopro|32|142.990763|3614.701|2025.92|
|merge54|Zopro|96|124.575468|6562.504|2098.09|

All plain latency captures precede the profiler panels. These three-block
results are descriptive, without confidence intervals. The existing
[41-shape comparison](../2026-11-18-full-catalog-c7a244/README.md) remains the
broader comparison; Aeglos merge54 was not repeated in this focused run.

## Counter observations and observer failure

After plain latency, each context/block has an additional plain observer,
grouped counter panel and simultaneous cycle/instruction sample panel. FIFO
control brackets the warmed SDK loop. All 72 main SDK processes finish before
late independent verifiers and profile decoding. No other owned CPU job runs
on the benchmark host during SDK capture. Independent local reader preparation
uses the separate workspace machine.

|96/32 paired ratio, median of three blocks|Zopro 1×2|Aeglos 1×2|Zopro merge54|
|---|---:|---:|---:|
|Plain request latency|1.147387|0.661148|0.896876|
|Plain CPU time/request|3.023952|1.577561|1.851222|
|Counted user cycles/request|3.067739|1.438139|1.669428|
|Counted instructions/request|1.252727|1.164757|1.075193|
|Counted cycles/instruction|2.448850|1.233433|1.551971|
|Generic cache misses/request|3.472028|1.080159|1.925284|

All 72 event readings report 100% scheduled, but **overall diagnostic
qualification fails**. Zopro 1×2 counter/plain median latency ratios are
1.090083 at 32 cores and 1.082318 at 96; both exceed the preregistered factor
1.05. Its 32-core dual-sample/plain ratio also fails, at 1.061974. These
counter comparisons remain observations of perturbed executions, not accepted
estimates of unprofiled costs. They are retained without exclusions or reruns.
Generic cache misses do not identify a cache level or prove DRAM bandwidth
exhaustion. The full observer table and every block are in the artifacts.

## Simultaneous samples

All 18 recordings pass event identity, fixed period 16,000,000, at least 1,000
samples per event, unknown symbols ≤5%, zero LOST records, and SDK process,
thread, CPU and executable-mapping bindings. The 96-core Zopro 1×2 dual-sample
observer ratio passes at 1.037578. Both Aeglos configurations and both merge54
dual-sample observer checks pass. The 32-core Zopro 1×2 failure above prevents
calling its sampled 32/96 contrast fully qualified.

For Zopro 1×2 at 96 cores, Crossbeam epoch cycle-sample shares across the three
blocks are **56.02%, 55.55%, 55.48%**. Corresponding retired-instruction shares
are **18.26%, 17.94%, 18.09%**. The leading symbols are the epoch pin handle
and `Global::try_advance`. At least half of cycle samples fall in epoch symbols
for 92, 94 and 92 of the 97 sampled worker/caller threads respectively.

This is consistent with expensive coordination rather than proportionate
growth in executed instructions. Flat attribution includes inlining and skid;
absolute sample-period coverage is unvalidated. Busy scheduler residence is
not equivalent to idle time, a removable wall fraction or proof that scheduler
changes will improve latency. Prior epoch, victim-scan, search-throttling and
queue experiments remain negative; this report does not reopen them unchanged.

The next structural candidate is dependency-driven solver execution: preserve
the existing arithmetic leaves while replacing global dependency-band joins
with explicit producer/consumer readiness on the existing Rayon pool. It needs
bounded metadata admission, error/unwind/concurrency tests and an end-to-end
screen. There is no implementation or performance claim for it in this report.

## Verification and resources

The unchanged current SDK is `d523f49e8548620c4373a1ab4be2712d705709fdd7a590c5ed73e9aff97424e3`.
This is the exact previously measured forced-cfg233 executable; native selection
was integrated in236, but a newly linked normal-selector SDK is not measured.
The retained Aeglos Sorted build is source `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`,
executable `eab9fbfd4b94aeef9ced3ce3e3ede21a37ab70ad38b217720f1afd06fceba99b`.
This is not a latest-upstream comparison. All188 production files remain exact.

The run completes 12,696 SDK calls, including primes, with 12,462 timed records
and 78 fresh exported proofs. All78 pass independent Gnark verification on the
server and again after hash-checked transport. Host re-verification does not add
distinct proofs. Canonical verified inventory is64211, SHA-256
`7c582f7f1c471c47b2799188aa6c52e22011aa37e7418f6ef698a9c45efaccba`.
The analysis parser passes six focused ownership/event-format tests.

The server command finishes in4m55.768s. Perf7.0.14 wrapper and actual tool
hashes match before/after. The unit has native30GiB/no swap headroom and
unlimited locked memory for observer rings; these are harness settings, not
architectural requirements. Unit/parent memory event fields are zero; root
fields and several CPU controller files are unavailable and retained as read
errors. The console512KiB peak is inconsistent with process measurements and
is not used as prover memory. No physical-pressure, swap, phone, minimum-RAM
or concurrent-proof qualification is claimed.

Cleanup verifies instance termination, no tagged EBS, deleted private transfer
bucket/objects/multipart uploads, and removal of local secret parameter files.
No live benchmark resource remains. The all-eight CPU10× objective remains
unmet; supplied CPU/Metal references and broader current results are linked in
the full-catalog report. This diagnostic adds no speedup or completed-goal claim.

[Protocol](artifacts/protocol.md), [summary](artifacts/summary.json),
[complete sanitized analysis](artifacts/analysis.json.gz), [checksums](SHA256SUMS).
Private keys, requests, proof bytes, executable binaries, raw profiles and
PID/TID/address streams are excluded. Committed locally; not pushed.
