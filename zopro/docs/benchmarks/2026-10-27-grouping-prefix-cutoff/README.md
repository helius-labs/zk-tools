# Exact early rejection of scalar grouping

The candidate retains a **3.998 µs primary paired saving versus accepted (0.048%)**, with an interval including parity. It improves against the fresh control, but five fresh/accepted calibration bounds fail and the full performance gate is false. It is **not adopted**. Positive estimates remain reported; no minimum gain floor is applied.

Primary32 process-median latencies are **8.325779 ms accepted, 8.424946 ms fresh and 8.330548 ms candidate**. These displayed medians differ from within-block paired estimates. The accepted 171-file implementation and exact 824164 SDK remain unchanged. The all-eight supplied M5 CPU 10× objective remains active, achieved 0/8.

## Change

The existing scalar-grouping loop now computes floor(n*percent/100) without overflow and stops on the next vacant hash entry after that many distinct nonzero values have been seen. Distinct counts cannot decrease, so full traversal would necessarily reject the same input. Occupied values at the boundary remain valid. Initial sampling, exact equality/hash, cutoff, insertion order, zero handling, capacity/reservation and successful stable output packing remain unchanged. This adds a comparison per new distinct value on accepted inputs and may affect generated code. It is separate from earlier unsuccessful bitmap lower-bound and borrowed-key candidates.

Only grouping.rs changes in non-test code; one independent oracle is appended to grouping/tests.rs. The other 169 accepted files remain exact. Closing inverse edits recover all 171 accepted bytes. There is no new buffer, point/key table, persistent state, field primitive, runtime selector, ISA flag, unsafe code or scheduler change. Existing memory forecasts are retained conservatively; no universal RSS reduction or minimum RAM is established.

## Complete SDK results

Ryzen 9950X, fifteen fixed rotated/reversed fresh-process blocks, three variants, all nine shapes at 32 workers plus primary16 on CPUs 0–15. Both primary cases run 151 requests after three primes; other 32-worker cases run 31 after three primes. All 450 latency processes precede six separate warm profiles. Timed work includes fresh solving, randomness, proving, SDK verification and serialization. Loading/setup/primes are excluded from latency and included in lifetime RSS. There is no short screen, extension, excluded observation, successful repeat or source/flag adjustment after capture.

Times are medians of process medians; ratios, wins and savings use within-block pairs. RSS is the maximum whole-process peak across all blocks.

|Case|Accepted ms|Fresh ms|Candidate ms|Paired C/A|Individual interval C/A|Wins|Paired saving µs|RSS A/C MiB|
|---|---:|---:|---:|---:|---|---:|---:|---:|
|1x2-w32|8.325779|8.424946|8.330548|0.999520|[0.996252, 1.001717]|8/15|3.998|134.50/135.25|
|2x2-w32|11.404087|11.520316|11.495481|1.002554|[1.000546, 1.009451]|3/15|-28.935|270.75/274.25|
|4x2-w32|15.950434|16.181622|15.891183|0.998251|[0.990197, 1.004223]|8/15|27.862|313.75/314.00|
|8x2-w32|24.138594|24.783831|24.023161|0.997362|[0.979006, 1.006462]|8/15|62.879|505.50/509.50|
|12x2-w32|33.784085|34.454529|33.888183|1.001029|[0.997591, 1.009660]|7/15|-34.736|714.75/708.25|
|16x2-w32|40.068185|40.842221|40.035247|0.994969|[0.992299, 1.012851]|8/15|201.129|832.50/848.75|
|24x2-w32|67.277087|67.381993|67.000664|0.999353|[0.987823, 1.005421]|8/15|43.522|1302.91/1352.50|
|merge-24x1-w32|66.225500|66.288750|66.230441|0.996635|[0.991405, 1.004702]|9/15|222.771|1200.19/1154.39|
|merge-54x1-w32|163.636011|164.219983|163.267591|0.999352|[0.989421, 1.005686]|9/15|106.468|1851.55/1872.32|
|1x2-w16|10.289343|10.301116|10.233569|1.007121|[0.988358, 1.065190]|6/15|-71.544|90.00/88.25|

|Case|Paired C/fresh|Individual interval C/fresh|Wins|CPU ms/request accepted/fresh/candidate|
|---|---:|---|---:|---:|
|1x2-w32|0.988795|[0.984105, 0.991206]|15/15|191.158/192.875/192.047|
|2x2-w32|0.993771|[0.981105, 1.000225]|11/15|263.430/265.801/264.315|
|4x2-w32|0.984901|[0.974493, 0.993136]|13/15|419.390/425.201/417.224|
|8x2-w32|0.972713|[0.942845, 0.996199]|13/15|664.956/680.460/662.707|
|12x2-w32|0.984059|[0.969177, 0.994982]|12/15|937.381/952.838/938.818|
|16x2-w32|0.983698|[0.965656, 0.993139]|13/15|1121.679/1146.009/1122.173|
|24x2-w32|0.992287|[0.976424, 1.000791]|11/15|1921.050/1931.871/1924.317|
|merge-24x1-w32|0.993444|[0.981019, 1.004064]|9/15|1836.040/1836.034/1835.831|
|merge-54x1-w32|0.992118|[0.988254, 1.004903]|11/15|4487.125/4512.255/4470.387|
|1x2-w16|0.983401|[0.911980, 1.005510]|10/15|124.482/126.664/126.093|

Primary C/A median 0.999520, interval [0.996252,1.001717], 8/15 wins; C/fresh 0.988795, interval [0.984105,0.991206], 15/15 wins. Original-eight paired aggregate C/A 0.998992 with interval [0.998428,1.001136]; C/fresh 0.990165 with interval [0.970950,0.993175]. The first comparison in each case includes parity. This preserves the small positive estimate without treating the fresh-control benefit as a demonstrated gain over accepted.

Other positive C/A estimates are 27.862 µs (4×2), 62.879 µs (8×2), 201.129 µs (16×2), 43.522 µs (24×2), 222.771 µs (24→1 merge) and 106.468 µs (54→1 merge). Their individual intervals include parity. The 2×2 C/A interval [1.000546,1.009451] is above parity, an observed slowdown in that comparison despite passing the 2% nonregression bound. Primary16 has a positive candidate displayed-median difference but a negative paired saving; keep the paired result. Primary CPU increases 191.158→192.047 ms/request versus accepted, while improving versus fresh 192.875; there is no demonstrated whole-proof CPU-work reduction versus accepted.

RSS is mixed, accepted/candidate: primary 134.50/135.25 MiB; 24×2 1302.91/1352.50 MiB; 54→1 1851.55/1872.32 MiB. These lifetime maxima do not establish minimum RAM, an architecture cap or universal memory regression/saving. The algorithm keeps original allocations/forecasts and can own fewer elements only on early rejection.

## Frozen checks and profiles

Individual exact median intervals use ranks 4..12 for 15 blocks, with 96.484375% nominal coverage under independent/exchangeable block assumptions, not familywise coverage. Requirements: all 10 fresh/accepted intervals within factor 1.03; candidate upper≤1.02 against both controls in all 10; primary32 upper<1 against both; original-eight aggregate upper≤1 against both. No failed bound is waived and no gain floor is introduced. Failure to bound regression is not alone proof of a causal slowdown.

Calibration failures: `["12x2-w32 fresh/accepted interval [1.0040562886341304,1.0352038612631933]","16x2-w32 fresh/accepted interval [1.0045385929524662,1.0300968829511885]","1x2-w16 fresh/accepted interval [0.9854261170681936,1.1463299800454394]","4x2-w32 fresh/accepted interval [1.0011696274487343,1.0359732036978873]","8x2-w32 fresh/accepted interval [0.9995285143782608,1.0526723426401923]"]`.

Nonregression failures: `["1x2-w16 candidate/accepted upper=1.0651897934858268"]`.

Primary gain failures: `["1x2-w32 candidate/accepted upper=1.0017174325829385"]`.

Aggregate failures: `["candidate/accepted original8 upper=1.0011360189772274"]`.

Six separate 199 Hz user-cycle profiles contain 84,224 samples, one sampled SDK PID per capture and zero lost records. These runs follow all latency. CPU and period-weighted categories are descriptive, not exclusive wall fractions or evidence of cache, spill, frequency or per-entry causes. No local helper/build/hash/archive/independent-verifier/second benchmark overlaps timed capture.

|Case|Variant|Samples|CPU ms/request|Grouping %|G1 %|Epoch %|
|---|---|---:|---:|---:|---:|---:|
|1x2-w32|accepted|9189|192.414|0.418|36.53|9.15|
|1x2-w32|fresh|9429|194.417|0.806|37.16|8.77|
|1x2-w32|candidate|9218|192.815|0.592|36.96|8.66|
|4x2-w32|fresh|18898|418.742|1.568|33.82|10.75|
|4x2-w32|candidate|18963|417.286|1.485|34.77|10.90|
|4x2-w32|accepted|18527|416.424|1.525|34.97|10.54|

## Supplied M5 comparison

M5 CPU/Metal values are supplied historical references, not new Apple measurements. Candidate remains experimental. No 54→1 M5 reference was supplied.

|Shape|Candidate ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required CPU10× ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.330548|37|4.441×|24|2.881×|3.7|
|2x2|11.495481|59|5.132×|30|2.610×|5.9|
|4x2|15.891183|109|6.859×|48|3.021×|10.9|
|8x2|24.023161|189|7.867×|74|3.080×|18.9|
|12x2|33.888183|284|8.381×|101|2.980×|28.4|
|16x2|40.035247|335|8.368×|124|3.097×|33.5|
|24x2|67.000664|528|7.881×|176|2.627×|52.8|
|merge-24x1|66.230441|507|7.655×|175|2.642×|50.7|
|54→1|163.267591|—|—|—|—|—|

## Correctness, provenance and decision

Fresh control passes 116 unfiltered native core tests; candidate 117. The added test checks 100 independent ordered-map cases across both sample policies, cutoff boundaries, zeros, full-word keys, early/late duplicates and several input sizes. Decision, first-occurrence values, counts and stable indices agree. Existing independent curve/MSM, concurrency and fallback tests remain included. Both variants pass two raw and six cubic prover tests; candidate portable/no-IFMA consumer checks pass. Field source and qualification remain inherited; no new portable-unit, pressure, giant or physical-phone qualification is claimed here.

The initial workflow plan was rejected before any build because helper JSON mutation inserted a null argv key into link steps. The first repair helper asserted a nonexistent key before writes. Original sources/plan/logs are retained; corrected repair removes only that null key and routes receipt paths to the new native-v2 output. No successful test, build or capture was repeated for the repair; candidate source and flags remain fixed.

Preflight contributes 36 SDK calls/nine exports; latency 26,100 calls/450 exports; profiles 1,524 calls/six exports. Total **27,660 SDK calls, 465 distinct independently Gnark-verified exports,930 proof/public members**, ledger **9,439→9,904**. Every SDK call verifies internally; all final exports meet 164-byte proof, exact public binding and freshness against all prior/new identities. Closing audits rehash 450 pins, 930 proof/public members and 171 production source files. Accepted 824164 SDK remains exact.

Do not adopt or reroll this unchanged candidate. Before attributing candidate/fresh gains, inspect exact existing build/link identities and actual caller code: several source-identical fresh controls fail calibration against accepted. This does not erase positive candidate/accepted estimates, resolve uncertainty or establish a cache/frequency cause. Use already captured artifacts rather than repeat acceptance timing. Prior opposite/modal scalar and FFT-overlap directions are already covered; see the linked audit. No new AWS/Aeglos/ARM/TEE/large/minRAM/pressure/concurrency/throughput claim is made. Earlier C7a figures belong to an older source. The original all-eight CPU 10× goal remains active, 0/8.

[Design](artifacts/design.md), [fixed protocol](artifacts/sdk-design.md), [source audit](artifacts/source-audit.json), [analysis](artifacts/analysis.json), [raw rows](artifacts/performance/rows.json), [proof inventory](artifacts/proof-inventory.json), [preparation repair](artifacts/preparation-failures.json), [next-direction audit](artifacts/next-direction-audit.md), [raw profile hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).
