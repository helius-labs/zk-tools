# Real-norm common-scale G2 work screen

The algebra passes. This fixed recipe is **not selected for native implementation**: it improves on a generic complex-scale construction, but increases the vector arithmetic proxy against the accepted norm-based reducer in every declared case. This is an engineering decision from a work/resource screen, not a measured slowdown or a lower bound on other algorithms. Production is unchanged.

## Construction and traversal

For dx_i in Fq2, let n_i=norm(dx_i), D=product(n_i) and q_i=product of the other norms, all in Fq. Then u_i=dy_i*conjugate(dx_i)*q_i satisfies u_i/D=dy_i/dx_i. The shared Jacobian lift is:

    X'_i = u_i² − (X1_i+X2_i)D²
    Y'_i = u_i(X1_i D²−X'_i) − Y1_i D³
    Z'   = ZD

Real Z stays in Fq. Cofactors and common squares/cubes are real; coordinate rescaling uses Fq2-by-Fq operations. The generic comparison uses complex D=product(dx_i), complex cofactors and the same traversal. No mathematical novelty or published CPU speed claim is asserted. The earlier [G1 caller experiment](../2026-11-07-common-z-callers176/README.md) remains closed; its failed gate is not rerun at a new endpoint.

The model uses eight prefix chains, a five-product butterfly, and one compact cofactor record per padded pair block. Its reverse pass recomputes denominators, completes cofactors and writes each pair to its original left slot. Pair inputs are disjoint. Odd carries rescale separately before forward compaction. This avoids a separate carry/cofactor-output buffer. Group boundaries, singleton survivors and final normalization are charged.

An earlier output-slot traversal was refined during source review, before any arithmetic or count execution; both designs are retained. Identity inputs, zero denominators/norms and late equal/opposite cases decline transactionally. The private inputs remain unchanged.

## Units and accepted-source comparison

VM is one logical eight-lane ordinary Fq product; VF is one fused sum/difference of two products. SM/SS/SF are scalar multiply/square/fused-two-product calls; I is one scalar inverse. **These units are kept separate.** The interpreter evaluates lanes using the unchanged scalar field library; it is not a new IFMA kernel or a latency benchmark.

The vector-only proxy 100 VM + 150 VF counts nominal IFMA sites for the accepted carry-cancel CIOS source. It excludes scalar work, inversion, shifts, correction, packing, memory traffic and scheduling. Its ratios are not speed ratios. The frozen field metadata confirms the carry-cancel flag.

Accepted G2 already uses norm-based shared inversion. Full pair blocks cost 7 VM + 5 VF in prefix mode, or 8 VM + 5 VF below 32 pairs, plus the per-level bridge of 5 VM + 1 SM + 1 I. Each scalar tail costs 7 SM + 5 SF. In particular, accepted x86 Fq2 scalar multiplication uses two fused products, not three ordinary multiplies. These are source-derived ordinary-pair counts with vector inversion enabled; the private production reducer and actual-witness branch populations are not executed here.

The following table retains all nineteen single-group lengths and three multiple-group populations. k is the accepted inversion count; the new recipe uses one inverse in every listed case. Scalar and inversion savings are fully listed in the JSON and are not priced as zero in a complete-work claim.

|Case|Vector proxy: accepted / real norm / generic complex|Real/accepted proxy|I: accepted→real|Required field payload: accepted→real bytes|
|---|---:|---:|---:|---:|
|single2|500 / 3300 / 5200|6.6000|1→1|256→512|
|single3|1000 / 6700 / 10700|6.7000|2→1|384→640|
|single4|1000 / 6300 / 10100|6.3000|2→1|512→768|
|single7|1500 / 9700 / 15600|6.4667|3→1|896→1152|
|single8|1500 / 9300 / 15000|6.2000|3→1|1024→1280|
|single15|2000 / 12700 / 20500|6.3500|4→1|1920→2176|
|single16|3550 / 12300 / 19900|3.4648|4→1|4928→2304|
|single31|5600 / 17900 / 28000|3.1964|5→1|6848→4480|
|single32|7150 / 17500 / 27400|2.4476|5→1|9856→4608|
|single63|12300 / 27500 / 40700|2.2358|6→1|16704→9088|
|single64|13450 / 27100 / 40100|2.0149|6→1|16512→9216|
|single127|24100 / 45900 / 63800|1.9046|7→1|26496→18304|
|single128|25550 / 45500 / 63200|1.7808|7→1|27264→18432|
|single255|47800 / 81900 / 107700|1.7134|8→1|48000→36736|
|single256|49250 / 81500 / 107100|1.6548|8→1|48768→36864|
|single511|94700 / 153100 / 193200|1.6167|9→1|91008→73600|
|single512|96150 / 152700 / 192600|1.5881|9→1|91776→73728|
|single1024|189450 / 294300 / 361300|1.5534|10→1|177792→147456|
|single4096|747250 / 1140700 / 1364300|1.5265|12→1|693888→589824|
|multi-3x16|7200 / 11900 / 17100|1.6528|2→1|11904→6656|
|singletons-and3|1000 / 44300 / 67100|44.3000|2→1|33024→33280|
|mixed|8200 / 22700 / 32700|2.7683|4→1|15808→8192|

At 4,096 points the new recipe saves eleven inversions and reduces the generic complex vector proxy by 16.39%. Against accepted arithmetic its vector proxy rises by 52.65%, while required retained field buffers fall from 693,888 to 589,824 bytes, about 15.0%. That exchange does not justify a native whole-tree campaign for this recipe. It does not prove that inversions, scalar tails or cache effects could never change a particular timing. No selective small-size policy, threshold sweep or favorable native subset is selected.

## Memory lifetimes

Native source/type checks give 32-byte Fq, 64-byte Fq2 and 128-byte point storage. There is no key-point layout saving. Accepted prefix records require 640 bytes per complete eight-pair block, and captured Group records require 2,880 bytes; their retained buffers coexist after different levels have used them. The new canonical record has 256 bytes, versus 512 for the generic complex record.

Tables include required point and retained prefix/Group/cofactor field payloads. The JSON separately reports pair-index and group metadata. Vec headers, allocator capacity rounding and stack/compiler temporaries are excluded. These are typed requirements and source lifetime counts, not measured native capacity, RSS, admission bounds or minimum RAM. Small and singleton-heavy populations can use more field-buffer space than accepted source. A shared-scale algorithm must also rescale untouched survivors; omitting that work would be incorrect.

## Qualification and review

The new model checks 580 complete reductions and 2,658 instrumented level-count identities across real/complex constructions, four deterministic G2 walks, odd/even lengths, mixed/zero/singleton groups and three initial real scales. Outputs agree with complete original projective addition, and inputs remain unchanged. Fourteen initial/late singular, identity and zero-scale cases decline. Native field arithmetic is shared with the complete group oracle; the model does not requalify that field.

An independent F43[u]/(u²+1) rational implementation checks 4,896 pair outputs and 576 carry identities for the real-norm construction. It uses direct all-other products, no native field/prefix/butterfly code, and exhausts all 1,849 norm-zero equivalences. It is a small-field algebra oracle, not BN254 SIMD or portable qualification.

Independent review reconstructs all 22 operation/payload rows from closed level geometry, checks source branch fragments, all 176 accepted source hashes and the three compile/model command receipts. An initial compile failed on a sum type annotation and iterator lifetimes; only that unexecuted model stage was repaired. The first successful arithmetic/count execution passes. Failed source/receipts are retained, and successful field, G1, core/prover, arithmetic, capture or decode stages are not rerun. No CPU profile or timing exists for this recipe.

A separate [generator oracle](artifacts/generator-oracle.rs) adds 160 reductions on 80 new arguments. It checks both constructions against one generator multiplication by `a*n + step*n*(n-1)/2`, using rejection-sampled full-width Fr starts/steps and previously unused lengths. Two sampled scalars have bit 253 set. These cases share the accepted field and use a different group algorithm from serial point addition. Both new workflow commands pass on their first executions; the original 580 cases, analysis and review remain untouched. [Results and hashes](artifacts/generator-oracle.json), [command/source audit](artifacts/generator-oracle-audit.json).

## Decision and accepted comparisons

Close the fixed real-norm recipe without native integration. Retain its correct algebra and advantage over the generic complex lift. The next search should target substantial main-MSM work rather than treat inversion removal as a free reduction in total arithmetic. No key table/copy, duplicate solver plan, production pool, field rebuild or C/ASM backend is introduced.

The following latest accepted local 32-worker full-proof observations and supplied historical M5 references are inherited. [Accepted source manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json), [full-proof evidence](../2026-11-06-window-batch-proof-capture176/README.md).

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU/local|Metal/local|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1×2|7.915095|37|24|4.675×|3.032×|3.7|
|2×2|11.169812|59|30|5.282×|2.686×|5.9|
|4×2|16.085882|109|48|6.776×|2.984×|10.9|
|8×2|24.517520|189|74|7.709×|3.018×|18.9|
|12×2|33.838626|284|101|8.393×|2.985×|28.4|
|16×2|40.083672|335|124|8.358×|3.094×|33.5|
|24×2|65.506198|528|176|8.060×|2.687×|52.8|
|merge24|65.200982|507|175|7.776×|2.684×|50.7|

Accepted merge54 remains 161.718956 ms without a supplied M5 reference. The all-eight CPU 10× goal remains active and unmet, 0/8. There are zero new SDK requests or proofs; the canonical distinct-proof ledger stays at 31,387. No AWS/Aeglos/Gnark comparison, portable/mobile, pressure, huge-circuit, swap, concurrency or minimum-RAM result is added.

[Design](artifacts/design.md), [final traversal](artifacts/recipe-final.md), [interpreter](artifacts/model.rs), [independent finite-field oracle](artifacts/small_prime.rs), [counts and qualification](artifacts/analysis.json), [independent review](artifacts/independent-review.json), [decision](artifacts/decision.json), [source/library pins](artifacts/pins.json), [workflow](artifacts/continuation-plan.json), [checksums](SHA256SUMS). No actual point/witness/key/proof data or executable/library payload is published.
