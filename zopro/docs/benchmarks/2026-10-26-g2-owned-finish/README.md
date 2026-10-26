# Owned-batch storage in G2 finishing

The fixed whole-SDK speed gate fails and the candidate is rejected. Accepted production remains unchanged.

Primary paired ON/accepted is 1.018920038, with 1/7 wins; ON/OFF is 1.006002210, with 1/7 wins. Original-eight paired aggregates are 1.008635209 and 1.009658718. These are complete verified SDK request measurements.

## Change and ownership

The [accepted dispatch-counter diagnostic](../2026-10-26-front-end-counters/README.md) found more backend-blocked than frontend-empty slots across its whole warm SDK cases. It did not identify register spills or predict a gain. This experiment changes the actual G2 dataflow while preserving field arithmetic, rather than selecting an isolated field microbenchmark.

The forward norm/prefix pass is unchanged. A full-batch reverse pass computes the same inverse recurrence, slope and new x, then reuses the consumed prefix/norm record for the slope pair. Before writing the new bucket x, it saves the original bucket x into the owned incoming point's now-dead x field. Bucket y remains original. A separate pass computes y=slope*(saved_old_x-new_x)-old_y.

The caller owns and clears this batch after application; it reads only unchanged indices to reset busy bits. Two existing prefix-record slots and the existing batch are reused. There is no additional heap buffer, larger record capacity, lookup table, key copy, representation, solver, scheduler, Key/Plan or workspace-forecast change. Field formulas and inversion count are unchanged. Extra batch-x stores and indexed traversal remain real costs.

Only msm.rs and msm_affine.rs change among the accepted sources, plus a new private G2 module and focused test module. Both original files recover through the documented inverse edits; the other 168 accepted files match. ON adds only --cfg zopro_g2_owned_finish. Default trait application, immutable consumers, nonfull batches, disabled prefix/direct paths and portable configurations retain the existing implementation. This differs from the rejected G1 finish-phase trial: G2's two-slot prefix records lack old-x storage, so this design also consumes the incoming batch to provide it.

## Complete SDK comparison

Ryzen 9950X, seven fixed rotated/reversed blocks, accepted/fresh OFF/ON. All nine 32-worker shapes measure 31 requests after three primes; primary16 measures 101 on CPUs 0–15. Loading is excluded. Solving, fresh randomness, proving, SDK verification and serialization are included. All latency finishes before profiles; no owned build, profile or archive overlaps latency.

| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w32 | 8.577467 | 8.646547 | 8.752187 | 1.018920038 | 1.006002210 |
| 2x2-w32 | 11.710566 | 11.754229 | 11.756693 | 1.003938921 | 1.002654126 |
| 4x2-w32 | 16.177090 | 16.051282 | 16.295763 | 1.002241730 | 1.009866988 |
| 8x2-w32 | 24.693000 | 24.417330 | 24.560720 | 1.003449196 | 1.004583071 |
| 12x2-w32 | 34.216192 | 34.381042 | 34.729938 | 1.011375754 | 1.014570936 |
| 16x2-w32 | 40.802843 | 40.759301 | 41.219929 | 1.003865438 | 1.007812620 |
| 24x2-w32 | 68.618172 | 68.372968 | 69.292993 | 1.010062899 | 1.014812422 |
| merge-24x1-w32 | 67.988865 | 68.103993 | 68.025503 | 1.001939329 | 1.000196018 |
| merge-54x1-w32 | 179.145661 | 176.559591 | 178.219631 | 0.995409671 | 1.009483488 |
| 1x2-w16 | 10.816439 | 10.512366 | 10.735637 | 0.980748470 | 1.016872626 |

Times are medians of process medians; ratios are medians of block-paired ratios. The frozen gate requires primary and original-eight aggregate improvement against both controls, primary at least 4/7 wins, each case within 2% paired median and 5% each block, and OFF calibration within a factor of 1.03. There are 8 failed checks; all OFF calibration checks pass: true. No minimum gain floor, excluded observations or unchanged retry. Physical16 retains ON/accepted block ratios of 1.451365 and 1.069711; primary32 and original-eight failures independently reject this candidate.

| Unprofiled resource observation | Accepted | OFF | ON |
| --- | ---: | ---: | ---: |
| Primary CPU ms/request | 199.216 | 198.533 | 200.102 |
| Primary maximum process RSS MiB | 113.500 | 110.500 | 110.750 |
| Merge54 CPU ms/request | 4914.099 | 4879.510 | 4893.515 |
| Merge54 maximum process RSS MiB | 1916.660 | 1904.422 | 1908.840 |

CPU values are medians of process means. RSS is the maximum whole process including load/primes, not minimum RAM or an allocation forecast. Fault and switch counts remain in every raw observation. Equal source capacities do not establish a measured memory saving.

## Actual caller code and warm profiles

| Variant and routine | Bytes | Instructions | IFMA sites | Literal stack subtraction bytes | RSP operand lines |
| --- | ---: | ---: | ---: | ---: | ---: |
| accepted G2 reduction | 52064 | 8686 | 1550 | 5696 | 1483 |
| accepted G2 prefix false | 51371 | 8479 | 1450 | 6080 | 1412 |
| accepted G2 prefix true | 50288 | 8326 | 1450 | 5696 | 1377 |
| accepted canonical wires | 6030 | 1054 | 0 | 1976 | 423 |
| off G2 reduction | 52064 | 8686 | 1550 | 5696 | 1483 |
| off G2 prefix false | 51371 | 8479 | 1450 | 6080 | 1412 |
| off G2 prefix true | 50288 | 8326 | 1450 | 5696 | 1377 |
| off canonical wires | 6030 | 1054 | 0 | 1976 | 423 |
| on G2 reduction | 52064 | 8686 | 1550 | 5696 | 1483 |
| on G2 owned false | 31485 | 5001 | 1450 | 4928 | 598 |
| on G2 owned true | 30406 | 4836 | 1450 | 4416 | 552 |
| on G2 prefix false | 51371 | 8479 | 1450 | 6080 | 1412 |
| on G2 prefix true | 50288 | 8326 | 1450 | 5696 | 1377 |
| on canonical wires | 6030 | 1054 | 0 | 1976 | 423 |

The new full-batch routine and old tail-capable prefix routine have different scope. Whole-function instruction differences include removed scalar-tail paths. These are actual linked SDK routines, but static sites, stack subtractions and RSP operand lines are not dynamic instruction/spill/traffic counts or causal explanations. No standalone component speed selection precedes this trial.

After latency, nine warm acknowledged FIFO/monotonic cycles:u captures use primary/101,12x2/51 and merge54/9 requests for each variant. The new owned routine has sampled period in ON primary and is absent from accepted/OFF captures.

| Primary profile | CPU ms/request | G1 sample % | G2 sample % | Epoch+scheduler sample % | FFT sample % | New owned routine period |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| accepted | 200.561 | 38.053 | 29.306 | 13.926 | 5.167 | 0 |
| off | 199.508 | 38.065 | 29.025 | 14.280 | 4.729 | 0 |
| on | 199.633 | 37.937 | 29.708 | 13.666 | 4.582 | 12911201576 |

All 306348 samples are retained with zero lost records and one PID per capture. Sample shares locate work but do not measure exclusive opcode costs, spills or removable wall time.

## Validation and limits

Native OFF passes 109 core tests and ON 112, including three new independent complete-G2 tests. Full8/16/80/200/352/512 batches cover permuted distinct indices, signed points, canonical/inverse modes, exact consumed-x postconditions, unchanged indices and equal prefix capacity. Fallback tests preserve every input byte for empty/ragged/disabled/directfalse cases; reused layers compare with independent generator-weight results. Both builds pass two raw-prover and six cubic-prover tests. Field/generator source is unchanged; accepted field qualification is inherited, not counted as a new primitive test. Generator consistency passes.

All nine shapes pass OFF/ON four-worker preflight: 72 SDK requests and 18 independent Gnark exports. Actual FFT Plan ABI is 1728 bytes/alignment 32 in all three builds. The whole experiment completes 9,192 SDK requests and 237 distinct independently Gnark-verified exports, 474 proof/public members. Proofs are 164 bytes; public hashes match established fixtures. Preflight checks freshness against 4,831 identities and performance against 4,849, including the 18 preflight exports.
All 450 performance pins and original build/preflight inputs match at closeout. Both original files recover, new module templates match, and the accepted 170 source files and executable remain unchanged. No prover source, test, compiler-flag, gate or timing repair occurred.

No further portable, small-stack/concurrent, real-pressure, large-circuit, physical-phone, AWS, Aeglos or minimum-RAM qualification follows the failed speed gate. This dataflow variant remains unadopted.

## Supplied M5 comparisons

These ON diagnostics do not replace accepted measurements. CPU and Metal references are supplied historical timings, not fresh Apple measurements.

| Shape | ON ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 | 8.752187 | 37.0 | 4.228× | 24.0 | 2.742× |
| 2x2 | 11.756693 | 59.0 | 5.018× | 30.0 | 2.552× |
| 4x2 | 16.295763 | 109.0 | 6.689× | 48.0 | 2.946× |
| 8x2 | 24.560720 | 189.0 | 7.695× | 74.0 | 3.013× |
| 12x2 | 34.729938 | 284.0 | 8.177× | 101.0 | 2.908× |
| 16x2 | 41.219929 | 335.0 | 8.127× | 124.0 | 3.008× |
| 24x2 | 69.292993 | 528.0 | 7.620× | 176.0 | 2.540× |
| merge-24x1 | 68.025503 | 507.0 | 7.453× | 175.0 | 2.573× |

No M5 merge54 reference was supplied. The accepted checkpoint remains historically 8.594267 ms primary and 178.891799 ms merge54. The original all-eight CPU 10× objective remains active and unmet; this experiment is not an adoption.

[Protocol](artifacts/experiment.json), [paired results](artifacts/pilot-result.json), [raw rows](artifacts/perf/rows.json), [design](artifacts/design.md), [source audit](artifacts/source-audit.json), [linked callers](artifacts/codegen.json), [profiles](artifacts/profile-analysis.json), [proof inventory](artifacts/proof-inventory.json), [closing audit](artifacts/closeout.json), [artifact index](artifacts/artifact-index.json). Raw and expanded profile streams remain locally hash-indexed and are omitted from Git. Executables/caches are represented by identities and sources.
