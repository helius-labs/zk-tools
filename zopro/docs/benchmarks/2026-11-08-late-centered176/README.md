# Late-centered Montgomery arithmetic feasibility

**The signed affine arithmetic and a native-shaped unsigned-CIOS lowering pass their mathematical checks. Native execution and performance remain unmeasured; production is unchanged.** This selects one private native implementation/codegen qualification, not an SDK trial or adoption.

The hypothesis is to remove modular corrections between point-addition products by keeping signed numerical Montgomery intermediates within the complete prefix/reverse circuit. It does not reduce the source multiplication count. Unsigned IFMA sign compensation, normalization, final coordinate export and register pressure are additional costs and can erase the benefit. No proof-speed estimate follows from these models.

## Construction

With R=2²⁵⁶ or 2²⁶⁰, let q=(−T p⁻¹) mod R. Center only its final digit: replace q by q−R when q≥R/2. Then v=(T+qp)/R satisfies |v|≤|T|/R+p/2. Both primes obey 5p<R, so a raw ordinary or fused expression with |T|≤2p² returns |v|<p without a canonical correction. Earlier quotient digits remain ordinary unsigned digits; no per-row signed centering is introduced.

This follows the standard Montgomery congruence. [Montgomery 1985, sections 1–2](https://langevin.univ-tln.fr/cours/MLC/extra/montgomery.pdf) and [Hurchalla's positive-inverse discussion](https://jeffhurchalla.com/2022/04/28/montgomery-redc-using-the-positive-inverse-mod-r/) supply background; the latter's native-word performance is not transferred here. The local recipe uses additive negative-inverse cancellation. No external implementation body is ported and no novelty is claimed.

The mathematical vector representation has four normalized unsigned radix52 digits and a signed top digit. Signed intermediates persist within each batch, while host key and bucket coordinates remain canonical R256 between updates. This does not widen existing Fp/Ifma8/get/compact contracts. R260 is a representation cross-check, not a new coordinate-isomorphism or persistent recoded-key proposal.

For S=2²⁶⁰ and a=A−sₐS, b=B−sᵦS, unsigned and signed products have the same quotient modulo R. After unsigned CIOS, subtract (S/R)(sₐB+sᵦA); the remaining sₐsᵦ(S/R)S vanishes modulo S. Center the quotient and interpret the final five digits in (−S/2,S/2). The exact result's bound makes this signed decoding unique. Fused products compensate all four signs; a difference encodes −c and must pay its negation/normalization. [Full derivation and costs](artifacts/late-compensation.md).

This late compensation keeps the accepted unsigned six-coefficient CIOS schedule instead of retaining an eleven-coefficient signed convolution in registers. The integer lowering models the canceled-low-product carry, four 52-bit rows, final 48/52-bit row and mixed packing literally. Source IFMA counts remain 100 ordinary/150 fused. These are integer recipe counts, **not newly decoded machine instructions**. Up to 10/20 masked limb compensations, mixed-radix shifts, five centering corrections, signed normalization, exports and live operands remain additional work.

## Executed checks

The first scalar model uses exact signed limb products and cancellation coefficients. Independent full-radix Euclidean quotient arithmetic and Arkworks modular arithmetic check each primitive. Boundary representatives extend through ±5p; the narrower |v|<p closure condition applies only within its declared input bounds.

|Field / radix|Primitive outputs|Closure checks|Cancellation rows|Negative outputs|
|---|---:|---:|---:|---:|
|Fq / 256|7419|7023|37095|3648|
|Fr / 256|7419|7023|37095|3627|
|Fq / 260|7419|7023|37095|3581|
|Fr / 260|7419|7023|37095|3533|

Each field/radix has 13 signed edges crossed into 169 tuples, 2,048 deterministic signed full-width tuples, and 256 changing tuples. All three operations execute for each tuple: 7,419 outputs, 29,676 total. The 28,092 closure checks concern tuples inside the declared ±p domain.

Complete scalar G1/G2 models follow eight interleaved prefix chains and reverse recovery at lengths 8/16/200, positive/negative/mixed arrivals, three changing passes, both radices: 108 cases and 8,064 point-output comparisons. Expected curve outputs are generated once using independent Arkworks addition, then reused across the new radix/length cases; comparison counts are not all distinct random points. Every output matches. Canonical inverse-boundary arithmetic uses Fermat inversion, separately checked against Arkworks. This does **not** qualify the actual native eight-lane inversion bridge, scalar tails, queues or exceptional dispatch.

|Curve / radix|Point outputs|Signed reductions inside point models|Cancellation rows|
|---|---:|---:|---:|
|G1 / 256|2016|12096|60480|
|G1 / 260|2016|12096|60480|
|G2 / 256|2016|24192|120960|
|G2 / 260|2016|24192|120960|

The first model checks 102,252 reductions and 511,260 cancellation rows across primitive and point circuits, plus 2,187 exhaustive toy outputs over p=13, R=256 with two 4-bit steps. Its largest observed signed coefficient is 73,464,968,921,481,216. Exact per-operation signed64 assertions complement the source bounds; finite observations are not a native overflow proof.

The separately declared late-compensation helper is a **new lowering algorithm**, not a replay of the passed affine/oracle model. Thirteen signed edges and 1,024 new signed random tuples per configuration produce 14,316 exact outputs across both primes/radices and three operations. It checks the unsigned CIOS result, each quotient digit, equal signed/unsigned quotients, exact compensated representative, signed decoding and field residue. All pass. The largest observed coefficient is 284,287,637,318,467,192, below signed64. Its 100/150 IFMA recipe count is checked on every output.

Conservative graph bounds give G1 |X|,|Y|<3p, and G2 |X|,|Y|<4p. Intermediate finishing differences reach 4p/5p respectively. Final export therefore costs more than the ordinary <2p boundary and remains mandatory after every update. Signed root normalization is also charged at the inversion bridge. The original point work stays 6M per G1 group and 7M+5F per G2 group, plus its original inverse bridge. There is no claimed operation-count or timing gain from these unchanged products.

## Provenance and limits

The original first build succeeds; its first raw −p×−p check then fails before any REDC completes. The both-negative product term was incorrectly placed one radix position too high. The exact source and binary identity, failed exit 101 and stderr are retained. A one-term correction changes that coefficient from 2⁵² to 1. The changed model requires its own new build; the original successful build is not replayed. Population, oracles and continuation rule remain unchanged. The corrected model and the separately prepared lowering build/run then each complete once.

An offline reader checks 200 and 209 input pins, all 176 unchanged production hashes, four successful continuation command/exit/stream receipts, the retained initial failure, exact one-term repair, every reported population/count, source bounds and output hashes. It does not execute either model or its oracles again. All scalar builds use Rust 1.98.1, -O and overflow checks; these are mathematical tooling binaries, not native AVX consumers. No old field/core test, decoder, capture, prover or verifier is replayed.

The models' BigInt/Arkworks allocations are diagnostic storage. No native allocation/capacity/stack/RSS/minimum-RAM, device/portable, pressure/swap, concurrent-proof, AWS/Aeglos or full-proof performance result is added. No production key copy/table, solver plan, thread pool or C/ASM rewrite is introduced. Production source 176, accepted SDK 6d83 and canonical 35,171 are unchanged.

## Next qualification

Implement one private native late-centered kernel/type, preserving the existing public field contracts and compact key format. Qualify exact broad raw-input bounds, sign compensation, mixed48 packing, inverse bridge and fixed wider export. Then qualify full G1/G2 callers including tails/original exceptional fallback, key preservation, resources and shared-small-stack workers; inspect actual linked code once. Only after that freeze complete matched caller timing and automatic profile gates. A component win still requires verified SDK primary, all8+54/primary16, Gnark, resource and portable qualification before adoption. [Concrete next stage](artifacts/next.md). No scale/layout/compiler/worker sweep is selected.

## Accepted proof comparison, inherited

No new SDK request or proof timing is added. Rows are inherited from the [preceding complete-caller report](../2026-11-08-winograd16-callers176/README.md): latest primary plus seven explicitly older full-campaign observations. Supplied M5 references use different hardware.

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

Accepted local 1×2 remains **about 8 ms**: the same binary previously measured 7.915095/7.928660/7.995682 ms; latest 8.002770 ms. Variation cause remains unconfirmed, without an adopted regression. Merge54 remains inherited 162.240109 ms without a supplied M5 reference. The original all-eight verified CPU 10× goal remains active and unmet, **0/8**.

[Full model results](artifacts/result.json), [new lowering results](artifacts/compensation-result.json), [offline review](artifacts/review.json), [frozen design](artifacts/design.md), [reading/history scope](artifacts/research.md), and [exact UTF8 evidence](artifacts/evidence.json.gz). The archive maps relative paths to {utf8,bytes,sha256}; every payload and decompression roundtrip is checked. Sources, old/fixed model, helpers, plans/pins and all receipts remain. Executables/libraries, keys/witness/proof payloads, private ledgers and external full papers/source bodies are excluded.

Public prose copies trim trailing blank lines, and this README is copyedited after generation. The archive preserves the frozen prose and helper bytes; model/result/source payloads remain exact.
