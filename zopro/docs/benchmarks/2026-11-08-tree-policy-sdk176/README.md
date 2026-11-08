# Tree-operation policy snapshot: complete-proof comparison

The candidate does **not establish a proof-speed improvement** and is not adopted. Local 32-worker 1×2 measures **8.032363 ms ON, 8.027648 ms fresh OFF, and 8.040649 ms accepted**. All 546 exported proofs pass independent Gnark verification. The independent results reader passes; 36 of 45 prospective performance checks pass, with nine failures retained. Production 176 and accepted SDK 6d83 remain unchanged.

The [native-qualified change](../2026-11-08-tree-policy-owner176/README.md) reads the existing tree-vector-inverse configuration once per operation and copies the boolean into scratch initializers. Later operations read again. It removes repeated reads without changing arithmetic, scheduling or point records and adds no table, retained plan, key copy, production pool or release-path allocation. The earlier 512→256 reads across instrumented tree tests did not predict a twofold proof improvement. This is the first SDK measurement of this candidate.

## Fixed experiment and verification

Both fresh SDKs use the same qualified 177-source snapshot and unchanged accepted SDK consumer; only `--cfg zopro_tree_policy_owner` differs. All eight libraries in each arm are bound to their actual Cargo artifact stream. Four successful build/link receipts establish Rust 1.98.1, native target, release optimization level 3, fat LTO and one codegen unit. Original native/generic field arithmetic and the [native tree tests](../2026-11-08-tree-policy-owner176/README.md) are inherited rather than replayed. Those tests include repeated configuration changes, generator oracles, nested Rayon and actual small-stack workers.

Before any SDK call, inputs, artifact hashes, order, request counts, analysis and thresholds were frozen and independently checked. On the local Ryzen 9 9950X, masks 0–15/0–31 select 16/32 workers. The serialized workflow records 30 preflight processes, 498 latency processes and 18 separate warm diagnostic processes. Only the live workflow handle is polled during recordings. All 546 independent Gnark verifiers and 45 offline profile steps run after every recording. No owned build/helper/hash/analyzer/decoder/verifier/report/push overlaps recording. No successful capture, test, verifier or decoder is repeated, extended or excluded.

Primary32 uses 31 rotated/reversed matched blocks ×151 measured requests per process. Other contexts use 15 blocks, with 151 requests for primary16 and 51 for other shapes. Every process has three priming requests. Primary16 runs as a separate cohort before32. Total 42,630 SDK requests including priming; 546 fresh independently verified exports, not 42,630 independent Gnark verifications. The canonical inventory advances 36,477→37,023 without counting transport copies. All exported proofs are 164 bytes, public bindings 44 bytes, fresh against the prior ledger, and match the expected image, public-input and verification-key hashes.

## Latency and resources

These are medians of process means. Paired ratios below use all matched blocks and can differ from ratios of displayed medians. RSS is the maximum whole-process observation, including loaded key and caches; it is not minimum RAM.

|Context|Accepted ms|Fresh OFF ms|ON ms|Accepted / OFF / ON peak MiB|
|---|---:|---:|---:|---:|
|1x2-w32|8.040649|8.027648|8.032363|135.00 / 135.50 / 134.25|
|2x2-w32|11.149097|11.190670|11.182543|293.25 / 294.75 / 296.50|
|4x2-w32|16.069415|16.082353|16.172210|333.25 / 336.00 / 348.00|
|8x2-w32|24.131178|24.298741|24.109189|545.00 / 534.75 / 545.00|
|12x2-w32|33.940628|33.881514|33.888325|748.50 / 762.00 / 751.50|
|16x2-w32|40.164083|40.121942|40.160658|864.00 / 864.50 / 865.50|
|24x2-w32|66.187704|66.412140|66.185741|1356.25 / 1336.88 / 1395.93|
|merge-24x1-w32|65.361571|65.203297|65.570281|1209.00 / 1226.46 / 1228.77|
|merge-54x1-w32|160.890372|162.031049|162.505327|1938.18 / 1940.01 / 1925.43|
|1x2-w16|9.398702|9.395049|9.429751|90.75 / 89.00 / 90.75|

Primary32 ON/accepted paired geometric ratio is 0.998893319, a positive 0.111% point estimate, with 17/31 wins. Its prospective sorted-rank interval is [0.995159282, 1.004321705], which includes parity. ON/OFF is 1.002609679, or 0.261% slower by the point estimate, with 13/31 wins and rank interval [0.998076915, 1.009461200]. Neither comparison passes the declared gain/win conditions. Fresh OFF/accepted calibration passes at 32 workers. Retain the small positive accepted comparison; it does not establish a candidate improvement over fresh OFF.

The original-eight aggregate ON/accepted ratio is 1.000570479 with stratified bootstrap 95% interval [0.998365467, 1.002783827]; ON/OFF is 1.001136867 [0.998876199, 1.003474955]. Both intervals include parity. The 12×2 accepted comparison has a positive 0.503% estimate with an interval below parity, but its ON/OFF interval includes parity; this is not a separate promotion. Merge54 is 0.922% slower than accepted [1.005583013, 1.012982534], while its ON/OFF interval includes parity. The merge24 ON/OFF lower bound is just above parity at 1.000004676; this narrowly excludes parity under the inherited per-context bootstrap assumptions, without a familywise guarantee.

All 32-worker calibration, nonregression and all ten RSS screens pass the declared tolerances. The primary16 fresh-control calibration interval is [0.894201782, 1.015010512], outside the reciprocal 1.03–1.03 range. Its two nonregression checks also fail. Its wide intervals cannot support a calibrated optimization conclusion; no early block or extreme observation is removed. The candidate already lacks a 32-worker or aggregate verified gain, so no primary16 rescue or unchanged reroll is selected. Bootstrap assumptions require exchangeable blocks within shapes; cross-shape shared-host dependence is not established.

Failed prospective checks:

- `1x2-w32:candidate/accepted-uppergain`
- `1x2-w32:candidate/accepted-wins`
- `1x2-w32:candidate/OFF-uppergain`
- `1x2-w32:candidate/OFF-wins`
- `1x2-w16:OFF/accepted-calibration`
- `1x2-w16:candidate/accepted-nonregression`
- `1x2-w16:candidate/OFF-nonregression`
- `original8:candidate/accepted-aggregate-gain`
- `original8:candidate/OFF-aggregate-gain`

## Automatic profiles

All nine warm profiles and nine gated four-counter panels pass quality checks: 127,502 samples, zero reported loss, 100% assigned execution samples, at most 1.843% unknown, and 100% PMU running. The panels and profiles are separate from latency recording. They cover primary16, primary32 and merge54 for all three arms; only one panel/profile is collected per case and arm. Positive FFT-leaf samples are required by the inherited reader.

|Context|ON/OFF retired instructions|ON/OFF cycles|
|---|---:|---:|
|1x2-w16|0.999890|1.002762|
|1x2-w32|1.000339|0.998366|
|merge-54x1-w32|1.000752|0.985411|

The sampled hot code remains the main G1 stream, G2 borrowed processing, inversion and tree reduction; scheduler/epoch samples are also visible at 32 workers. The configuration change does not produce a large instruction or cycle reduction in these panels. These observations do not identify a unique cache, DRAM, spill, lock or critical-path cause. Sample shares are CPU-sample fractions, not removable wall-time fractions. No additional local profile or AWS run is justified for this unchanged candidate.

## Supplied M5 CPU and Metal references

The references are supplied historical timings on different hardware. Local timings below are measured in this study. There is no supplied merge54 reference. All eight verified CPU 10× requirements remain active and unmet, 0/8.

|Shape|Accepted local ms|Candidate local ms|M5 CPU ms / candidate speedup|M5 Metal ms / candidate speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|---:|
|1x2|8.040649|8.032363|37.0 / 4.606×|24.0 / 2.988×|3.7|
|2x2|11.149097|11.182543|59.0 / 5.276×|30.0 / 2.683×|5.9|
|4x2|16.069415|16.172210|109.0 / 6.740×|48.0 / 2.968×|10.9|
|8x2|24.131178|24.109189|189.0 / 7.839×|74.0 / 3.069×|18.9|
|12x2|33.940628|33.888325|284.0 / 8.380×|101.0 / 2.980×|28.4|
|16x2|40.164083|40.160658|335.0 / 8.341×|124.0 / 3.088×|33.5|
|24x2|66.187704|66.185741|528.0 / 7.978×|176.0 / 2.659×|52.8|
|merge-24x1|65.361571|65.570281|507.0 / 7.732×|175.0 / 2.669×|50.7|

Historical sub-8 ms observations remain valid: the same accepted executable measured 7.915095 ms, and private candidates measured 7.859756/7.869400 ms. Today's accepted 8.040649 ms does not establish an adopted regression or its cause. No slower replacement is adopted. Describe accepted local primary latency as approximately 8 ms while retaining the individual observations.

The candidate is closed without adoption. The next useful experiment needs a distinct change in main MSM arithmetic/work generation supported by these profiles and the prior experiment record; repeating this configuration placement, changing thresholds or selecting a favorable subset is not justified. No AWS/Aeglos, giant-circuit, ARM/mobile/TEE, swap/pressure, minimum-RAM or concurrent-proof qualification is added.

[Latency and all matched observations](artifacts/latency-analysis.json), [automatic diagnostics](artifacts/diagnostic-analysis.json), [independent reader](artifacts/review.json), [coherent build audit](artifacts/artifact-audit.json), [prospective protocol](artifacts/protocol.json), [prospective reader](artifacts/plan-review.json), [source and complete receipts](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
