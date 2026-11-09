# Shared-body unit-Z mixed-add integration

**The actual integrated method passes native/generic qualification, and matching OFF/ON SDKs are built. Performance is unmeasured; production is unchanged.** This follows the [real-proof eligibility census](../2026-11-09-unit-z-census176/README.md), which found primary ordinary unit-Z calls in39.05%of scalar G1 mixed adds and21.95%of G2. Main SIMD affine streams bypass this operation, so those fractions are not proof-time savings.

The candidate specializes coordinate preparation and final Z while sharing the original identity/exception checks and H/HH/I/J/r/V/X/Y arithmetic. Unit inputs skip three multiplications and two squares, using the standard4M+2S instead of7M+4S path. Nonunit inputs retain the original projection expressions and finalZ. The private OFF definition has literal original method tokens. Only field curve.rs changes among176 accepted files;175 remain byte-exact. No field multiplication/inverse/IFMA, solver/FFT, queue, geometry, bitmap reset, allocator, typed forecast, buffer, table, key copy, plan, pool or memory cap is added or changed.

The first direct integration qualified, but emitted two arithmetic bodies and grew about59%in code size. Before any timing, the shared-body implementation was qualified separately. Actual native qualification executables decode as follows, including alignment padding:

|Curve|Original bytes|Direct unit-Z bytes|Shared-body bytes|Original / direct / shared MULX sites|
|---|---:|---:|---:|---:|
|G1|12,960|20,544|13,248|352 /544 /352|
|G2|36,384|57,904|37,280|928 /1440 /928|

The shared body retains original products once in conditional/common blocks; unit execution skips the unnecessary blocks. Static sites, function sizes and stack operands are not executed instructions, cycles, stack peaks or proof speed. The standalone wrappers call the real method. OFF's body merges with the separately named original reference in its test executable. No compiler attributes, worker subset or threshold are selected from timing results; no clock has run.

All actual-method outputs match both a separately named copy of the exact original formula and independent Arkworks group addition. Reference adaptation changes only self/Self names and the generic function signature. Tests include exact Jacobian coordinates, affine/on-curve outputs, identity, same/opposite points, signs, unit/nonunit scalings, immutable inputs, changing chains and unusual identity representations. Point ABI sizes remain64/96/128/192bytes for G1affine/projective and G2affine/projective.

|Qualified artifact|Arkworks affine output executions|Additional chain transitions|Tests|
|---|---:|---:|---:|
|Native OFF|19,320|2,576|2|
|Native direct ON|19,320|2,576|2|
|Generic direct ON|11,520|1,536|1|
|Native shared ON|19,320|2,576|2|
|Generic shared ON|11,520|1,536|1|

These implementations/backends reuse the same argument population. Counts are executions against different artifacts, not a sum of distinct mathematical fixtures. Each native artifact also covers all53 actual broadcast workers across1/4/16/32 pools with requested512KiB stacks, plus four OS callers sharing an eight-worker pool. Generic execution is x86 without ASM/IFMA, not physical ARM/phone qualification. Original unaffected field/core/Window/prototype/census runs are inherited rather than repeated. The original OFF library/test/decode is reused after sharing; no unchanged OFF rebuild is performed.

The direct qualification workflow completes11commands; the shared version completes7new commands. Native/source readers check exact176-source recovery, the effective OFF mapping, literal shared arithmetic, pins, arguments/exits/stream hashes, raw test summaries, ABI checks and actual codegen. All18qualification commands pass first execution. Readers are separate helpers by the same agent, not external audits.

Eight matching native core/prover/zolana/SDK artifacts now link against their corresponding qualified fields, preserving the original fixed-placement driver, allocator/default stacks and nativeO3/fatLTO/oneCGU link. No qualification wrappers or census counters remain in release symbols. The initial SDK-OFF link lacked the qualified field's transitive dependency-search directory; three completed OFF libraries are reused. Appending only the matching field directory to the five remaining link commands fixes that build, without runtime source changes. Eight artifact builds succeed, and the failed link is retained. A link-reader helper's borrowed comparison needs a compile-only fix; its initial source and failed/missing-binary tool attempts are retained. Helper compiler/review/publisher invocations have tool-transcript evidence rather than complete xtask receipts.

Next is the complete matched all8+54/primary16/solo SDK experiment against the accepted bitmap-reset REF and fresh OFF, with automatic profiles and prospective readers. Source, fields and SDKs are ready; protocol/helper adaptation must finish before any preflight. No positive performance claim or production adoption follows from this qualification.

This stage adds **zero SDK requests, proof exports, latency captures or profiles**. Canonical inventory remains **41,803**. No new RSS/minimum-RAM, ARM/mobile/TEE, huge250/4999, pressure/swap, AWS/Aeglos or concurrent-throughput result is added. The following local accepted observations are inherited from [bitmap-reset adoption](../2026-11-09-busy-reset-adoption176/README.md); supplied M5 CPU/Metal figures are historical references on different hardware.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.809034|37 / 4.738x|24 / 3.073x|3.7|
|2x2|10.940974|59 / 5.393x|30 / 2.742x|5.9|
|4x2|15.750370|109 / 6.920x|48 / 3.048x|10.9|
|8x2|23.951790|189 / 7.891x|74 / 3.090x|18.9|
|12x2|33.857141|284 / 8.388x|101 / 2.983x|28.4|
|16x2|39.726526|335 / 8.433x|124 / 3.121x|33.5|
|24x2|65.984808|528 / 8.002x|176 / 2.667x|52.8|
|merge-24x1|65.660588|507 / 7.722x|175 / 2.665x|50.7|

Full verified all-eight CPU10x goal remains active and unmet, **0/8**. Primary accepted peak process RSS remains132.50MiB; merge54 remains163.167792ms without an M5 reference. The baseline was already below8ms; historical7.699867ms uses a different campaign/statistic.

[Qualification](artifacts/review-result.json), [direct phase](artifacts/direct-review.json), [source recovery](artifacts/source-audit.json), [coherent links](artifacts/link-review.json), [link repair](artifacts/link-repair.json), [design](artifacts/design.md), [continuation](artifacts/next.md), [source and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Full raw decodes/nm, unchanged full source snapshots, binaries/libraries and key/image/witness/proof payloads are omitted. Selected disassembly is archived; omitted raw text has local byte/hash indexes.
