# Shared top-limb products: native qualification

Follow-up: the [completed G1/G2 caller screen](../2026-11-08-top-cross52-callers176/README.md) rejected this candidate. The qualification below records the state before that performance screen.

**Native correctness and code generation pass; performance is unmeasured.** All twelve linked consumer kernels retain the predicted 92 IFMAs for ordinary multiplication and 134 for fused operations, versus the original recipe's 100/150. The new kernel passes 554,496 raw lane-output checks and as many canonical export checks against independent Arkworks arithmetic. Production remains unchanged; nothing is adopted.

The [exact model](../2026-11-08-top-cross52-model176/README.md) established the six-slot schedule, signed range bounds and unchanged R256/R260 representation. This stage implements it in a private field child. Each of four ordinary CIOS rows shares its diagonal and the precomputed top diagonal to combine ai·b4+a4·bi with one signed difference product. The last row retains only the top diagonal. There is no key table/copy, retained plan, new heap payload or production pool.

Six accumulator slots exclude the persistent top-product halves and other temporaries. The real linked wrappers show the additional compiled work below; these static counts are not retired instructions, spill traffic, a whole-stack bound or a latency measurement.

|Actual linked consumer|Bytes|Instruction sites|IFMA sites|Explicit RSP/RBP sites|ZMM stack-operand sites|Literal stack-subtraction sum, bytes|
|---|---:|---:|---:|---:|---:|---:|
|tested_mul::<zopro_bn254::FqConfig, false>|1401|229|92|6|4|0|
|tested_mul::<zopro_bn254::FqConfig, true>|1457|236|92|6|4|0|
|tested_mul::<zopro_bn254::FrConfig, false>|1401|229|92|6|4|0|
|tested_mul::<zopro_bn254::FrConfig, true>|1457|236|92|6|4|0|
|tested_sum::<zopro_bn254::FqConfig, false>|2470|390|134|26|24|504|
|tested_sum::<zopro_bn254::FqConfig, true>|2623|415|134|23|19|248|
|tested_sum::<zopro_bn254::FrConfig, false>|2470|390|134|26|24|504|
|tested_sum::<zopro_bn254::FrConfig, true>|2623|415|134|23|19|248|
|tested_diff::<zopro_bn254::FqConfig, false>|2514|403|134|19|15|120|
|tested_diff::<zopro_bn254::FqConfig, true>|2732|435|134|25|19|120|
|tested_diff::<zopro_bn254::FrConfig, false>|2514|403|134|19|15|120|
|tested_diff::<zopro_bn254::FrConfig, true>|2732|435|134|25|19|120|

All twelve wrappers have zero calls. Lower IFMA count alone is insufficient: sign preparation, recombination, register allocation and caller context remain material. The previous paired-tile and factor-placement failures remain closed. This schedule shares the top diagonal across four original CIOS rows, rather than repeating either earlier implementation.

## Correctness and provenance

Both primes and both radices cover all three operators: ordinary product, sum2 and positive-complement difference2. Each panel has 52 boundary values, 2,704 boundary rows, 2,048 random rows and 1,024 changing rows. Eight lanes and three outputs give 138,624 lane outputs per panel. Boundary values include numeric 2p. Every raw lazy digit matches the unchanged original operation exactly; every canonical export matches Arkworks. The inherited harness uses a new random seed on this new kernel. These finite checks supplement the parent mathematical bounds; they are not cryptographic proofs or a formal machine-code proof.

All four qualification commands and twelve linked-function decodes succeed on their first executions. A single symbol-size query identifies the actual consumers. The native field library uses O3/native/one codegen unit; the oracle binary uses fat LTO with overflow and debug checks. The generic no-assembly/no-IFMA field build passes. Existing accepted field/core/curve suites are inherited unchanged and are not replayed or counted as new tests. No complete point caller or SDK has yet used this kernel.

Eighteen accepted field-package members recover exactly: seventeen byte-exact copies plus one append-only parent and the new child. The reader checks both pin sets, all command/stream receipts, source reversals, exact inherited cancellation/packing and test-harness changes, and independently counts every instruction/IFMA from saved decodes. No successful model, field test, decode or verifier is repeated. The parent model binds all 176 unchanged production sources.

Next is coherent core integration in a private copy, preserving original arithmetic as a fresh control, then complete changing signed G1/G2 caller correctness and threaded measurement with automatic profiles. The caller must include inversion, tails, packing and output checks. All-eight SDK proof latency, primary16, merge54, independent Gnark, portable, resource and concurrency qualification remain required before adoption. No width/layout/compiler/worker sweep or performance decision is made here.

## Full-proof comparison, inherited

Earlier full-proof medians were already below 8 ms: private candidates 7.86–7.87 ms and the same accepted build 7.92 ms. Its latest matched measurement is 8.018438 ms, with the unadopted default-carry candidate at 7.897108 ms. Cross-campaign absolutes do not establish a regression or a new record. These CPU/Metal comparisons are inherited from the [latest full-proof campaign](../2026-11-08-uniform-carry-default-performance183/README.md), not measured with the new kernel. Supplied M5 references use different hardware.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|8.018438|37 / 4.614×|24 / 2.993×|3.7|
|2x2|11.199751|59 / 5.268×|30 / 2.679×|5.9|
|4x2|15.957517|109 / 6.831×|48 / 3.008×|10.9|
|8x2|24.250900|189 / 7.794×|74 / 3.051×|18.9|
|12x2|33.735233|284 / 8.418×|101 / 2.994×|28.4|
|16x2|40.165709|335 / 8.340×|124 / 3.087×|33.5|
|24x2|65.802777|528 / 8.024×|176 / 2.675×|52.8|
|merge-24x1|64.926690|507 / 7.809×|175 / 2.695×|50.7|

Accepted merge54 remains 162.442299 ms without a supplied M5 reference. The verified CPU AVX 10× objective remains active and unmet, 0/8. Canonical inventory stays 36,477; zero SDK requests or independent Gnark exports are added. No new AWS/Aeglos, huge-circuit, pressure/swap, minimum-RAM, phone/ARM/TEE or concurrent-throughput result is added.

[Qualification](artifacts/qualification.json), [linked code](artifacts/codegen.json), [source recovery](artifacts/source-shape.json), [native source](artifacts/top_cross52.rs), [evidence](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
