# On-demand FFT factor preparation: native columns

The predeclared eight-context threaded column aggregate is **9.31% faster than fresh original arithmetic**, paired ratio **0.906858**, bootstrap 95% CI **[0.903788, 0.909787]**, 15/15 winning blocks. Against frozen original arithmetic the ratio is 0.905329 [0.901776, 0.908372]. These are complete-column caller measurements, not full-proof latency. **The original selection gate fails four checks, and profile symbol quality fails. Production is unchanged.**

The new constructor prepares the gap-one root once per 32-point column, uses the existing fixed-factor product for its 16 butterflies, and discards it. The other 64 products are unchanged. Both FFT directions and both root layouts are included. Preparation and canonical root decoding are inside the clock. No per-key table, retained cache, plan copy, heap allocation or production pool is added. The factor payload is 640 bytes plus arithmetic temporaries, with a 48-byte source reciprocal constant per prime; neither establishes a stack or minimum-RAM bound.

Native Fq/Fr qualification passes 7,168 prepared lanes and 14,336 product lanes, including lazy roots, R260/R256 variable operands and rare quotient correction branches. Complete-column independent scalar oracles pass 5,419,008 output comparisons **per arm**, both directions/layouts, changing inputs, offset levels and 1/16/32 workers with 512 KiB configured worker stacks. Existing field bodies are unchanged; the 84-IFMA fixed product matches its previously qualified body after type/method renames. The [integer model](../2026-11-07-fft-column-prep176/README.md) is inherited without replay.

The fixed caller uses q=256/8192, both directions/layouts and 1/16/32 workers, 15 rotated/reversed paired blocks, three arms, and 16,384 column waves per fixture after three 128-wave checked primes. CPU masks are 2, 0–15 and 0–31. Frozen original links the accepted field library; fresh original and prepared share one executable/new field library. Each wave includes input restoration, packed loads/stores, all butterflies, preparation and Rayon dispatch/join. A final scalar oracle check is outside the clock; intermediate results are blackboxed. All 1,176 caller outputs pass, timed allocations are zero, and corresponding output hashes match across arms. A total of 440,009,880 native columns plus 7,520,256 priming columns run. No proofs are generated.

Measurements run locally on an AMD Ryzen 9 9950X, 16 physical cores and 32 logical CPUs, using Rust 1.98.1 with optimization, native CPU targeting, fat LTO and one codegen unit for the caller. This is not an AWS measurement.

The standalone context aliases native field types directly and uses a fixture Twiddles structure. The numerical operations and extracted original kernel/leaf match production, but its struct and optimizer context differ from the SDK. This limitation motivates a separate full-proof test.

Times below are median process means in microseconds per column wave. Ratios and intervals are paired across blocks; a wave contains one column per worker. Rows are retained without exclusion.

|q|Direction|Layout|Workers|Frozen µs|Fresh µs|Prepared µs|Prepared/fresh (95% CI)|Wins /15|
|---:|---|---|---:|---:|---:|---:|---|---:|
|256|inverse|layers|1|1.711943|1.712288|1.750521|1.021649 [1.018060, 1.025261]|0|
|256|inverse|layers|16|1.920177|1.916182|1.967680|1.025971 [1.023685, 1.028365]|0|
|256|inverse|layers|32|3.294397|3.293664|3.332078|1.012608 [1.005804, 1.020201]|1|
|256|inverse|cached|1|1.704974|1.704037|1.722308|1.013735 [1.010920, 1.016628]|0|
|256|inverse|cached|16|1.921319|1.921452|1.945915|1.012234 [1.007548, 1.016664]|1|
|256|inverse|cached|32|3.240881|3.250212|3.278101|1.011761 [1.005849, 1.020632]|1|
|256|forward|layers|1|1.967966|1.969549|1.980554|1.006556 [1.003135, 1.009967]|2|
|256|forward|layers|16|3.001171|3.019945|2.188786|0.715035 [0.704852, 0.724880]|15|
|256|forward|layers|32|3.827439|3.814794|3.462086|0.904714 [0.898546, 0.910365]|15|
|256|forward|cached|1|1.943436|1.948807|1.960702|1.009617 [1.007197, 1.012218]|0|
|256|forward|cached|16|3.144925|3.017546|2.192565|0.714620 [0.703317, 0.725701]|15|
|256|forward|cached|32|3.794678|3.850562|3.403881|0.885079 [0.877553, 0.892800]|15|
|8192|inverse|layers|1|2.534387|2.516778|2.491191|1.000261 [0.981976, 1.021837]|9|
|8192|inverse|layers|16|2.490641|2.499790|2.525436|1.001774 [0.985712, 1.016775]|4|
|8192|inverse|layers|32|4.060516|4.058607|4.102939|1.007356 [0.991092, 1.025223]|9|
|8192|inverse|cached|1|1.871655|1.865108|1.910391|1.026347 [1.022190, 1.030749]|0|
|8192|inverse|cached|16|1.955528|1.956527|1.986755|1.012354 [1.006928, 1.017573]|2|
|8192|inverse|cached|32|3.281909|3.273511|3.308266|1.008857 [1.000396, 1.015106]|2|
|8192|forward|layers|1|2.683795|2.678931|2.631697|0.981495 [0.966692, 0.995757]|13|
|8192|forward|layers|16|3.392203|3.398879|2.744023|0.798663 [0.777314, 0.825679]|15|
|8192|forward|layers|32|4.533353|4.492044|4.339578|0.957874 [0.944541, 0.973705]|13|
|8192|forward|cached|1|2.640892|2.651882|2.535986|0.957501 [0.954403, 0.960808]|15|
|8192|forward|cached|16|3.042777|3.045525|2.260992|0.740384 [0.732407, 0.747056]|15|
|8192|forward|cached|32|3.851394|3.848459|3.467840|0.900076 [0.894703, 0.904179]|15|

The intended aggregate uses q256/layers and q8192/cached, both directions and 16/32 workers, with equal context log weights inside each paired block. Forward contexts improve strongly, while intended inverse contexts regress about 1–3%. The gate retains two fresh/frozen calibration failures (q256/forward/cached/16 and q8192/inverse/layers/1), plus prepared/frozen and prepared/fresh nonregression failures in q8192/inverse/cached/1. All-context calibration requires the entire CI inside [1/1.03, 1.03]; nonregression requires its upper bound <=1.03. Aggregate upper bounds must be <1. Statistics use the predeclared 20,000 resamples; no gain floor, exclusion, extension or reroll is applied. Full ratios and intervals are in the [analysis](artifacts/latency-analysis.json).

Automatic diagnostics run after all latency measurements: 12 grouped PMU panels, then 12 user-cycle profiles, followed by saved-stream decoding. Counters all have >=99.9% running time. Profiles contain 87,691 samples with zero LOST records and valid PID/TID/CPU/MMAP/event/FIFO checks, but all exceed the declared 5% unknown-symbol limit (about 5.01–6.17%). Those quality flags remain false. Of 4,950 unknown samples, the saved mapping audit associates 4,915 with libc and retains 35 unresolved addresses; it does not assign function names, establish a memcpy cause, quantify exclusive wall time or rescue the gate. An initial audit failed on an unmapped IP; the replacement retains explicit unmapped bins without recapture or decoder replay.

PMUs show more instructions per wave for prepared arithmetic in both directions. Forward cycles are lower and inverse cycles higher in the single diagnostic panels. These panels include fixture restoration and do not explain the latency result causally. Emitted original inverse/forward leaf sizes are 15,567/15,573 bytes; prepared sizes are 23,696/23,778 bytes. Static IFMA sites grow from 200 to 614 across whole functions including both layout paths; these are not dynamic operation counts. The largest literal stack subtraction, 4,096 bytes, is one probe instruction and is not a whole-frame estimate. Codegen evidence retains an initial output-path collision before inspection; the unchanged executable was moved and inspection then succeeded once.

The q8192 shared root fixture carries about 80 MB of benchmark root data, independent of worker count; it models existing root access and is not a new production table. Process RSS includes setup and fixtures and is not minimum prover RAM or a pressure/mobile result. No new full-SDK, AWS, Aeglos, phone, huge-circuit or swap result is claimed.

**Decision:** preserve the failed original gate and profile-quality result; do not adopt this implementation. As an explicit engineering exception, select a separate full-SDK experiment because the predeclared threaded aggregate shows a substantial, consistent gain. It must use the same gap-one preparation in both directions and layouts, without forward-only selection or parameter/compiler/layout rerolls. Native and portable guards, all-nine fresh proof correctness, independent Gnark checks, matched proof latency, automatic profiles and resource qualification remain required.

The latest accepted full-proof measurement remains 8.032278 ms for 1×2, with earlier 7.92–7.99 ms results for the same binary. The [all-shape CPU/Metal comparison](../2026-11-07-sign-mask-allshape176/README.md) and 33,449-proof ledger are unchanged. The original verified CPU 10× goal remains active and unmet: **0/8 shapes**.

[Qualification](artifacts/native-qualification.json), [review](artifacts/review.json), [decision](artifacts/decision.json), [profiles and PMUs](artifacts/diagnostic-analysis.json), [codegen](artifacts/codegen.json), [mapping audit](artifacts/unknown-dso-audit.json), [sources, plans, pins and receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
