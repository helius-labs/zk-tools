# G2 norm replay: complete SDK pilot

Halving the G2 prefix record does **not** improve primary proof latency. ON is **1.30% slower than fresh OFF** and **1.34% slower than accepted** by paired median, losing all 15 blocks against both controls. Calibration passes. The fixed recipe is closed without adoption, an unchanged reroll or a worker, compiler, layout or favorable-subset rescue.

## Change and qualification

The [native qualification](../2026-11-07-g2-prefix-replay-native176/README.md) establishes 126 OFF/126 ON core tests and exact recovery of three changed parent files; all 18 field parents remain exact. Both owned and signed-borrowed G2 streaming discard the forward norm and recompute it from the denominator already reconstructed in reverse. Record payload falls **640→320 bytes per eight-point group**, with one additional fused norm. G1, generic trees, scalar tails, keys, solver, queues and scheduler remain unchanged.

Coherent OFF/ON SDKs use eight libraries from one Cargo dependency directory per mode and the unchanged accepted driver/perf support. Actual links use O3, native CPU, fat LTO and one codegen unit. Two prover-library and six cubic integration tests pass. Accepted/OFF/ON SDK hashes begin `6d83f1d6`, `484386df` and `49b741ce`; [full identities](artifacts/sdk-identities.json).

Before timing, **27 fresh exports** pass independent Gnark verification across all eight supplied shapes plus merge54 and all three arms. Their image/public/proof bindings and ledger freshness pass the separate reviewer. Preflight clocks are diagnostic and excluded from performance inference.

## Matched primary latency

Ryzen 9 9950X, 32 logical workers pinned to CPUs 0–31. Fifteen fixed rotated/reversed blocks; each arm performs three primes and 151 complete SDK requests per process. All latency processes finish before export verification, counters or profiles. No other owned CPU workload overlaps recording. Every observation is retained.

|Arm|Median process mean, ms|Median process maximum RSS, MiB|Largest observed process maximum RSS, MiB|
|---|---:|---:|---:|
|Accepted|7.957454|130.00|134.75|
|Fresh OFF|7.959647|130.25|133.50|
|ON|8.082184|130.25|134.50|

|Paired comparison|Median ratio|Interval|Wins of 15|
|---|---:|---:|---:|
|OFF/accepted|1.003266|0.993700–1.005904|6|
|ON/OFF|1.012971|1.010123–1.018885|0|
|ON/accepted|1.013422|1.010936–1.019133|0|

Ratios pair process means by block and need not equal ratios of the displayed medians. Sorted ranks 4–12 give nominal 96.484375% marginal median coverage under independent, exchangeable blocks; no familywise guarantee is claimed. The frozen gate requires ON upper ratios below 1 and at least 10 wins against both controls, with OFF/accepted bounds inside [1/1.03, 1.03]. Both gain checks fail; calibration passes. There is no minimum gain threshold, exclusion or extension.

## Counters, profiles and memory

After latency, each arm runs one 31-request counter panel and one 251-request profile, with three primes and FIFO gating. Profiles use `cycles:u`, period 16 million and 512 mmap pages. All counters report 100% running.

|User counter per request|OFF|ON|ON/OFF|
|---|---:|---:|---:|
|Cycles|818,359,068|830,902,010|1.015327|
|Instructions|1,386,622,522|1,401,187,810|1.010504|
|Cache misses|2,705,184|2,883,146|1.065785|
|Branch misses|914,940|891,670|0.974567|

These single diagnostic panels are not replicated timing estimates or a causal cache/DRAM study. More instructions and user cycles accompany the unfavorable latency result; the smaller record does not produce an observed compensation here.

Accepted/OFF/ON profiles contain 12,381/12,389/12,562 samples, zero recorded LOST events and 1.72%/1.92%/1.35% unknown symbols. Review checks headers/events, periods, one PID, 33 TIDs and CPUs 0–31. These are sampled user cycles, not exclusive wall time or removable critical-path fractions. No exact IP-to-ELF or independent second LOST parser is claimed.

The actual G2 caller has **1,450→1,600 static IFMA sites**, matching the extra norm, and its prologue RSP subtraction grows **5,952→6,016 bytes**. G1 stays at 800 sites and 4,288 bytes. Both callers push six registers and align separately. These selected prologues exclude alignment loss, nested frames and concurrent workers; static sites are not dynamic counts.

The 50% logical record reduction remains a valid result. Whole-process RSS shows no demonstrated overall memory reduction in this pilot. It includes setup/mappings and is not minimum RAM or an incremental live-memory bound. No new table, key copy, retained plan, pool or heap allocation site is added. Mobile, huge-circuit, pressure and swap qualification remain outside this experiment.

## Evidence and outcome

All 45 latency exports and six diagnostic exports pass independent Gnark and freshness review. With preflight, the study completes **7,902 SDK requests including primes and 78 fresh independently verified exports**. The canonical ledger advances **31,465→31,543**; intermediate and prior ledgers remain immutable. Successful native, field, model, build, preflight and verifier stages are not replayed for bookkeeping.

The outcome-neutral reviewer was compiled before recording. It independently reconstructs timing gates, counters/sample totals, 117 capture/verification/offline receipts, 246 input pins and all 51 post-preflight export bindings. The separate caller reader checks four new decode receipts/streams. All SDK, preflight, capture and review workflow commands in this stage pass on first execution. Earlier native preparation/reviewer repairs remain documented in the native report; no successful native suite is repeated here.

Production/source176 and accepted SDK remain unchanged. Accepted historical matched local **1×2 remains 7.915095 ms**, versus supplied M5 CPU 37 ms and Metal 24 ms. This pilot's 7.957454 ms accepted control remains a separate contemporaneous observation. Merge54 remains 161.718956 ms. The verified CPU10× objective is active and unmet on all eight shapes; [inherited full comparisons](../2026-11-07-g2-real-norm-z176/README.md). No new AWS or Aeglos comparison is added.

[Protocol](artifacts/protocol.json), [latency](artifacts/latency-analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [independent review](artifacts/independent-review.json), [caller stack](artifacts/caller-stack-analysis.json) and [preflight review](artifacts/preflight-review.json) accompany sources, pins, plans and compressed exact raw receipts/streams. The archive is compared against its originals. Actual proofs/public payloads, keys, requests, libraries, binaries and raw perf data remain private with hashes retained. Private root: `/home/ubuntu/src/zopro-perf-work/g2-prefix-replay176`.
