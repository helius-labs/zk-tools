# Native qualification of local Karatsuba tiles

**Native correctness and code-generation checks pass; performance remains unmeasured.** The fixed local-tile implementation passes 554,496 raw field-result comparisons, 554,496 canonical-export comparisons and exact original CIOS lazy-digit equality. All twelve linked operation consumers contain the intended 96/142 IFMAs and no calls. Production remains unchanged; no speedup or adoption follows from these checks.

This implements the [separately qualified mathematical schedule](../2026-11-08-blocked-pair52-model176/README.md): two local 2×2 Karatsuba tiles inside paired CIOS rows, with seven coefficients. Other raw products, sign orientation, modulus constants, final 48/52-bit cancellation, normalization and positive-complement difference remain as declared. Source IFMA counts fall from 100/150, but sign/recombination and other ALU work remain. The previous full-pair, deferred-sign and unchanged row-pair negative experiments stay closed.

## Native coverage

One coherent native field build, one fat-LTO test link, one focused test execution and one generic no-ASM/no-IFMA field build complete successfully once. The test applies the previous field-output oracle population to a different kernel and adds inclusive 2p endpoints; these are reused oracle families, not new independent proof vectors. No existing successful model, native test or capture is replayed.

|Field/radix|Boundary values|Boundary/random/changing rows|Raw results|Canonical exports|
|---|---:|---:|---:|---:|
|Fq/R256|52|2704 / 2048 / 1024|138,624|138,624|
|Fq/R260|52|2704 / 2048 / 1024|138,624|138,624|
|Fr/R256|52|2704 / 2048 / 1024|138,624|138,624|
|Fr/R260|52|2704 / 2048 / 1024|138,624|138,624|

Each row checks eight lanes for multiply, sum2 and difference2. Every normalized raw digit array equals original CIOS, each numerical output is strictly below 2p, independent Arkworks and BigInt references agree, and compact R256 export is canonical. Changing chains consume native outputs. Raw inputs are constructed only in the investigation's unsafe SIMD constructor; no invalid public scalar field objects are created.

All 176 accepted production hashes remain exact. In the isolated 177-file copy, 175 originals are byte-identical, removing one appended module include recovers the remaining parent exactly, and the new child matches its frozen source. No production dispatcher selects it. The unchanged generator and full native suites are inherited, not newly run. Generic field compilation checks module gating; it does not qualify ARM, the complete portable prover or mobile devices.

## Actual linked code

Each tested operation uses a no-inline consumer, so all twelve field/radix/operation bodies remain linked and executed by the oracle test. Each is decoded once. Both primes have the following per-consumer counts:

|Operation/radix|Symbol bytes|Instruction sites|IFMA sites|Explicit RSP memory operands|
|---|---:|---:|---:|---:|
|mul/R260|1,227|200|96|6|
|mul/R256|1,283|207|96|6|
|sum2/R260|1,864|303|142|8|
|sum2/R256|2,093|337|142|12|
|difference2/R260|2,027|328|142|12|
|difference2/R256|2,254|360|142|18|

All have zero call instructions. These are static test-consumer counts, not SDK code, retired instructions, spills, latency or RAM peaks. Additional ALU work remains visible despite lower IFMA counts. The seven-coefficient source array excludes inputs, temporaries and compiler allocation and does not establish a stack or minimum-RAM bound. No lookup table, key copy, retained plan or production pool is introduced.

The offline reader checks all 381 codegen pins, 16 successful command receipts with exact argv/stream hashes, source recovery, native-output accounting and every saved linked decode. It does not rerun arithmetic, models, native tests, disassembly or proof verification.

Preparation failures are retained: the initial pin helper tried hashing an unbuilt test executable; an xtask setup then lacked pins; its output directory prevented reuse after pin repair. None ran a native command. A fresh output directory executes the unchanged four-step plan once. A later reader assertion incorrectly equated source absolute-value operations with `vpabsq` mnemonic counts in difference consumers. Its repair records actual counts; IFMA, call, source and output checks remain. No kernel, test, decode or performance gate changes. [Preparation notes](artifacts/preparation-failures.json) and the exact initial/fixed reader sources are archived.

The qualified reader source is `finish-qualified.rs` inside the archive, matching its hash in qualification.json. The subsequent publisher changes only the archive member list to include repair evidence; qualification is not replayed. Inherited unused-import/dead-code warnings and two harmless helper/test unused-import warnings remain without cleanup rebuilds.

## Next experiment and unchanged goal

Next is a separately frozen matched changing-input screen: original frozen field code, fresh OFF, paired-row schoolbook and local tiles, including threaded execution and automatic profiles. It must retain full outputs, allocation/capacity checks, all observations and old failures. No source-count gain is promoted to proof speed. Complete SDK/all-eight/54/primary16/Gnark/resource/portable qualification remains necessary before adoption.

There are **zero new timings, profiles, SDK requests or proofs** here. Production176/accepted SDK6d83 and canonical proof count35,171 remain unchanged. The full [inherited all-eight M5 CPU and Metal comparison](../2026-11-08-blocked-pair52-model176/README.md#accepted-cpu-and-metal-comparison-inherited) remains current: local 1×2 about 8 ms, previously 7.915/7.929/7.996 ms with the same binary and latest 8.003 ms. The cause of variation remains unconfirmed, without an adopted regression. The CPU10× target for primary is 3.7 ms; the full objective remains active and unmet, **0/8**. No fresh AWS/Aeglos, huge-circuit, mobile, pressure/swap, concurrent-proof, peak-RSS or minimum-RAM claim is made.

[Native design](artifacts/design.md), [kernel](artifacts/blocked_pair52.rs), [native oracle](artifacts/native-tests.rs), [qualification](artifacts/qualification.json), [linked counts](artifacts/codegen.json), [native plan](artifacts/native-plan.json), [pins](artifacts/native-pins.json), [exact sources/receipts/decodes](artifacts/evidence.json.gz) and [checksums](SHA256SUMS) are retained. Every UTF-8 archive payload and gzip roundtrip is checked. Unchanged full sources, compiled binaries/libraries, raw perf data, keys/images/witnesses/proofs and third-party full text are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/blocked-pair52-native176`.
