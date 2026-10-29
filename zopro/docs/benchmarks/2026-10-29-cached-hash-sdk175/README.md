# Temporary cached hashes in complete SDK proofs

The new complete-request performance gate **fails**. Production remains unchanged. This measures the [cached-hash constructor](../2026-10-29-cached-hash-groups175/README.md) across all nine circuits. Its new constructor gate passes: all-18 aggregate improves 5.17% versus original and 5.06% versus uncached parent, with all original-relative nonregression checks passing. The earlier uncached constructor and SDK gates remain failed.

The positive full-proof result is **54→1: 158.333945 ms versus accepted 162.838936 ms**, a paired **2.68% gain with 15/15 wins**. Primary peak RSS falls **35.22%, from 132.75 to 86.00 MiB**. Primary latency is 8.073931 ms versus accepted 8.056177 ms, with its paired bounds spanning parity; this capture does not establish a primary speed gain. All existing nonregression and relative RSS checks pass, and all calibration diagnostics are within range. Only three gain checks fail.

## Implementation and scope

A private native cfg selects one hash pass with compact original-row/count records, existing row classes and a separate temporary u64 hash vector. Table growth reads the cached class hash instead of revisiting and rehashing the representative input. Full-value equality still compares all four immutable input words. Hashes and the table are dropped before scalar materialization. Full scalar values and usize counts are materialized only after exact admission; records are released before stable output packing. The compact dictionary checks all four words through borrowed immutable input. First-occurrence order, zeros, cutoffs, sample policy and query independence remain. No point copy, key table, duplicate plan, new pool, unsafe code or field operation is added.

The clean private snapshot has 176 files: 172 of 175 originals are exact; grouping.rs, core Cargo.toml and Cargo.lock have reversible changes, plus one child. The native-only hashbrown 0.16.1 dependency has default features disabled and inline-more enabled. All 17 field files remain exact and their 33-test qualification is inherited. The original grouping allocation forecast covers the new record lifetimes; this is not a minimum-RAM or pressure qualification.

This new complete-request protocol follows a passing constructor screen. It measures latency and memory across all nine circuits without waiving earlier failed gates or selecting a favorable subset.

## Complete request measurements

Ryzen 9 9950X, 16 physical cores and 32 logical CPUs, with 32 workers pinned to CPUs 0–31. Fifteen fixed rotated/reversed case and variant blocks compare the exact accepted executable, fresh OFF and ON. Each primary process times 151 requests after three primes; other circuits time 31 after three primes. Loading/preparation are outside latency and inside lifetime RSS. Fresh solving, randomized proving, SDK verification and serialization are timed. Ratios pair process medians within blocks and need not equal ratios of displayed medians.

|Circuit|Accepted ms|OFF ms|ON ms|ON/accepted ratio|Bounds|Wins / 15|ON/OFF ratio|Bounds|Wins / 15|
|---|---:|---:|---:|---:|---|---:|---:|---|---:|
|1x2|8.056177|8.041961|8.073931|1.007296279|[0.997762133, 1.009217054]|5|1.003859361|[0.999949057, 1.009887464]|4|
|2x2|11.336486|11.300638|11.279108|0.993769054|[0.986744704, 1.004455864]|9|1.001053079|[0.995441657, 1.006154517]|6|
|4x2|16.165656|16.081457|16.028418|0.991055117|[0.982085828, 0.996895579]|12|0.987668698|[0.979112978, 1.018281514]|8|
|8x2|24.331289|24.162864|24.388408|1.002347553|[0.997094796, 1.010808655]|6|1.008463690|[0.990997150, 1.017798670]|5|
|12x2|33.908997|33.890713|33.551914|0.993405384|[0.984815643, 0.997553799]|14|0.988996219|[0.985311518, 0.996754615]|14|
|16x2|40.177051|40.055373|39.853234|0.991396827|[0.987722142, 1.007302473]|11|0.997220011|[0.995134241, 1.000649300]|11|
|24x2|66.390604|66.198222|65.311431|0.987452114|[0.980449221, 0.994343277]|14|0.995976533|[0.983195811, 0.999294588]|12|
|merge-24x1|65.599179|65.608057|64.783803|0.988357836|[0.982934096, 0.992794128]|15|0.987213862|[0.981272212, 0.996975321]|12|
|merge-54x1|162.838936|161.561478|158.333945|0.973155337|[0.965582682, 0.982021845]|15|0.980538909|[0.977144859, 0.982612502]|15|

|Original eight aggregate versus|Ratio|Bounds|Wins / 15|
|---|---:|---|---:|
|OFF|0.996145697|[0.992251964, 1.003110294]|10|
|accepted|0.993914516|[0.991552033, 0.997435690]|14|

The original-eight aggregate improves 0.609% versus accepted. The 0.385% gain estimate versus OFF has bounds spanning parity, so that check fails despite ten winning blocks. For 54→1, the paired improvement versus OFF is 1.946%, also with 15 wins. These observations retain the larger-circuit improvement without substituting it for the unproven primary gain.

## CPU time and memory

RSS is the maximum process lifetime high-water mark across blocks, including loading and primes, in MiB. It is not incremental workspace or minimum RAM. CPU time sums user and system time per request.

|Circuit|Accepted CPU ms|OFF CPU ms|ON CPU ms|Accepted peak RSS MiB|OFF peak RSS MiB|ON peak RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2|182.617|181.819|183.389|132.75|132.25|86.00|
|2x2|262.679|261.758|261.867|275.50|272.00|221.50|
|4x2|422.189|425.393|417.909|309.50|316.75|289.50|
|8x2|671.922|667.903|669.918|498.00|509.50|493.50|
|12x2|938.573|939.808|928.537|721.50|727.50|683.25|
|16x2|1123.196|1115.153|1119.378|849.75|841.25|808.00|
|24x2|1900.601|1888.044|1877.899|1304.75|1284.25|1226.75|
|merge-24x1|1817.511|1808.054|1792.636|1165.41|1154.45|1160.50|
|merge-54x1|4472.503|4440.509|4361.435|1871.30|1837.06|1802.11|

## Declared checks and calibration

Primary ON/accepted and ON/OFF require upper paired ratio <1 and at least 10 wins. Both original-eight equal-log aggregates require upper <1 and at least 10 wins. Every circuit's ON/control upper ratio must be ≤1.02 against both controls, and observed ON peak RSS must be ≤1.05× each control. The memory allowance is relative, not a fixed architecture RAM cap. Ranks 4–12 of 15 give nominal 96.484375% individual median coverage under independent/exchangeable blocks, not a familywise guarantee. No gain floor, exclusion, extension, layout/seed selection, threshold change or unchanged rerun is applied.

OFF/accepted intervals within factor 1.03 are reported as calibration diagnostics in this new protocol, not a performance gate. Both exact-artifact comparisons remain required. A control difference limits source-level causal attribution; it does not erase the directly paired candidate/accepted observation. Earlier failed calibration gates and the accepted build's documented physical-16 engineering exception remain unchanged.

Failed performance checks:

- `1x2-w32/ON/OFF`
- `1x2-w32/ON/accepted`
- `original8/ON/OFF/gain`

Calibration diagnostics outside factor 1.03:

None.

## Correctness and automatic profiling

Fresh OFF/ON pass 124/128 unfiltered native core tests and eight/eight prover tests. Four native child tests adapt the parent constructor's collision, whole-output, cutoff, immutable-input and allocation-bound checks, plus the exact-one-hash invariant under forced growth. The parent algorithm is unchanged apart from result/reservation ownership and the private dispatch. No new field suite is counted.

All 27 preflight exports pass independent Gnark verification before latency. All 405 latency processes precede nine separate FIFO-gated 199 Hz cycles:u recordings: primary at 16 and 32 workers with 251 requests, and 54→1 at 32 workers with 31 requests, accepted/OFF/ON, each after three primes. CPU IDs and timestamps are explicit. Physical-16 profiles are diagnostic, not a new latency qualification. All recordings precede ownership transfer, the remaining 414 independent verifications, owner-user decoding and lost-record audits. No owned compiler/helper/hash/archive or second CPU workload overlaps capture.

|Profile|Samples|Lost records|
|---|---:|---:|
|cycles-1x2-w16-OFF|5501|0|
|cycles-1x2-w16-ON|5572|0|
|cycles-1x2-w16-accepted|5381|0|
|cycles-1x2-w32-OFF|8795|0|
|cycles-1x2-w32-ON|9004|0|
|cycles-1x2-w32-accepted|9180|0|
|cycles-merge-54x1-w32-OFF|25773|0|
|cycles-merge-54x1-w32-ON|25356|0|
|cycles-merge-54x1-w32-accepted|26122|0|

All profiles have one sampled SDK process and zero lost records. Flat symbols and static assembly do not establish exclusive cache, allocator, spill or critical-path costs. All 918 workflow steps complete.

The trial totals **20,904 internally verified SDK requests and 441 fresh independent Gnark exports**, with 882 proof/public members. Every retained proof is 164 bytes, fixture/public-bound and distinct from all 21,703 prior proof identities. The ledger is **22,144** after verification. Source, receipt and proof archives round-trip byte-exactly.

Publication initially stopped at its first destination-exists assertion: the copied publisher still named the earlier representative SDK report. That assertion prevented any overwrite. The subsequent summary attempt stopped while reading the missing new report. A separate publisher corrects only the destination and include path, retaining every original pinned helper and successful workflow receipt. No test, capture or independent verification is repeated. [Repair source](artifacts/publication-repair/repair.rs), [corrected publisher](artifacts/publication-repair/publish-fixed.rs), [failure record](artifacts/publication-repair/failure.txt).

## Supplied M5 comparisons and decision

M5 CPU/Metal values are supplied historical references, not newly measured Apple results. ON times below are experimental full-proof measurements, not adopted production performance.

|Circuit|ON ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.073931|37|4.583×|24|2.973×|3.7|
|2x2|11.279108|59|5.231×|30|2.660×|5.9|
|4x2|16.028418|109|6.800×|48|2.995×|10.9|
|8x2|24.388408|189|7.750×|74|3.034×|18.9|
|12x2|33.551914|284|8.464×|101|3.010×|28.4|
|16x2|39.853234|335|8.406×|124|3.111×|33.5|
|24x2|65.311431|528|8.084×|176|2.695×|52.8|
|merge-24x1|64.783803|507|7.826×|175|2.701×|50.7|
|merge-54x1|158.333945|—|—|—|—|—|

The candidate reaches 0/8 supplied CPU 10× latency targets in these measurements. The original objective remains active and unproven. There is no supplied M5 reference for 54→1.

The declared complete-request gate fails, so this implementation is not adopted. Every positive estimate, memory observation and failed check remains in the record; no favorable subset is promoted.

Accepted production remains 175 files, with qualified local 1×2 8.022165 ms and 134 MiB peak RSS. The latest AWS 1×2 remains Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96. No new AWS, phone/ARM/TEE, forester 250/4999, pressure/swap, minimum-RAM or concurrent-throughput result is claimed. [Current local/AWS summary](../2026-10-29-current-cpu-summary/README.md).

[Protocol](artifacts/design.md), [all observations and checks](artifacts/analysis.json), [implementation](artifacts/representative.rs), [source audit](artifacts/source-audit.json), [source](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [publication review](artifacts/publication-review.json).
