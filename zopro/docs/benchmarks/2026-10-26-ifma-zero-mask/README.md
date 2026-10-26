# IFMA zero-accumulator mask audit

The proposed cleanup removes no AND instructions from the 24 tested field kernels. All 12 eight-lane kernels match the accepted instruction streams after relocation/constant normalization. The compiler already removes the 52-bit cancellation mask in these arithmetic contexts. The audit closes without timing, proofs or adoption; production remains unchanged.

## Instruction contract and proposed change

With a zero accumulator, IFMA low-half multiplication produces a value below 2^52, so AND with 2^52−1 leaves it unchanged. Both operands contribute only their low 52 bits. This follows from the primary Rust documentation for the [512-bit intrinsic](https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_madd52lo_epu64.html) and [256-bit intrinsic](https://doc.rust-lang.org/core/arch/x86_64/fn._mm256_madd52lo_epu64.html). The inference applies specifically to zero accumulation; it is not permission to remove arbitrary limb or mixed-radix masks.

The private candidate changes two cancellation sites in the four-lane template and regenerates its eight-lane counterpart. Compile-time `zopro_unmask_m52` selects the unmasked zero-accumulator result only for radix-52 cancellation. The final radix-48 correction, every limb normalization, arithmetic recurrence and field layout remain. Both whole original files recover through documented inverse edits; the other 168 accepted sources match. No key, heap buffer, table, core policy or representation changes.

Initial inspection of the actual accepted G2 caller found two IFMA/AND pairs using the required 48-bit constant. Its ELF virtual address is 0xab98, with little-endian bytes `ff ff ff ff ff ff 00 00`. Matching nearby instructions alone does not establish dataflow or a redundant mask. The full preceding [SDK report](../2026-10-26-streamed-complex-sdk/README.md) retains those caller disassemblies.

## Independent ISA checks

The raw probe checks 100,032 lane results against independent u128 multiplication: 33,344 four-lane and 66,688 eight-lane results. Boundary and random full-width u64 operands cover ignored high operand bits. Every result has its upper 12 bits clear and equals its masked form. A further 200,064 high-half product checks validate both masked and unmasked feeds into IFMA. Twelve counterexamples demonstrate that a 48-bit result mask changes the value and must remain where the algorithm requires it. These are intrinsic conformance checks, not an independent complete field proof.

| ISA context | Four lanes | Eight lanes |
| --- | --- | --- |
| Masked/unmasked result fed into IFMA | same normalized instructions | same normalized instructions |
| Masked/unmasked result returned directly | same normalized instructions | different instructions |

The isolated eight-lane return still has an AND52, while both feed contexts eliminate it. That return context is not the prover's cancellation chain. The four-lane 48-bit mask lowers to a blend rather than an AND; an AND count alone does not identify every mask implementation.

## Actual field code comparison

Two probes link the same 24 exported wrapper signatures against the qualified accepted field library and the freshly built candidate. The matrix covers Fq/Fr, R256/R260, four/eight lanes and multiplication/sum-of-two-products/difference-of-two-products. Field probes are compiled for inspection, not run as field tests or timed.

Every disassembled instruction is bound to its exact ELF bytes and contiguous function extent. Normalization retains opcodes, registers and immediates while replacing RIP-relative displacements with the referenced constant bytes. There are no calls in these wrappers. Instruction, AND, IFMA, RSP-reference and literal stack-subtraction counts match in all 24 pairs. Sixteen complete normalized streams match; eight four-lane streams differ, so full machine-code equivalence is not claimed for those eight.

Each row applies separately to both Fq and Fr; their static counts agree. Counts below are the same in accepted and candidate.

| Radix | Lanes | Operation | Instructions | AND sites | IFMA sites | Matching normalized pairs |
| --- | ---: | --- | ---: | ---: | ---: | ---: |
| 256 | 4 | mul | 191 | 5 | 105 | 0/2 |
| 256 | 4 | sum2 | 280 | 10 | 155 | 0/2 |
| 256 | 4 | difference2 | 301 | 15 | 155 | 2/2 |
| 256 | 8 | mul | 191 | 9 | 100 | 2/2 |
| 256 | 8 | sum2 | 278 | 14 | 150 | 2/2 |
| 256 | 8 | difference2 | 298 | 19 | 150 | 2/2 |
| 260 | 4 | mul | 172 | 4 | 105 | 0/2 |
| 260 | 4 | sum2 | 232 | 4 | 155 | 0/2 |
| 260 | 4 | difference2 | 255 | 9 | 155 | 2/2 |
| 260 | 8 | mul | 171 | 4 | 100 | 2/2 |
| 260 | 8 | sum2 | 231 | 4 | 150 | 2/2 |
| 260 | 8 | difference2 | 253 | 9 | 150 | 2/2 |

The four-lane scheduling/register/encoding differences do not remove the targeted mask operations. For example, R256 multiplication changes from 1113 to 1106 bytes but remains 191 instructions with five AND sites. The fixed screen requires a remaining mask-elimination mechanism before performance work; none is observed. Incidental layout changes are not presented as an optimization or a speed result. The next step needs a material arithmetic/dataflow change rather than this source cleanup.

## Verification and limits

The native build, generator consistency check, ISA run and both exact-library links pass. All 183 input pins and both linked libraries/probes pass the closing hash audit. Both modified field files recover their complete original digests; accepted production's 170 sources and qualified SDK remain unchanged. The inverse-edit audit initially expected an unformatted one-line template in the generated eight-lane file; its formatter had expanded the conditional. The check was repaired to match that generated formatting while still requiring the original whole-file SHA. The initial audit source and repair note remain; field/build/probe/codegen sources and all 183 pins did not change.

No timing benchmark, new proof, Gnark export, profile, complete field oracle, SDK trial, portable device, pressure, large-circuit, AWS, Aeglos or minimum-RAM qualification follows from this screen. It describes this native compiler/artifact set and does not claim that every compiler or target removes every redundant mask. No adoption is made.

## Inherited accepted prover context

These are unchanged accepted timings against supplied historical M5 CPU/Metal references, not measurements from this audit.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 | 8.594267 | 37.0 | 4.305× | 24.0 | 2.793× |
| 2x2 | 11.697247 | 59.0 | 5.044× | 30.0 | 2.565× |
| 4x2 | 16.158262 | 109.0 | 6.746× | 48.0 | 2.971× |
| 8x2 | 24.477005 | 189.0 | 7.722× | 74.0 | 3.023× |
| 12x2 | 34.386319 | 284.0 | 8.259× | 101.0 | 2.937× |
| 16x2 | 40.867383 | 335.0 | 8.197× | 124.0 | 3.034× |
| 24x2 | 69.265160 | 528.0 | 7.623× | 176.0 | 2.541× |
| merge-24x1 | 68.150148 | 507.0 | 7.439× | 175.0 | 2.568× |

Accepted merge54 remains 178.891799 ms; no M5 merge54 reference was supplied. The all-eight CPU 10× goal remains active and unmet (0/8).

[Declared screen](artifacts/experiment.json), [candidate edits](artifacts/edits.json), [all code comparisons](artifacts/codegen.json), [disassembly](artifacts/codegen.txt), [ISA source](artifacts/isa.rs), [ISA results](artifacts/build/isa-check/stdout.log), [closing audit](artifacts/closeout.json), [inverse-check repair](artifacts/closeout-repair.json) and [artifact index](artifacts/artifact-index.json). Executables/compiler caches are represented by source, commands and pinned hashes.
