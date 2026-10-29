# Current-source PGO: broader latency qualification

PGO improves the newly measured seven-reference-shape aggregate by 0.771% versus accepted and 1.162% versus fresh plain, winning all 15 aggregate blocks against both. The inherited 1×2 pilot improves by 0.828% versus accepted (7.924730 versus 7.988771 ms). All candidate nonregression checks pass. **Full qualification fails only the plain/accepted physical16 calibration check; the candidate is not adopted and production is unchanged.** Positive results remain recorded. This is a small compiler-guided gain, not a 2× improvement.

## What failed

Calibration compares two builds without PGO: the accepted SDK and the freshly rebuilt plain SDK. For 1×2 on sixteen physical cores their paired ratio interval is [0.970947, 1.049961], outside the fixed [1/1.03, 1.03] range at its upper end. The median paired ratio is 0.995654, with individual ratios from 0.905438 to 1.310104. All observations are retained. The failure does not establish that PGO is slower: its physical16 medians are 9.601386 ms versus 9.719106 accepted and 9.825013 plain. The PGO/accepted interval [0.951877, 1.018386] passes the 2% nonregression bound but does not establish a strict physical16 gain. No cause is assigned to the variability. The prior production engineering exception is not transferred, and no capture is repeated or excluded.

## Fixed experiment and provenance

This continues the [published primary pilot](../2026-10-29-proof-pgo175/README.md), commit `e91a83283926697b7de787a12de4b14db5250a02`. All 175 prover source files, original SDK paths, libraries, compiler flags, proof-only profile and request fixtures remain unchanged. Accepted SDK SHA starts `20fde19f`, plain `2e3634f0`, PGO `144621f1`. No new native build or training occurs. The parent nine-shape native preflights, primary32 latency blocks and logical32 profiles are inherited; they are not independent replication. Parent field/core/prover test results (33/124/8 for each compiler build) are inherited. Two new analysis-helper tests pass, checking paired ranks, ties and inherited block identities.

Ryzen 9950X, sixteen physical / 32 logical CPUs, 128 GiB RAM. Thirty-two-worker cases use CPUs 0–31; the physical16 case uses CPUs 0–15, verified to cover sixteen distinct cores. The host is shared, not exclusive. All helpers and inputs were prepared before capture, with no owned build, hash, archive or other CPU helper overlapping capture.

Nine new contexts comprise the eight other logical32 shapes, including 54→1, and primary physical16. Three physical16 preflights precede fifteen rotated/reversed blocks of all nine contexts and three variants. Primary16 processes measure 151 requests after three primes; other shapes measure 31 after three primes. All 405 latency processes finish before their 405 independent Gnark checks. Three FIFO-controlled four-event stats and three separate 199 Hz user-cycle profiles follow, with priming excluded; all six finish before their independent verification and decoding. Loading is excluded; witness solving, fresh randomized proving, internal verification and serialization are included.

All 837 workflow steps finish successfully. The 20,046 SDK requests include primes and diagnostics; 414 newly exported 164-byte proofs pass independent Gnark, image/public binding and hash-freshness checks. Proof ledger 18,847→19,261; 828 proof/public members are retained. No successful test, measurement or proof capture is repeated.

## Latency, CPU service and peak RSS

Medians are displayed for context and need not reproduce within-block paired ratios. RSS columns are maximum observed process RSS in MiB, not minimum RAM requirements. `Inherited` applies only to the three primary32 rows.

|Context|Accepted ms|Plain ms|PGO ms|Accepted CPU ms|Plain CPU ms|PGO CPU ms|Accepted max RSS|Plain max RSS|PGO max RSS|Phase|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
|1x2-w32|7.988771|8.032753|7.924730|180.976|181.528|178.232|133.50|133.25|132.25|Inherited|
|2x2-w32|11.310836|11.358166|11.235135|261.604|262.234|258.559|277.75|269.00|277.50|New|
|4x2-w32|16.075319|16.003197|15.776922|420.185|421.643|413.541|314.00|310.75|323.75|New|
|8x2-w32|24.070460|24.416193|24.287429|664.087|673.887|665.302|510.75|513.25|522.25|New|
|12x2-w32|33.732464|34.130900|33.514530|932.797|942.120|925.573|710.75|716.25|722.50|New|
|16x2-w32|39.953736|40.120406|39.484628|1120.290|1125.676|1106.164|816.00|822.25|834.50|New|
|24x2-w32|66.130235|66.679674|65.213433|1892.525|1899.073|1864.296|1292.71|1335.25|1290.64|New|
|merge-24x1-w32|65.320311|65.558140|64.781429|1799.796|1816.199|1796.407|1188.18|1169.50|1182.50|New|
|merge-54x1-w32|161.523532|163.023383|161.603688|4423.254|4445.033|4447.061|1847.91|1833.75|1884.32|New|
|1x2-w16|9.719106|9.825013|9.601386|119.246|119.083|118.278|89.25|91.50|88.25|New|

## Paired estimates and declared checks

Fifteen-block ratios use median rank 8 and bounds at ranks 4 and 12. Nominal individual coverage is 96.484375% conditional on independent, exchangeable blocks; it is not a simultaneous or familywise guarantee. Shared-host and temporal variation remain limitations. No positive-gain floor is used. Plain/accepted bounds must lie within [1/1.03,1.03] in all ten contexts. PGO upper bounds must be ≤1.03 against both controls, or ≤1.02 for physical16. Inherited primary32 must retain strict gain and at least 10/15 wins; new gain evidence requires the equal-log aggregate of all seven remaining reference shapes to have upper bound <1 and at least 10/15 wins against both controls. Any failed condition fails qualification; no threshold waiver or subset promotion is applied.

|Comparison|Paired ratio|Bounds|Wins / 15|Paired saving µs|Check|
|---|---:|---|---:|---:|---|
|12x2-w32/OFF/accepted|1.010178563|[1.003387640, 1.015777197]|3|-345.222|true|
|12x2-w32/ON/OFF|0.987715725|[0.975908471, 0.990461058]|13|416.556|true|
|12x2-w32/ON/accepted|0.994600938|[0.988008181, 0.999236670]|13|182.725|true|
|16x2-w32/OFF/accepted|1.002045899|[0.996351971, 1.010091190]|6|-81.814|true|
|16x2-w32/ON/OFF|0.986936608|[0.973826968, 0.990820963]|14|517.646|true|
|16x2-w32/ON/accepted|0.988828705|[0.983070968, 0.995997241]|14|446.612|true|
|1x2-w16/OFF/accepted|0.995653746|[0.970946656, 1.049961369]|9|41.959|false|
|1x2-w16/ON/OFF|0.984329133|[0.928134253, 0.992325812]|12|150.804|true|
|1x2-w16/ON/accepted|0.981004572|[0.951877036, 1.018386326]|10|183.096|true|
|1x2-w32/OFF/accepted|1.003080634|[0.998415298, 1.011906179]|4|-24.346|true|
|1x2-w32/ON/OFF|0.986795194|[0.982345646, 0.993194738]|15|104.678|true|
|1x2-w32/ON/accepted|0.991719002|[0.987876367, 0.996250474]|13|66.155|true|
|24x2-w32/OFF/accepted|1.002475052|[0.992785600, 1.017779356]|7|-164.951|true|
|24x2-w32/ON/OFF|0.986048598|[0.973103618, 0.996551011]|12|929.882|true|
|24x2-w32/ON/accepted|0.989055550|[0.978031595, 0.995146676]|13|732.262|true|
|2x2-w32/OFF/accepted|1.005891462|[0.995473124, 1.010891503]|7|-66.204|true|
|2x2-w32/ON/OFF|0.987147133|[0.981572595, 0.995140784]|13|145.985|true|
|2x2-w32/ON/accepted|0.991787527|[0.984233046, 1.002541438]|11|93.075|true|
|4x2-w32/OFF/accepted|1.000786417|[0.992398522, 1.007815005]|7|-12.864|true|
|4x2-w32/ON/OFF|0.980404495|[0.971436363, 0.993769542]|13|313.501|true|
|4x2-w32/ON/accepted|0.991721360|[0.975232650, 0.993751752]|12|130.826|true|
|8x2-w32/OFF/accepted|1.010608862|[1.002879974, 1.017207661]|2|-253.949|true|
|8x2-w32/ON/OFF|0.995955605|[0.991019999, 1.000854689]|11|99.688|true|
|8x2-w32/ON/accepted|1.005050547|[0.998823931, 1.012585000]|6|-121.569|true|
|merge-24x1-w32/OFF/accepted|1.003361671|[0.996251096, 1.006044140]|6|-220.877|true|
|merge-24x1-w32/ON/OFF|0.988983627|[0.982396651, 0.995047492]|12|723.924|true|
|merge-24x1-w32/ON/accepted|0.992315907|[0.985814703, 1.001061698]|11|502.367|true|
|merge-54x1-w32/OFF/accepted|1.006342023|[0.998512478, 1.016494498]|5|-1024.386|true|
|merge-54x1-w32/ON/OFF|0.997991451|[0.986219832, 1.000386793]|11|327.989|true|
|merge-54x1-w32/ON/accepted|1.003012264|[0.995904203, 1.012632868]|7|-486.598|true|

|Aggregate|Ratio|Bounds|Wins / 15|Role|
|---|---:|---|---:|---|
|new7/ON/OFF|0.988382171|[0.982536382, 0.989647699]|15|New gain gate|
|new7/ON/accepted|0.992286051|[0.989155394, 0.994748198]|15|New gain gate|
|original8_mixed_phases_descriptive/ON/OFF|0.987559922|[0.982854801, 0.989814106]|15|Mixed-phase descriptive only|
|original8_mixed_phases_descriptive/ON/accepted|0.991560495|[0.989993027, 0.994935859]|15|Mixed-phase descriptive only|

The original-eight aggregate combines inherited primary32 and seven new contexts and is descriptive only. It is not contemporaneous eight-shape replication. Individual cases vary: the displayed PGO median for 8×2 is 24.287429 versus 24.070460 ms accepted, and 54→1 is 161.603688 versus 161.523532 ms. Positive aggregate evidence does not mean every shape improves.

## Supplied M5 references

These are supplied M5 CPU/Metal timings, not new same-machine M5 measurements. Primary32 is inherited. Every original reference shape remains above its CPU-10× target; 0/8 meet the objective. No M5 reference was supplied for 54→1.

|Shape|M5 CPU ms|M5 Metal ms|PGO ms|CPU speedup|Metal speedup|CPU-10× target ms|Phase|
|---|---:|---:|---:|---:|---:|---:|---|
|1x2-w32|37|24|7.924730|4.669×|3.028×|3.7|Inherited|
|2x2-w32|59|30|11.235135|5.251×|2.670×|5.9|New|
|4x2-w32|109|48|15.776922|6.909×|3.042×|10.9|New|
|8x2-w32|189|74|24.287429|7.782×|3.047×|18.9|New|
|12x2-w32|284|101|33.514530|8.474×|3.014×|28.4|New|
|16x2-w32|335|124|39.484628|8.484×|3.140×|33.5|New|
|24x2-w32|528|176|65.213433|8.096×|2.699×|52.8|New|
|merge-24x1-w32|507|175|64.781429|7.826×|2.701×|50.7|New|

Accepted production remains the prior 8.022165 ms / 134 MiB primary result and its retained full-performance-gate failure; this report does not silently reclassify it as fully passing. Its exact timings and reference comparisons are in [accepted comparison](artifacts/accepted-comparison.json).

## Profiles and limits

All four physical16 events run at 100%; all three profiles contain one target PID and zero decoded lost events. Sample counts accepted/plain/PGO are 5,120/5,226/5,328. Instructions/request are 1,338.263 million PGO versus 1,381.862 million plain; cycles are 582.008 versus 585.530 million. These are separate diagnostic captures, not paired latency results or proof of an exclusive cause. Logical32 profiles are inherited from the pilot. Raw physical16 perf data, decoded receipts and demangled symbols are archived. Inclusive symbol residence is not an exclusive wall-time fraction.

There is no new AWS, Aeglos, portable, mobile, TEE, 250/4999, memory-pressure, concurrency or minimum-RAM qualification. Compatibility/resource/final-artifact work has not been performed for adoption.

## Paper 2026/2667

[Faster complete formulas and their application to curves with efficient endomorphisms](https://eprint.iacr.org/2026/2667), Aranha and Barreto, offers a five-product complete-addition block, complete tripling, a six-point τ-NAF scalar method and imaginary Toom–Cook extension arithmetic. Our hot bulk MSM uses batched affine additions, so the complete projective formulas are not a direct replacement. Individual scalar callers and Fq6 multiplication are plausible separate trials, without per-key point-table expansion. No implementation, timing or full-proof gain from the paper is claimed here. This review did not change the frozen PGO experiment.

## Artifacts

[Analysis](artifacts/analysis.json), [fixed protocol](artifacts/design.md), [candidate identity and failed qualification](artifacts/candidate-manifest.json), [receipts](artifacts/workflow-receipts.json.gz), [proof/public bytes](artifacts/proof-public-members.json.gz), [source snapshot](artifacts/source-snapshot.json.gz), [raw physical16 profiles](artifacts/physical16-perf-data.json.gz), [demangled symbols](artifacts/demangled-profile-symbols.json.gz), [parent provenance](artifacts/parent-evidence.json), [closeout](artifacts/closeout.json). Archives encode every member with SHA-256 and UTF-8 or hex and are verified by decompression round-trip. Artifact index and SHA256SUMS cover the report.
