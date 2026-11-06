# Joint complex complete-caller qualification

The joint signed-pair complex API is integrated into the actual G2 product paths and passes changed-core correctness checks. The final G2 caller has **1,230 static IFMA sites versus 1,450** in both controls. **No latency benchmark has run for this candidate.** Additional instructions and stack-addressed operations remain risks for the upcoming complete-caller measurement.

## Integration and tests

Three reversible core changes select the new API for ordinary complex multiplication and both denominator-conjugated numerator paths. Original expressions remain as OFF branches. Norms, scaling, inverses, prefix records, scalar tails, exceptional-point guards, queues and scratch geometry retain their implementations. Ordinary field arithmetic, FrFFT and G1 are unchanged. The [qualified field libraries](../2026-11-06-joint-pair52-native176/README.md) are reused without rebuilding or rerunning their tests. Each fresh core explicitly links its respective qualified OFF/ON field, and each new driver links that core; no old core is attached to a changed field.

The ON build passes **126 unfiltered core tests**, including the new signed borrowed G2 test across 13 empty/tail/full lengths and 1/3/17 reuse counts against scalar-generator expectations. OFF runs that one new test, with 125 unchanged tests filtered. Field/math qualification is inherited, not counted as new. These tests exercise the integrated code.

All eight build/test/link commands succeed on the first attempt. Review verifies 217 pins, all receipts, source recovery, flags and coherent libraries. The 179-file snapshot has 175 native-parent files byte-exact and all 178 native-parent files recoverable; 171 accepted originals remain exact and all 176 recover. G1/G2 diagnostic adapter bodies are unchanged, and the driver only renames its compile-time arithmetic flag. No driver clock or profile is executed in this stage.

## Actual final consumers

Nine newly decoded G1/G2/inverse functions cover the frozen accepted-source reference and both new executables. Counts describe whole static functions, including prefixes,tails and cold branches; they are not dynamic instructions, exclusive cycles or spill costs.

| Function/build | IFMA sites | Instruction rows | Vector RSP/RBP operand rows | Bytes |
|---|---:|---:|---:|---:|
| G1/FROZEN | 800 | 4425 | 286 | 26198 |
| G1/OFF | 800 | 4425 | 286 | 26198 |
| G1/ON | 800 | 4425 | 286 | 26198 |
| G2/FROZEN | 1450 | 8590 | 603 | 51830 |
| G2/OFF | 1450 | 8590 | 603 | 51830 |
| G2/ON | 1230 | 9312 | 743 | 56981 |
| inverse/FROZEN | 500 | 1139 | 67 | 6979 |
| inverse/OFF | 500 | 1139 | 67 | 6979 |
| inverse/ON | 500 | 1139 | 67 | 6979 |

The actual G2 static IFMA count falls 15.17%, while instruction rows rise 8.41% and vector stack-addressed rows rise 23.22%. Its literal RSP subtractions change from `0xfc0 + 0x780` to `0xfc0 + 0xec0`; this excludes alignment, pushes, callees and other frames, so it is not a measured lifetime stack peak or minimum RAM. G1 and inverse retain their counts. The joint-complex arithmetic is inlined in the G2 function. These observations justify a matched actual-caller measurement; adoption requires further evidence.

## Controls and provenance

The frozen control is accepted-source binary `389307b91908fe4822d9b4a788ff507944257f064bd0f11cde317ebd2cf6782b`, inherited from the prior reference audit. Its scope is a diagnostic caller, distinct from the production SDK. The recorded Cargo link input field path currently hashes to `b991c821121372aeb0ce07dfd9f7d3b5c839579384dbbcff1a96ea9d00d73d04`. The audit's separate `new_field_library` entry belongs to its word-export candidate. The [provenance note](artifacts/provenance-note.md) corrects earlier private field1200 shorthand for this caller. Exact binary hashes and actual code are checked; paired timing calibration remains required.

Before any native build, a mistyped illustrative hash in private design text was removed; only that document pin was corrected, with original text/pins retained. Successful setup was not repeated. No native execution failed or was rerun. A setup-helper unused-constant warning did not prompt a rebuild. These preparation notes do not change source, flags or evidence.

## Next measurement and baseline

Freeze a separate actual-caller protocol before any driver clock: G2 sizes 200/352 and 201/353 tails, workers 1/16/32, frozen/fresh OFF/ON builds, unchanged G1 controls, fixed paired blocks, output/capacity/zero-allocation guards and automatic PMU/cycle profiles with verified ownership/samples/loss. Caller gains can select full all-eight plus merge54 SDK, resource/portable and independent Gnark proof qualification before adoption.

Production source176/SDK6d83 remain unchanged. Accepted local 32-worker 1×2 is **7.928660 ms**, merge54 **161.883899 ms**; inherited 132.75 MiB RSS is not a new peak or minimum. The canonical proof ledger remains 30,846. The inherited [all-eight M5 CPU and Metal comparisons](../2026-11-06-signed-pair52-model176/README.md#accepted-full-proof-baseline-inherited) remain current, and the 10× CPU goal is unmet for all eight shapes. This stage adds correctness and code-generation evidence only.

[Codegen](artifacts/codegen.json), [review](artifacts/review.json), [source audit](artifacts/source-audit.json), [pins](artifacts/codegen-pins.json), [control identity](artifacts/frozen-reference.json), [design](artifacts/design.md) and [source/receipts](artifacts/source-and-receipts.tar.gz) are preserved. The public archive contains selected UTF-8 sources, receipts and decodes. Executables, libraries, binary fixtures, keys, witnesses and proof blobs are excluded.
