# Native fixed-factor truncated multiplication

**The fixed-factor arithmetic passes checked SIMD and normal release wrapper qualification. Speed remains unmeasured.** Generated code contains the predicted 76 IFMAs and five VPMULLQs, but the Fr/R260 butterfly grows from 266 to 310 instruction rows. The next step is actual bounded FFT-consumer integration and validation. No optimization is adopted; accepted local 1×2 full-proof latency remains **7.857170 ms**.

This implements the [qualified integer recipe](../2026-11-09-fixed-factor-trunc176/README.md). Given normalized raw x≤2p and a fixed ordinary canonical w<p, it prepares Q=floor(w·2^260/p), uses truncated high/low products, and returns x·w modulo p below 2p. The input's R256 or R260 radix is preserved. Original field, CIOS and generator bodies remain exact; one private field child and one append-only export parent recover all 176 accepted sources. [Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [native review](artifacts/review.json).

## Preparation and native checks

An 80-byte scalar factor stores ten radix-52 digits for ordinary w and Q. Preparation rejects noncanonical factors and computes the quotient with fixed arrays and 260 division bits using existing four-word primitives. It adds no production heap allocation or BigUint dependency. The expanded eight-factor type holds 640 bytes; this is payload, not a stack/RSS or live-register bound.

Four serialized commands succeed on their first execution: optimized checked field build, oracle link, four new unfiltered native tests, and generic noASM/noIFMA field compilation. The generic build checks selector isolation; it is not ARM/mobile execution. The checked field retains debug assertions and overflow checks.

Each Fq/Fr × R256/R260 configuration checks 6,432 rows: 72 input edges ×68 factor edges, 1,024 full-width random rows and 512 output-fed rows. There are 51,456 changing-lane outputs and 3,032 broadcast outputs per configuration, totaling **217,952 output comparisons**. Factor preparation is checked against independent BigUint division; raw result representatives against full integer arithmetic; canonical exports against Arkworks and the original Montgomery-product control. Inclusive 2p, digit/modulus boundaries, changing factors and invalid-factor rejection are included. Counts are comparisons, not distinct points or proofs. The earlier million-case integer model is not replayed.

## Normal release consumers

Five further serialized commands build the exact field source with debug assertions and overflow checks disabled, omit diagnostic cfgs, link four new normal release oracle tests, execute them, then list and decode the same tested ELF. All five commands pass on their first execution.

The release tests add **98,304 field-output comparisons** across both fields and radices. They exercise original and new multiplication/butterfly wrappers, native lazy output-fed chains, full-width changing factors and zero/one/minus-one factors against independent Arkworks results. These are arithmetic and two-point butterfly consumers, **not complete FFT transforms**. No root-cache preparation, full transform scheduling, allocation admission or proof is qualified by them.

Sixteen named arithmetic wrappers resolve to twelve distinct instruction ranges because the compiler aliases some identical functions. Counts below are the same for Fq and Fr. [Release identities and codegen review](artifacts/codegen-review.json).

|Consumer|Old rows, bytes|New rows, bytes|IFMAs old/new|VPMULLQ old/new|Stack-operand rows old/new|
|---|---:|---:|---:|---:|---:|
|R256 multiplication|191, 1193|215, 1319|100 /76|0 /5|6 /6|
|R260 multiplication|171, 1052|215, 1319|100 /76|0 /5|6 /6|
|R256 butterfly|286, 1835|310, 1961|100 /76|0 /5|18 /18|
|R260 butterfly|266, 1693|310, 1961|100 /76|0 /5|18 /18|

All named wrappers have zero call sites. Stack-operand rows do not measure spills or peak stack. Instruction sites do not establish cycles, throughput or proof latency. The saved IFMAs coexist with more total instructions, so no performance conclusion or timing selection follows from this table alone.

## Integration scope and review

The provisional FFT plan uses only the bottom seven scalar stages: five across quads plus two within. Compact factor payload remains bounded at 31,680 bytes for three directions if original roots are retained. Storing pre-expanded eight-lane factors would double that amount. Actual capacities, metadata, setup work and admission must be charged; this is not an implemented cache, RAM cap or minimum-RAM claim. No whole-domain quotient table, copied key, retained solver plan, new pool or C/ASM rewrite is added.

The existing production Euler/product path uses `stored_transform` and bounded `dif_serial`/`dit_serial` leaves. A generic quotient test may use older transform paths, so the next integration must prove that the intended production consumers use the new arithmetic. It must preserve higher-stage fallback, ordinary/twisted roots, table release/rebuild, resource denial and threaded behavior before a frozen matched performance trial. All-eight plus merge54 SDK/Gnark/resource/portable/concurrency checks remain required for adoption. [Continuation](artifacts/next.md).

Separate readers pass on their first compile/execution: 192 native pins/four receipts and 199 release pins/five receipts, with source recovery, test/output cardinalities, flags, actual ELF identity and all named IFMA counts checked. The pin sets overlap and are not unique-input totals. No failures or repairs occur in this native/release stage; the parent integer screen retains its earlier failures. These finite checks are not a formal machine-code proof.

## Accepted CPU and Metal comparison, inherited

These unchanged local Ryzen 9950X full-proof observations use 32 workers and common fixed placement. Supplied M5 CPU/Metal references use different hardware. None is a new measurement from this qualification stage.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

Production176/SDK6d83 and the 40,154-verified-proof ledger remain unchanged. Goal active and unmet, 0/8. No new timing, profile, complete FFT/core/prover qualification, SDK request, proof, Gnark export, AWS/Aeglos, mobile/ARM, huge-circuit, memory-pressure, swap, minimum-RAM or throughput result is added. No job or owned cloud resource remains live.

[Source and receipt archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
