# SDK link flags: additive arguments and query calibration

**Adding an extern library accidentally removed the final query driver's release profile in two recent studies.** Their `rustc_flags` lists replaced the defaults instead of extending them. The backend libraries remained native release builds. The accepted full-proof SDK retained the correct profile and relinks byte-identically as `6d83f1d6…`; its accepted timings remain unchanged. No previous candidate is promoted or retimed here.

The workflow now accepts `additional_rustc_flags`, appended after the existing base profile. `rustc_flags` keeps its explicit full-override meaning for portable/custom builds. Empty additions are omitted when serializing, preserving old link protocol roundtrips. All 37 xtask tests pass, including three new regressions for extern additions retaining release defaults, explicit portable overrides, and old protocol serialization. The new tool builds in a separate target; old pinned tool binaries and experiment plans are unchanged.

## Scope audit and actual links

A frozen inventory of 391 local plan paths contains 390 workflow plans and 216 SDK link steps. Exactly four non-test explicit base overrides omit O3: control/candidate links in coalesce-query176 and fused-epoch-pin176. Their recorded commands also omit native CPU, fat LTO and single-codegen-unit flags. The fused study's frozen original caller reuses the first study's original binary. This audit covers the enumerated local workflow plans, not every direct compiler command or external runner. Test-only links are recorded separately rather than treated as performance evidence.

Both affected reports receive a compiler-scope note. Their observations, source, receipts and failed gates remain unchanged. They establish results in their recorded driver configuration, without proving a corresponding optimized full-proof outcome. Neither the correction nor this calibration establishes a revised performance sign for either rejected implementation.

The corrected original-query link uses the exact same driver bytes, all eight frozen original native libraries and the same SHA2 library as the old caller. Only the final compiler profile changes. The accepted full SDK separately uses its exact original source/libraries and default compiler recipe, differing only in output path; the produced binary is byte-identical. No cryptographic source is rebuilt.

| Artifact | SHA256 |
|---|---|
| Old original-query caller |`d3d1872f4ace88c99af26fdd807c0114fc63960f50708a65a01bb781ea55c8be`|
| Corrected release original-query caller |`0ba4cd4ebd2747db943c484de7b4784ee32c53c946e0a45ec30ea2f94d15420b`|
| Accepted SDK and default-profile relink |`6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`|

Selected linked functions in the corrected query caller match the accepted SDK's static sizes, instruction/IFMA counts and RSP-operand counts. This is not a whole-program byte-identity claim. The old query caller differs.

| Function | Old query bytes / instructions / IFMA | Release query and accepted SDK |
|---|---:|---:|
| Borrowed G1 stream |26,216 / 4,412 / 800|26,230 / 4,428 / 800|
| Borrowed G2 apply |51,848 / 8,600 / 1,450|51,862 / 8,591 / 1,450|
| Single-job steal |328 / 80 / 0|356 / 84 / 0|

Static counts do not measure executed traffic, spills, critical-path cost or a proving gain. The corrected original-query executable is a reusable diagnostic baseline compiled with the intended profile.

## Fixed compiler-profile calibration

Ryzen 9 9950X, 16 physical cores/32 logical CPUs. Compare the exact old original-query executable with its corrected release link, using the same point/scalar fixtures, full independent Arkworks references, driver bytes, native libraries and original A/B2 APIs. This is a compiler-profile calibration, without an algorithm gain/adoption gate. No old rejected candidate is rerun.

Custom Rayon workers use 512 KiB stacks, individually pinned to CPU 2 / CPUs 0–15 / CPUs 0–31 for 1/16/32 workers. Timers include scalar classification, all point work, ordinary/top fills, component/part merges, weighting, normalization, allocation/drop and pool dispatch/join. B2 scalar reset is inside both timers. Loading, pool setup and independent reference checks are outside. Every timed result is retained in preallocated storage and checked afterward, alongside three checked primes and one post per process; points stay unchanged.

Fifteen fixed paired rotated/reversed blocks cover all 12 contexts, 20 timed queries per process. All 360 latency processes finish before diagnostics. Paired ranks 4–12 of 15 give nominal 96.484375% marginal median intervals, without familywise coverage. There is no exclusion, extension, favorable subset, parameter sweep or unchanged rerun. No owned build/hash/archive/analysis/other benchmark overlaps timing; exclusive host use is not claimed.

| Shape/query | Workers | Old final profile ms | Release final profile ms | Release/old ratio | Bounds | Wins/15 |
|---|---:|---:|---:|---:|---:|---:|
|1x2/A|1|14.672031|14.577134|0.993493|[0.992100,0.996262]|15|
|1x2/A|16|1.546338|1.531635|0.993231|[0.988999,0.995375]|13|
|1x2/A|32|1.206110|1.210634|1.000101|[0.990150,1.021466]|7|
|1x2/B|1|37.241407|37.330670|1.001335|[1.000006,1.003559]|3|
|1x2/B|16|3.498944|3.675231|1.051319|[1.043460,1.066058]|0|
|1x2/B|32|3.089499|3.287474|1.072187|[1.052509,1.090653]|0|
|2x2/A|1|14.530920|14.558530|1.001908|[1.000416,1.003752]|3|
|2x2/A|16|2.144254|1.888153|0.883485|[0.867185,1.048152]|10|
|2x2/A|32|1.755571|1.537643|0.877522|[0.850171,0.893803]|15|
|2x2/B|1|39.797242|39.943816|1.002466|[1.000493,1.003795]|1|
|2x2/B|16|3.991901|4.220299|1.055380|[1.047798,1.072715]|0|
|2x2/B|32|3.596152|3.882992|1.082958|[1.042512,1.100012]|0|

The primary equal-log A/B2 ratio at 32 workers is **1.033617 [1.024078, 1.052536], 0/15 wins**: the corrected driver is 3.36% slower in this calibration. Primary B2 slows 7.22% at 32 workers; primary A is near parity. Conversely, 2×2 A at 32 workers improves 12.25%, with 15/15 wins. Marginal time medians need not divide into paired estimates. The mixed direction demonstrates why the recorded final compiler profile matters; it does not identify a single causal instruction or change the accepted SDK.

## CPU and process memory

| Shape/query | Workers | Old/release CPU ms/query | Old/release process high-water RSS MiB |
|---|---:|---:|---:|
|1x2/A|1|14.672/14.577|10.25/9.50|
|1x2/A|16|16.974/16.955|18.75/19.00|
|1x2/A|32|25.093/24.806|23.75/23.75|
|1x2/B|1|37.240/37.330|15.75/15.50|
|1x2/B|16|44.798/44.848|26.00/27.25|
|1x2/B|32|70.531/71.095|33.25/35.25|
|2x2/A|1|14.531/14.558|13.75/12.75|
|2x2/A|16|17.730/17.665|22.25/23.00|
|2x2/A|32|26.958/26.541|27.75/28.75|
|2x2/B|1|39.797/39.943|27.75/27.25|
|2x2/B|16|48.143/48.081|43.00/43.75|
|2x2/B|32|76.395/76.373|60.75/54.00|

RSS includes loading and primes; it is not complete prover RAM, incremental field workspace or minimum RAM. Raw minor faults and context switches remain in the observations. The B2 wall-time changes are much larger than its CPU changes; compiler/link layout and task execution are combined here, without an isolated scheduling or memory cause. No point/key table, plan, pool, device cap or prover buffer is added by the tooling repair.

## Automatic diagnostics and verification

After all latency: eight FIFO-gated grouped counter panels and eight separate fixed-16-million-cycle profiles, primary A/B2 at 16/32 workers, 200 timed queries after three excluded primes. All four counter events report 100% coverage. These are descriptive panels, without paired counter confidence intervals.

| Query/workers/link | Cycles M/query | Instructions M/query | Generic cache misses M/query | Branch misses M/query | Samples |
|---|---:|---:|---:|---:|---:|
|A/16/old|85.716|208.431|0.209506|0.137746|1063|
|A/16/release|85.374|207.339|0.204067|0.136294|1062|
|A/32/old|114.795|213.997|0.282945|0.163830|1416|
|A/32/release|113.870|213.215|0.284829|0.169352|1408|
|B/16/old|223.642|550.621|0.874345|0.267321|2790|
|B/16/release|223.464|549.785|0.746998|0.265002|2788|
|B/32/old|319.380|569.400|1.235648|0.317869|3976|
|B/32/release|321.863|567.841|1.158118|0.312241|4030|

All 18,533 samples have the expected executable PID, registered worker/caller TID, worker CPU, binary MMAP, worker FORK/task evidence and fixed period. Every recording has zero lost records and at least 1,000 samples; unknown shares stay below 1.37%. Symbols and single panels do not establish exclusive costs, removable wall time, local CPI, DRAM bandwidth or absolute sampling coverage.

The two link steps and 440 capture steps complete once: 24 preflights, 360 latency processes, 16 diagnostics, eight planned ownership changes on only generated perf.data, and 32 read-only decoders. All 400 callers check 12,024 complete-query outputs including primes/post, against existing independent full-MSM references. There is no witness solve, oracle rerun, new proof request or Gnark export. Original field/core qualification is inherited from unchanged libraries; no new cryptographic suite is claimed.

Independent review checks 442 raw command/exit/stream receipts, 594 pin entries, all 176 production source hashes, the four actual extern-only commands, unchanged old tool identity, new regression-test results, corrected compiler arguments, exact default-SDK relink identity, fixtures/oracles/libraries, every result receipt, all paired wall/CPU/RSS distributions, all counter values and every profile sample/symbol bin. It reruns no query or proof. No build, link, test, capture or review fails in this study. A pre-link utility comparison was narrowed to actual rustc arguments so timeout-wrapper differences are excluded; it does not change compilation or measurements.

## Accepted proof comparisons — inherited, unchanged

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge 24→1|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Accepted merge 54→1 is 161.883899 ms without a supplied M5 reference. Accepted primary process RSS is 132.75 MiB. The [adoption report](../2026-10-30-g2-vector-sign-adoption176/README.md) retains its physical-16 calibration engineering exception and qualification scope. No new memory or platform result follows.

Latest AWS primary remains historical source 175 on c7a.24xlarge: Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96, each campaign's best tested configuration. That does not compare current source 176. No AWS resources were created here.

The original all-eight CPU 10× goal remains active and unmet, 0/8. This repair makes future caller experiments use the intended compiler profile; it supplies no new full-proof speedup or backend adoption. No new AWS, mobile/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput result follows.

[Design](artifacts/design.md), [prospective calibration protocol](artifacts/capture-design.md), [plan inventory](artifacts/flag-audit.json), [actual links and identities](artifacts/link-review.json), [all observations](artifacts/analysis.json), [symbols](artifacts/symbols.json), [independent review](artifacts/review.json), [static code](artifacts/codegen.json), [37 xtask tests](artifacts/xtask-tests.log). Archives: [tooling source and helpers](artifacts/workflow.tar.gz), [all raw link/capture receipts and counter CSVs](artifacts/receipts.tar.gz), [selected disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS).

The corrected baseline is retained locally at `/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/links/query-release-sdk`. Raw inputs, executables, libraries, perf.data and witness-derived point outputs are not published; hashes are retained.
