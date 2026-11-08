# G2 real-factor placement: native qualification

**Native correctness passes; performance is unmeasured.** One algebraic reassociation moves the shared real factor onto the two denominator components before the complex product. The complete signed borrowed G2 caller passes all 128 core tests, including three new cases. The generic/no-assembly/no-IFMA compile also passes. Production remains unchanged; no optimization is adopted.

For real f, the original slope expression `(dy · conjugate(dx)) · f` equals `dy · conjugate(f · dx)`. Both use two ordinary Fq products and two fused products: the same nominal 500-IFMA arithmetic block. Norms, inversion/prefix recurrence, tails, signs, coordinate formulas, scatter and scratch retain their exact original code. The private experiment changes one call inside a clone of the accepted borrowed kernel.

The hypothesis concerns intermediate lifetime: the real factor can die before the fused complex outputs are produced. Fewer field products are not claimed. Actual linked code shows a mixed tradeoff:

|Actual G2 apply function|Bytes|Instruction sites|IFMA sites|ZMM stack-operand sites|All explicit RSP/RBP sites|Literal stack-subtraction sum, bytes|
|---|---:|---:|---:|---:|---:|---:|
|original|51830|8590|1450|500|1471|5952|
|candidate|51954|8621|1450|479|1465|5760|

The candidate has 21 fewer static ZMM stack-operand sites and 192 fewer bytes in the literal stack-subtraction sum, alongside 31 more instruction sites. These are descriptive properties of this linked executable, not retired instructions, removable spill cost, a whole-stack bound or a speed measurement. The caller's own stack frame is additional. A complete caller benchmark is still required.

The new helper checks 32,768 Fq2 lane outputs against the unchanged scalar-field expression and original vector expression, including changing canonical/lazy differences and zero/boundary inputs. Complete point tests cover 17 lengths through 511, changing signed updates, scalar tails, immutable inputs, scratch reuse and original/candidate outputs. Independent workers run in 1/4/16/32-worker pools with configured 512 KiB stacks. Scalar/curve references share the accepted field implementation; they are not a new independent Arkworks or cryptographic proof verifier.

The field implementation and its earlier qualification are inherited unchanged. The caller fixture, driver, allocator observer and perf handshake retain exact earlier source bytes; only the private selector name changes. The manifest/lock changes only the root package name. All 176 accepted originals recover exactly: 174 unchanged files, two append-only diagnostic parents and one new child. There is no new key table/copy, execution plan, heap vector, production pool, field representation, C or assembly backend.

All three Rust/xtask native commands and four read-only linked-code commands pass on first execution. The new reader reconstructs source recovery, command/stream/binary hashes and every static instruction/IFMA count from saved disassembly. It reuses the established scalar/curve test implementations rather than supplying a second independent cryptographic implementation. No successful prior test, recording, oracle or decoder is replayed.

The [prospective early screen](artifacts/early-protocol.md) fixes 200/352-pair complete callers, 1/32 workers, exact frozen/fresh-OFF/ON controls, seven blocks, full generator checks, zero-allocation/capacity checks and automatic post-latency PMU/profiles. It is an early rejection/selection stage, not adoption. No timing has run, so this report supplies no performance decision. A passing component would still require full caller and all-shape SDK/resource verification.

The bounded source/history review distinguishes this factor association from prior reduced-Gauss, norm-scale, two-chain and shared-product recipes. Their earlier failures remain unchanged. No exhaustive novelty or twofold proof-speed claim is made. The separate [default carry experiment](../2026-11-08-uniform-carry-default-performance183/README.md) remains failed at three 16-worker checks, despite its verified positive results at 32 workers; this qualification does not rescue it.

## Accepted CPU and Metal comparison, inherited

The following exact accepted-SDK observations come from that completed campaign. None measures this new factor-placement candidate. Supplied M5 references use different hardware.

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

Accepted merge54 remains 162.442299 ms, without a supplied M5 reference. The verified CPU AVX 10× goal remains active and unmet, 0/8 shapes. No SDK requests or independently verified proofs are added; canonical inventory stays 36,477. No new AWS/Aeglos, huge-circuit, phone/ARM/TEE, pressure/swap, minimum-RAM or concurrent-throughput result is claimed.

[Design](artifacts/design.md), [prospective screen](artifacts/early-protocol.md), [native qualification](artifacts/native-qualification.json), [linked code](artifacts/codegen.json), [review](artifacts/review.json), [evidence archive](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
