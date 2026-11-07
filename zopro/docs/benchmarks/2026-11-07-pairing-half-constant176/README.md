# Compile-time half constant in G2 preparation

The candidate replaces one runtime inversion of 2 in each nonidentity G2 constructor with its compile-time Fq value. Correctness and emitted-call checks pass, but the complete build fails the declared all-nine nonregression check: several larger shapes are slower. **The candidate is rejected; accepted production remains unchanged.** Local 1×2 is 8.006 ms versus accepted 8.041 ms, without a demonstrated paired gain.

## Change and correctness

Only the local `two_inv` binding in `G2Prepared::new` changes. A Rust const block evaluates `(p+1)/2` in the existing Montgomery representation. The identity early return, line formulas/order, vector capacity and API remain original. The logical value occupies 32 bytes independent of keys or circuit size; this is not a whole-binary or process-memory bound. No new allocation, buffer, key table, retained plan, pool or runtime option is added.

An independent four-limb integer calculation checks `2·half=p+1`. The focused field test also checks `2·half=1` in Fq and equality with the original inverse. It compares all prepared coefficients with an unchanged original-constructor reference over 34 public inputs: identity, generator and 32 full-width derived points. The 33 nonidentity inputs produce **2,871 lines**. Both native and portable configurations reapply this same population; it is not 68 distinct inputs.

Each configuration also reruns the existing independent-Arkworks pairing test: eight full-width pairing comparisons plus a pairing-product identity and a negative case. Native runs 2 tests with 34 unrelated tests filtered; portable generic/no_asm/no_ifma runs 2 with 10 filtered. These are fresh checks of the changed preparation path, not fresh full field/core/prover suites or physical-phone qualification. All four native build/test/link workflow steps pass on first execution.

The isolated source has 176 files: 175 accepted originals exact and one parent with a reversible binding replacement plus a test-only suffix. Reversing both edits recovers every accepted hash. Production remains unchanged.

## Actual linked constructor

The candidate SDK coherently links eight libraries with the actual final `-O`, native CPU, fat LTO and one codegen unit. Accepted SDK6d83 is the frozen comparison. Its saved symbol listing is reused; only the new candidate is listed, and each previously unaudited constructor is decoded once.

|Linked constructor|Bytes|Decoded instructions|Direct Fq inverse calls|
|---|---:|---:|---:|
|Accepted|38,297|7,046|1|
|Candidate|38,156|7,025|0|

The original compiler did not fold that inverse call. The candidate removes the call and 21 constructor instructions. This checks emitted work at this boundary, not dynamic proof instructions, cache behavior or latency. Other prover inversions remain; no causal whole-proof speed fraction is inferred.

The first descriptive row counter included byte-continuation rows: 8,236/8,223 addressed rows, including 1,190/1,198 continuations. The independent reader separates actual mnemonic rows and supplies the corrected table above. Original counts and sources remain. The inverse-call counts used for selection were correct and unchanged; no decoder, qualification, capture or verifier is repeated for this classification correction.

## Complete proof correctness

All nine supplied shapes run with 32 workers on CPUs 0–31, each with three primes and one observed complete SDK request: **36 requests and nine fresh exports**. Independent pinned Gnark verification checks 164-byte proofs, 44-byte public inputs, image/public/VK bindings and freshness. Canonical proof ledger advances **31,913→31,922**.

All 18 SDK/verifier commands succeed once. Driver clock/RSS values are correctness-preflight diagnostics, without a matched performance comparison. No speed, RSS saving, minimum RAM, pressure/swap, concurrent-proof, ARM/phone, 250/4999 or new AWS/Aeglos claim follows.

The independent reader checks 312 analysis-input pins, 25 workflow receipts, three read-only listing/decode receipts, all 176 source recoveries, the integer half identity, native/portable results, coherent binary/libraries and all nine fresh proof bindings. It independently classifies instruction versus continuation rows. It is not a second cryptographic verifier or decoder. Its review passes on first execution.

## Matched complete-build comparison

Exactly 15 paired blocks cover all eight reference shapes plus merge54. Both frozen builds use 32 workers on CPUs 0–31. Case order rotates and reverses; arm order alternates. Each process has three primes, then 151 observed complete requests for 1×2 or 31 for other shapes. Every observation is retained. All 270 SDK processes finish before 270 independent Gnark verifications; diagnostics follow. Latency includes witness solving, fresh blinding, proving, internal verification and serialization, excluding loading.

The unit is the arithmetic mean of requests in a process. Times below are medians of those 15 process means. Paired percentages are candidate/accepted block ratios, whose sorted ranks 4/8/12 form the interval/median. Nominal marginal median coverage is 96.484375% under independent, exchangeable blocks; this is not a familywise guarantee. Ratios of the displayed arm medians need not equal paired medians.

This was a separately declared engineering cleanup decision: every shape's upper ratio bound had to be ≤1.03, with correctness and no new allocation structure. No positive-gain floor was required. This does not change earlier failed gates or equate nonregression with a speedup. Five shapes fail. The exact build is closed, without extensions, rerolls, layout/worker sweeps or adoption of a favorable subset.

|Shape|Accepted ms|Candidate ms|Paired change % [interval]|Wins /15|Upper ≤1.03|
|---|---:|---:|---:|---:|---|
|1x2|8.041470|8.006417|-0.464 [-1.543, +0.208]|9|pass|
|2x2|11.237839|11.230082|-0.099 [-1.042, +0.765]|10|pass|
|4x2|16.143230|16.787973|+4.064 [+2.767, +5.272]|0|fail|
|8x2|24.397442|25.397525|+3.704 [+1.873, +5.408]|0|fail|
|12x2|33.736405|35.266034|+4.521 [+3.511, +4.969]|0|fail|
|16x2|40.082192|41.516657|+3.461 [+2.803, +4.307]|0|fail|
|24x2|65.765454|67.698775|+2.426 [+2.152, +3.957]|0|fail|
|merge-24x1|65.350546|66.907291|+2.523 [+1.379, +2.667]|0|pass|
|merge-54x1|161.495859|163.900818|+1.708 [+0.874, +2.176]|0|pass|

The original-eight equal-log aggregate has paired ratio 1.026492 [1.020471, 1.028450], 0/15 wins: descriptive evidence of a roughly 2.65% slower build, not an additional adoption gate. Primary's paired interval crosses 1 despite its lower displayed median. The old and new coherent SDK binaries are `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661` and `b23c9c2cf367111126c1047ea57cc4634cc25aef5912db5c6b2c163b4ab38a33`. No freshly rebuilt OFF executable was measured. These are whole-build differences, including compiler context/layout; the measurements do not isolate the constant change as their cause.

## Observed process memory

|Shape|Accepted maximum MiB|Candidate maximum MiB|
|---|---:|---:|
|1x2|133.50|131.50|
|2x2|274.00|276.00|
|4x2|317.50|308.50|
|8x2|512.25|502.75|
|12x2|729.50|733.50|
|16x2|826.75|845.75|
|24x2|1291.21|1350.50|
|merge-24x1|1186.29|1163.84|
|merge-54x1|1877.49|1875.86|

These are maxima of the 15 per-process `ru_maxrss` readings, including loading and primes. Allocator/residency variation is visible; these values are not minimum required RAM, an allocation/lifetime proof, or a pressure/swap/mobile result. The source adds only a universal logical Fq constant and no new heap structure. No synthetic RAM cap is used.

## One-shot diagnostics and review

After latency and its verifiers, each arm runs one warm FIFO-gated grouped PMU panel (31 observed requests) and one `cycles:u` profile (251), with three primes each, a 16-million-cycle period and 512 mmap pages. All four exports freshly verify. All counters run at 100%; both profiles pass the declared ≥1,000 samples, ≤5% unknown and zero-LOST checks, including owned PID/TID/CPU/MMAP/event metadata.

|Panel|Accepted|Candidate|
|---|---:|---:|
|Cycle samples|12418|12307|
|Unknown sample %|1.707199226928652|1.8038514666449987|

|Counter|Candidate / accepted|
|---|---:|
|cycles:u|0.992813|
|instructions:u|0.998472|
|cache-misses:u|1.039203|
|branch-misses:u|1.000377|

These single 1×2 panels have no repeated-panel uncertainty and cannot explain the larger-shape regressions or attribute a tiny constant's causal effect. Removing one inverse from G2 preparation does not remove inversions from the prover's main field/MSM paths. No new DRAM, fault-service-time, exclusive wall-time or precise IP-to-ELF inference is made. The existing qualified loss decoder is reused once for each new profile.

The independent offline reader passes 558 workflow receipts, 3,486 input-pin checks (including duplicate paths), all 274 fresh export bindings, raw means/ranks/all-nine gates, rotated order, diagnostic quality and all 176 unchanged production-source hashes. It reuses qualification and saved constructor decodes; no successful capture, test, decoder or verifier is replayed. It is not a second cryptographic verifier.

The matched stage makes **13,356 complete SDK requests / 274 fresh exports**; including qualification, **13,392 / 283**. The canonical ledger advances **31,922→32,192→32,196**. SDK requests include primes; only final exported proofs count toward the ledger.

## Accepted CPU / M5 references and next work

Accepted production176/SDK6d83 remain unchanged. The fresh local accepted-build observations below are from this experiment, not new adopted code or new M5 measurements. The supplied M5 CPU/Metal reference values remain historical; different machines do not establish equal hardware efficiency.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU reference / local|CPU 10× limit ms|
|---|---:|---:|---:|---:|---:|
|1x2|8.041470|37|24|4.601×|3.7|
|2x2|11.237839|59|30|5.250×|5.9|
|4x2|16.143230|109|48|6.752×|10.9|
|8x2|24.397442|189|74|7.747×|18.9|
|12x2|33.736405|284|101|8.418×|28.4|
|16x2|40.082192|335|124|8.358×|33.5|
|24x2|65.765454|528|176|8.029×|52.8|
|merge-24x1|65.350546|507|175|7.758×|50.7|

The all-eight CPU 10× objective remains **active and unmet, 0/8**. Merge54 has no supplied M5 reference. No fresh AWS/Aeglos, 250/4999, phone, pressure or minimum-RAM comparison is claimed. The next hypothesis must remove materially more measured MSM/field work or scheduling cost; this rejected constant/build experiment is not a route to a twofold improvement. Reuse the new profiles and whole-field census to select that distinct question, without repeating closed arithmetic/scheduler variants.

[Artifacts](artifacts/) retain the prior qualification plus `matched.tgz`, exact plans/receipts, raw samples, diagnostic decodes/metadata, analyzers, independent review, proof-hash ledgers and frozen design. Keys/images/requests/witnesses, proof/public payloads, raw perf data, targets and executable/library blobs are excluded. `matched-members.txt` and `matched-source-sha256.txt` bind every new archive member; a single-pass tar comparison succeeds before publication.
