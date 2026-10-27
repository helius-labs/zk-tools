# Lossless packed G1 batch records

Do not adopt this candidate. Primary latency regresses by a paired 1.005% against accepted and 1.035% against fresh; both intervals stay above parity. The original-eight aggregate regresses by 0.341% and 0.668%, also with intervals above parity. All ten control-calibration checks pass. The smaller records do not compensate for the measured regression.

Merge 54 retains positive paired estimates: 806.843 µs against accepted and 374.946 µs against fresh, with 11/15 and 10/15 wins. Both intervals cross parity, so a reliable gain is unproven. Its observed peak RSS falls from 1,870.75 to 1,839.41 MiB against accepted, while primary rises from 134.00 to 135.25 MiB. Memory observations remain mixed.

Full performance gate: **false**. Primary candidate latency is **8.404229 ms**, versus **8.310633 ms accepted** and **8.317877 ms fresh**. Its paired candidate/accepted median ratio is **1.010052888**, interval **[1.008125184, 1.014328776]**, 1/15 wins, with a paired saving of **-82.905 µs**. Positive and negative estimates remain recorded without a gain floor; process medians and paired estimates use different aggregation.

The candidate is not adopted by this study. Accepted 172-file bounded-FFT source and SDK14da8231 remain unchanged. The all-eight supplied M5 CPU 10× goal remains active, achieved 0/8.

## Implementation

G1 streaming records retain all five values using existing lossless numerical SIMD packing: 1,600→1,280 bytes per eight-point record. Reverse arithmetic expands each stored operand. No coordinate recomputation, key reread, new field arithmetic, runtime flag, table, pool or unsafe code is introduced. G2, tree scratch and unsupported four-lane/portable storage remain unchanged. Existing signs, tails, initialization, batching and windows remain. The resource forecast deliberately charges the old five-expanded-value record, so narrower storage cannot increase admission during the study. Two original files change and one private module is added; 170 other original files are exact. See the [storage review](artifacts/integration-review.md).

This differs from earlier compact/late-coordinate trials, which removed and reconstructed or reloaded fields. The hypothesis is a different cache and spill tradeoff. A 20% record reduction is not a claim of 20% total RAM or latency improvement.

## Complete SDK results

Fifteen fixed rotated/reversed blocks use accepted, fresh and candidate binaries. All nine shapes use 32 workers on CPUs 0–31; primary also uses 16 workers on physical CPUs 0–15. Primary32 uses 151 requests after three primes, physical16 uses 501, and other shapes use 31. All 450 latency processes finish before six separate 199 Hz profiles. Loading and primes are excluded from latency and included in process RSS; fresh solve, randomness, prove, internal verification and serialization are timed. No helper, build, hash, archive or second owned CPU workload overlaps requests. Required independent verifiers run serially between captures. No successful capture is extended, omitted or repeated.

|Case|Accepted ms|Fresh ms|Candidate ms|Paired C/A|Individual interval|Wins|Saving µs|RSS A/C MiB|
|---|---:|---:|---:|---:|---|---:|---:|---:|
|1x2-w32|8.310633|8.317877|8.404229|1.010052888|[1.008125184, 1.014328776]|1/15|-82.905|134.00/135.25|
|2x2-w32|11.444606|11.429629|11.479884|1.004995077|[0.997158126, 1.007452479]|5/15|-57.117|272.25/272.25|
|4x2-w32|16.023665|16.030242|16.045488|0.995621900|[0.988245696, 1.000556707]|11/15|71.033|313.00/310.25|
|8x2-w32|24.256758|24.227915|24.297092|1.008460191|[0.993783382, 1.016941082]|6/15|-207.592|509.25/505.75|
|12x2-w32|33.649030|33.917661|33.939158|1.009314863|[0.986853018, 1.019669988]|5/15|-313.221|724.75/698.00|
|16x2-w32|39.961403|40.056113|40.539226|1.011541995|[1.002246623, 1.019402220]|3/15|-459.838|837.07/808.25|
|24x2-w32|66.222937|66.312657|66.523544|1.005774140|[0.998714631, 1.010141211]|4/15|-384.567|1321.15/1308.30|
|merge-24x1-w32|65.028733|65.327417|65.382236|1.004516367|[0.999370489, 1.012036248]|5/15|-294.596|1177.30/1155.44|
|merge-54x1-w32|162.039299|161.523860|161.446096|0.995043839|[0.988374641, 1.005112018]|11/15|806.843|1870.75/1839.41|
|1x2-w16|10.097085|10.070642|10.223082|1.012206562|[1.003608171, 1.023378041]|2/15|-122.201|93.00/94.00|

## Frozen checks

All ten fresh/accepted individual median intervals must stay within factor 1.03; candidate upper bounds must be ≤1.02 against both controls. Primary32 upper bounds must be below parity; original-eight within-block geometric aggregate upper bounds must be ≤1. Fifteen-block order intervals use ranks 4–12, nominal coverage 96.484375% under independent/exchangeable blocks, without simultaneous familywise guarantees. No failed check is relabeled.

Calibration: 0 failure(s).

Nonregression bounds: 4 failure(s).

- 1x2-w16 candidate/accepted upper=1.0233780410979931

- 1x2-w16 candidate/fresh upper=1.0254342343170069

- 2x2-w32 candidate/fresh upper=1.022557166991784

- 8x2-w32 candidate/fresh upper=1.0201830091825757

Primary gain: 2 failure(s).

- 1x2-w32 candidate/accepted upper=1.0143287759910458

- 1x2-w32 candidate/fresh upper=1.013611784493848

Aggregate: 2 failure(s).

- candidate/accepted original8 upper=1.0076703096042832

- candidate/fresh original8 upper=1.007815261988024

Original-eight candidate/accepted: 1.003412938, interval [1.001258254, 1.007670310].

Original-eight candidate/fresh: 1.006684629, interval [1.003403832, 1.007815262].

## Warm profiles

Sampled CPU shares are not exclusive critical-path costs, memory bandwidth or removable wall-time fractions. The six profiles report zero lost records and one SDK PID each.

|Case|Variant|Samples|CPU ms/request|G1 %|G2 %|Inverse %|Epoch %|Scheduler %|
|---|---|---:|---:|---:|---:|---:|---:|---:|
|1x2-w32|accepted|9103|190.399|38.95|29.27|7.05|10.23|3.52|
|1x2-w32|fresh|9308|192.013|37.42|29.76|7.50|9.65|3.89|
|1x2-w32|candidate|9492|192.894|38.07|29.35|7.05|8.82|4.14|
|merge-54x1-w32|fresh|216683|4416.944|28.18|8.83|2.09|2.13|2.03|
|merge-54x1-w32|candidate|218949|4467.174|28.95|9.11|2.00|2.03|2.03|
|merge-54x1-w32|accepted|216775|4415.025|28.44|8.76|1.97|2.21|2.06|

## Supplied M5 references

These are supplied historical CPU/Metal references, not new Apple measurements. Ratios below use the candidate local32 medians. No 54 reference was supplied.

|Shape|Candidate ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required CPU 10× ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.404229|37|4.403×|24|2.856×|3.7|
|2x2|11.479884|59|5.139×|30|2.613×|5.9|
|4x2|16.045488|109|6.793×|48|2.991×|10.9|
|8x2|24.297092|189|7.779×|74|3.046×|18.9|
|12x2|33.939158|284|8.368×|101|2.976×|28.4|
|16x2|40.539226|335|8.264×|124|3.059×|33.5|
|24x2|66.523544|528|7.937×|176|2.646×|52.8|
|merge-24x1|65.382236|507|7.754×|175|2.677×|50.7|

## Validation and scope

Fresh passes 119 native core tests; candidate passes 121. Two new contracts compare stored arithmetic against scalar fields and check the footprint/forecast bound. Existing complete-curve G1 oracles exercise signs, owned/borrowed batches, all store and inverse modes, tails and scratch reuse. Both builds pass two raw and six cubic tests; candidate portable/no-IFMA consumer checks pass. Native field source and its 33-test qualification are inherited unchanged, not newly executed here.

Nine preflights add 36 SDK calls/nine exports; latency adds 41,850/450 and profiles add 1,524/six. Total **43,410 SDK calls, 465 distinct independently Gnark-verified exports and 930 proof/public members**. All proofs are 164 bytes with exact fixture bindings and fresh against prior/new identities. The ledger reaches **12,125 from 11,660**. All pins are checked again at closure. Six profiles contain 680310 samples. Raw perf data and executables remain local with hashes; decoded symbols, proofs and receipts are archived.

No new AWS, Aeglos, physical-phone/ARM/TEE, memory-pressure, minimum-RAM, throughput or giant-circuit performance claim is made. A positive result requires separate compatibility, concurrent and resource review before adoption. The original all-eight CPU 10× goal remains active.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [rows](artifacts/performance/rows.json), [source edits](artifacts/edits.json), [native workflow](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [raw hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).
