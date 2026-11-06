# Signed radix-52 native qualification

The fixed native signed-difference multiplication passed the focused oracle and code-generation checks. It qualifies one prospective matched primitive timing screen. Production is unchanged, and no speedup has been measured.

The [exact arithmetic model](../2026-11-06-signed-pair52-model176/README.md) preceded this implementation. The isolated snapshot has 178 files: 174 original files unchanged, two recoverable parent edits (eight-lane selector/include and generator), and two new kernel/test children. The four-lane kernel and accepted field remain unchanged. No old core was linked to a changed field.

Four new native tests compared **240,096 actual SIMD outputs and 240,096 exports against Arkworks**, covering Fq/Fr, R256/R260, ordinary/sum2/complement-difference2, normalized lazy inputs including 2p, boundaries, full-width random tuples and changing chains. Per field/radix: 14,884 boundary tuples, 4,096 random tuples, four padding tuples and 1,024 chain tuples, with three operations each. The chains have eight independently changing lanes over 128 steps. The 35 old tests were filtered; broader integration qualification is still required before adoption. All 16 primitive preflight cases matched accepted and fresh OFF controls.

The generator check and ten native build/test/preflight commands succeeded in one xtask attempt. Actual final caller command receipts contain O3/native/fat-LTO/one-CGU flags. A separate review verified 262 pins, eleven command receipts, all four test populations and 48 newly decoded optimized primitives, without repeating their execution. Accepted and fresh OFF have identical instruction text in all sixteen complete decoded function bodies; this is not a binary-file identity or timing calibration claim.

## Generated code

The IFMA reduction survives optimization, but additional instructions and stack-addressed operations increase. Counts below cover the largest backward-branch span containing IFMAs, including conditional paths. They are static rows, not dynamic instructions, spill service time or an exclusive cycle attribution.

| Fq/R256 loop | Accepted IFMA | Candidate IFMA | Accepted rows | Candidate rows | Accepted vector RSP/RBP operand rows | Candidate rows |
|---|---:|---:|---:|---:|---:|---:|
| mul1 | 100 | 80 | 169 | 215 | 5 | 18 |
| mul4 | 100 | 80 | 190 | 241 | 20 | 20 |
| sum1 | 150 | 110 | 255 | 312 | 12 | 62 |
| diff1 | 150 | 110 | 276 | 394 | 20 | 63 |

All selected backedge spans have no calls. Candidate arithmetic signed shifts are present. The ordinary single-stream Fq/R256 function's explicit RSP subtraction grows from `0x328` (808 bytes) to `0x618` (1,560 bytes); this excludes other stack ownership and is not a measured live-stack peak or minimum RAM. Four-stream callers keep distinct states, but the compiler rolls the stream loop rather than holding four unrolled products simultaneously in registers; their listed IFMA count is one product body. Fused callers have fixed second-product inputs across iterations, allowing some preprocessing to move outside the loop. These are primitive screens, not complete G1/G2 callers. No layout/attribute sweep was used to change the generated code.

The initial snapshot helper stopped when it attempted to decode a binary fixture as UTF-8. It was repaired to copy files as bytes and decode only the two known textual edits, before any native build or execution. The partial snapshot and original helper/binary remain local; its source/failure summary and binary identity are preserved in the archive/review. The test build emitted nine unused-import warnings (eight inherited, one new); no warning-only rebuild was performed.

## Scope and next stage

The extra arithmetic and register pressure may outweigh the saved IFMAs. Freeze one paired primitive protocol with the accepted field, fresh OFF and candidate before any clock; retain both fields, both radices and all declared contexts. A primitive win alone does not justify adoption: matched complete callers and verified all-eight plus merge54 full SDK trials, resource checks, portable fallbacks and independent Gnark verification remain necessary.

There is no new proof timing, AWS/Aeglos comparison, RSS peak, minimum-RAM, large-circuit or mobile result. No lookup table, key copy, execution plan or production thread pool is added. Accepted local32 1×2 remains **7.928660 ms**, merge54 **161.883899 ms**, and inherited RSS **132.75 MiB** is not a new measurement or minimum requirement. Proof ledger remains **30,846**. The complete inherited [eight-shape CPU and Metal comparison table](../2026-11-06-signed-pair52-model176/README.md#accepted-full-proof-baseline-inherited) is unchanged; the 10× CPU goal remains unmet for all eight shapes.

Evidence: [codegen](artifacts/codegen.json), [review](artifacts/review.json), [source audit](artifacts/source-audit.json), [pins](artifacts/source-pins.json), [design](artifacts/design.md), [source and receipts](artifacts/source-and-receipts.tar.gz). The public archive includes only selected UTF-8 sources/deltas/receipts/decodes, with no executable, library, binary fixture, key, witness or proof blobs. The public mathematical fixture is preserved in the parent model archive rather than duplicated.
