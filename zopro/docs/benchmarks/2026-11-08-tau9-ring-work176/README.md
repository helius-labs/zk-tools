# Endomorphism-radix G2 work screen

**The fixed τ⁹ representation passes its necessary arithmetic/count screen. No native speedup is established.** The primary dense model requests 562,072 generic additions, versus 634,405 for the inherited accepted model and 573,432 for the earlier binary-ring model. This permits a complete queue/tree/tail/resource census, not native implementation, timing or adoption. The earlier binary-ring native regression remains a material negative reference.

## Direction and history

Use τ=1−ω in the Eisenstein ring, where ω²+ω+1=0, and the fixed block radix ρ=τ⁹=81+162ω. Its norm is 19,683. This quotient ideal differs from the earlier binary radix-128 ring. Eighteen signed digits replace that model's nineteen positions. There is one fixed geometry; no width, flag, layout or host sweep is performed.

The current abstract of [Xu et al., ePrint2024/1906](https://eprint.iacr.org/2024/1906) motivates τ as a prime-field scalar-multiplication endomorphism. Its PDF fetch returned403; the full paper was not read. This MSM quotient construction is independently derived. No reported single-scalar runtime or projective-map cost is transferred to native G2/Fq2 arithmetic. The paper's endomorphism basis is known; no novelty claim is made.

The preceding [local-tile experiment](../2026-11-08-blocked-pair52-screen176/README.md) remains rejected. Its exploratory schoolbook control combines known paired rows and mixed-radix packing. The packing identity [passed small-proof timing](../2026-10-25-cios-pack-sdk/README.md), but its later [4999 comparison](../2026-10-25-cios-pack-large-pair/README.md) failed a declared adjacent-pair criterion; pooled variant0 was1.016091 while an adjacent ratio was1.202747. The separate row-pair/fused-SDK trials also failed. These observations do not authorize an unchanged retry, a new label for packing or promotion of that control.

Research also rechecked [SRE](https://arxiv.org/html/2610.27452v1) and [memory-constrained MSM](https://eprint.iacr.org/2026/1316). Their cross-window reuse and bounded-bucket mechanisms already have local studies and are not reimplemented here. The [binary-ring native trial](../2026-11-06-g2-joint-ring-native176/README.md) remains closed: it was about60% slower than fresh control despite fewer modeled additions. This new screen does not erase that result.

## Exact recoding

Residue classes are indexed by `(a mod81,b mod81,(floor(a/81)+floor(b/81)) mod3)`. Exact nearest-quotient enumeration constructs bounded representatives and combines six units into three signed planes. Dividing coordinates by81 before forming the quotient avoids overflowing full-width signed intermediate combinations. The quotient's inverse matrix has denominator243.

One new Rust/xtask compile and execution pass on their first runs. Checks cover:

- All19,683 classes, including the zero class and one nonzero two-class unit orbit; the other3,280 nonzero orbits each have six classes. There are3,281 representatives, each with norm≤6,561.
- All19,682 nonzero class seeds,169 signed edge pairs,10,000 full-width signed pairs and both retained real query populations. Every recoded pair reconstructs exactly with independent BigInt arithmetic and separately in Fr using the original endomorphism eigenvalue.
- The existing signed Phi2 helper remains byte-identical and its scalar/eigenvalue relation is checked for every fixture row. Its old tests and earlier mathematical models are inherited without replay.
- Eighteen-digit termination follows from the checked integer inequality `139·2^127 <58·140^18`, since input coordinates are strictly below2^126, |ρ|>140 and digit radius≤81. No floating-point termination assumption is used.

The real-fixture work model checks every window's weighted result against direct dot products in two integer modules:72 complete window checks across the two cases. These are linear algebra checks, not native elliptic-curve or proof verification. Equal-x cases, cancellations, group exceptions, inversions and native point normalization are not implemented by this model.

A separate offline reader verifies202 pins, both successful command receipts/argv/stream hashes, the quotient matrix and lattice index, unit-orbit accounting, an independent integer termination bound and all36 per-position work rows. It does not independently replay the complete token/curve population or rerun the model. The inherited split helper's four unused-function warnings are preserved without a cleanup rebuild. [Model](artifacts/model.rs), [complete output](artifacts/analysis.json), [review](artifacts/review.json).

## Work and costs still missing

The dense model preserves two original point parts. It charges ordinary fill, part/unit-plane merging, grouping into both coefficient axes, consecutive dense weighted sums and the final axis combination. Unit endomorphism maps and Horner operations remain separate. Inherited comparison counts use exactly the same dense definition; their earlier models are not rerun. These totals differ from the later exact stream/tree census and are not native operation or latency observations.

|Raw G2 query|Inherited accepted dense adds|Prior binary-ring dense adds|τ⁹ dense adds|Reduction vs accepted|Reduction vs prior binary|
|---|---:|---:|---:|---:|---:|
|1×2,30,089 rows|634,405|573,432|562,072|11.40%|1.98%|
|2×2,52,274 rows|1,085,418|956,616|931,602|14.17%|2.61%|

The primary necessary gate requires strictly fewer generic additions than both inherited comparators, with no percentage floor. It passes. Raw2×2 is diagnostic because production admits global scalar grouping there; it is not the candidate's qualified runtime population.

|Separate unit-map quantity|Accepted dense|Prior binary|τ⁹|
|---|---:|---:|---:|
|Primary affine φ maps|41,022|94,042|104,230|
|Raw2×2 affine φ maps|41,071|95,162|106,219|

Each modeled G2 affine φ map uses two Fq multiplications; axis-result maps are separate. There are17 ρ-Horner steps. Algebraically ρ=81φτ, so one possible step uses four triplings, τ and a unit map; this is not a qualified or priced native formula. The new method saves only1.98% of generic work relative to the already slow primary binary-ring method while increasing unit maps. Recoding, all map costs, branching, packing, allocation, actual trees, scalar tails, exceptions and scheduling remain unpriced. The count screen is insufficient evidence of a practical gain or a twofold improvement.

An optional packed scalar-metadata representation would use52,490 payload bytes independent of proving-key size. That is a theoretical u16-map/i16-coordinate layout, not a native allocation, point lookup table or memory peak. Three point planes require9,843 bucket slots per part, versus8,193 for the previous binary ring and2,048 for an original ordinary window. This can increase working memory materially. The model's host allocations are not a prover memory bound. Any future native design must charge all simultaneous/suspended parents, digit staging, axes and scratch and retain a valid low-memory fallback. No key copy, shifted-point table, retained solver plan or production pool is added.

Next is the exact stream/queue/top/tree/tail census and a complete cost/resource design, retaining this small margin and the older native regression. A native candidate is not selected. The fixed geometry closes at this stage if those costs invalidate the opportunity; no favorable subset or unchanged reroll is permitted.

## Accepted CPU and Metal comparisons, inherited

Production176, accepted SDK6d83 and the35,171-proof inventory remain unchanged. There are zero new timings, profiles, SDK requests or proofs. These local32 full-proof observations and supplied historical M5 references are inherited, not results of the work model. The same accepted binary previously measured7.915/7.929/7.996ms for1×2 and latest8.003ms; the cause of that variation remains unconfirmed, with no adopted regression.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.002770|37 /4.623×|24 /2.999×|3.7|
|2×2|11.241463|59 /5.248×|30 /2.669×|5.9|
|4×2|15.933816|109 /6.841×|48 /3.012×|10.9|
|8×2|24.234311|189 /7.799×|74 /3.054×|18.9|
|12×2|33.855565|284 /8.389×|101 /2.983×|28.4|
|16×2|40.071563|335 /8.360×|124 /3.094×|33.5|
|24×2|65.917462|528 /8.010×|176 /2.670×|52.8|
|merge-24×1|65.305023|507 /7.764×|175 /2.680×|50.7|

Source: [preceding full comparison](../2026-11-08-blocked-pair52-screen176/README.md). Seven nonprimary rows remain from the earlier full campaign. Merge54 remains162.240109ms without a supplied M5 reference. The verified all-eight CPU10× goal stays active and unmet, **0/8**. No fresh AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM or concurrent-throughput result is added.

[Design](artifacts/design.md), [history audit](artifacts/history-audit.json), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [exact source/receipt archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json) and [checksums](SHA256SUMS) retain the new mathematical evidence. Every UTF-8 payload and gzip roundtrip is checked. Private scalar/point/key/witness/proof arrays, compiled libraries/binaries, unchanged full production source and full papers are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/tau9-ring-work176`.
