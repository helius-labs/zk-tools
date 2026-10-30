# Radix-13 VNNI instruction-budget screen

30 October 2026. **The proposed direct-convolution recipe is rejected before field-kernel implementation.** Its optimistic multiplication-instruction bounds are 210 ordinary / 310 fused per eight field elements, versus 100 / 150 IFMAs in the accepted cancel-carry CIOS recipe. AMD lists equal multiplication throughput and latency for these instructions on Zen 5. This is an engineering screen of one recipe, not a measured runtime regression or a general impossibility result for VNNI.

## Recipe and accounting

The idea uses 20 radix-8192 digits, with Montgomery radix R=2^260. A signed-word dot instruction produces 16 dwords, each accumulating two products. It could process eight independent field elements and two coefficient positions per vector, subject to packing and accumulation limits. The proposed reduction forms T, computes the low product q=T*(-p^-1) mod R, and computes only the upper portion of q*p. The carry shortcut overlaps the previously studied [truncated REDC mechanism](../2026-10-24-truncated-redc-component/README.md); it is not a new discovery here.

|Direct-convolution work per field|Ordinary|Two-product sum|
|---|---:|---:|
|Full raw product(s)|400|800|
|Low triangular product for q|210|210|
|q*p columns 18 through 38|229|229|
|Total word products|839|1,239|
|Optimistic eight-field vector instruction bound|210|310|
|Bound with odd per-coefficient terms padded|220|315|
|Accepted cancel-carry CIOS IFMAs|100|150|

The first bound grants perfect use of all 32 word-product slots per instruction, even across coefficient boundaries. The padded bound additionally charges one unused product for each odd coefficient length, still granting ideal placement of coefficient work into vectors. Neither is an emitted kernel instruction count. Carries, shuffles, widening, normalization, register pressure and R256/R260 conversion are excluded, making the screen optimistic. Other multiplication algorithms could have different counts.

The accepted count is five CIOS rows, each with ten raw-product IFMAs (twenty for fused), one cancellation-factor IFMA and nine reduction IFMAs using the accepted cancel-carry shortcut. Its source and enabled flag are pinned. The prospectively declared rule rejects this recipe if both optimistic bounds reach or exceed the equal-throughput accepted instruction counts. Both do. No field implementation or timing follows.

## Hardware and semantic check

The [AMD Zen 5 optimization guide, revision 1.00](https://docs.amd.com/v/u/en-US/58455_1.00) supplies an instruction-latency workbook. The relevant entries are:

|Instruction|Latency|Throughput column|Execution unit|
|---|---:|---:|---|
|VPMADD52LUQ|4|2|FP0/1|
|VPMADD52HUQ|4|2|FP0/1|
|VPDPWSSD|4|2|FP0/1|

The comparison uses equality of the published characteristics, without predicting complete-kernel cycles from them. Original ZIP, workbook and XML hashes are retained; full vendor documents stay local. [Extracted facts](artifacts/instruction-rows.json), [extractor](artifacts/extract.rs), [source hashes](artifacts/pins.json).

On this Ryzen 9950X, runtime feature detection confirms AVX512F and AVX512VNNI. Native VPDPWSSD outputs agree with scalar i64/wrapping-i32 arithmetic for 1,044 vectors / 16,704 dwords, including signed extremes and overflow. An additional 16-dword check accumulates forty maximum radix-13 products: 2,683,699,240, interpreted as unsigned bits after signed accumulation. Twenty such products total 1,341,849,620 and fit signed i32. These are instruction semantics and coefficient-capacity checks, not a Montgomery or field-arithmetic qualification. Disassembly confirms a ZMM VPDPWSSD instruction in the native helper.

The [recent multi-limb matrix-multiplication paper](https://www.texmacs.org/joris/amx/amx-v2.html) is useful background on hardware dot products, IFMA and packing. Its matrix results are not a BN254 proof-speed prediction. No source code is imported from it.

## Validation and outcome

The one-step Rust/xtask workflow completes once. All 191 final source/helper/vendor pins, including all 175 accepted production files, close. Product totals also match independent triangular-count formulas. No prover source, field representation, key format or retained memory structure changes. There are zero new timing samples, witness solves, SDK requests or Gnark exports; the distinct-proof ledger remains **25,231**.

The first XML reader rejected an attributed text tag before producing output; its initial source is retained. The repaired reader's successful extraction is reused. The first workflow plan omitted its required version and was rejected before any step ran. Original source, pins, plan and failure log are retained alongside the corrected plan and terminal receipt. No successful ISA or proof execution is repeated. [Failure notes](artifacts/failures.md), [protocol](artifacts/design.md), [workflow and receipts](artifacts/receipts.json.gz), [review](artifacts/publication-review.json).

This eliminates one speculative instruction-set change from the next implementation choices. It does not establish VNNI as universally unhelpful, nor does it qualify the proposed REDC carry formula. Further optimization should remove work across the actual proof path with a favorable arithmetic and dependency budget before full-proof capture.

## Accepted comparisons remain unchanged

Local 32-worker proof latency/RSS remains as below. CPU and Metal timings are supplied historical M5 references, not matched-hardware captures. Speedup is reference/Zopro latency; no new Apple measurement is added.

|Circuit|Zopro ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|134.00|37|4.612×|24|2.992×|
|2x2|11.335969|275.00|59|5.205×|30|2.646×|
|4x2|15.945544|315.00|109|6.836×|48|3.010×|
|8x2|24.090485|512.50|189|7.845×|74|3.072×|
|12x2|33.844855|707.25|284|8.391×|101|2.984×|
|16x2|39.931557|826.25|335|8.389×|124|3.105×|
|24x2|66.233309|1328.75|528|7.972×|176|2.657×|
|merge-24x1|65.273224|1175.50|507|7.767×|175|2.681×|
|merge-54x1|162.250464|1831.87|—|—|—|—|

The CPU 10× objective remains unmet on all eight reference shapes. RSS is observed lifetime process peak, not minimum RAM. Latest measured c7a.24xlarge primary remains Zopro 11.915591 ms at 32 workers versus Aeglos Sorted 7.959184 ms at 96; this study adds no AWS or Aeglos measurement. The existing accepted-build calibration exception remains documented in the [complete snapshot](../2026-10-30-current-bench-snapshot175/README.md), together with the full AWS matrix and measurement limits. No gain for mobile/ARM/TEE, huge circuits, memory pressure or concurrent throughput follows from this screen.

[Analysis](artifacts/analysis.json), [native helper and accounting](artifacts/screen.rs), [exact accepted rows](accepted-rows.json), [checksums](SHA256SUMS).
