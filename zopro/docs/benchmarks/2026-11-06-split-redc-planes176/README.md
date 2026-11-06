# Split product/reduction coefficient planes: mathematical screen

The split-accumulator recurrence passes **80,844 exact output comparisons and 404,220 row-state checks**, plus 2,796,192 reduced-radix carry checks. Its source-level dependency screen improves with unchanged IFMA count. **No native candidate has been qualified or timed, and no production optimization is adopted.**

## Why this hypothesis

The [hardware calibration](../2026-11-06-instruction-calibration176/README.md) established IFMA accumulator latency about 4 cycles and vector add/carry-shift latency about 2 cycles on this Ryzen 9950X. A new offline census reuses all 9 saved caller decodes and all 18 saved profile panels (119,638 samples), with exact ELF executable LOAD/MMAP2 address translation and instruction-boundary/symbol agreement. No recording, decode or proof is repeated.

The prominent OFF G2 block at +0x8aa2 contains a selected connected chain of 28 IFMAs and one add. Its partial score changes only 113→114 when substituting calibrated weights. This cautions against inferring from slower carry instructions that final normalization is the main opportunity. Across the three G2 OFF panels this block contains 32.45–33.07% of sample locations. These are block-location counts with sampling skid, not exclusive cycles, a proven hardware critical path or removable proof time.

| OFF consumer | Instruction rows | Basic blocks | Calibrated register-only rows | Largest connected score |
|---|---:|---:|---:|---:|
| G1 | 4425 | 154 | 1181 | 92 |
| G2 | 8590 | 191 | 2169 | 114 |
| inverse | 1139 | 4 | 638 | 92 |

Unknown vector writers, masks, memory operands, uncalibrated instructions and block boundaries break the scored chains. Loads/stores, register copies and inter-block dependencies are absent. The scores are neither a complete schedule nor a proven latency lower bound. The graph parser is checked against 16 already decoded instruction-calibration loops and explicit alias/mask/memory exclusions. The independent reviewer checks every reported chain edge against actual intervening register writes and partitions every instruction exactly once. The first offline pass rejected the renamed G2 validated-ON symbol; the saved-symbol repair is retained and no performance evidence changes. All 18 panels and the rejected ON arm remain in the archive.

## Recurrence

For the first four radix-52 CIOS rows, product halves update a plane P and Montgomery-reduction halves update Q. The quotient uses their low-coefficient sum z=P0+Q0. The existing cancelled-low carry identity updates Q1, then both planes shift. Their sum recovers the accepted unnormalized coefficient state exactly. Merge them before the fifth row, preserving the original final radix-52/mixed-48 step, normalization and correction.

This changes accumulation dependencies, not the field representation or the number of products. It adds at most nine vector additions and six live vector coefficients; compiler allocation and stack traffic may erase the opportunity. No heap buffer, table, key/point copy, additional plan or pool is proposed. This differs from the rejected FIPS, paired cancellation, row-quotient and independent-complex-output mechanisms; those results stay closed. The earlier zero-accumulator mask audit also found that LLVM already removes the proposed 52-bit masks, so it was not rerun.

The checked-u128 digit interpreter agrees after each row with an independent BigUint whole-state recurrence, full-R quotient and field reference. Coverage includes both BN254 fields, R256/R260, ordinary/sum2/original-complement difference2, inclusive[0,2p] boundaries, 4,096 random tuples and 1,024 dependent chain cases per context. The [range argument](artifacts/bounds.md) bounds coefficients strictly below 64B=2^58. Finite tests and source algebra are not a formal proof of future native code.

## Necessary dependency screen

The idealized source DAG uses calibrated weights IFMA = 4, add = 2, right-shift-by-52 = 2 and includes extra frontier and merge additions. Constants/input limbs start ready. Register allocation, spills, uncalibrated final radix-48 instructions and the common final normalization are omitted. Values below are **dependency scores, not measured cycles or speed predictions**.

| Operation | Fourth quotient, ordinary layout | Fourth quotient, split planes | Merged next-row frontier, ordinary | Merged next-row frontier, split | First-four-row IFMAs, either |
|---|---:|---:|---:|---:|---:|
| Ordinary multiply |54|44|62|52|80|
| Sum/difference of products |82|62|94|70|120|

The final common row brings nominal totals to 100/150 IFMAs. The necessary screen passes; the next stage is an actual native implementation, independent output tests and linked-caller codegen inspection. A positive model alone does not justify SDK timing or adoption. Actual caller/topology and full all-shape SDK/Gnark/resource/portable gates remain necessary.

Preparation retained one pre-execution JSON-macro compilation correction and a corrected bound-text equality; the corrected mathematical model ran once successfully. No mathematical result, successful native capture, gate or input subset was replaced.

## Unchanged full-proof baseline

Inherited accepted-176 local 32-worker single-proof times below are not new measurements. The previously documented physical-16 calibration engineering exception remains. The original all-eight CPU 10× goal is active and unmet (0/8).

| Shape | Accepted CPU ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
|1x2|7.928660|37|4.667×|24|3.027×|
|2x2|11.151932|59|5.291×|30|2.690×|
|4x2|16.086386|109|6.776×|48|2.984×|
|8x2|24.179453|189|7.817×|74|3.060×|
|12x2|33.884455|284|8.381×|101|2.981×|
|16x2|40.077381|335|8.359×|124|3.094×|
|24x2|65.850777|528|8.018×|176|2.673×|
|merge24|65.060937|507|7.793×|175|2.690×|

Inherited merge54 remains 161.883899 ms with no supplied M5 reference. Inherited 132.75 MiB RSS is not a new peak or minimum RAM. Proof ledger stays 30,846. No fresh AWS/Aeglos, SDK/Gnark proof, portability, memory-pressure or mobile result is claimed.

[Model result](artifacts/model.json), [design](artifacts/design.md), [range argument](artifacts/bounds.md), [integer/work model](artifacts/model.rs), [dependency census source](artifacts/dependency-census.rs), [census protocol](artifacts/dependency-protocol.md), [independent review](artifacts/review.json), [review/publication source](artifacts/review-publish.rs), [source/results archive](artifacts/source-and-receipts.json.gz), [member hashes](artifacts/archive-index.json) and [input pins](artifacts/input-pins.json). The archive contains all basic blocks, selected chains, 18 panels, new source and repair records; original recorded streams and decodes remain at their pinned local paths and prior reports.
