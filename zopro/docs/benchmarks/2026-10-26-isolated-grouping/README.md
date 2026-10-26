# Isolate the small-query grouping precheck

This refactor preserves the original grouping source and generated instruction structure while adding the bitmap precheck at two canonical query call sites. It passes proof checks, but **local performance qualification fails two per-block calibration checks**. Production is unchanged. No C7a experiment or adoption follows this result.

The preceding [C7a bitmap trial](../2026-10-26-lower-bound-c7a/README.md) showed primary gains at high worker counts together with OFF/accepted calibration and larger-case regressions. This experiment tests a different implementation, rather than repeating those measurements unchanged.

## Source and generated code

The original grouping.rs file remains a byte-exact prefix. A short selector applies only to canonical G1 and exclusive G2 queries. Eligible8192≤n<32768 inputs with runtime1 enter a separate non-inlined helper. It reserves the existing grouping80percent forecast and runs the same occupied-bit lower bound. An occupied count exceeding floor(n*80/100) safely declines full grouping. Otherwise it releases the bitmap/reservation and calls the original grouping function, including its original sample and map/packing code. Other sizes, runtimeOFF and shared/prepared/Montgomery callers retain their original path.

The precheck now precedes the cheap sample, which can add a scan for vectors that the sample would reject cheaply. This is an explicit tradeoff tested by the full SDK comparison. The16–64KiB bitmap fits within the existing forecast classes-vector phase, is released before full grouping, and creates no retained or multigiB key table.

| Shared grouping function | Text bytes | Instructions | Static stack-reference instructions | Prologue frame bytes |
|---|---:|---:|---:|---:|
|Accepted|7624|1584|363|424|
|Previous bitmap candidate|8332|1761|383|424|
|Isolated candidate|7624|1584|363|424|

The accepted and isolated candidate's normalized instruction sequences match after direct branch address normalization and ignoring RIP-relative displacement/nearest-symbol comments. There are21nearest-symbol comment differences. Stack offsets and register instructions match; the3normalizer tests cover relative branches, preserved stack/register operands and RIP-symbol handling. This establishes code shape, not equal global referents, dynamic spill counts, latency or regression causality. Source hashes and independent proof checks provide separate evidence. [Code-generation review](artifacts/codegen-review.json) and [raw grouping disassembly](artifacts/audit-current/candidate-native.grouping.asm.txt).

## Verified SDK latency

Local Ryzen9950X,32workers,CPUs0–31. Three rotated/reversed fresh-process blocks,31timed requests after3primes. Accepted is the frozen production SDK; OFF/ON use the same isolated-candidate executable. Each cell is the median of three process medians. Fresh witness solving, randomness, proving, verification and serialization are timed; loading/preparation is excluded. No owned build/profile/archive overlaps latency captures.

| Shape | Accepted ms | OFF ms | ON ms | Paired ON/OFF geometric mean | ON maximum RSS MiB | Supplied M5 CPU / Metal ms | Candidate CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.630346|8.618765|8.648521|1.009979|74.50|37 /24|4.28× /2.78×|
|2x2|11.644130|11.741084|11.728430|1.004972|270.50|59 /30|5.03× /2.56×|
|4x2|16.049760|16.249636|16.411722|1.004835|306.00|109 /48|6.64× /2.92×|
|8x2|24.957800|24.624952|24.495017|0.994920|517.75|189 /74|7.72× /3.02×|
|12x2|34.464068|35.197182|34.475460|0.985416|733.25|284 /101|8.24× /2.93×|
|16x2|41.489780|40.418188|40.574112|0.996042|843.25|335 /124|8.26× /3.06×|
|24x2|68.818480|68.327324|68.699957|1.013184|1272.00|528 /176|7.69× /2.56×|
|merge-24x1|67.536854|68.619765|67.305908|0.981551|1181.81|507 /175|7.53× /2.60×|
|merge-54x1|176.621746|176.714420|175.906976|1.001169|1842.92|not supplied|—|

Primary ON/OFF=1.009979, about1.00%slower. Original-eight aggregate ON/OFF=0.998806; ON/accepted=0.999149, essentially flat. M5 values are supplied historical references, not new Apple measurements. RSS covers the entire latency process including load/setup/primes; it is not incremental bitmap storage or minimum RAM.

## Qualification outcome

The [frozen protocol](artifacts/design.md) requires OFF/accepted calibration within1.03 paired and1.05 per block, each shape's paired ON/OFF and ON/accepted≤1.02, and original-eight aggregates≤1.00. No minimum local primary improvement is required: this is nonregression qualification for a possible separate C7a test, not adoption.

Every shape passes the declared paired nonregression checks and both aggregate checks. Calibration fails:

- 1x2-w32 b0 OFF/accepted=0.945480100697349
- 8x2-w32 b0 OFF/accepted=1.0513407297182358

Every per-case paired-median OFF/accepted check passes, but the two per-block failures remain. No result is excluded, repeated or promoted. Restoring the shared grouping function's instruction structure does not remove the observed control variation, and the data does not identify function growth as the cause of earlier regressions. It also does not prove that code layout or scheduling has no effect. Before another tuning trial, an identical-binary control can test how much of this fresh-process variation occurs without a source change. That control has not been run here.

## Verification and profiles

Native115unfiltered core tests pass:109accepted tests,4retained bitmap tests and2new tests comparing complete selected group structures with the original and checking routing boundaries. Two raw and six cubic proof tests, the generated-IFMA check and3normalizer tests pass. Field source is unchanged; existing field qualification is inherited, with no new field-suite claim.

All-nine OFF/ON correctness preflights complete72SDK18Gnark. The81latency processes complete2754SDK81exports, followed by threeprimary199Hz cycles profiles completing162SDK3exports. Total **2988SDK calls,102distinct independently Gnark-verified exports and204proof/public members**. Fixture hashes,164-byte proofs, source/binary pins and fresh randomness are checked against5612preceding proof identities. The next complete prior set is5714. No successful capture is repeated.

The3profiles contain9513samples, zero lost records and one sampled SDK PID each. Profiles occur after all latencies and do not decide acceptance. The grouping category now includes lower_bound helper names; compiler inlining and sampler skid still prevent treating these bins as exclusive stage CPU.

| Variant | Samples | Grouping category % | Epoch + scheduler % |
|---|---:|---:|---:|
|accepted|3234|0.885|15.625|
|off|2990|0.360|12.857|
|on|3289|0.194|16.458|

No cloud resources are created. All170 production source files and the accepted SDK are unchanged. The candidate remains private and unadopted; no new phone, TEE, pressure, swap, concurrency, giant-circuit or target-machine result is claimed. The original all-eight CPU10× objective remains unmet.

[Full analysis](artifacts/analysis.json), [source edits](artifacts/edits.json), [native checks](artifacts/native/result.json), [frozen runner configuration](artifacts/perf-plan.json), [raw timing/profile rows](artifacts/performance/rows.json), [proof inventory](artifacts/proof-inventory.json) and [artifact hashes](artifacts/artifact-index.json) retain the evidence. Large keys, binaries and raw perf.data remain local with hashes.
