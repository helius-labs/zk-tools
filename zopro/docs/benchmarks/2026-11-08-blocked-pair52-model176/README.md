# Local Karatsuba tiles in paired Montgomery rows

**The exact arithmetic and range screen passes; native implementation is next.** Two local 2×2 Karatsuba tiles reduce the source IFMA proxy from 100→96 for ordinary products and 150→142 for fused products, using a seven-coefficient frontier. This is a mathematical result, without new native execution, timing, proof or production change. It does not establish a speedup.

## Fixed construction

Accumulate product rows 0/1, cancel twice, then repeat for rows 2/3. Within each pair, replace only its diagonal 2×2 tile with three products. If D0=ai·bi, D1=aj·bj and P=(ai−aj)(bj−bi), the cross term is D0+D1+P. Split those products into radix-52 words and add them at their original positions. All other raw products remain schoolbook. The final row retains the original radix-48/R256 or radix-52/R260 cancellation, normalization and fused correction. Difference-of-products retains the positive 2p−c complement.

The sign recipe uses the previously qualified absolute-first/oriented-second identity. It does not reuse the latest deferred-sign recipe. The new schedule has two local signed products per elementary field product, rather than ten in the full signed-pair convolution. It adds sign and recombination operations; fewer IFMAs alone do not predict runtime.

|Source property|Original CIOS|Earlier row-pair|Earlier full signed-pair|New local tiles|
|---|---:|---:|---:|---:|
|Ordinary IFMA proxy|100|100|80|96|
|Fused IFMA proxy|150|150|110|142|
|Coefficient-array words|6|7|10|7|

These are source counts, excluding constants, moves, compiler allocation, stack traffic and other ALU work. Coefficient-array size excludes input and temporary lifetimes and is not a stack or RAM bound. The proposed arithmetic adds no table, heap buffer, copied key/point array, retained plan or production pool.

The [earlier two-row CIOS](../2026-10-25-cios-rowpair-component/README.md) and its [fused SDK integration](../2026-10-25-cios-fused-rowpair-sdk/README.md) remain rejected. This new local decomposition changes raw multiplication work; it does not rerun their fixed schedule. The [latest full-pair comparison](../2026-11-08-deferred-limb-sign-screen176/README.md) also remains closed: its sign change improved the old pair recipe but lost against original CIOS. No historical gate is replaced.

## Exact checks

One new Rust/xtask compile and one mathematical execution pass on their first runs. A separate offline reader then passes without reexecuting the population.

- **87,948 complete products and 439,740 cancellation steps:** both primes, R256/R260, ordinary/sum2/complement-difference operations. Every quotient and whole integer after cancellation agrees with an independent whole-integer recurrence. Final results agree with full-R REDC and modular-inverse references, with normalized output below 2p.
- **293,160 local tiles** are checked against independent 2×2 schoolbook cross terms, including negative pair products. Each field/radix has 47 boundary values, all 2,209 ordered boundary tuples, 4,096 full-width deterministic tuples and 1,024 changing-chain steps per operation. Inclusive 2p inputs occur in every panel.
- **69,904 exhaustive small-radix tiles** cover radix 2,4,8,16. These are algebra checks, not native SIMD tests.
- Conservative interval propagation bounds the actual schedule below 64B before packing and 1024B throughout, B=2^52. An independent coarse argument gives bounds of 51B and 820B; both fit signed64. These are general envelopes, distinct from observed maxima.

The reader validates 187 original pins, two model command receipts and stream hashes, all count identities, the 25-term global schoolbook polynomial and a separate conservative range argument. Its own workflow binds 208 pins and completes once. All 176 accepted source hashes remain exact. The compiler's harmless unnecessary-parentheses warning is preserved; no cleanup rebuild or successful stage replay occurs. There are no new clocks or profiles in this stage.

The coefficient schedule is feasible, but native lowering can add enough ALU work or register pressure to erase the saving. Next is one fixed native implementation, independent field-output tests and linked code inspection. Subsequent measurement must compare original frozen/fresh arithmetic, row-pair schoolbook and local tiles in changing-input threaded callers with automatic profiles. A positive component result still needs complete SDK/all-shape/Gnark/resource/portable qualification before adoption. No result here qualifies phone memory, minimum RAM, large circuits or concurrent proving.

## Accepted CPU and Metal comparison, inherited

Production176/accepted SDK6d83 and the 35,171-proof inventory remain unchanged. These local 32-worker SDK observations and supplied historical M5 references are inherited, not measurements of this model. The same accepted SDK previously measured 7.915/7.929/7.996 ms for 1×2; its latest observation is 8.003 ms. The cause of that variation remains unconfirmed, with no adopted regression.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.002770|37 / 4.623×|24 / 2.999×|3.7|
|2×2|11.241463|59 / 5.248×|30 / 2.669×|5.9|
|4×2|15.933816|109 / 6.841×|48 / 3.012×|10.9|
|8×2|24.234311|189 / 7.799×|74 / 3.054×|18.9|
|12×2|33.855565|284 / 8.389×|101 / 2.983×|28.4|
|16×2|40.071563|335 / 8.360×|124 / 3.094×|33.5|
|24×2|65.917462|528 / 8.010×|176 / 2.670×|52.8|
|merge-24×1|65.305023|507 / 7.764×|175 / 2.680×|50.7|

Source: [preceding retained full-proof comparison](../2026-11-08-deferred-limb-sign-screen176/README.md). Seven nonprimary rows remain inherited from the earlier full campaign. Merge54 remains 162.240109 ms, without a supplied M5 reference. No fresh AWS/Aeglos, proof latency, peak-RSS or minimum-RAM result is added. The original verified CPU 10× goal remains active and unmet, **0/8**.

[Design](artifacts/design.md), [model source](artifacts/model.rs), [complete result](artifacts/result.json), [offline reader](artifacts/review.rs), [review](artifacts/review.json), [plan](artifacts/plan.json), [pins](artifacts/pins.json) and [exact sources/receipts](artifacts/evidence.json.gz) are retained. Archive members have UTF-8 payloads, lengths and SHA256s; every payload and gzip roundtrip is checked. Compiled binaries/libraries, unchanged full sources, keys/images/witnesses/proofs and third-party full text are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/blocked-pair52-model176`.
