# Private-deque bitmap: complete SDK comparison

The ready-victims bitmap is **rejected for adoption**. At 32 workers, 1×2 takes **7.937564 ms**, versus **7.846253 ms** for the fresh bitmap-disabled control and **7.864582 ms** for the accepted executable. The paired candidate/control ratio is **1.009619 [1.007835, 1.011402]**, with **0/31 wins**; the all-eight equal-log aggregate is **2.21% slower** than the fresh control. Peak RSS and total CPU time fall for primary, but proof latency increases. Production remains unchanged.

All eleven fresh-control calibrations pass. This is a measured negative result under the declared protocol, rather than a failure to calibrate. Independent raw-data review passes and reproduces all **79 checks: 66 pass, 13 fail**. The exact candidate recipe is closed without an unchanged rerun, favorable worker/shape subset or AWS rescue.

The private Rayon 1.13.0 mechanism advertises nonempty owner-private deques in an atomic bitmap and searches observed set bits. It adds owner updates and changes discovery/sleep behavior. It introduces no new pool, proving-key table or solver plan. At 96 workers on this 64-bit host, the bitmap words contain 16 bytes, excluding its object, per-worker state, alignment and allocator overhead; this is not measured RSS. [Source qualification and limitations](../2026-11-10-ready-victims176/README.md) remain applicable.

REF is the accepted Zen5 executable; OFF and ON are coherent fresh eight-library builds from the same accepted 176-file source, with only reversible Cargo path-patch metadata and the qualified private Rayon fork. ON enables `zopro_ready_victims`; both preserve all accepted flags including `zopro_dense_busy_reset`. Rust 1.98.1 / LLVM 22.1.8, native O3, fat LTO, one codegen unit, default SDK worker stacks and the accepted fixed-placement driver are used. The private model checker is not a prover dependency.

Measurements run serially on the local Ryzen 9950X, 16 physical cores / 32 logical CPUs. Workers use CPU 2 for solo, 0–15 for 16 workers and 0–31 for 32. This is a shared host; exclusivity is not claimed.

|Condition|Accepted REF ms|Fresh OFF ms|Bitmap ON ms|ON/OFF paired ratio [95% CI]; wins|ON/REF paired ratio [95% CI]; wins|
|---|---:|---:|---:|---|---|
|1x2-w32|7.864582|7.846253|7.937564|1.009619 [1.007835, 1.011402]; 0|1.008251 [1.006631, 1.009797]; 2|
|2x2-w32|11.013809|10.973340|11.245271|1.023093 [1.018742, 1.027755]; 0|1.021463 [1.015024, 1.028069]; 1|
|4x2-w32|15.916562|15.795607|15.995879|1.018480 [1.007896, 1.029707]; 2|1.014331 [1.004395, 1.024711]; 4|
|8x2-w32|24.100975|24.072703|24.546871|1.021216 [1.010137, 1.032193]; 2|1.022378 [1.011980, 1.033303]; 1|
|12x2-w32|33.846893|33.736389|35.008620|1.037038 [1.028685, 1.047050]; 0|1.030071 [1.020212, 1.039712]; 1|
|16x2-w32|39.951809|39.798608|40.948675|1.031202 [1.025209, 1.038511]; 0|1.025946 [1.019156, 1.033809]; 0|
|24x2-w32|66.273552|66.558512|67.623824|1.013584 [1.008738, 1.018460]; 2|1.017692 [1.011406, 1.022096]; 1|
|merge-24x1-w32|65.654589|65.620056|66.950110|1.022723 [1.017359, 1.027741]; 0|1.024905 [1.019159, 1.031230]; 0|
|merge-54x1-w32|162.996094|163.670933|162.735741|0.995633 [0.991443, 0.999531]; 9|0.996505 [0.992104, 1.001005]; 9|
|1x2-w1|95.216713|94.192135|94.587734|1.004211 [1.002070, 1.006293]; 3|0.993533 [0.991758, 0.995413]; 15|
|1x2-w16|9.263141|9.204481|9.260193|1.006404 [1.005087, 1.007776]; 1|1.000112 [0.998834, 1.001466]; 16|

Latencies are medians of process means. A paired ratio below 1 favors ON; it need not equal the ratio of displayed medians. Primary 16/32 use 31 matched blocks of 151 requests; other conditions use 15 blocks of 31, except solo with 11 requests. Arm/case order rotates and reverses prospectively. Each process primes three times. Every timed request solves, generates a freshly randomized proof, internally verifies and serializes it; loading, pool creation, primes and export are outside the SDK interval.

The bootstrap uses the inherited qualified paired-log statistic, 20,000 resamples, fixed SplitMix64 seed `0x551c36a48ee2d971` and ranks 499/19,499. The equal-log all-eight aggregate resamples within each shape independently, with 31 primary and 15 other blocks. It is 1.022086 [1.019462, 1.024766] against OFF and 1.020609 [1.017882, 1.023330] against REF. Nominal individual coverage assumes exchangeable blocks; there is no familywise guarantee.

54→1 has a small favorable paired result against OFF, but its interval against REF crosses 1. The complete campaign fails its required primary and aggregate checks; selecting that one shape is not an adoption decision.

|Condition|REF CPU ms / peak MiB|OFF CPU ms / peak MiB|ON CPU ms / peak MiB|
|---|---:|---:|---:|
|1x2-w32|178.598 / 134.750|178.128 / 132.750|172.502 / 104.500|
|2x2-w32|258.690 / 271.000|258.111 / 276.000|252.800 / 273.250|
|4x2-w32|422.590 / 323.500|420.940 / 320.500|404.648 / 305.500|
|8x2-w32|676.854 / 503.750|675.653 / 508.250|668.013 / 491.500|
|12x2-w32|949.816 / 725.250|945.221 / 722.000|955.704 / 649.750|
|16x2-w32|1135.686 / 850.250|1131.151 / 816.250|1131.780 / 748.164|
|24x2-w32|1927.953 / 1299.578|1927.457 / 1301.500|1933.671 / 1028.500|
|merge-24x1-w32|1842.861 / 1167.488|1850.379 / 1151.176|1846.557 / 991.301|
|merge-54x1-w32|4523.315 / 1985.016|4533.180 / 1968.898|4471.615 / 1694.496|
|1x2-w1|95.214 / 34.250|94.190 / 34.250|94.585 / 34.250|
|1x2-w16|118.300 / 88.500|117.163 / 89.000|116.439 / 80.750|

CPU is the median per-request summed process CPU time. RSS is the maximum lifetime process observation across latency processes, including load, primes and allocator history. It is not minimum RAM, incremental scratch, a concurrency capacity estimate or a measurement under memory pressure. All 22 declared RSS nonregression checks pass. The bitmap's tiny source allocation does not itself explain the measured RSS difference.

At primary/32, ON reduces median CPU from 178.128 to 172.502 ms and peak RSS from 132.75 to 104.50 MiB versus OFF. User CPU falls from 173.360 to 165.714 ms/request, while system CPU rises from 4.734 to 6.850. Median voluntary context switches rise from 359.50 to 690.07 per request; involuntary switches are 3.17 versus 3.06. Minor faults fall from 129.13 to 75.40, with median major faults zero in both. These are diagnostic observations: context switches are not syscall counts, minor faults are not disk paging, and fault counts do not measure servicing time.

Automatic PMU/profile panels use primary at 16/32 workers and 2×2 at 32. Each arm has one 31-request PMU capture and one 251-request cycle profile, FIFO-gated around the SDK loop, with fixed `cycles:u` period 16,000,000. All four counters run at 100% in all nine panels. The nine profiles contain 117,640 samples, zero reported LOST events and 1.356–2.3234% unknown samples. Binary, PID, TID, worker CPU and allowed coordinator masks are checked against raw records.

At primary/32, OFF/ON epoch-symbol shares fall from 10.01% to 3.25%, while runtime-symbol shares rise from 4.52% to 7.52%. Primary/16 epoch shares fall from 3.96% to 1.43%; 2×2/32 falls from 10.75% to 3.53%. These are flat-symbol diagnostic classifications: runtime symbols can include inlined useful payload, shares have no confidence interval, absolute sampled-period coverage is unvalidated, and they are not idle time or removable wall time. Lower discovery/epoch sampling alone did not shorten the critical path. Changed handoff/parallelism costs are consistent with the observations but are not isolated causally.

Primary/32 PMU per-request OFF/ON cycles are 805.28M / 768.77M, instructions 1,369.63M / 1,336.60M, cache misses 2.661M / 2.681M and branch misses 0.879M / 0.847M. Lower counted work coexists with slower complete proofs. These single-panel values have no paired uncertainty estimate.

|Declared check group|Pass / total|
|---|---:|
|ALL8_gain|0 / 2|
|PMU_quality|9 / 9|
|RSS|22 / 22|
|calibration|11 / 11|
|nonregression|15 / 22|
|primary_gain_and_wins|0 / 4|
|profile_quality|9 / 9|

Calibration requires OFF/REF's entire paired interval inside [1/1.03, 1.03]. Nonregression requires ON/control's upper bound ≤1.03; RSS requires ON maximum ≤1.05× each control maximum. Primary requires upper ratio <1 and ≥21/31 wins against each control. Both all-eight aggregate upper bounds must be <1. PMU requires four finite counters with ≥99.9% running; profiles require ≥1,000 samples, ≤5% unknown, zero LOST and checked ownership/period. All gates are frozen before capture. The 13 failed checks are:

```text
12x2-w32/ON/OFF/nonregression
12x2-w32/ON/REF/nonregression
16x2-w32/ON/OFF/nonregression
16x2-w32/ON/REF/nonregression
1x2-w32/ON/OFF/primary-gain
1x2-w32/ON/OFF/primary-wins
1x2-w32/ON/REF/primary-gain
1x2-w32/ON/REF/primary-wins
8x2-w32/ON/OFF/nonregression
8x2-w32/ON/REF/nonregression
ALL8/ON/OFF/gain
ALL8/ON/REF/gain
merge-24x1-w32/ON/REF/nonregression
```

Fresh native integration passes 127 core, 2 prover and 6 cubic tests; the generic no-ASM/no-IFMA check compiles successfully. The prior 114 Rayon passes, two ignored tests and three bounded models are inherited without replay. The qualification-only checker repair, original failed stock-checker outcomes, finite model bounds and unavailable overwritten original model executable remain explicit in the preceding report. Generic host compilation is not ARM/mobile execution.

There are **1,331 successful workflow commands**: 7 runtime build/link/test commands, 4 initial helper builds, 4 repaired-validator builds, 1 typed-link-aware manager build, 52 functional-preflight commands and 1,263 capture/offline/verification commands. The preflight makes 104 SDK calls and exports 26 new proofs. Capture records 591 latency processes plus nine PMU and nine profile processes, making 44,106 SDK calls including primes and exporting 609 new proofs. All recordings finish before 45 offline commands and 609 independent Gnark checks. Total: **44,210 fresh SDK requests and 635 distinct, independently Gnark-verified exports**. The canonical unique-proof ledger advances **46,167 → 46,193 → 46,802**. No successful proof, native qualification, decoder or verifier is replayed for bookkeeping. Direct preparers/readers and the successful PMU capability check are outside the workflow-command count.

Preparation limitations are retained. The first plan's unsupported `additional_rustc_flags` is rejected before any workflow command; equivalent complete `rustc_flags` fixes the schema. The first preflight preparer expects PATH in a receipt that intentionally redacts ambient PATH/RUSTC and stops before any SDK job. Compiler identity is instead checked from pinned Cargo rustc-info and ELF markers. Only affected validator helpers are rebuilt, preserving their originals; successful runtime builds are not repeated. A typed-link-aware manager is added prospectively without altering sampling, statistics, gates or the frozen preflight helpers. Original sources, repairs and receipts are archived.

Independent review by a separate reader in the same agent reconstructs all 591 raw process means, paired/aggregate decisions, 609 new proof bindings and profile ownership. It shares the qualified statistics and diagnostic parsers, so it is not an independent implementation of those components or external review. Supplementary closeout checks all 1,331 command receipts/streams, 198 resource medians, profile category counts, source recovery and the 46,802-entry unique ledger. All 176 accepted production files remain exact; two private Cargo parents reverse to accepted bytes.

The original verified CPU AVX single-proof 10× objective remains **active and unmet, 0/8**. Below, fresh local REF/OFF/ON are this campaign. M5 CPU and Metal are supplied historical references on different hardware, not same-host competitor measurements.

|Shape|Fresh accepted REF ms|Fresh OFF ms|Rejected ON ms|M5 CPU ms|M5 Metal ms|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.864582|7.846253|7.937564|37|24.0|3.7|
|2x2|11.013809|10.973340|11.245271|59|30.0|5.9|
|4x2|15.916562|15.795607|15.995879|109|48.0|10.9|
|8x2|24.100975|24.072703|24.546871|189|74.0|18.9|
|12x2|33.846893|33.736389|35.008620|284|101.0|28.4|
|16x2|39.951809|39.798608|40.948675|335|124.0|33.5|
|24x2|66.273552|66.558512|67.623824|528|176.0|52.8|
|merge-24x1|65.654589|65.620056|66.950110|507|175.0|50.7|

54→1 has no supplied M5 reference. No new AWS or Aeglos measurement occurs here. The accepted [c7a comparison](../2026-11-10-accepted-current-c7a176/README.md) remains separate: best tested 1×2 Zopro is 11.243892 ms at 32 workers versus Aeglos 7.902802 ms at 96; at the same 32 workers Aeglos is 11.870012 ms. Best 54→1 Zopro is 120.757128 ms versus Aeglos 171.908504 ms. These accepted AWS measurements do not benchmark this rejected bitmap candidate. No owned cloud resources remain.

The next investigation should target a distinct reduction in useful online work, beginning with an algebraic/source census rather than another adjustment to this closed scheduler recipe. No new phone, ARM, TEE, batch-250/4999, pressure/swap, minimum-RAM or concurrent-throughput result is established.

Evidence: [full analysis](artifacts/analysis.json), [independent reader](artifacts/review-result.json), [closeout](artifacts/closeout.json), [resource/profile summary](artifacts/report-summary.json), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [prospective protocol](artifacts/protocol.md), [authored helpers, raw rows, plans and command streams](artifacts/evidence.json.gz), [losslessly compressed original nine perf captures](artifacts/perf-data.tar.gz), [publication checks](artifacts/quality.json), [checksums](SHA256SUMS). Source paths are local absolute paths requiring adaptation to reproduce. Keys, requests, solver/witness/proof payloads, full canonical ledgers, executable/library blobs, build targets and full third-party trees are excluded.
