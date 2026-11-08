# PMNS truncation at BN254 word widths

**This schoolbook truncation route has no product-volume advantage over the current kernel and is closed before native implementation.** Its optimistic minimum is109 ordinary/159 fused IFMAs per eight independent fields, versus current100/150. This is a source-operation screen, not a measured slowdown or a general impossibility result for PMNS. Production and accepted proof times are unchanged.

[Didier et al., arXiv2610.09767v1](https://arxiv.org/html/2610.09767v1#S5) truncate low product halves during PMNS internal reduction. Their coefficient count n and words-per-coefficient s are distinct. Earlier local reports already mention the paper; this is a new exact width/count check, not a new discovery. Sections4–5 were inspected. Its coefficient-parallel SIMD counts cannot be compared directly with eight independent field products. No paper runtime is transferred here.

## Exact count screen

Let n be polynomial coefficient count, s the number of52-bit words per coefficient and t=ns. Both actual BN254 primes have254bits, so t≥5 is necessary for full coefficient-word capacity. This does not establish usable PMNS parameters, closure or headroom. Keeping eight independent fields per vector, the new literal low/high-term observer checks:

|Source operation|Full template|Truncated template|
|---|---:|---:|
|Ordinary|5n²s²|n²(4s²+2s−1)|
|Fused AB+CD|7n²s²|n²(6s²+2s−1)|

The fused extension is local accounting: double the raw input convolution, retain one quotient and reduction. Its arithmetic range is not qualified. Both templates save n²(s−1)² product halves. Thus s=1 removes none. The earlier six-coefficient one-word schoolbook template stays180IFMAs, not a newly faster version of that rejected implementation.

|Capacity-only geometry|Ordinary truncated|Fused truncated|
|---|---:|---:|
|n1,s5: degenerate integer representation|109|159|
|n5,s1|125|175|
|n2,s3|164|236|
|n3,s2|171|243|
|Current accepted CIOS source recipe|100|150|

Every n,s from1 through16 and both modes is counted:512 schedules, including496 capacity-admissible rows. The minima extend beyond that table: t≥6 alone gives ordinary≥4t²≥144 and fused≥6t²≥216; at t=5 only(1,5) and(5,1) occur. Even the optimistic degenerate minima would change the ordinary G2 group's seven ordinary/five fused product proxy from1450 to1558, **7.45% more**. Other operations are excluded; these are not retired instructions or cycles.

This closes only the specified dense schoolbook coefficient/word template under the stated output layout. Polynomial Karatsuba/Toeplitz, special sparse reduction polynomials, alternative lane packing and other representations are outside this proof. Existing failed PMNS variants keep their original results. More IFMAs alone do not prove that every possible implementation must be slower, but this route supplies no product-removal case for the required jump.

## Validation and resources

The two new Rust1.98.1/xtask commands compile and execute once, both passing. Exact BigUint products independently check4800 accumulated-product reconstructions, including120 boundary cases, bases2¹²/2⁵², one/three/six products and one through five coefficient words. The truncated reconstruction is checked only when the omitted part is smaller than the coefficient radix and the complete numerator divides exactly. A deliberately insufficient small-base case demonstrates the need for that guard. These checks do not qualify a native PMNS kernel or parameter set.

A separate reader checks209 pins, both command/stream receipts, all496 geometry rows using independent closed-form counts, the global-minimum extension, primary-source anchors/hash and the unchanged37,023-proof inventory. It does not replay the4800 cases or independently reimplement their BigUint arithmetic. No successful historical model/test, profile, proof or verifier is repeated. All new workflow and review commands pass first execution.

No runtime buffer, key/point table, key copy, retained plan, production pool or arbitrary RAM cap is added. Diagnostic host allocations are not a prover memory qualification. No new timing, profile, SDK request or proof is produced. The downloaded primary HTML stays private; full paper text and unchanged full production source are excluded from publication.

## Inherited full-proof comparisons

Production176/SDK6d83 remains accepted. These are the latest saved local32 observations, inherited from the [tree-policy SDK comparison](../2026-11-08-tree-policy-sdk176/README.md), against supplied M5 references on different hardware. The same accepted executable previously measured7.915ms for1×2; private7.860–7.869ms candidates were not adopted. Latest-versus-earlier variation does not establish a code regression.

|Shape|Local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.040649|37 /4.602×|24 /2.985×|3.7|
|2×2|11.149097|59 /5.292×|30 /2.691×|5.9|
|4×2|16.069415|109 /6.783×|48 /2.987×|10.9|
|8×2|24.131178|189 /7.832×|74 /3.067×|18.9|
|12×2|33.940628|284 /8.368×|101 /2.976×|28.4|
|16×2|40.164083|335 /8.341×|124 /3.087×|33.5|
|24×2|66.187704|528 /7.977×|176 /2.659×|52.8|
|merge24×1|65.361571|507 /7.757×|175 /2.677×|50.7|

Merge54 remains160.890372ms without a supplied M5 reference. Verified CPU single-proof≥10×M5 CPU on all supplied shapes remains **active and unmet,0/8**. No fresh AWS/Aeglos, mobile, huge-circuit, pressure/minimum-RAM or concurrent-proof result is added. The next investigation must supply a distinct main-path work/dependency case; this paper's multiword-coefficient saving cannot be applied to the old one-word PMNS by treating five polynomial coefficients as s=5.

[Counts](artifacts/analysis.json), [source](artifacts/model.rs), [design](artifacts/design.md), [review](artifacts/review.json), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json) and [checksums](SHA256SUMS) preserve the result. Every UTF-8 member hash and gzip roundtrip is checked. Private payloads/binaries are excluded. Helper compilation/preparation/publication are tool-transcript observations rather than extra xtask receipts. Private root:`/home/ubuntu/src/zopro-perf-work/pmns-truncation-width176`.
