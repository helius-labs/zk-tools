# Frozen 54 to 1 CPU diagnostic

29 October 2026. This diagnoses the same frozen accepted, OFF and ON executables from the [complete representative-grouping experiment](../2026-10-29-representative-sdk175/README.md). That experiment's gate remains **failed**: 54→1 was 6.84% slower versus accepted, with zero wins across 15 blocks. Its primary peak RSS reduction from 134.50 to 83.75 MiB remains a positive observation. Production is unchanged.

No SDK, arithmetic or source rebuild occurs here. These are single diagnostic captures, not a new latency qualification or a rerun of the earlier speed gate.

## Capture method

Ryzen 9 9950X, 16 physical cores and 32 logical CPUs, 128 GiB RAM. Each frozen SDK uses 32 workers pinned to CPUs 0–31, the exact parent key/image/request, and the parent allocator/environment. Each process makes three primes followed by 31 fresh randomized, internally verified SDK requests. FIFO control excludes loading, preparation, primes and output from counters/profiles.

Six captures run sequentially: grouped PMU stat in accepted/OFF/ON order, then separate 199 Hz cycles:u profiles in ON/OFF/accepted order. The fixed PMU group is cycles:u, instructions:u, cache-misses:u and branch-misses:u. All six finish before ownership transfer, independent verification, decoding, lost-record and MMAP audits. All helpers were compiled before capture; no owned compiler, hash/archive/analysis helper or second CPU workload overlaps capture.

## Hardware counters

Counts are normalized by 31 requests. Each required event must have a finite nonnegative count, positive runtime and at least 99.9% running coverage for a qualified panel. Unsupported or multiplexed events are retained as unqualified, without resampling. Cache misses are the generic PMU event, not a byte-bandwidth measurement.

|Executable|Instructions per request|Cycles per request|IPC|Cache misses per request|Branch misses per request|Qualified|
|---|---:|---:|---:|---:|---:|---|
|accepted|18014458225|21405284917|0.842|81068536|6772206|yes|
|OFF|18015861802|21289826939|0.846|80324239|6784379|yes|
|ON|18128207687|22184905006|0.817|78385663|6809620|yes|

All event counts, runtimes and running percentages remain in [analysis.json](artifacts/analysis.json) and the raw semicolon CSV files. One capture per executable does not establish a statistical speed difference or separate all machine-state effects.

ON retires 0.631% more instructions and consumes 3.642% more cycles than accepted in this panel; generic cache misses fall 3.309%. Against OFF, the corresponding changes are +0.624%, +4.204% and −2.413%. Thus the regression is not accompanied by a broad increase in this cache-miss count. The counters do not distinguish dependency stalls, cache-hit latency, bandwidth, code generation or runtime placement as the cause.

## Sampled CPU work

Profiles retain explicit CPU IDs, timestamps and MMAP/task metadata for exact ELF mapping. Each profile must have one sampled process and zero lost records. The period-weighted categories below use literal symbol names with a fixed priority; inlined arithmetic may appear in a caller. They are flat sampled residence, not exclusive wall time or independently measured causal costs.

|Category|Accepted period per request|OFF period per request|ON period per request|Accepted share|OFF share|ON share|
|---|---:|---:|---:|---:|---:|---:|
|FFT|8970909226|8999398980|8822956763|44.86%|44.75%|42.13%|
|MSM_curve_inverse|8232083783|8123461817|8929372547|41.16%|40.39%|42.64%|
|allocator|13375343|15779418|19701407|0.07%|0.08%|0.09%|
|grouping|319088743|325865334|588778446|1.60%|1.62%|2.81%|
|other|596569022|751531151|807919299|2.98%|3.74%|3.86%|
|other_hashing_tables|35334833|38311987|844825|0.18%|0.19%|0.00%|
|scheduler_synchronization|664888567|724463203|688915843|3.32%|3.60%|3.29%|
|solver|1165821484|1129159492|1083015821|5.83%|5.61%|5.17%|
|verification_field_extensions|1516396|3590704|729468|0.01%|0.02%|0.00%|

|Executable|Samples|Lost records|Profile CPU ms per request|Profile process peak RSS MiB|
|---|---:|---:|---:|---:|
|accepted|26551|0|4494.550|1700.87|
|OFF|26119|0|4475.320|1709.73|
|ON|27071|0|4665.938|1728.79|

RSS is the lifetime process high-water mark, including loading and primes, not minimum RAM or incremental workspace. Profile CPU time and sampled periods come from separate recordings from the stat panels. Full demangled symbol totals and per-request periods remain in analysis.json.

## Findings and next implementation

Sampled FFT period per request is approximately stable between the three executables, while ON has more sampled period in grouping and MSM. The G1 BatchAdd apply symbol is also approximately stable at 3.17–3.18 billion period units per request. Increases appear in grouped-point preparation and reduction trees, rather than uniformly across all hot arithmetic. Allocation symbols occupy below 0.1% of each profile; this is not evidence of a large exclusive allocator CPU cost.

The representative dictionary's rehash symbol is a concrete new lead:

|Scalar dictionary growth symbol|Accepted period per request|OFF period per request|ON period per request|
|---|---:|---:|---:|
|reserve_rehash|34588693|37483379|259301287|

The accepted and OFF tables store scalar keys inline. ON stores class IDs; its rehash callback follows a class ID to a representative row and hashes all four input words again. This extra indirection is visible in the source and the rehash symbol has substantially more sampled residence. It does not explain the entire proof regression or establish its memory-hierarchy cause.

The next targeted revision can retain the already-computed 64-bit hash with each proof-local representative record, so table growth reads that hash rather than revisiting and hashing the input. It adds eight temporary bytes per distinct class, not a persistent per-key table. The revised allocation forecast must account for those bytes, preserve the original bounded-memory admission policy and full-value equality, and release records before output packing. A new constructor experiment must retain all 18 real inputs; any later SDK qualification must retain all nine circuits. The grouped-point MSM increase remains unresolved and must not be credited to this revision in advance.

## Verification and decision

All 30 workflow steps complete. The trial makes **204 SDK requests**, including primes, and exports **six fresh 164-byte proofs**, all independently verified with Gnark and bound to the expected image/public-input hashes. All six are distinct from the prior 21,697 proof identities; the new ledger is **21,703**. Source, receipt and proof archives round-trip byte-exactly. Raw perf recordings remain local with recorded hashes; ELF payloads are not duplicated into the report.

The diagnostic selects a concrete source-level revision for measurement. It does not adopt the representative builder, change the prior gate, or demonstrate minimum RAM, memory-pressure/swap performance, concurrent throughput or AWS/mobile/ARM/TEE performance. Accepted source and binaries remain unchanged. This findings section is an offline interpretation of the retained captures; the precompiled publisher's original text and runtime helpers remain archived unchanged.

The all-eight verified CPU 10× objective remains unmet. Accepted local 1×2 remains 8.022165 ms with 134 MiB peak RSS; latest AWS remains Zopro 11.915591 ms at 32 workers versus pinned Aeglos Sorted 7.959184 ms at 96. [All nine accepted local/AWS rows and original eight supplied CPU/Metal references](../2026-10-29-current-cpu-summary/README.md), [fixed benchmark snapshot](../2026-10-29-cpu-benchmark-report/README.md).

[Protocol](artifacts/design.md), [counter and profile analysis](artifacts/analysis.json), [frozen executable identities](artifacts/identities.json), [source pins](artifacts/pins.json), [workflow](artifacts/plan.json), [helper sources](artifacts/source.json.gz), [execution receipts](artifacts/receipts.json.gz), [verified proofs](artifacts/proofs.json.gz), [archive round-trip audit](artifacts/archive-review.json).
