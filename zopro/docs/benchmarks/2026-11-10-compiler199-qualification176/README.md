# Current-source Rust 1.99.1 qualification

**Rust 1.99.1 / LLVM 23.1.1 passes current-source native and generic correctness qualification. Performance is unmeasured; no build is adopted.** Both optimized SDK executables are ready for a fixed complete-proof comparison. The accepted source, dependency versions, production executable and all-eight 10× objective remain unchanged.

This compares whole compiler packages: fresh OFF uses Rust 1.98.1 / LLVM 22.1.8; fresh ON uses installed Rust 1.99.1 / LLVM 23.1.1. Both copy all 176 accepted files byte-exactly, including Cargo.lock and registry Rayon. All accepted manual flags, including `zopro_dense_busy_reset`, are retained. Native O3, fat LTO, one codegen unit, debug information, the accepted allocator/driver and default worker stacks match. Explicit absolute compiler paths and matching eight-library sets prevent cross-version linkage. No bitmap, key table, solver plan, production pool, source optimization or dependency update is introduced.

|New-compiler suite|Native passes|Generic no-ASM/no-IFMA passes|
|---|---:|---:|
|BN254 field/curve/pairing library|35|11|
|Core|127|97|
|Prover library|2|2|
|Cubic integration|6|6|
|Total|170|116|

All suites are unfiltered, with zero failed or ignored tests. Field tests use independent Arkworks arithmetic. Generic execution is on this x86-64 host, not ARM or a phone. A compiler change can affect both implementations; independent Gnark proof verification supplies a separate implementation check. Original 1.98 correctness qualification is inherited after exact source/flag checks, while its fresh libraries provide a contemporaneous control.

The final SDK links use explicit O3/native/fat-LTO/one-CGU arguments. ELF compiler markers match each selected version and exclude the other. Symbol receipts and exact command/exit/stdout/stderr hashes are checked. Both selected and transitive library identities are frozen before requests. Initial compiler-library selection matched `.so` suffixes; a prospective pin extension also binds all root library files, including the actual non-`.so`-suffix libLLVM target. No compilation or successful test is repeated for this extension.

All 26 functional preflight processes pass. Each makes three primes and one SDK request; all 26 final exports independently pass Gnark with image/public bindings and are distinct from the prior ledger. This adds **104 SDK calls and 26 distinct exports**, advancing the canonical inventory **46,802 → 46,828**. Coverage is all eight supplied circuits plus 54→1 at 32 workers, with primary and 54→1 also at 1/16 workers. Preflight summaries are not performance observations.

All **68 workflow commands** succeed on first execution: eight compiler/version/build/test steps, four explicit link/symbol steps, four analysis-helper builds, and 52 SDK/verifier preflight steps. Preparers, helper compilation, inventory and prospective review are direct invocations outside that count. No runtime, test, proof, decoder or verifier is replayed for bookkeeping. Source and compiler inputs remain pinned; production is unchanged.

The full performance plan is frozen and has not started at this checkpoint: accepted REF plus fresh OFF/ON; 591 latency processes across all eight shapes plus54 and primary1/16; nine PMU panels and nine fixed-period cycle profiles; then45offline commands and609independent Gnark checks. Its79unchanged rules cover11control calibrations,22latency nonregressions,22RSS guards,4primary gain/win checks,2all-eight aggregate gains,9PMU and9profile quality checks. The same qualified paired-log bootstrap, fixed schedule and CPU placement are retained. Both raw readers are frozen before the first preflight. Any failed condition closes this exact recipe; a favorable subset is not adoption. No speedup from a release note, compiler version or static source identity is assumed.

Research consulted the official [Rust 1.99 announcement](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/) and [LLVM 23 release notes](https://releases.llvm.org/23.1.0/docs/ReleaseNotes.html). Their existence does not establish a benefit for this prover. The historical Framework build used1.99on different source/hardware; it is not a matched current176comparison. This experiment measures frontend, optimizer, backend, standard-library and other package effects together, not LLVM alone. Revisited MSM tables, SRE and memory-banded algorithms already have local studies. A proposed A/B1 difference basis is excluded by the current CPU path: rerandomized proofs already omit B1.

The following accepted local observations are inherited from the preceding bitmap campaign, using the unchanged accepted executable, not either new compiler build. Supplied M5 CPU/Metal values are historical references on different hardware. The verified CPU AVX ≥10× M5 CPU objective remains **active and unmet, 0/8**.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.864582|37|24|3.7|
|2×2|11.013809|59|30|5.9|
|4×2|15.916562|109|48|10.9|
|8×2|24.100975|189|74|18.9|
|12×2|33.846893|284|101|28.4|
|16×2|39.951809|335|124|33.5|
|24×2|66.273552|528|176|52.8|
|24→1|65.654589|507|175|50.7|

Inherited local54→1is162.996094ms, with no supplied M5 reference. No new latency, peak/minimum RAM, memory-pressure/swap, AWS/Aeglos, huge-circuit, phone/ARM/TEE or concurrent-throughput result is established. The [accepted AWS comparison](../2026-11-10-accepted-current-c7a176/README.md) remains separate; no cloud resource is added here.

[Source/build review](artifacts/build-review.json), [source identities](artifacts/source-audit.json), [selected libraries](artifacts/libraries.json), [prospective review](artifacts/prospective-review.json), [preflight review](artifacts/preflight-review.json), [design](artifacts/design.md), [authored helpers, plans, pins and raw command streams](artifacts/evidence.json.gz), [publication checks](artifacts/quality.json), [checksums](SHA256SUMS), [preceding full-SDK comparison](../2026-11-10-ready-victims-sdk176/README.md). Keys, requests, solver/witness/proof payloads, full canonical ledgers, executable/library blobs, unchanged production source and complete third-party trees are excluded. Absolute local paths require adaptation to reproduce. Private root: `/home/ubuntu/src/zopro-perf-work/compiler199-sdk176`.
