# Signed-ternary G2 complete-query comparison

**Not adopted.** The fixed radix-6561 caller is about 6–7% faster with one worker, but 6.7–13.1% slower at 16/32 workers versus the same executable's original path. The declared complete-caller gate fails. Primary32 diagnostics nevertheless record 8.6% fewer instructions and 7.1% fewer user cycles versus fresh OFF. Lower total work does not establish lower parallel elapsed time. Production and accepted proof performance remain unchanged.

## Implementation and correctness

The preceding [signed-ternary screen](../2026-11-07-signed-ternary-screen176/README.md) measured a 6.85–6.92% reduction in modeled generic joins. This implementation tests that fixed representation without a parameter sweep: eight trits per window, radix6561, ten magnitude-3280 ordinary windows padded to3328slots, one magnitude-1 carry, and the original two point partitions. A random-access integer codec reads the existing32-byte signed component storage; no expanded digit array, per-point precomputation table, duplicate key/plan or production thread pool is added.

The child reuses the original borrowed Window's pending/collision queues, BatchAdd arithmetic, signs and exceptional-point handling. Batch size stays200. The high component is transformed before feeding the low component. The carry uses two signed indexed groups and existing grouping/merge arithmetic. Ordinary weighted sums use thirteen full256-element subtrees with zero padding. Basic Horner tripling uses doubling plus addition. Original grouping/Phi2 and outside-range fallbacks remain.

The isolated178-file snapshot has173accepted originals exact, three append-only parents and two children. Reversing the additions recovers all176accepted hashes; field arithmetic/generator and original runtime bodies are unchanged. All128unfiltered native core tests pass:125original plus three new tests. New coverage includes complete generator/exponent oracles, full-width scalars, grouping, identities/repeated/opposite points, carry boundaries and1/4/16/32workers with512KiBtest stacks. Field/Phi/prover qualification is inherited, not rerun.

The original workflow stopped after the successful test command because required-test metadata used short names instead of full module paths. Its first offline checker then expected RUSTC in a deliberately restricted environment receipt. The second checker validates the exact managed projection and full test names. Original sources/plans/failures remain; no successful suite is repeated. Driver compilation separately stopped on a missing Debug derive on a local metadata type. The corrected driver copy changes only that derive and reuses the successful lockfile/core build; one repair-helper compile correction is retained. No cryptographic source, test, input or successful capture is changed or rerun for these harness repairs.

## Controls and recording

Fresh OFF/ON share binary `b3cb2d26210e243a7b2a014771da3d5541af800fd9fe1c92ad953f5c0e6a31c6`. Frozen uses a previously optimized original-algorithm diagnostic caller, binary `1ed45e0144420f4f8eb0a801c40fd1ad9fc9929de9a7d31b6da58a7dff509118`, with both packing/draining selectors disabled. It is not the accepted SDK. Source checks bind its active original query and kernel bodies; its disabled queue guard remains. Registry packages match and six direct dependencies are hashed. Both consumers' actual final rustc commands includeO3,native CPU,fatLTOand one codegen unit.

Frozen normalizes after pool return on the main thread; fresh OFF/ON normalize inside the worker callback. Both placements are included in all clocks. This explicit difference and compiler context limit isolated attribution to the representation alone.

Eighteen preflights precede31fixed rotated/reversed blocks across six contexts and three arms:558latencyprocesses. Every latency process runs three primes,17observed complete queries and one postcheck. WorkerCPUs are2 /0–15 /0–31 for1/16/32workers, with512KiBstacks. Canonical scalar reset, original grouping, all point work, allocation/drop, normalization and pool dispatch/join are inside timers. Loading, pool construction, immutable-input hashes and full independent output checks are outside. Every output matches the previously pinned complete Arkworks MSM oracle; fixtures/oracles are not regenerated.

Times are medians of31process elapsed-loop means. Paired bounds use sorted ratios at ranks10/16/22, nominal97.0550626516%marginal median coverage under independent exchangeable blocks, not familywise coverage. No observation is excluded or schedule extended.

|Context|Frozen ms|OFF ms|ON ms|ON/OFF [bounds],wins/31|ON/frozen [bounds],wins/31|
|---|---:|---:|---:|---|---|
|1x2-B-w1|37.204511|37.328566|34.813463|0.932514 [0.931127, 0.934406],31|0.935538 [0.934375, 0.936784],31|
|1x2-B-w16|3.714871|3.479106|3.942947|1.130968 [1.122287, 1.143237],0|1.059984 [1.052908, 1.073523],0|
|1x2-B-w32|3.322107|3.065364|3.333961|1.091620 [1.078285, 1.098713],0|1.003319 [0.993043, 1.008430],13|
|2x2-B-w1|39.860234|39.739079|37.390981|0.940345 [0.939734, 0.942218],31|0.937731 [0.936402, 0.938967],31|
|2x2-B-w16|4.280199|3.959486|4.377553|1.108078 [1.099765, 1.117500],0|1.023665 [1.018209, 1.032933],0|
|2x2-B-w32|3.963119|3.583859|3.807375|1.067157 [1.056774, 1.083971],0|0.966230 [0.941879, 0.984588],27|

## Declared decision

Every OFF/frozen interval must fit[1/1.03,1.03]; every ON/control upper bound must be≤1.03in all six contexts. Primary32and the equal-log two-population32aggregate must have upper<1and≥21/31wins against both controls. There is no minimum gain floor. All four threaded OFF/frozen calibrations fail; the fresh original path is faster than the frozen caller beyond the declared range. This is not an established cause or permission to reroll.

The same-executable comparison is unfavorable at every threaded context, with0/31wins. Primary32and aggregate gain checks fail versus OFF; primary32also fails versus frozen. Nonregression fails in four ON/OFF contexts and two ON/frozen contexts. Thirteen total checks fail. All gates and positive observations remain.

|Two-population32aggregate|Ratio [bounds],wins/31|
|---|---|
|ON/OFF|1.079861 [1.075722, 1.087062],0|
|ON/FROZEN|0.984857 [0.972707, 0.993411],29|

The aggregate versus frozen is positive, while calibration and the fresh-control comparison fail. It does not qualify a baseline improvement. Solo gains do not replace the declared threaded checks. Close this exact fixed implementation without SDK follow-up, unchanged retry, extension or favorable worker/subset promotion.

## CPU service and observed process memory

|Context|CPU ms/query frozen / OFF / ON|Maximum RSS MiB frozen / OFF / ON|
|---|---|---|
|1x2-B-w1|37.204 / 37.328 / 34.813|15.50 / 15.25 / 15.25|
|1x2-B-w16|44.938 / 45.029 / 42.745|28.00 / 27.50 / 32.00|
|1x2-B-w32|70.726 / 71.000 / 63.569|34.50 / 34.50 / 37.25|
|2x2-B-w1|39.860 / 39.739 / 37.391|27.25 / 27.00 / 27.00|
|2x2-B-w16|48.107 / 48.329 / 46.067|44.00 / 43.25 / 47.25|
|2x2-B-w32|76.564 / 77.364 / 69.698|61.25 / 56.00 / 60.75|

CPU is summed process user+system service per query, summarized across process means. Primary32OFF→ON falls71.000→63.569CPUms while elapsed time rises3.065→3.334ms. This supports lower service demand with worse parallel completion; it does not identify the critical path, useful CPU utilization, a unique scheduler defect or a memory bandwidth limit.

RSS is the maximum per-process high-water across latency processes, including input load/primes. It is not complete prover RAM, minimum RAM, a rigorous allocation/lifetime peak or pressure qualification. The preceding affine-bucket formula grows about3.11MiB with padding, while the original32-byte scalar storage remains. Binary workspace admission is unchanged and is not claimed valid for ternary; the diagnostic path is never wired into the SDK.

## Fixed diagnostics

After all latency/output checks, primary32has one stat panel(201observed queries)and one separate cycles:u profile(251)per arm, each with three primes and one postcheck. FIFO gates exclude primes/output checks. Four grouped events run100%. Fixed16million-cycle profiles use512mmap pages; all pass≥1000samples,≤5%unknown,zeroLOSTand ownedPID/TID/workerCPU/MMAP/event checks.

|Arm|Cycle samples|Unknown %|LOST|
|---|---:|---:|---:|
|FROZEN|5037|0.854|0|
|OFF|5035|0.854|0|
|ON|4707|0.999|0|

|Counter|ON/OFF|ON/frozen|
|---|---:|---:|
|cycles:u|0.928984|0.935133|
|instructions:u|0.913756|0.911984|
|cache-misses:u|0.931892|1.114536|
|branch-misses:u|0.807296|0.825992|

Borrowed-apply samples fall2018→1815and reduce-layer804→341from OFFtoON; inverse samples fall376→187. Ternary feed has500samples versus328for the original feed, and epoch-pin samples469→495. These are flat inlined sample bins in separate one-shot profiles, not exclusive time or removable wall fractions. The total work reduction persists, while elapsed time worsens. No repeated-panel uncertainty, precise IP-to-ELF mapping, critical-path, DRAM or fault-service causality is claimed. The qualified raw-loss helper is reused once per new profile.

## Review and accounting

The independent offline reader passes3,153pin occurrences,598capture/validation/offline command receipts,all582caller processes,13,188complete queries,raw means/ranks/rotations/gates,diagnostic counts and176unchanged production hashes/source reversals. It verifies the Debug-only driver repair and reuses native/core/Phi/Arkworks qualification. It is not a second curve oracle, cryptographic verifier, disassembler or loss decoder.

Counts:90preflight queries +11,718latency +1,380diagnostic. These are complete MSM queries, not proofs. No new SDK request or independent proof export exists; the canonical ledger stays32,196. No successful test, model, capture, oracle or decoder is repeated for bookkeeping.

## Accepted CPU and M5 references, inherited

Production176/SDK `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661` remain unchanged. These local proof observations come from the preceding [half-constant comparison's accepted build](../2026-11-07-pairing-half-constant176/README.md); they are not ternary proof measurements. M5CPU/Metal values are supplied historical references; different machines do not establish equal hardware efficiency.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU10×limit ms|
|---|---:|---:|---:|---:|
|1x2|8.041470|37|24|3.7|
|2x2|11.237839|59|30|5.9|
|4x2|16.143230|109|48|10.9|
|8x2|24.397442|189|74|18.9|
|12x2|33.736405|284|101|28.4|
|16x2|40.082192|335|124|33.5|
|24x2|65.765454|528|176|52.8|
|merge-24x1|65.350546|507|175|50.7|

Merge54remains161.495859ms,without a supplied M5reference. All-eightCPU10×goal remains **active/unmet,0/8**. No fresh AWS/Aeglos,large250/4999,phone/ARM,minimumRAM,pressure/swap or concurrent-throughput claim is made.

Next inspect the already-captured worker/sample distributions to resolve a concrete parallel-completion question. This does not reopen the fixed ternary candidate or rerun profiles; any further implementation needs a distinct mechanism, bounded-memory argument and new declared test.

[Artifacts](artifacts/) preserve designs, original/repair source and failures, changed source parents/children,driver/lockfiles,pins,raw query observations,exact command records,profile decodes/metadata,analyzers and independent review. Private point/scalar/key/witness inputs,raw perf data,executables,libraries and targets are excluded. Archive members are individually hashed and a single-pass tar comparison passes. Reproduction requires the pinned toolchain and private fixtures.
