# Unit-Z mixed-add qualification

**Correctness and emitted-code checks pass; performance is unmeasured and production is unchanged.** A private Rust prototype specializes mixed addition when the left Jacobian point has Z=1. It uses the standard [Bernstein–Lange mmadd-2007-bl formula](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html), costing 4 base-field multiplications and 2 squares instead of the general method's 7 multiplications and 4 squares. Equal points, opposite points, identity and nonunit-Z inputs retain the accepted complete behavior. This is a standard formula specialization, not a mathematical novelty or a proof-speed result.

Both curves pass the new independent Arkworks oracles. Outputs also match the accepted operation's **exact Jacobian coordinates**, and arguments remain unchanged. Inputs include rejection-sampled full-width scalars and field scales, signed points, identity, doubling, cancellation, unit/nonunit scales, unusual identity representations and successive mixed-add transitions.

|Execution|Independent affine output checks|Additional changing-chain transitions|Tests|
|---|---:|---:|---:|
|Native, main + workers/concurrent callers|19,320|2,576|2|
|Generic x86, no ASM / no IFMA|11,520|1,536|1|

The generic main population repeats the native main arguments through a different original field backend; these are execution counts, not wholly distinct mathematical fixtures. Native worker coverage broadcasts to all 53 workers across 1/4/16/32 pools with requested 512KiB stacks. Four OS callers also share one eight-worker Rayon pool. Eight is the pool configuration, not evidence that every worker executed a concurrent call. These are component correctness checks, not SDK throughput or physical ARM/phone qualification.

The actual native qualification executable is decoded once. Both private entrypoints call their retained generic kernels; the nonunit fallback and equal-point double remain out of line. The inspected kernel bodies contain the specialized arithmetic:

|Curve|Unit-Z kernel MULX sites|Accepted general mixed-add MULX sites|
|---|---:|---:|
|G1|192|352|
|G2|512|928|

These are static instruction sites in different function bodies, not executed instructions, cycles, stack peaks or proof speed. The unit-Z wrapper adds a prologue and checks, and its nonunit fallback repeats the accepted identity guards. A production integration would use the guards already inside the existing method and must qualify its actual artifact. The present decode does not establish a general improvement at every callsite or eliminate compiler/inlining effects. No IFMA stream arithmetic is changed.

Five serialized Rust/xtask commands pass on first execution: native build/test, generic build/test and actual native decode. The review verifies 190 pins, all five command arguments/exits/stream hashes, test summaries, unchanged 176 accepted production files, kernel guards and the selected symbols/counts. Helper compiler/review/publisher invocations have tool-transcript evidence rather than complete xtask receipts. One unused-import warning is retained; no successful build or test is repeated for cleanup. The review is a separate reader by the same agent, not an external audit.

There is no heap allocation expression or unsafe block in the kernel, and no new table, key/point copy, solver plan, pool, workspace forecast or memory cap. That source observation does not prove stack, RSS or minimum-RAM bounds. The accepted field libraries and existing field/core/Window/proof qualification are reused; no old suite is replayed. No coherent core or SDK integration exists yet.

Next, count unit-Z eligibility in actual complete proofs across shapes before deciding whether a timing campaign is useful. Main SIMD affine streams bypass this scalar operation, so this specialization cannot be projected onto their profile shares. Earlier endpoint, queue-drain, bucket-tree and normalization experiments remain separate results. If eligible work is material, integrate into the original method and qualify that artifact before freezing matched complete-proof controls and automatic profiles.

Zero new SDK requests, Gnark exports, profiles or latency observations are added. The canonical independently verified proof ledger remains **41,792**. No AWS/Aeglos, huge-circuit, ARM/mobile/TEE, pressure/swap/minimum-RAM or concurrent-throughput comparison is added. There is no measured speedup from this prototype.

The following observations are inherited from the [adopted bitmap-reset campaign](../2026-11-09-busy-reset-adoption176/README.md), not results of this kernel. M5 CPU and Metal timings are supplied historical references on different hardware.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.809034|37 / 4.738x|24 / 3.073x|3.7|
|2x2|10.940974|59 / 5.393x|30 / 2.742x|5.9|
|4x2|15.750370|109 / 6.920x|48 / 3.048x|10.9|
|8x2|23.951790|189 / 7.891x|74 / 3.090x|18.9|
|12x2|33.857141|284 / 8.388x|101 / 2.983x|28.4|
|16x2|39.726526|335 / 8.433x|124 / 3.121x|33.5|
|24x2|65.984808|528 / 8.002x|176 / 2.667x|52.8|
|merge-24x1|65.660588|507 / 7.722x|175 / 2.665x|50.7|

The full verified CPU10x objective remains active and unmet, **0/8**. Accepted primary peak process RSS is 132.50MiB; merge54 remains 163.167792ms without a supplied M5 reference. The baseline was already below 8ms; historical 7.699867ms used a different campaign/statistic.

The first publisher stopped while validating its checksum link before creating the checksum file. The initial publisher source and partial publication are retained; adding the missing placeholder fixes publication ordering. All qualification, test, review and decode results are reused unchanged.

[Design](artifacts/design.md), [kernel](artifacts/kernel.rs), [independent curve tests](artifacts/qualification.rs), [review and symbol counts](artifacts/review-result.json), [continuation](artifacts/next.md), [source and command receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Executables/libraries, unchanged source snapshots, full raw decode and all key/witness/proof payloads are omitted. Selected disassembly is in the archive; the full local decode has a hash and size in the index.
