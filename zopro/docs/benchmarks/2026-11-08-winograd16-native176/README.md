# DFT16 factorization and native packing qualification

**The new 16-point identity and its packed Rust implementation pass correctness qualification. Performance remains unmeasured.** The fixed field circuit uses 13 constant products and 74 add/sub operations per transform, versus 17 nonunit products and 64 add/subs for unit-pruned radix2. Two independent sequences pack into four eight-lane product calls instead of six in the original diagnostic control. Those are source counts, not an emitted-instruction count or measured speedup. Production remains unchanged.

This native qualification stage is complete. The next stage is an actual complete-caller protocol and codegen qualification, including both directions and all routing costs, before any performance recording. No SDK integration or adoption is selected. The original all-eight CPU10× objective remains active and unmet.

## Construction and distinction from prior work

After forming x[j]±x[j+8], the even frequencies use the existing local four-product DFT8 identity. The odd frequencies use three base products and two symmetric three-product rotations. All 13 products have a linear input stage; no product depends on another product. [Derivation](artifacts/derivation.md) gives every coefficient and output sign. Multiplication by the fourth root is charged as a dense Fr product. Root coefficients depend only on the primitive 16th root, not circuit or witness data. No novelty claim is made.

The [earlier DFT8 caller](../2026-10-29-winograd8-caller175/README.md) remains rejected at its measured gate. This is not a rerun or width extension of that complete kernel: the new odd half uses nine products rather than seven twiddles plus a separate four-product DFT8. The global DFT16 consequently uses 13 products rather than the composed 15. The [ordinary split-radix equality](../2026-11-08-split-radix176/README.md) also remains unchanged.

The [2022 Raciborski–Cariow paper](https://www.mdpi.com/2079-9292/11/9/1342) was a research lead. Its abstract and partial indexed equations were available; direct publisher requests returned 429 and the PDF was inaccessible. The full paper or its matrix factorization was not reproduced. This identity was independently derived from the DFT definition and symmetric rotations. No GPL implementation, paper performance or external source body is copied. The even-half expression reuses the already published local mathematical identity and local routing code.

## Mathematical checks

All eight primitive roots pass 4,248 fresh vector cases in total, comprising basis/boundary/fixed full-width inputs, plus 1,032 changing states. Independent Arkworks direct DFT checks both DIF natural→bit-reversed and DIT bit-reversed→natural interpretations. All 2,048 entries of the eight 16×16 matrices are checked. These are executed comparison counts, not a claim that repeated zero/unit/basis inputs are distinct random values. Matrix equality establishes the linear identity conditional on inherited field arithmetic.

The source counter returns exactly 13 dense products and 74 add/subs for every case. All 13 coefficients are nonzero and not ±1 for every tested primitive root. The necessary mathematical gate passes and selects native packing qualification only. The offline model reader checks 19 pins, both successful continuation receipts/stream hashes, the retained failed initial compile, all 176 exact accepted production hashes, type-only source repair, eight root-count rows and selection scope. It does not execute the field oracles again.

## Native implementation

The [kernel](artifacts/kernel.rs) uses the existing Ifma8<FrConfig> arithmetic. Its even-half product occupies one vector; the odd base/rotation products occupy three. There are 26 logical product lanes, 32 physical lanes and six explicit dummy lanes for two sequences. Four coefficient vectors have 1,280 bytes of raw payload per root. Control and candidate share the same immutable root owner; control roots and routing state are additional. No measured stack, process RSS or minimum-RAM claim follows.

DIF emits the original bit-reversed order. DIT explicitly permutes its input and output around the same factorization. **Both DIT permutations remain in the candidate and must be timed.** No cheaper transposed circuit, selected root or direction is substituted after observation. Extra add/subs, correction, permutations, stores, constant loads, compiler lifetimes and six dummy lanes can erase the saved product calls.

The original diagnostic control uses the exact accepted dif_within/dit_within fragment with the original two upper radix2 layers. It is not a timing of a complete accepted FFT or SDK. Both link field library 1200e635…, the exact accepted source176 artifact. New tests compile with Rust 1.98.1, -O,target-cpu=native,fat LTO and one codegen unit. Existing unchanged field/core/prover suites are inherited, not replayed.

Three new unfiltered native tests pass on their first execution:

- All eight roots, all 32 two-sequence basis vectors and both orders.
- Boundary/full-width values and 129 changing states per root, both orders.
- Immutable roots shared by 1/4/16/32 scoped workers with 512 KiB stacks, each checking 17 changing states per root and both orders.

Every native result, including the original control, matches independent Arkworks direct DFT. There are **1,617,920 native field-output comparisons**, including controls and repeated identities; they are not all unique random inputs. No test is filtered, ignored or reported as a full prover test. The transform source requests no heap allocation and introduces no unsafe code; timed allocation behavior is not yet measured. No new key copy/table, input-sized buffer, retained solver plan, pool or C/ASM rewrite is added. Portable/ARM/mobile or production-dispatch compatibility is not qualified by this native diagnostic.

The native reader checks 14 source pins, two successful command receipts/stream hashes, actual compiler flags, all three test names/counts, exact original within fragment, the four candidate vector-product source calls, retained DIT permutations and 176 exact production hashes. It does not disassemble a final caller or rerun native tests/model/oracles. [Qualification](artifacts/qualification.json).

Preparation failures remain explicit. The initial model compile needed an explicit [Fr;16] intermediate array type; only that annotation changes in the model before its first execution. The first repair-helper compile had an ownership error, fixed by borrowing its path; the failed helper never mutated the model. Its following missing-binary/missing-plan invocations ran no model command. Original sources and actual failed model-build streams remain archived. Repair-helper diagnostics are in the tool transcript without an original raw stderr file. The corrected two-step model continuation and two-step native workflow finish once; no successful test/build/capture is rerun for bookkeeping.

## Accepted proof comparison, inherited

No performance capture, CPU profile, SDK request or proof is added. All proof rows below are inherited from the [preceding report](../2026-11-08-split-radix176/README.md), with the latest primary plus seven earlier full-campaign rows. Supplied M5 timings use different hardware.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.002770|37.0|24.0|4.623×|2.999×|3.7|
|2x2|11.241463|59.0|30.0|5.248×|2.669×|5.9|
|4x2|15.933816|109.0|48.0|6.841×|3.012×|10.9|
|8x2|24.234311|189.0|74.0|7.799×|3.054×|18.9|
|12x2|33.855565|284.0|101.0|8.389×|2.983×|28.4|
|16x2|40.071563|335.0|124.0|8.360×|3.094×|33.5|
|24x2|65.917462|528.0|176.0|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507.0|175.0|7.764×|2.680×|50.7|

Accepted local 1×2 remains **about 8 ms**: same binary previously 7.915–7.929 and 7.995682 ms, latest 8.002770 ms. The inter-run cause remains unconfirmed, without a slower adopted replacement. Merge54 remains inherited 162.240109 ms without a supplied M5 reference. Production176/SDK6d83 and canonical 35,171 remain unchanged. The verified CPU10× goal remains active/unmet, **0/8**. No new AWS/Aeglos, huge-circuit, pressure/swap, mobile/ARM/TEE, peak/minimum-RAM or concurrent-throughput result is added.

Next: freeze and qualify the actual changing-state complete caller, original-only/fresh original/ON controls, all primitive roots, both orders and 1/16/32 workers, with automated profiles and actual emitted code. No timing plan is frozen or capture selected by this report. Only a passing full caller can select a separate real packed-FFT/Euler operator, followed by actual SDK/Gnark/resource/portable/all-shape evidence before adoption. This is incremental work; the primary 3.7 ms and all other targets remain required.

The [evidence archive](artifacts/evidence.json.gz) maps relative UTF8 paths to {utf8,bytes,sha256}, retaining full model/native rows, original/fixed helpers, plans/pins, failed model-build streams and successful exact receipts. Every extracted payload and archive roundtrip is verified. The public model result is byte-exact, without omitted rows. External source/full papers, libraries/binaries, keys/images/witness/proof payloads and private ledgers are excluded.
