# Joint X-coordinate and Y-delta qualification

**The new joint endpoint arithmetic passes native field and independent curve checks. Speed is unmeasured.** It computes X and the following Y delta together from the squared slope, removing the source dependency on an already normalized X. Original multiplication/cancellation bodies stay unchanged. This is private qualification, not adoption or a full-proof improvement. Accepted full-proof1×2 remains **7.857170ms**; the CPU10× goal remains unmet.

The [consumer-work audit](../2026-11-09-consumer-work-audit176/README.md) showed why a field-product proxy is incomplete. This proposal changes endpoint dataflow without reducing multiplication count. Its [design](artifacts/design.md), [exact child](artifacts/field.rs), [range/native review](artifacts/review.json), [codegen review](artifacts/codegen-review.json) and [curve review](artifacts/curve-review.json) preserve the scope.

## Fixed arithmetic

Use mixedR256 and normalized radix52 digits. For numerical0≤S<2p and canonical anchors0≤A,B<p:

    X0 = S + 2p - A - B
    D0 = 4p + A - X0 = 2p + 2A + B - S
    X0 + D0 = 4p + A

Here S is a normalized squared-slope component. X0<4p and D0<5p. One existing conditional subtraction of2p gives X<2p and an opaque Delta3 value D<3p. Both are congruent to the original X and A−X modulo p. Each uncarried X coefficient also forms the corresponding D coefficient; two separate signed carry chains then normalize them. This avoids waiting for complete X normalization before beginning D.

The opaque delta is consumed only by new bounded finishing methods. It cannot be exported as an ordinary coordinate. For G1, D·slope has T<6p², hence its Montgomery output is<3p; one existing2p correction restores<2p. For G2, the real numerator uses D0·s0+(2p−s1)·D1, complementing the slope rather than the wide delta. Imaginary uses D0·s1+D1·s0. Each T<12p² and4p<R imply Montgomery output<4p, so the original fused correction restores<2p. No raw multiplication or Montgomery-cancellation body is rewritten.

Coefficient extrema give signed carries in[-2,1]forX and[-2,3]forD, comfortably within signed64 bounds. Before partial correction the complete integers fit260bits; afterward the returned X/delta fit256bits. The separate reader checks the symbolic identity, field bounds and128local carry extrema. Finite native checks complement these arguments; no formal machine-code proof is claimed.

The old [loose-delta SDK trial](../2026-10-30-loose-delta-sdk175/README.md) formed D after X and changed only the finishing product. The old [weighted-anchor trial](../2026-10-28-g1-weighted-anchor-sdk/README.md) prepared2A+B forward. The [affine-offset method](../2026-11-08-affine-redc-offset176/README.md) fused offsets into product normalization. Their fixed negative results remain closed. This jointly formed, partially reduced endpoint pair is a different local dataflow; no mathematical novelty is claimed.

History checks found existing carry-prefix, layout, GLV, alternate-field, exact-FMA and point-formula trials; none was rerun. Four inspected accepted G1 hotspot addresses resolved to function-level synthetic CGU locations with unknown source lines, so no broader DWARF or source-line attribution is claimed. New paper searches selected no imported body or transferred speedup.

## Native qualification

Four serialized Rust1.98.1/xtask commands pass first execution: optimized field build with debug assertions/overflow checks, oracle link, two new unfiltered tests and generic noASM/noIFMA compile. The two tests check **623,616 coordinate outputs** and **730,560 finishing-product outputs**, totaling **1,354,176**, across Fq and Fr.

Each prime uses68canonical-anchor,71square and74delta edges,4,096full-width random rows and1,024output-fed rows. It checks normalized digits, exact numerical X/D representatives, range bounds, original formulas and independent Arkworks/BigInt arithmetic. A separate full-wide-delta finishing population crosses74×74edges. Each prime includes1,640joint outputs with D≥2p, exercising the opaque wider range. Counts include repeated/padded lanes and are not cryptographic proof counts.

Two additional commands build and run one new unfiltered curve test against independent Arkworks complete addition: **2,048G1+2,048G2 point comparisons**,12,288base-field coordinate comparisons, including512zero-delta cases P+(−2P)=−P. Native original square formulas feed the joint endpoints and final Ay subtraction. Sixty-four full-width-scalar points per curve are reused across256sign masks; the comparison count is not a count of unique input points.

Only ordinary nonidentity/distinct-X inputs reach these endpoint tests. Prefix/inversion integration, dispatch exceptions, scalar tails, complete caller scratch reuse and worker stacks remain unqualified for this new path. No old model/test population is replayed. The qualified field is reused unchanged for the curve and codegen stages. The generic compile is not ARM/mobile execution.

## Emitted endpoint code, not timing

Three further commands build one native/fatLTO/oneCGU consumer, list symbols and decode it once. Both wrappers and reused field retain debug assertions and overflow checks. Four endpoint functions are retained by their addresses; the consumer is not executed. Input packing, prefix/inversion, dispatch, signs/tails and complete-core behavior are outside this codegen scope.

|Endpoint|Instruction rows, old/new|Bytes, old/new|IFMA sites, old/new|RSP/RBP operand rows, old/new|
|---|---:|---:|---:|---:|
|G1|359 /395|2301 /2571|100 /100|24 /36|
|G2|987 /919|6355 /6086|300 /300|120 /96|

G2 has fewer static instruction rows; G1 has more. Both candidates retain two cold assertion-failure call sites, versus none in the originals. G1's wider delta also requires an extra correction after its ordinary Y product; G2's existing fused correction already covers its wider operands. These facts do not isolate the source of every instruction difference.

Static sites/stack operands do not measure dynamic work, spills, stack peaks, cycles or full-proof latency. This checked artifact is not assigned a release performance result. Current endpoint counts alone do not select timing or a favorable curve subset. Next is coherent complete-caller qualification and actual consumer review; a normal release artifact requires explicit identity and correctness checks before any frozen matched measurement.

## Source, memory and remaining work

The private snapshot has177source entries:175original entries exact, one append-only field parent and one child. Removing the suffix recovers all176accepted originals. All production files, the generator and original field methods stay exact. Delta3 has320bytes of native vector payload, equal to one existing eight-lane field value. This is not a stack/RSS/minimum-RAM bound. No helper heap buffer, lookup table, copied key, retained plan, production pool or synthetic cap is added.

The range/source reader checks187pins and four receipts; the codegen reader checks193pins and three receipts; the curve reader checks196pins and two receipts. These pin sets overlap and must not be added as unique inputs. All nine native/codegen/curve commands pass first execution. An initial offline reader compile failed on a redundant BigInt borrow; the one-token repair, original reader and failure record remain. Scientific source, bounds, builds and tests are unchanged and not repeated.

The next core integration must preserve original OFF bodies, complete exceptions and tails, resource accounting, immutable inputs and1/4/16/32-worker small-stack behavior. Only qualified complete consumers can justify a frozen timing/profile trial, followed by all-eight plus merge54 SDK/Gnark/resource/portable/concurrency checks before adoption. No performance gate passes here.

## Accepted CPU and Metal comparison, inherited

These are unchanged local9950X full-proof measurements with32workers and common fixed placement, compared with the supplied M5 references on different hardware. They are not endpoint timings or new measurements.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

Production176/SDK6d83 and the40,154verified-proof ledger are unchanged. Goal active and unmet,0/8. No new timing, profile, SDK request, Gnark export, proof, AWS/Aeglos, mobile/ARM, huge-circuit, pressure/swap, minimum-RAM or throughput result is added. No job or cloud resource remains live.

[Continuation](artifacts/next.md), [reader repair](artifacts/review-compile-repair.json), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
