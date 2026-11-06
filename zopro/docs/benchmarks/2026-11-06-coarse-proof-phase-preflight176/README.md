# Coarse full-proof recorder: preflight failure

**Phase attribution failed before latency measurement.** The first enabled primary preflight returns an internally verified proof, then rejects its diagnostic graph because the canonical phase is absent. No latency block, PMU panel or cycle recording starts. This attempt is closed; source, qualified libraries, caller receipts and failure remain preserved. No performance or proving optimization is adopted.

## What ran

The private snapshot adds an 18-slot coarse recorder and hooks at prover encode/solve/verify, canonical solver/scalar preparation/FFT/query completion/serialization and final assembly. It changes no field arithmetic, FFT, solver or MSM algorithm, task geometry, pool or key data. Of 176 originals, 174 remain byte-exact; all originals recover by reversing diagnostic insertions. One new recorder module gives 177 files. Its four tests cover nested and disabled sessions, duplicate/unwind faults, concurrent proof-local callback ownership across 1/4/16/32 workers with 512 KiB stacks, and outstanding-span reset/snapshot protection.

All 129 unfiltered core tests, two prover unit tests and six cubic integration tests pass. The first prover command selected only library tests; the missing cubic target is added as a separate stage without repeating successful tests. Field qualification is inherited unchanged. Coherent libraries freeze and the SDK links with verified `-O`, native CPU, fat LTO and one codegen unit.

The first capture completes four commands: accepted caller, accepted Gnark verification, disabled caller and disabled Gnark verification. The next command, enabled caller, exits 101 in its first priming request with `required phase canonical`, observed state 0 versus expected 2. The failed workflow has no result.json; the terminal tool handle and per-step exits establish that it stopped. No latency block, counter panel or profile runs. Preflight clocks are retained but not used as performance results. The earlier source-generation helper also stopped on an encode anchor matching sync and async methods. Its logs and partial source are retained; narrowing the synchronous anchor precedes the first native build. No successful qualification or recording is repeated for that repair.

## Attribution boundary

The accepted small-core-entry path moves primary solving onto a Rayon worker and recursively calls solve_inner. The initial recorder selects context from caller TLS at method entry but does not transport it across that existing worker boundary. The downstream canonical hook consequently has no recorder. This is a diagnostic ownership failure, not evidence of a missing proving phase or failed proof. The next isolated revision must pass the owning context explicitly through the original worker entry and borrow it into callbacks. It must prove actual entry activation and graph completeness before latency, with fresh qualification for the changed integration. The original attempt remains immutable and is not restarted.

The recorder collects fixed coarse wall spans and no field, witness or key values. The disabled executable reports an **888-byte recorder payload** through the same compiled type; the enabled process stops before printing its summary. This type size excludes Arc bookkeeping, compiler frames, allocator overhead and up to three saved snapshots. No process-peak, minimum-RAM, pressure or portable-device result follows. Coarse elapsed spans include nested Rayon helping and waiting; they cannot establish exclusive CPU or removable-time fractions.

## Proof accounting and review

Accepted and disabled callers each prime three requests and time one; the enabled caller completes its first internally verified request before graph validation stops it. Thus nine SDK requests verify internally, but only two 164-byte proof exports exist. Both independently pass pinned Gnark verification with image/public/VK identity checks and are distinct from all 30,708 prior exports and from each other. The ledger advances once to **30,710**. No enabled proof export exists and no independent verification is claimed for it.

Failure review checks 777 pin occurrences, ten successful workflow commands and the one failed caller, all stream hashes, empty PMU/profile directories, proof lengths and verification bindings. Planned requests in the sealed capture plan are not executed-request counts. Successful proof/public pairs are retained; private keys, images and request witnesses are excluded. No new all-shape timing, AWS/Aeglos, phone, pressure, minimum-RAM, swap or concurrent-throughput result is claimed.

Accepted SDK `6d83…` and full-proof timings remain unchanged. The all-eight CPU10× goal is active and unmet: 0/8. The following local measurements and supplied historical M5 references are inherited.

|Shape|Accepted ms|M5 CPU ms|M5 Metal ms|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37|24|3.7|
|2x2|11.151932|59|30|5.9|
|4x2|16.086386|109|48|10.9|
|8x2|24.179453|189|74|18.9|
|12x2|33.884455|284|101|28.4|
|16x2|40.077381|335|124|33.5|
|24x2|65.850777|528|176|52.8|
|merge24|65.060937|507|175|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference.

[Failure review](artifacts/failure-review.json), [source recovery](artifacts/pre-native-review.json), [design](artifacts/design.md), [proof inventory](artifacts/proof-inventory-after-failure.json), [ledger](artifacts/proof-sha256-after-failure.json), [public exports](artifacts/public-proofs.hex.json.gz), [source](artifacts/source-snapshot.hex.json.gz), [receipts/helpers](artifacts/receipts.json.gz), [checksums](SHA256SUMS). Archives round-trip exact bytes, lengths and hashes. Executables, libraries and private proving inputs remain local with hashes. Reproduction requires the pinned toolchain, dependencies and private inputs. Private root: `/home/ubuntu/src/zopro-perf-work/coarse-proof-phase176`.
