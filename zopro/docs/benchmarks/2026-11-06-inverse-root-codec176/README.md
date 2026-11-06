# Native lazy inverse-root product codec

**Correctness and emitted-work qualification passed; performance is not measured.** The codec defers canonicalization until after the scalar root product. In the actual optimized inverse consumer it removes the vector canonical comparison and reduces static instruction rows by 24. This selects a separate matched complete-caller experiment, not production adoption.

The native codec passes correctness and emitted-work qualification and selects a separate complete-caller timing experiment. It is not adopted, and no performance gain is claimed yet.

The actual final ON consumer removes the vector comparison used for the pre-product canonical subtraction. Its shared inverse has 1,128 static instruction rows versus 1,152 in OFF, with 500 IFMA sites in both. ZMM operand rows fall 865→832 and RSP operand rows 85→80; packing changes also increase YMM and unpack rows. Both functions retain a 0x438-byte explicit stack adjustment, plus the same six register pushes. These are static observations, not measured spills, total stack peaks or cycles. The named G1/G2 streaming functions retain their 4,435/8,600 instruction-site counts and 800/1,450 IFMA sites; this does not assert whole-code identity.

Two new field test populations pass against independent Arkworks values: mixed R256 and the original uniform R260 conversion fallback, each exercising both BN254 primes. Across the four field/radix configurations, 17,168 root-product configurations cover all 49 endpoint root pairs, generated full-width/lazy inputs and add/subtract/multiply chains. Every product is canonical and matches separately canonicalized roots as well as the independent field reference. Raw lazy inputs are SIMD limbs, not invalid Fp objects. The actual field summary is two passed, 35 original tests filtered; those old field suites are inherited without rerun.

The existing focused shared-inverse population is reapplied to the changed codec: one test passes with 125 original core tests filtered, covering 1,025 product chains, 8,200 inverse output lanes and 2,050 representation comparisons. It is not counted as a new mathematical population. The earlier 22,616-case raw scalar range screen is inherited, not executed again. No new full-curve caller or proof result is claimed by this stage; the complete G1/G2 drivers have been linked but not executed.

All ten generator/test/library/core/consumer commands pass on first execution. The generator check matches the updated generated source. The 179-file isolated snapshot retains 171 accepted originals exactly; reversing the core midpoint and generator edits and stripping appendices recovers all 176 production sources. The three children contain the field method/tests, unchanged complete-caller adapters and reapplied core inverse test. One changed field library is shared coherently by both fresh core builds and the core test; no old-core/new-field mixture is used. The independent reviewer checks 434 pins, raw receipts/stream hashes, both actual optimized final consumers and new field/core identities. A source patch context mismatch while preparing that reviewer was corrected before compiling or executing it; it did not change native code or repeat a successful stage.

The field method packs raw normalized R256 lanes 0/4, calls the unchanged scalar backend multiply within the qualified range, and constructs Fp from its canonical result. Scalar inverse, reverse products, point formulas, old field-operation bodies, key/image formats, allocations, pools and workspace forecasts remain unchanged. R260 and unsupported modulus bounds use the original conversion/product fallback; no arbitrary-prime or portable-device qualification is added.

The next evidence must cover complete G1/G2 callers at actual batch sizes and tails under solo/physical16/SMT32 contention, both controls, allocations/resources and automatic CPU profiles. Component success alone cannot establish full-proof speed or adoption. Accepted production176/SDK6d83, local 1×2 7.928660 ms, inherited 132.75 MiB RSS and the 30,846-proof ledger remain unchanged. All eight CPU10× requirements remain active and unmet.


## Implementation and qualification

The [raw scalar range proof](../2026-11-06-inverse-lazy-root-product176/README.md) establishes that the existing scalar backend accepts the proposed normalized lazy input range and returns a canonical field. Its 22,616 cases are inherited without rerun. The new `Ifma8::root_product` packs raw lanes 0 and 4, calls that unchanged backend and constructs Fp only after the canonical result. R260 uses the original conversion/product fallback. The [half-root export experiment](../2026-11-06-inverse-half-roots176/README.md) remains closed: it did not defer a reduction across multiplication and added work.

One changed field library is shared by the fresh OFF/ON core and test consumers. OFF retains the accepted midpoint root conversion/product; ON selects the new method through a compile-time flag. Both use the same caller source. The pre-existing frozen control is source176 with the matching complete G1/G2 adapters and optimized native link, not the accepted proof SDK. All final links explicitly use native targeting, O3, fat LTO and one CGU. Every immutable identity and field link is retained.

Ten new Rust/xtask commands pass once: the changed generator check; field-codec test build/run; changed field library; OFF/ON cores; focused shared-inverse test build/run; OFF/ON final caller links. Two new Arkworks populations cover mixed R256 and uniform R260 with Fq/Fr, 17,168 root-product configurations. The field test process reports two passed and 35 original tests filtered. The existing shared-inverse population is reapplied to the changed codec, one passed and 125 originals filtered; its 8,200 lane checks are not a new independent population. No original successful suite, scalar range harness, performance capture or decoder is rerun.

The source snapshot has 179 files: 171 originals exact, five reversible/append-only original-file changes and three children. All 176 production source bytes recover exactly. Generator and generated source match. Existing field-operation bodies, scalar inverse, point arithmetic and workspace forecasts remain unchanged. The independent review checks 434 source/support pins and command/exit/stream hashes before decoding the two actual final consumers once.

## Actual optimized final consumers

Static counts are instruction/operand rows, not dynamic execution cost or measured spills.

|Function|Arm|Instruction rows|IFMA sites|Vector compare rows|RSP operand rows|YMM operand rows|ZMM operand rows|
|---|---|---:|---:|---:|---:|---:|---:|
|`FqBatch::inverse_nonzero`|OFF|1152|500|1|85|15|865|
|`G1 stream_g1_impl`|OFF|4435|800|6|591|190|2064|
|`G2 borrowed apply`|OFF|8600|1450|30|1469|299|3711|
|`FqBatch::inverse_nonzero`|ON|1128|500|0|80|21|832|
|`G1 stream_g1_impl`|ON|4435|800|6|591|190|2064|
|`G2 borrowed apply`|ON|8600|1450|30|1469|299|3711|

Both inverse functions reserve 0x438 bytes explicitly and push six registers. This is not a total stack peak, and neither stack operands nor instruction counts establish latency. Actual allocation/RSS/fault/CPU and speed evidence is deferred to complete callers. There is no new architectural table, key/plan copy, pool, heap buffer or RAM cap.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.




## Continuation and artifacts

The [first unfinished stage](artifacts/next.md) is a separately frozen complete G1/G2 caller protocol with actual batch sizes and tails, solo/physical16/SMT32, both controls, resources and automatic profiles. No candidate clock observation, SDK/Gnark proof, AWS/Aeglos, huge-circuit, phone/ARM/TEE, pressure/swap, minimum-RAM or proof-throughput result is added here. The full all-eight CPU10× goal remains active and unmet.

The publication includes eight exact changed/new UTF8 source members relative to accepted176, all command/test receipts, selected complete function bodies, new field/core/caller identities and hash/roundtrip verification. Full disassembly remains local with a byte/hash index; it is not decoded again for packaging. No private input/key/image/witness/proof bytes or executable/library blobs are embedded.

[Design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [new field method/tests](artifacts/root_product.rs), [source audit](artifacts/source-audit.json), [review](artifacts/review.json), [identities](artifacts/identities.json), [codegen](artifacts/codegen.json), [OFF bodies](artifacts/selected-codegen-OFF.json), [ON bodies](artifacts/selected-codegen-ON.json), [native plan](artifacts/build-plan.json), [pins](artifacts/build-pins.json), [source deltas](artifacts/source-deltas.json.gz), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [full-decode index](artifacts/full-disassembly-index.json), [publication](artifacts/publication.json), [checksums](SHA256SUMS).
