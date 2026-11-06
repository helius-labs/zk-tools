# Shared inverse half-root export

**Closed before timing; not adopted.** The new inverse output checks pass, but generated code shows that the original already exports only the two required roots. Splitting the export through the four-lane halves repeats reduction/packing work rather than removing six materialized host fields. No proving-time gain or measured slowdown is claimed.

The half-root export candidate is closed before timing and is not adopted. Correctness checks pass, but actual final-consumer code disproves its intended work-removal premise: the original eight-lane get already materializes only the two required host roots after optimization.

In fresh OFF, those roots occupy two 32-byte stores at stack offsets 0x30 and 0x50 before the scalar root multiplication. ON writes the same 64-byte payload through four 16-byte stores, performs two four-lane canonical reductions instead of one eight-lane reduction, and adds surrounding packing work. The matched inverse function grows from 1,152 to 1,195 static instruction rows, YMM operand rows from 15 to 62, RSP operand rows from 85 to 93 and unpack rows from two to eight. Both have 500 IFMA sites and the same 0x438-byte explicit stack adjustment. Counts are static code observations, not dynamic costs, full stack peaks or a measured slowdown.

Both final complete-caller binaries link successfully with native/O3/fatLTO/one-CGU flags. Their direct G1/G2 kernel instruction-site counts remain 4,435/8,600 and IFMA sites 800/1,450 in both builds; this is not whole-function machine-code equivalence. The shared inverse is changed. The field crate, its exports/arithmetic, point formulas, allocation paths and existing workspace forecasts are unchanged.

One new focused native test passes: 1,025 product-chain cases check full-width values, lazy products and boundary patterns; 8,200 output inverse lanes match the unchanged scalar arithmetic oracle and multiply back to one, while 2,050 half-root exports match the corresponding original lanes. The actual summary is one passed, 125 original tests filtered. These are new cross-representation/scalar-oracle checks, not new independent Arkworks cases. Original field/core suites inherit the unchanged accepted qualification and were not rerun. Complete G1/G2 caller drivers were linked, not executed or newly curve-qualified.

The isolated 178-file snapshot retains 173 accepted files exactly; reversing one inverse-block edit and removing the three diagnostic appendices recovers all 176 originals. Two diagnostic children hold the unchanged caller bodies and the focused test. Reuse of the existing frozen caller was audited against its exact hash, 174 exact originals plus two append-only parents, matching adapters/driver and original optimized link. All six new build/test/link commands passed on first execution. The separate source/receipt/identity reviewer passes 428 pins and decodes each actual final consumer once.

No clock observation, PMU/profile capture, SDK request, proof export, Gnark verification or new AWS/device/resource result follows. No gain or regression is inferred from static counts. This stops the specific half-root route without an alignment, attribute, layout or width sweep, and does not establish that every specialized root-product boundary is unhelpful. Accepted production176/SDK6d83, local proof timings, inherited RSS and the 30,846-proof ledger remain unchanged. The full all-eight CPU10× goal stays active and unmet.


## Exact change and preparation

Native FqBatch::inverse_nonzero keeps its existing pair/quad product tree. OFF calls quads.get and uses lanes 0/4. ON uses the existing four-lane low/high halves, calls get()[0] for each, and performs the same scalar root product/inverse and reverse multiplications. The accepted field library and both export implementations are reused unchanged. The portable/nonIFMA body is unchanged and is not newly phone-qualified. No table, key/image/plan copy, pool, runtime memory cap or new production allocation is introduced.

This is distinct from the [direct inverse finish](../2026-10-28-inverse-finish-sdk/README.md), [whole-array word export](../2026-11-06-word-export176/README.md) and [LogJumps](../2026-10-31-ifma-logjumps176/README.md) studies; their failed or unqualified observations remain unchanged. The prototype tries to remove unused root materialization, not to change field reduction or inverse arithmetic.

Six Rust/xtask commands pass once: OFF/ON core builds, focused test build/run and OFF/ON complete-caller links. Test execution is one new test with 125 originals filtered, not a fresh full-core suite. It checks 1,025 lazy/full-width/pattern cases, 8,200 inverse lanes and 2,050 root exports against the unchanged scalar and representation oracles. These are not new independent Arkworks populations. The previous field qualification is inherited without rebuilding the field. Complete G1/G2 adapter bodies are copied exactly from their earlier checked callers; the new drivers are linked but not executed here.

The source/receipt/identity reviewer checks 428 pins, six command/exit/hash receipts, all 176 accepted source recovery, immutable library/consumer identities and actual native/O3/fatLTO/oneCGU flags. The two actual final consumers are decoded once after preparation. No clock observation, profiling, SDK proof or additional capture is selected.

## Actual final-consumer code

Counts include the saved instruction rows and operands; they are not dynamic execution counts, measured spills or cycle costs.

|Function|Arm|Instruction rows|IFMA sites|RSP operand rows|YMM operand rows|ZMM operand rows|Unpack rows|
|---|---|---:|---:|---:|---:|---:|---:|
|000000000004c920 <<zopro_core::simd::FqBatch>::inverse_nonzero>|OFF|1152|500|85|15|865|2|
|00000000000517b0 <zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>|OFF|4435|800|591|190|2064|48|
|000000000007d8d0 <zopro_core::msm::g2_borrowed::apply::<zopro_bn254::curve::G2Config, true>>|OFF|8600|1450|1469|299|3711|72|
|000000000004c960 <<zopro_core::simd::FqBatch>::inverse_nonzero>|ON|1195|500|93|62|832|8|
|00000000000518d0 <zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>|ON|4435|800|591|190|2064|48|
|000000000007d9f0 <zopro_core::msm::g2_borrowed::apply::<zopro_bn254::curve::G2Config, true>>|ON|8600|1450|1469|299|3711|72|

The root-export reader inspects the saved code between the final forward IFMA and the first scalar root-product multiply. OFF uses two 32-byte root stores; ON uses four 16-byte stores. Both materialize 64 bytes, not 256. The exact instructions, consumer hashes, scalar product/call positions and 0x438-byte explicit frame adjustment are retained in the observation artifact. This is a source-premise rejection, not a timing gate or proof that instruction count alone predicts speed. No alignment/attribute/layout/width sweep follows.

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

The [next question](artifacts/next.md) is whether canonicalization can be deferred across the root product itself, subject to an exact range/implementation/history audit. It is not yet an implemented or qualified candidate. The current half-root route stays closed.

Publisher checks all embedded archive hashes and lossless roundtrips, source deltas, preserved command and stream bindings, and root-store payload observations without repeating native builds/tests or decodes. Full snapshots/binaries and unselected full disassembly stay local with their saved hashes. Git contains all selected inverse/G1/G2 code, source bindings/deltas, tests and receipts. No private key, image, witness, point/scalar cache, proof or executable/library bytes are embedded.

[Design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [source audit](artifacts/source-audit.json), [review](artifacts/review.json), [test](artifacts/tests.rs), [identities](artifacts/identities.json), [codegen](artifacts/codegen.json), [root export observation](artifacts/root-export-observation.json), [OFF bodies](artifacts/selected-codegen-OFF.json), [ON bodies](artifacts/selected-codegen-ON.json), [native plan](artifacts/build-plan.json), [pins](artifacts/build-pins.json), [source deltas](artifacts/source-deltas.json.gz), [helpers and receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS).
