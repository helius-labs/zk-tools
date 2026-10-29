# Instruction and cycle attribution in the frozen SDK

29 October 2026. This diagnostic samples architectural instructions and cycles in the exact accepted source-175 SDK. It changes no source or executable and qualifies no new performance result. The preceding PMU comparison found only 4.97% more retired instructions at 32 versus one worker despite 89.89% more summed CPU time; those observations remain.

MSM/curve symbols account for a median 81.84% of sampled instruction period at 16 workers and 78.74% at 32. Scalar inverse symbols account for another 9.81% and 8.23%. At 32 workers, crossbeam epoch symbols have 9.53% of sampled cycle period versus 4.35% of instruction period. These flat categories guide inspection; they do not measure exclusive stage cost or establish a removable wall-time fraction.

## Method

Ryzen 9 9950X. Sixteen workers use physical CPUs 0–15; 32 workers use logical CPUs 0–31, sharing the same 16 physical cores through SMT. Three fixed rotated/reversed blocks compare separate cycles:u and instructions:u profiles, twelve processes in total. Each performs 151 fresh internally verified SDK requests after three primes, using the same primary 1×2 image, request and accepted environment. FIFO control gates 199 Hz sampling to the request loop; loading, primes, proof export and independent verification are outside it. No uprobes are attached.

All captures finish before ownership transfer, all twelve independent verifications, decoding and event/loss/MMAP audits. All helpers were compiled before capture; no owned compiler, hash/archive/analysis helper or second CPU workload overlaps capture. The host is shared. The existing independently verified topology preflights are reused after their pinned input/receipt checks; they are not recounted as new proofs.

## Qualification

Each panel requires one sampled SDK PID, no lost records, correct event/frequency, explicit CPU/time metadata, valid affinity and the accepted ELF mapping. Symbol attribution additionally requires at least 1,000 samples and at most 5% unknown-symbol period. Unqualified panels remain in the record without resampling or exclusions. Fixed literal-name categories are declared in the protocol and analyzer. Inlined payloads can appear under generic runtime symbols; these are flat sampled residence, not exclusive stage CPU time.

All attribution panels qualified: **true**.

|Panel|Samples|Unknown period %|Attribution qualified|Lost records|
|---|---:|---:|---|---:|
|b0-w16-cycles|3146|1.949537|true|0|
|b0-w16-instructions|3706|0.426809|true|0|
|b0-w32-cycles|6233|1.293875|true|0|
|b0-w32-instructions|7205|0.562431|true|0|
|b1-w16-cycles|3147|1.455367|true|0|
|b1-w32-instructions|7219|0.281603|true|0|
|b1-w32-cycles|6286|1.587005|true|0|
|b1-w16-instructions|3609|0.479294|true|0|
|b2-w32-cycles|6331|1.434778|true|0|
|b2-w32-instructions|7080|0.152613|true|0|
|b2-w16-cycles|3176|1.192477|true|0|
|b2-w16-instructions|3388|0.311455|true|0|

## Sampled period shares

Medians across three profiles per topology/event. These medians need not sum to 100%. All per-panel counts, periods, ranges and top 30 symbols are retained in analysis.json; the complete symbol counts are in profile-symbols.json.gz.

|Category|16-worker instructions %|16-worker cycles %|32-worker instructions %|32-worker cycles %|
|---|---:|---:|---:|---:|
|FFT|2.276|3.637|2.370|5.748|
|MSM_curve|81.840|75.358|78.738|68.589|
|allocator|0.355|0.140|0.308|0.218|
|crossbeam_epoch|1.160|2.905|4.349|9.530|
|grouping|0.320|0.687|0.426|1.100|
|inverse|9.805|8.496|8.231|6.268|
|other|1.552|2.044|1.595|1.480|
|runtime_or_inlined_payload|0.261|1.727|0.491|3.268|
|solver|1.370|2.000|1.487|1.462|
|unknown|0.427|1.455|0.282|1.435|
|verification_field_extensions|0.427|0.780|0.493|0.241|

Separate instruction and cycle profiles are not simultaneous per-IP counters. Their shares and normalized period estimates can guide source inspection, but are not measured local CPI, a causal stall attribution or directly removable wall time. Sampling skid, inlining, different execution placement, SMT and host variation remain limitations. A long generic function name may include another component without proving an exclusive cost. No timing gate, gain floor, subset promotion, extension, threshold change or unchanged rerun is used.

Summed sample periods also fall below the preceding nonsampled PMU totals. The median instruction-period estimates per request are 1.205 billion at 16 workers and 1.065 billion at 32, versus nonsampled counts of 1.381 and 1.421 billion. Cycle-period estimates are 504.489 million and 614.542 million, versus nonsampled counts of 588.027 million and 838.069 million. These are separate runs, and the cause of the accounting difference is unresolved. Do not interpret the smaller sample-period totals as reduced instruction work or use them to calculate local CPI. The declared sample-count and attribution checks pass; they do not validate absolute period coverage. The underlying per-profile period estimates are preserved in [analysis.json](artifacts/analysis.json), and the earlier nonsampled counts in the [worker-count diagnostic](../2026-10-29-worker-work175/README.md).

## Verification and accepted comparisons

All 84 capture/offline steps complete. The trial makes 1,848 internally verified SDK requests and exports twelve fresh 164-byte proofs. Every export independently verifies with Gnark, matches image/public hashes and is distinct from the preceding 22,198 identities; the new ledger is **22,210**. Six parser tests cover fields, unknown symbols, headers, Rust demangling, category precedence and malformed PID/period rejection. Accepted field/core/prover qualification is inherited because source and executable are unchanged. Source, receipt, proof and full-symbol archives round-trip byte-exactly. Raw perf recordings remain local with hashes.

Accepted local 1×2 remains **8.022165 ms / 134 MiB observed peak RSS**. Latest AWS c7a.24xlarge remains Zopro 11.915591 ms at 32 workers versus Aeglos Sorted 7.959184 ms at 96. These are inherited observations, not profile-process performance. The current [all-nine local/AWS and eight supplied CPU/Metal comparisons](../2026-10-29-current-cpu-summary/README.md) remain unchanged. The original all-eight CPU 10× target remains active and unmet, 0/8; the accepted physical-16 calibration exception and separate final-artifact qualification remain documented. No source adoption, AWS, mobile/ARM/TEE, memory-pressure/swap, giant-circuit, minimum-RAM or concurrent-throughput qualification follows.

[Protocol](artifacts/design.md), [all profiles and checks](artifacts/analysis.json), [complete symbol counts](artifacts/profile-symbols.json.gz), [verification inventory](artifacts/proof-inventory.json), [source](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [archive review](artifacts/publication-review.json).
