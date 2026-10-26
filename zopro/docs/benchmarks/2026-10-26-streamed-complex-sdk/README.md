# Streamed quadratic-extension multiplication in the CPU prover

The full SDK candidate fails its fixed speed gate and is rejected. Primary paired latency regresses 4.753% against accepted and 4.327% against fresh OFF, with 0/7 and 1/7 wins. The original-eight aggregate regresses 2.921% and 2.560%, losing every block against both controls. All OFF calibration checks pass. Production remains unchanged.

## Integration and scope

The [preceding component](../2026-10-26-streamed-complex-b/README.md) passed its field benchmark: roughly 15% faster than ordinary arithmetic and 0.656% faster than the earlier shared-CIOS method, with a 3.202% gain in its affine-expression pattern. This trial integrates that exact streamed method into the actual native IFMA8 G2 lane product and conjugate-numerator product. It retains three raw products, two simultaneous signed CIOS accumulators and two reductions, precomputes summed A and streams exact summed B one radix-52 digit per row.

Two field files gain the byte-identical tested methods; a core FqBatch bridge and two G2 methods select them at compile time. All four original files recover through documented inverse edits; the other 166 accepted files match. OFF uses accepted native flags; a separate ON executable adds only `--cfg zopro_streamed_complex_b`. There is no per-call environment check, runtime selector, table, cache, heap buffer, representation, worker, scheduler, solver, query-order, Key/Plan or workspace-forecast change. The ordinary four-lane/portable path remains.

## Complete SDK comparison

Ryzen 9950X, 16 physical cores and 32 logical CPUs. Seven rotated/reversed process blocks compare accepted, freshly compiled OFF and ON. Every 32-worker shape measures 31 requests after three primes; primary with 16 workers measures 101 on CPUs 0–15. Requests include solving, fresh randomness, proving, SDK verification and serialization. Loading and primes are excluded. All latency finishes before profiles; no other owned build, profile or archive overlaps latency.

| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w32 | 8.652529 | 8.674601 | 9.063745 | 1.047525527 | 1.043270662 |
| 2x2-w32 | 11.808991 | 11.789546 | 12.040880 | 1.024627940 | 1.026665677 |
| 4x2-w32 | 16.377227 | 16.176368 | 16.748528 | 1.025691346 | 1.032070320 |
| 8x2-w32 | 24.731095 | 24.764446 | 25.820529 | 1.052614988 | 1.036461728 |
| 12x2-w32 | 34.567235 | 34.520765 | 35.248192 | 1.023086021 | 1.028623763 |
| 16x2-w32 | 41.199613 | 41.085516 | 41.905536 | 1.027296016 | 1.026173278 |
| 24x2-w32 | 69.268377 | 69.022089 | 70.644302 | 1.029315235 | 1.035213005 |
| merge-24x1-w32 | 67.832392 | 68.665609 | 68.508132 | 1.020700949 | 1.001476654 |
| merge-54x1-w32 | 177.767922 | 175.332904 | 178.935113 | 1.006565813 | 1.018192632 |
| 1x2-w16 | 10.769231 | 10.661376 | 11.242183 | 1.037427093 | 1.063467072 |

Times are medians of process medians; ratios are medians of paired block ratios and can rank differently. The gate requires primary and original-eight improvement against both controls, primary winning at least 4/7 blocks, every case within 2% paired median and 5% in each block, and OFF calibration within a factor of 1.03. Forty-one checks fail. Primary and aggregate failures independently reject the candidate. Maximum OFF calibration factor is 1.013990000. No minimum positive-gain floor, observation exclusion or unchanged retry applies.

| Unprofiled resource observation | Accepted | OFF | ON |
| --- | ---: | ---: | ---: |
| Primary CPU ms/request | 200.383 | 199.484 | 208.584 |
| Primary maximum process RSS MiB | 116.250 | 116.000 | 116.250 |
| Merge54 CPU ms/request | 4831.983 | 4829.649 | 4902.360 |
| Merge54 maximum process RSS MiB | 1904.383 | 1874.441 | 1898.273 |

CPU entries are medians of process means. RSS includes loading and primes; it is not minimum RAM or a live-workspace bound. Primary ON and accepted reach the same observed maximum RSS, while ON uses more CPU time and takes longer. Fault and context-switch counters remain in every raw row. These observations do not diagnose a general RAM-bandwidth limit.

## Actual generated code and profiles

The standalone component's smaller frame did not carry into the prover. All three SDKs emit the same canonical-wire caller counts. Both G2 prefix specializations and G2 reduction use fewer static IFMA sites in ON, while the functions and literal stack allocations grow.

| Variant and actual caller | Bytes | Instructions | Static IFMA sites | Literal stack subtraction bytes | RSP operand lines |
| --- | ---: | ---: | ---: | ---: | ---: |
| accepted G2 reduction | 52064 | 8686 | 1550 | 5696 | 1483 |
| accepted G2 prefix false | 51371 | 8479 | 1450 | 6080 | 1412 |
| accepted G2 prefix true | 50288 | 8326 | 1450 | 5696 | 1377 |
| accepted canonical wires | 6030 | 1054 | 0 | 1976 | 423 |
| off G2 reduction | 52064 | 8686 | 1550 | 5696 | 1483 |
| off G2 prefix false | 51371 | 8479 | 1450 | 6080 | 1412 |
| off G2 prefix true | 50288 | 8326 | 1450 | 5696 | 1377 |
| off canonical wires | 6030 | 1054 | 0 | 1976 | 423 |
| on G2 reduction | 58581 | 9581 | 1450 | 11840 | 1923 |
| on G2 prefix false | 58671 | 9530 | 1350 | 12160 | 1901 |
| on G2 prefix true | 57894 | 9405 | 1350 | 12160 | 1897 |
| on canonical wires | 6030 | 1054 | 0 | 1976 | 423 |

Static sites are not executed instruction counts. Literal stack subtractions omit alignment, pushes and callees; RSP operand lines do not count dynamic spills or bytes transferred. The larger caller and extra arithmetic/operand work are consistent with the SDK regression, but these data do not isolate a causal cost. Fewer multiplications alone did not deliver a proof-speed gain.

After latency, nine declared warm FIFO-controlled captures use monotonic user-cycle sampling at primary/101, 12x2/51 and merge54/9 requests. All 301,177 samples are retained with zero lost records and one PID per capture.

| Primary warm profile | CPU ms/request | G1 sample % | G2 sample % | Epoch + scheduler sample % | FFT sample % |
| --- | ---: | ---: | ---: | ---: | ---: |
| accepted | 199.524 | 39.016 | 29.939 | 13.580 | 4.417 |
| off | 199.840 | 38.504 | 29.154 | 13.818 | 4.040 |
| on | 209.664 | 37.372 | 31.203 | 13.826 | 4.559 |

After rejection, the qualified Rust instruction reader analyzes all three retained primary captures: 49,666 samples, with no new timings or captures. Recorded PID-specific MMAP2 and ELF PT_LOAD offsets determine relocation; every instruction's bytes match the executable and sampled executable IPs match exact boundaries. Outside-executable, aliases and unclassified code remain explicit. An independent JSONL check conserves each run's sample and period totals.

| Primary G2-prefix instruction observations | Scope share of capture % | IFMA within scope % | Vector add/sub within scope % | RSP operands within scope % |
| --- | ---: | ---: | ---: | ---: |
| accepted | 14.981 | 40.549 | 12.963 | 13.592 |
| off | 15.009 | 45.558 | 11.104 | 12.805 |
| on | 17.520 | 25.731 | 17.068 | 26.661 |

Instruction families and address-register tags are orthogonal and overlap. These are weighted cycle-IP associations, subject to sampling skid; they are not exclusive opcode costs, retired counts or removable latency. Even the unchanged accepted/OFF instruction streams differ in their finite sampled IFMA shares. RSP/RBP tags do not prove spills or traffic; RBP can be a general register. Literal named scopes omit inlined work attributed to other enclosing symbols. The ON data supports investigating total arithmetic and intermediate lifetimes rather than using raw IFMA count as the performance model.

## Validation and decision

Both separate native builds pass all 109 core tests, two raw-prover tests and six cubic tests. Required tests include independent G2-prefix/generator, odd-count/tail, reused-layer and combined-query oracles. The generator consistency check passes. The exact field methods inherit the preceding component's 91,136 BigInt/raw-row-model results and 729,088 Arkworks chained result lanes after byte matching; no new independent field-suite run is claimed.

All nine shapes pass OFF/ON four-worker preflight: 72 fresh SDK requests and 18 independent Gnark exports. Actual FFT Plan size/alignment is 1728/32 in accepted, OFF and ON. The full experiment completes 9,192 benchmark SDK requests and 237 distinct Gnark-verified exports, retaining 474 compressed proof/public members. Every proof is 164 bytes; public inputs match the established fixtures. Preflight freshness checks against 4,567 prior proof identities; performance adds the 18 preflight identities and checks against 4,585. Source proof/public digests from the preceding campaign are verified before building that set. Two fixed source fixture members are separate.

All 394 performance pins pass the closing audit, as do original build/preflight and instruction-analysis inputs. All four source edits recover whole original files, both field blocks still match the passed component, and the accepted 170 sources and qualified SDK remain unchanged. No source, compiler, gate or timing repair occurred during this SDK trial. The postmeasurement instruction analysis uses verified user-owned copies of three root-owned raw captures, with original/copy digests recorded. Original nine captures are retained and archived; those read copies need no duplicate archive bytes.

No further portable, small-stack, concurrent SDK, real-pressure, huge-circuit, mobile, AWS, Aeglos or minimum-RAM qualification was run after the failed speed gate. The component result remains valid in its scope; its integration is rejected. Next work must reduce the extra arithmetic or change the dataflow materially, then independently validate and measure the actual prover.

## Supplied M5 references

These candidate diagnostics do not replace accepted production measurements. CPU and Metal references are supplied historical M5 timings, not fresh measurements of those devices.

| Shape | ON ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 | 9.063745 | 37.0 | 4.082× | 24.0 | 2.648× |
| 2x2 | 12.040880 | 59.0 | 4.900× | 30.0 | 2.492× |
| 4x2 | 16.748528 | 109.0 | 6.508× | 48.0 | 2.866× |
| 8x2 | 25.820529 | 189.0 | 7.320× | 74.0 | 2.866× |
| 12x2 | 35.248192 | 284.0 | 8.057× | 101.0 | 2.865× |
| 16x2 | 41.905536 | 335.0 | 7.994× | 124.0 | 2.959× |
| 24x2 | 70.644302 | 528.0 | 7.474× | 176.0 | 2.491× |
| merge-24x1 | 68.508132 | 507.0 | 7.401× | 175.0 | 2.554× |

No M5 merge54 reference was supplied. Accepted production remains the [qualified checkpoint](../2026-10-25-grouping-module/README.md), historically 8.594267 ms for primary and 178.891799 ms for merge54. The all-eight CPU 10× goal remains active and unmet (0/8); this candidate is not adopted.

[Protocol](artifacts/experiment.json), [all paired results](artifacts/pilot-result.json), [all raw rows](artifacts/perf/rows.json), [source audit](artifacts/source-audit.json), [caller code](artifacts/codegen.json), [warm profiles](artifacts/profile-analysis.json), [instruction bins](artifacts/instruction-summary.json), [proof inventory](artifacts/proof-inventory.json), [closing audit](artifacts/closeout.json) and [artifact index](artifacts/artifact-index.json). Nine raw captures, nine expanded sample streams, and three full disassembly/metadata pairs remain local with hashes and are omitted from Git. Executables/compiler caches are represented by pinned identities and sources.
