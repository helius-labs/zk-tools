# Native joint signed-pair complex qualification

The fixed native joint complex API passes its focused oracle checks. Its190-IFMA loop count survives optimization, versus300for two retained fused products. **No latency or proving speedup is measured.** More instructions and stack-addressed operations remain material risks. The next measurement must use the actual G2 caller, where previous Gauss variants lost despite primitive gains.

The [parent exact model](../2026-11-06-joint-pair52-complex176/README.md) checked the joint coefficient algebra and signed reduction before implementation. The private snapshot has178files:174originals byte-exact, two recoverable parent append/generator edits and two kernel/test children. All176originals recover. Ordinary field, FrFFT and core function bodies remain unchanged; no old core is linked to a changed field. No heap/table/key-copy/plan/pool expansion is proposed.

## Actual SIMD checks

Four new native tests compare **324,224raw outputs and324,224exports against independent Arkworks arithmetic** overFq/Fr,R256/R260,normal/conjugated products,lazy[0,2p]endpoints,random tuples and changing chains. Each configuration has81,056outputs:14,884boundary tuples,4,096random,256endpoint-cartesian,four padding,and1,024changing-chain tuples,with four outputs each. Chains have eight independently changing lanes over128steps. Raw outputs must have normalized digits and lie below2p; exports must be canonical and preserve the Montgomery factor. No unreduced host Fp objects are used as inputs. The35old tests are filtered, not claimed rerun.

The generator check and ten native build/test/preflight commands complete first attempt. The test build has eight inherited unused-import warnings; no warning-only rebuild occurs. All16preflight cases match accepted,freshOFF andON. Three final consumers use actualO3/native/fat-LTO/one-CGU flags. A separate review checks263pins,eleven successful receipts,all four test populations,source identities and48new optimized decodes without repeating execution. Accepted/freshOFF instruction text agrees in all16complete decoded function bodies; this does not replace timing calibration.

## Generated loop tradeoff

Counts cover the largest backward-branch span containing IFMAs, including its conditional paths. They are static rows and literal operand references, not dynamic instructions, measured spills or stall cycles.

| Fq/R256 | Accepted IFMA | ON IFMA | Accepted rows | ON rows | Accepted vector RSP/RBP rows | ON vector RSP/RBP rows |
|---|---:|---:|---:|---:|---:|---:|
| normal1 | 300 | 190 | 504 | 853 | 35 | 143 |
| normal4 | 300 | 190 | 524 | 937 | 65 | 164 |
| conj1 | 300 | 190 | 499 | 881 | 27 | 159 |
| conj4 | 300 | 190 | 519 | 966 | 60 | 167 |

All selected spans have no calls, and candidate signed arithmetic shifts are present. The normal single-stateFq/R256 function has literal RSP subtractions0x1000+0x358, versus0x758accepted. Those exclude pushes,alignment and callees; they are not a measured lifetime stack peak. The candidate needs more static instruction rows and stack-addressed vector operations even while saving110IFMAs. Source20-coefficient storage is not a proof of register residency or a RAM bound.

Four-state loops are compiler-rolled, not four fully register-resident products. BothB operands are invariant across these diagnostic iterations, so some preprocessing can move outside the loop. The final drivers support preflight only and no timing mode. Their code is not a complete G2 caller or SDK. No width/layout/attribute sweep or old-kernel rerun was used to improve these counts.

## Next stage and unchanged baseline

Integrate the qualified API into a separate coherent core snapshot at the actual normal and denominator-conjugated G2 product sites, retaining norm/scale/prefix/inverse/tail behavior and original fallbacks. Qualify changed complete callers, then freeze200/352plus tails,1/16/32workers,accepted+freshOFF,candidate,unchangedG1controls and automaticPMU/cycle profiles before any clock. A caller win still needs verified all-eight+merge54 fullSDK/resource/portable/Gnark qualification before adoption. No additional primitive-only timing run is selected.

This is a new combined limb decomposition and raw joint complex dataflow, not a representation subset of the closed global ordinary signed-pair build or a rerun of the old250-IFMA Gauss caller. All earlier results remain retained. No new full proof, profiling, AWS/Aeglos, huge/mobile, memory pressure, swap or throughput result exists.

Accepted production176/SDK6d83 remain unchanged: local32 1×2 **7.928660ms**,merge54 **161.883899ms**,inherited132.75MiBRSS(not a new peak/minimum). Proof ledger remains30,846. The inherited [all-eight M5CPU and Metal comparisons](../2026-11-06-signed-pair52-model176/README.md#accepted-full-proof-baseline-inherited) remain current; the10×CPU goal is unmet on all eight shapes.

[Codegen](artifacts/codegen.json),[review](artifacts/review.json),[source audit](artifacts/source-audit.json),[pins](artifacts/codegen-pins.json),[design](artifacts/design.md) and[source/receipts](artifacts/source-and-receipts.tar.gz) are preserved. The archive includes only selectedUTF-8sources/receipts/decodes, with no executable,library,binary fixture,key,witness or proof blobs. The inherited public input fixture remains in its parent archive.
