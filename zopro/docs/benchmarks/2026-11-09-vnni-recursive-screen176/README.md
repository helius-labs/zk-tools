# Recursive-convolution VNNI work screen

**None of these four fixed arithmetic recipes selects a native field implementation.** Even the closest optimistic G2 packet requires 2,952 multiply instructions versus the accepted 2,900, before packing, carry or interpolation costs. Its evaluated words also exceed the native operand range. This is a recipe work/range screen, **not a measured slowdown or a lower bound on all VNNI algorithms**. Production and accepted proof performance remain unchanged; the full all-eight CPU10× objective remains active and unmet, 0/8.

The [earlier VNNI screen](../2026-10-30-vnni13-screen175/README.md) analyzed direct schoolbook convolution and left other multiplication algorithms open. This study tests recursive Karatsuba and Toom4, rather than replaying that recipe or its ISA/calibration cases. It answers whether their lower raw multiplication counts survive complete Montgomery reduction and the actual point-operation mixture.

## Fixed recipes and complete work

Karatsuba splits K16 as16→8→4, K20 as20→10→5, and K24 as24→12→6→3. Toom4 uses four six-digit blocks with evaluations at0,1,−1,2,−2,1/2,∞; the half evaluation scales inputs by8 and products by64. A fixed exact rational matrix reconstructs all seven product blocks. Signed integer coefficients and exact divisibility are retained.

Every recipe computes a full convolution T, then a full convolution of T mod R with −p⁻¹ mod R to obtain q mod R, and a full q·p convolution before division by R. Ordinary products therefore use three convolutions; fused sum/difference uses four. No unimplemented truncated recursive multiplication, free quotient or uncharged alternative reduction is treated as established.

The instruction-capacity column grants perfect use of all32 signed-word product slots per VNNI instruction across16 field elements. The literal-pairing column instead groups two terms per coefficient, padding odd terms. Both exclude carries, normalization, interpolation, conversions, masking, packing, register pressure and overflow repair. Invalid word ranges remain invalid even when their optimistic counts are shown.

|Recipe|Digits × bits|Word products/convolution|Ideal ordinary / fused, 16 fields|Literal paired ordinary / fused, 16 fields|
|---|---:|---:|---:|---:|
|K16|16×16|144|216 / 288|270 / 360|
|K20|20×13|225|338 / 450|405 / 540|
|K24|24×11|243|365 / 486|486 / 648|
|T24|24×11|252|378 / 504|441 / 588|

Accepted eight-lane IFMA costs100/150 for eight fields, hence200/300 for16. K16's ideal fused-only288 looks favorable against300, but its ordinary216 exceeds200. The source-derived full-group [point ledger](../2026-11-09-pmns-point-bridge176/README.md) uses six ordinary products for G1 and seven ordinary plus five fused operations for G2. Applying that unchanged mixture:

|Recipe|Ideal 16-point G1 packet|Ideal 16-point G2 packet|
|---|---:|---:|
|Accepted IFMA|1,200|2,900|
|K16|1296|2952|
|K20|2025|4613|
|K24|2187|4982|
|T24|2268|5166|

These are mathematical slot-capacity grants for explicit recipes, not emitted native field counts or cycle estimates. The old audited [AMD Zen5 workbook](https://docs.amd.com/v/u/en-US/58455_1.00) assigns the same published multiply throughput/latency entries to VPDPWSSD and IFMA. Its exact local file and extracted rows are hash-pinned; no new hardware calibration is claimed. The [existing local IFMA calibration](../2026-11-06-instruction-calibration176/README.md) is inherited. Complete native costs can differ further.

## Word ranges matter

|Recipe|Maximum evaluated word bound|Fits signed16|Central leaf-coefficient bound|Fits signed32|
|---|---:|---|---:|---|
|K16|262140|false|274869518400|false|
|K20|32764|true|5367398480|false|
|K24|16376|true|804520128|true|
|T24|30705|true|5656782150|false|

Canonical input2²⁰⁸−1, valid for both fields, produces K16 evaluated word262,140, beyond signed16. Toom4 has a concrete coefficient overflow even though its input words fit: for canonical2²⁵³−1, the central coefficient of the scaled-half leaf is **5,531,075,880**. All16 native DPWSSD lanes return the expected wrapped signed32 result **1,236,108,584**, which is not the required wide coefficient. K24's local word/leaf bounds fit, but this does not qualify an entire native Montgomery implementation.

Three new evaluated-word populations check1,024 vectors×16 lanes each, with(maxword,terms)=(16,376,3),(32,764,5),(30,705,6). Wide signed32 overflow counts are0/147/114. The additional16-lane canonical counterexample gives49,168 native lane comparisons. All wrapped outputs match exact scalar signed64 sums; overflow is retained as failure of a narrow-coefficient assumption, never promoted as exact field arithmetic.

## Arithmetic checks and review

Both Fq and Fr run256 boundary tuples and512 full-width deterministic random tuples per recipe, covering ordinary product, fused sum and fused difference. **18,432 new field outputs** and **67,584 constituent convolutions** pass. Every convolution evaluates to the independent BigInt integer product. Montgomery quotients use extended Euclid for −p⁻¹ mod R; the separate residue oracle uses Fermat inversion of R modulo p. Signed division, exact cancellation and output range[0,2p) are checked. The range inequalities prove one sign/correction step suffices for these radices; finite checks are not an exhaustive native-field proof.

The separately written same-agent reader reconstructs recursive leaf counts, both instruction ledgers, complete point mixtures, word/leaf bounds and the two explicit canonical counterexamples. It verifies the Toom interpolation identity on all16 free block monomials, without replaying the finite tuple population. All176 production files remain exact. The 189-pin workflow, model, one actual dot-helper decode and reader finish successfully on their first execution. The decoded helper contains VPDPWSSD; it is a word-test helper, not a field kernel. Shared BigInt arithmetic and ISA interpretation limit independence; this is not an external audit.

Two initial manual helper compilations fail before model execution: a redundant BigInt reference in a producer comparison and an ambiguous reader BigInt shift/subtraction expression. Reference/type annotations alone repair them; original sources are retained. No runtime arithmetic, case population or gate is changed, and no successful old suite, oracle or decoder is replayed. The helper compiler calls have tool-transcript evidence rather than workflow receipts.

There is no new proof, SDK/Gnark export, caller timing, profile, AWS resource, peak-RSS, minimum-RAM, pressure, swap, phone or TEE qualification. Model/BigInt and intrinsic-test allocations are not prover memory. No table, key/point/scalar copy, retained plan, production pool, C or assembly implementation is introduced. Canonical proof inventory stays37,633. No radix/interpolation/packing/truncation rescue or native caller is selected for these four recipes.

## Accepted CPU/Metal comparison, unchanged

These are inherited from the [latest accepted-SDK observations](../2026-11-09-query-neutralization-sdk176/README.md), not timings of this screen. Its primary and4×2 observations were fresh in that study; the other six came from its prior full-shape campaign. Supplied M5 references use different hardware.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1x2|8.031249|37 / 4.607×|24 / 2.988×|3.7|
|2x2|11.149097|59 / 5.292×|30 / 2.691×|5.9|
|4x2|15.961623|109 / 6.829×|48 / 3.007×|10.9|
|8x2|24.131178|189 / 7.832×|74 / 3.067×|18.9|
|12x2|33.940628|284 / 8.368×|101 / 2.976×|28.4|
|16x2|40.164083|335 / 8.341×|124 / 3.087×|33.5|
|24x2|66.187704|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.361571|507 / 7.757×|175 / 2.677×|50.7|

Merge54 remains160.890372 ms without an M5 reference. Historical same-build primary7.915095 ms remains valid; private7.859756/7.869400 ms candidates were unadopted. Latest8.031249 ms is another observation of the same accepted executable, with no adopted regression or established cause. The full verified all-eight CPU10× objective remains active and unmet,0/8.

[Analysis](artifacts/analysis.json), [review](artifacts/review.json), [design](artifacts/design.md), [model and native word helper](artifacts/screen.rs), [continuation loop](artifacts/next.md), [source/receipts/decoded helper](artifacts/evidence.json.gz), [checksums](SHA256SUMS). The archive has individual UTF8-member SHA256 hashes and verified gzip roundtrip. Binaries/libraries, original production source, full third-party guide/workbook and keys/witnesses/proofs/ledger payloads are excluded. Reproduction needs pinned dependencies and adapted absolute paths.
