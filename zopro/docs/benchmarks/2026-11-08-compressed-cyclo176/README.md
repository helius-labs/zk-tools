# Compressed cyclotomic parameter-power screen

**The algebra checks pass; the fixed recipe is closed without a native implementation or performance trial.** Compressed squaring removes coordinate work, but reconstructing the selected powers nearly erases that saving. Relative to the accepted source schedule, the complete modeled parameter exponent uses four fewer fused-two-product calls while adding 91 ordinary Fq multiplies, one Fq inverse and 44 nonresidue maps. We decline this tradeoff. This is not a measured slowdown or a proof-speed result. Production remains unchanged.

[Karabina's paper](https://eprint.iacr.org/2010/542) introduces compressed cyclotomic squaring. Its abstract was available; the PDF returned 403. The equations were checked against the locally installed [Gnark-crypto v0.22.0 source](https://github.com/Consensys/gnark-crypto/blob/v0.22.0/ecc/bn254/internal/fptower/e12.go), with its file hash pinned. This model derives a four-coordinate square from the exact accepted Granger–Scott implementation and separately implements the recovery equations; it does not translate the Go function body or import upstream timing. Earlier signed-window-four/static exponent and packed-Miller SDK experiments retain their recorded negative gates.

For X=4,965,661,367,192,848,881, the original binary loop has 63 cyclotomic squares and 28 Fq12 products. The candidate constructs 62 compressed successive squares and saves 27 selected powers, then uses norm-batched Fq2 denominators and one scalar Fq inverse to recover them. Its final product starts at f, so it already receives credit for removing the leading identity square/product. There is no signed-recoding, dictionary, layout or parameter sweep. The unchanged final conjugation lies outside both counted positive-parameter powers.

The compressed coordinates are g1=c0.c1,g2=c0.c2,g3=c1.c0,g5=c1.c2. Normal recovery uses g4=(xi*g5²+3*g1²−2*g2)/(4*g3) and g0=1+xi*(2*g4²+g3*g5−3*g2*g1). Zero-denominator cases require separate identity/degenerate handling. Only valid cyclotomic inputs are supported.

## Whole parameter-exponent work

Counts are semantic method entries, not equal-cost operations, emitted instructions, cycles or proof wall fractions. Source binding checks Fq2 square=two Fq multiplies, x86 Fq2 product=two fused-two-product calls, and Fq12 product=18 Fq2 products. Addition, negation, packing and other boundary work remain additional. Optimizer elimination of identity operations is not inferred from this source model.

|Operation|Accepted source schedule|Compressed recipe|
|---|---:|---:|
|Fq2 squares|378|329|
|Fq2 products outside Fq12 products|189|205|
|Direct Fq products for norm batching|0|189|
|Fq12 products|28|27|
|Lowered Fq ordinary multiply/square entries|756|847|
|Lowered Fq fused-two-product entries|1386|1382|
|Fq inverse|0|1|
|Fq2 nonresidue maps, including product nesting|385|429|

The raw componentwise-domination predicate is false because fused calls decrease. The design consequently requires a separate complete-tradeoff review; that review declines native work. Neither a false domination flag nor algebraic correctness is an adoption or timing-selection gate. No weighted cycle threshold or measured latency claim is substituted. Other compressed schedules or representations are not ruled out.

## Qualification and limits

The first optimized, overflow-checked compile and first model execution succeed, with an unused-import warning retained. There are 257 mapped inputs: identity plus 256 fixed full-width generated nonzero Fq12 values. Every easy map matches independent Arkworks. All 15,934 consecutive square/recovery cases agree with general production field squaring and independent Arkworks powering. Both complete parameter-exponent algorithms agree for all 257 inputs and with Arkworks general powering. These are parameter-power model checks, not complete final exponentiation, verifier, native SIMD or proof measurements. Diagnostic recovery/oracle work is excluded from the candidate counters.

The identity tuple is exercised. No nonidentity g3=0 case occurs, so that recovery branch remains untested. One malformed tuple is explicitly classified invalid; the rejecting recovery panic is not executed. Zero raw Fq12 is rejected by the easy map. The original design's phrase "zero invalid compressed tuple" is imprecise: the all-zero compressed tuple is the valid identity. These limitations remain documented without rerunning the successful model or relabeling the coverage.

The five fixed arrays have 28,736 requested data bytes: 16,384 compressed powers, 8,192 fraction pairs, 2,048 norms, 2,048 prefixes and 64 activity flags. This corrects the preliminary design forecast by including fraction storage. Scalar/Fq12 locals, compiler frames and model/oracle analysis allocations are extra. This is not a native stack bound, observed prover RSS, RAM minimum or mobile/pressure qualification. No key table/copy, input-sized cache, retained plan or production pool is proposed.

The offline reader checks 14 pins, all 176 accepted production hashes, the source-transformed original square, two successful command receipts/stream hashes, all 257 counter rows, operation lowering, array sizes and the separate native-selection decision. It neither reruns the model nor supplies a second independent execution of its field oracles.

## Accepted proof comparison, inherited

No SDK request, export, profile or latency recording is added. Canonical inventory 35,171 and accepted SDK6d83 and source176 remain unchanged. All rows below are inherited from the [preceding report](../2026-11-08-gcd-signedjump176/README.md), comprising the latest primary and seven earlier full-campaign rows. Supplied M5 references use different hardware.

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

The accepted local primary is **about 8 ms**: the same binary previously measured 7.915–7.929 and 7.995682 ms; latest 8.002770 ms. The inter-run cause remains unconfirmed, without a slower adopted replacement. Merge54 remains inherited 162.240109 ms without an M5 reference. Original all-eight verified CPU10× objective remains active and unmet, **0/8**. This incremental verifier screen does not replace the target of reducing dominant MSM work. No new AWS/Aeglos, huge-circuit, ARM/phone/TEE, pressure/swap, peak/minimum-RAM or concurrent-throughput result is added.

The [evidence archive](artifacts/evidence.json.gz) maps relative UTF8 paths to {utf8,bytes,sha256}, including the full raw model rows, source-bound square fragment, helpers, design/decision, plans/pins and exact receipt streams. Every extracted payload and the archive roundtrip are checked. The public model summary removes only per-case rows; raw and public-view hashes are separate. Libraries/binaries, full external sources/papers, keys/images/witness/proof payloads and private ledgers are excluded. External source identities remain pinned.
