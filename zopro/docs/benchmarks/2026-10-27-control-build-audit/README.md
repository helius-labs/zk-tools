# Source-identical SDK build audit

Two independent fresh builds produce the **same SDK executable**, SHA256 `6740f320700f954d1e9c2fa46883ea1aeaae5c7c551834c996833b8da3c3b430`. Compared with accepted `8241648f…`, all **826,692 decoded text instructions** match after normalizing only RIP displacements. All **5,344 identified readonly literal loads** read equal file-image payloads. The audit finds no changed arithmetic operation, register, stack offset, immediate or branch destination in this comparison.

This narrows the build discrepancy; it **does not explain the latency difference** or resolve the five failed control-calibration bounds in the preceding study. Production, the accepted SDK and all earlier measurements/verdicts remain unchanged. No new proof, native build, timing or profile capture is run.

## Inputs and comparison

All 171 production source hashes match the accepted manifest. Both fresh snapshots match those hashes. Standard driver/support bytes, recorded direct-rustc optimization flags and build runtime environment agree. Cargo core fingerprints agree on compiler, features, profile and Rust flags; build-script fingerprint values differ, but their retained directive outputs are identical. The direct link uses the same optimization defaults when flags are omitted from the plan. Command receipts, rather than omissions in plan JSON, establish this.

The selected libc, memmap2 and Rayon archive bytes match accepted. Selected serde_json, BN254, core, prover and SDK archives differ. Intermediate archive differences therefore cannot alone establish different executable work: the two fresh builds have differing archives but identical final executable hashes. This audit checks the recorded direct libraries and core fingerprint; it is not a reconstruction of every historical toolchain input.

Accepted and fresh ELF files are both 8,089,056 bytes with identical section layout. All 3,095 named text-function entries have the same names, virtual addresses and sizes. Static comparison covers the entire `.text`, including padding, and preserves instruction addresses, opcodes, registers, stack offsets, immediates and branch targets. Only RIP displacement values and disassembler comments are normalized.

|Section|Bytes|Different file bytes|
|---|---:|---:|
|.text|4,070,726|822|
|.rodata|527,344|21,341|
|.rela.dyn|39,648|1,199|
|.data.rel.ro|34,720|4|
|.symtab|126,768|15|
|.note.gnu.build-id|36|20|

There are 22,674 decoded RIP references; 794 refer to different virtual addresses. For 5,344 explicit-width reads into `.rodata`, the referenced file bytes agree at the decoded widths. Address-only LEAs, indirect calls, stores and mutable/relocated pointer tables require different reasoning and are not called literal-load equivalence. The full binary is not claimed byte-identical or proven semantically equivalent. Runtime ASLR, dynamic relocation state, cache behavior, scheduling and frequency were not measured by this audit.

## Limits and next action

The previous prefix-cutoff result remains: primary paired saving **3.998 µs (0.048%)** versus accepted, interval including parity; candidate/fresh gain about 1.12%, with failed control calibration. No small gain is reclassified as zero, and no failed bound is waived. The candidate remains unadopted.

Four observer tests pass, including distinct file/virtual offsets, preservation of non-RIP operands and exclusion of overlapping nonallocated debug sections from address lookup. Original failed fixtures and superseded observer outputs are retained with repair notes. No executable, source, latency capture or profile was changed or repeated to repair the observer. Complete disassemblies remain local with checked hashes; JSON summaries, reference metadata and observer sources are archived.

The user requested a fresh AWS comparison and confirmed login. Next, compare the exact adopted source with pinned Aeglos on one owned c7a.24xlarge, including all nine available circuit shapes and the 54→1 merge case. Declare build, timing, independent verification and cleanup before provisioning; report current CPU/Metal references without inferring AWS gains from local results. The old C7a primary measurements (12.327 ms Zopro, 8.121 ms Aeglos) belong to an earlier Zopro source. This audit adds no AWS result.

The global independent-proof ledger stays **9,904**. The original all-eight supplied M5 CPU 10× objective remains active, achieved 0/8. Historical CPU/Metal values and all shape results are in the [preceding SDK report](../2026-10-27-grouping-prefix-cutoff/README.md); no speed or memory qualification is newly claimed here.

[Design](artifacts/design.md), [identities and commands](artifacts/identities.json), [section comparison](artifacts/sections.json), [instruction comparison](artifacts/code.json), [RIP references](artifacts/RIP-references.json), [observer repairs](artifacts/observer-repairs.json), [closeout](artifacts/closeout.json), [artifact hashes](artifact-index.json).
