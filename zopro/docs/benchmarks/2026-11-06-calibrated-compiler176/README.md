# Partial Zen 5 compiler scheduling qualification

The isolated compiler experiment passes source/model review and **324 complete-caller correctness checks across 108 contexts**. It changes instruction ordering in the actual G1/G2 kernels while retaining their IFMA, instruction and vector-stack-operand counts. **No latency trial or adoption has occurred.** Production and accepted proof times remain unchanged.

## Change and provenance

Rust 1.98.1 commit [48a229ce](https://github.com/rust-lang/rust/tree/48a229ceaefd4985c50990b14116b6d856af0985/src/llvm-project) pins LLVM fork [52ed14fc](https://github.com/rust-lang/llvm-project/blob/52ed14fcd56afc30f9cccd8ca8ce237c2eef7e04/llvm/lib/Target/X86/X86.td), where `znver5` uses `Znver4Model`. This closes the earlier audit's uncertainty about the pinned Rust fork's source. It does not establish a compiler defect or identify the cause of a full-proof bottleneck.

The private backend adds a separately named copy of that model and changes four unmasked ZMM register opcodes: IFMA low/high and packed-qword add/subtract. The [completed hardware calibration](../2026-11-06-instruction-calibration176/README.md) supplies the selected costs. No new hardware calibration is run.

| Register instruction | Original model latency / reciprocal throughput | Private model latency / reciprocal throughput |
|---|---:|---:|
| IFMA low/high | 4 / 2 | 4 / 0.5 |
| qword add | 1 / 0.5 | 2 / 0.25 |
| qword subtract, nonzero idiom | 1 / 0.25 | 2 / 0.25 |

These are model cycles, not caller measurements. Eighteen model probes confirm all four changes, fourteen unchanged forms including subtraction's zero idiom, and unchanged tested Zen4 behavior. Memory/masked forms, all shifts, other widths and other CPUs retain inherited entries. The known shift mismatch remains; physical-resource mappings are inherited rather than independently calibrated. This is a partial model experiment.

Both owned backends build from the same pinned LLVM tree with the same clang18/configuration. Common dependencies are built once; the second build updates the changed model's generated tables and affected compiler objects. No system compiler is replaced. The source review checks 77,147 visible archived source files: only X86.td changes among those originals, with one added model. Independently reversing the namespace, opcode-list and model edits recovers the original files exactly.

One new accepted-Rust caller build emits final post-LTO bitcode, SHA256 `86e5bf731834dce075f359d1bd7bed6fc4d88fae86f0cdaae08e48fbdc40ee79`, and records its actual linker invocation. Both owned backends compile that same module with matching native/O3/section/PIC/trap settings; each link replaces only the original object and output path. The selected functions retain explicit `znver5`/IFMA attributes. Pinned Rust code-generation sources establish where final bitcode is emitted and the target options used.

## Final consumer code

Each cell is instruction rows / IFMA sites / vector stack-operand rows. Nine unique final functions are decoded once; no old decode or successful qualification is repeated. Counts cover whole functions, including cold paths.

| Function | Original Rust compiler | Owned OFF backend | Owned ON backend |
|---|---:|---:|---:|
| G1 borrowed signed | 4425 / 800 / 286 | 4424 / 800 / 286 | 4424 / 800 / 286 |
| G2 borrowed signed | 8590 / 1450 / 603 | 8592 / 1450 / 603 | 8592 / 1450 / 603 |
| Eight-lane inverse bridge | 1139 / 500 / 67 | 1139 / 500 / 67 | 1139 / 500 / 67 |

ON/OFF G1 and G2 opcode order differs. This does not prove a faster schedule, fewer retired instructions, lower spill cost or less RAM. Original Rust and re-emitted OFF are not byte-identical; future matched performance comparisons still require control calibration.

## Correctness and limits

Three artifacts × both curves × eighteen lengths × 1/16/32 workers give 324 successful processes. Lengths cover empty input, lane boundaries, full batches and tails through 512. Every process passes the unchanged three 128-round checked primes, reset and 64-round generator-point oracle, output-hash, worker-affinity, unchanged-capacity and zero-timed-allocation guards. Worker stacks are configured to 512 KiB. All three artifacts agree on complete outputs and scratch capacities in every context. Their arithmetic CLI arm remains OFF; ON here denotes the compiler artifact. The rejected split-REDC arithmetic is not used.

The independent Rust reviewer checks all 359 preparation/build/link/model/decode/caller command receipts, source recovery, pins, model probes, final IR attributes, link substitutions and all 324 outputs. Its own build and execution also succeed once. These caller tests do not constitute fresh unfiltered field/core/prover suites, full-SDK/Gnark proof qualification, a precise RAM peak or minimum-RAM evidence.

The compiler changes neither Rust field algorithms nor the prover's architecture. No lookup table, copied key/point plan, production pool, runtime flag or C/assembly prover rewrite is introduced. Compiler build resources are not prover resources. No AWS/Aeglos, phone/TEE, huge-circuit, pressure, swap or throughput result is added. The canonical proof ledger remains30,846.

Preparation observations are retained: two guessed model filenames returned404; an IR launch stopped at the host lock before any command; CMake warns that one supplied option is unused; Rust warns about adapting multiple output names. Source review corrected subtraction's existing zero-idiom override and the inverse-function selector before their first relevant native compilation/decode. No successful measurement or qualification is rerun to resolve those observations.

Next: freeze a complete-caller matched latency/PMU/profile protocol against both the original Rust artifact and owned OFF, including tails and1/16/32workers. Only a qualified caller gain advances to all-eight plus merge54 SDK, independent Gnark/freshness, portability and resource qualification. There is no predicted twofold gain or performance adoption from this stage.

## Accepted CPU and Metal comparisons, inherited

These are accepted local32-worker proof times and supplied M5 references, not new compiler results. All eight CPU10x targets remain unmet. The accepted physical16 calibration engineering exception and earlier false full gates remain documented in the [accepted report](../2026-10-30-g2-vector-sign-adoption176/README.md).

| Shape | Accepted ms | M5 CPU ms / speedup | M5 Metal ms / speedup | CPU10x target ms |
|---|---:|---:|---:|---:|
| 1x2 | 7.928660 | 37 / 4.667x | 24 / 3.027x | 3.7 |
| 2x2 | 11.151932 | 59 / 5.291x | 30 / 2.690x | 5.9 |
| 4x2 | 16.086386 | 109 / 6.776x | 48 / 2.984x | 10.9 |
| 8x2 | 24.179453 | 189 / 7.817x | 74 / 3.060x | 18.9 |
| 12x2 | 33.884455 | 284 / 8.381x | 101 / 2.981x | 28.4 |
| 16x2 | 40.077381 | 335 / 8.359x | 124 / 3.094x | 33.5 |
| 24x2 | 65.850777 | 528 / 8.018x | 176 / 2.673x | 52.8 |
| merge24 | 65.060937 | 507 / 7.793x | 175 / 2.690x | 50.7 |

Merge54 remains161.883899ms without a supplied M5 reference. Inherited primary132.75MiB RSS is not a new peak or minimum RAM. Accepted SDK SHA256 remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`; the all-eight CPU10x goal remains active and unmet0/8.

[Design](artifacts/design.md), [correctness protocol](artifacts/correctness-plan.md), [independent qualification](artifacts/qualification.json), [model patcher](artifacts/patch-model.rs), [reviewer](artifacts/qualify.rs), [preparation notes](artifacts/preparation.md), [source/helpers/receipts](artifacts/helpers-and-receipts.json.gz), [archive member index](artifacts/archive-index.json), and [checksums](SHA256SUMS). The archive excludes compiler archives, LLVM source copies, optimized IR, objects, executables/libraries, private inputs and keys. Those local artifacts are identified by hashes and pinned source revisions.
