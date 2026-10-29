# Static audit of the grouping OFF control

29 October 2026. Read-only analysis of existing executables after the [combined grouping/queue-drain experiment](../2026-10-29-cached-drain175/README.md). No prover is rebuilt or timed. No proof is added; the distinct-proof ledger remains 22,990. Production and every accepted benchmark result remain unchanged.

The combined OFF executable is **byte-identical to the earlier cached-hash grouping OFF executable**. The standalone queue-drain OFF executable is **byte-identical to accepted**. The observed control difference therefore predates the composition and is associated with the grouping integration; this does not isolate its runtime effect.

## Executable inventory

|Executable|SHA-256|Text bytes|Function symbols|
|---|---|---:|---:|
|Accepted / queue OFF|`20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f`|4,161,206|3,119|
|Grouping OFF / combined OFF|`8a51fc78d89cd823dff7bda6d133e33cfaa2c7eecdd926adb72d2e58e4950d47`|4,161,206|3,119|

All **3,036 demangled function-name inventories have identical size multisets**. This is a static observation, not runtime equivalence. Code-section hashes differ: 54,038 bytes differ at the same text offsets. Some names have aliases or multiple implementations, and symbol hashes change with crate identity.

Matching by demangled names rather than raw Rust symbol hashes finds:

|Comparison|Names/functions|
|---|---:|
|Exact address/size multisets|2,992 names|
|Unique matched function names|2,985|
|Unique matched functions with byte-identical bodies|2,918|
|Unique matched functions with changed body bytes|67|
|Names requiring separate alias/multiple-implementation review|51|

The initial raw-symbol comparison reports 2,349 mismatches, including absent hash-changed names. That count does **not** establish 2,349 moved functions. The later demangled-name and byte-range comparison above corrects that interpretation. Similarly, same-offset text differences do not identify changed instructions inside a moved function. All intermediate evidence is retained.

## Grouping and inversion

The 7,624-byte `grouping::scalar_groups` body occupies the same address in accepted and OFF. Its eight changed bytes are **eight RIP-relative displacements to two identical 16-byte constant payloads**, each relocated by 16 bytes. Instruction addresses, operation mnemonics, register operands and the remaining instruction text match after removing symbol annotations. The inspected body has no changed loop or arithmetic schedule.

Both reviewed `inverse::<FqConfig>` implementations retain identical address ranges and **byte-identical 7,723-byte bodies**. This resolves that particular duplicate-name ambiguity. The `PreparedEqualWitness` drop-glue aliases do have body/range differences; this audit does not declare all runtime or allocation code equal.

These checks give no evidence of a substantial OFF arithmetic rewrite that would justify another full benchmark of the same algorithm. Constant placement, other function placement, metadata, runtime behavior and shared-host variation are not isolated. Equal constant payloads do not prove equal cache behavior. This audit neither explains nor dismisses the earlier 8×2 interval or 16-core calibration failure.

## Consequence for optimization

The combined candidate's measured local 1×2 result remains **7.846378 ms and 87 MiB peak RSS**, a paired 2.471% latency gain and 35.675% lower RSS versus its fresh accepted control. Its declared full-request gate remains **false**. The 8×2 ON/accepted nonregression interval and physical16 OFF/accepted calibration remain failed; no threshold, sample or adoption decision changes here.

The audit closes a speculative code-generation explanation without claiming a speedup. Further optimization needs a materially different implementation or a diagnostic that isolates a runtime mechanism. Moving source lines or optional dependency declarations solely to seek another timing result is not supported by this evidence.

The [current summary](../2026-10-29-current-cpu-summary/README.md) retains all nine accepted local/AWS circuits and all eight supplied M5 CPU/Metal comparisons. Accepted local 1×2 remains 8.022165 ms; the CPU 10× target is 3.7 ms. The objective remains unmet on all eight reference shapes. No new AWS/Aeglos, ARM/mobile/TEE, giant-circuit, memory-pressure, swap, minimum-RAM or throughput result follows from static inspection.

## Reproduction and records

Run the four retained Rust helpers in order: `audit`, `text-differences`, `function-differences`, then `literal-review`, using the existing hash-pinned executables. The first three are inventories/refinements; the last verifies the grouping displacements and constant bytes. GNU `readelf`, `nm` and `objdump` supply section and symbol/disassembly records. The helpers use serde_json, sha2 and rustc-demangle from the existing tool environment. They do not execute the prover or verifier. Raw-symbol and same-offset intermediate comparisons retain their stated limitations.

[Initial inventory](artifacts/analysis.json), [demangled function comparison](artifacts/function-differences.json), [literal and alias checks](artifacts/literal-review.json), [complete static source/evidence archive](artifacts/static-evidence.json.gz), [archive validation](artifacts/publication-review.json). ELF payloads remain local under the hashes above. Archive extraction is verified byte-for-byte against every included source, symbol table, assembly listing and intermediate result. The report itself adds no benchmark or verification samples.
