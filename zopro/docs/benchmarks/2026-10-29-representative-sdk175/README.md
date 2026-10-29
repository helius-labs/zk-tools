# One pass representative grouping in complete SDK proofs

The new complete-request performance gate **fails**. Production remains unchanged. This independently measures the [representative constructor](../2026-10-29-representative-groups175/README.md) in complete SDK requests across all nine circuits. Its earlier constructor gate remains failed: primary gains of 21.5%/23.3% and an improving aggregate coexist with eight larger constructor regressions. None of those observations is relabeled or excluded.

Primary ON measures **8.033347 ms and 83.75 MiB peak RSS**, versus accepted **8.152022 ms and 134.50 MiB** in this capture. The paired accepted-relative gain is 1.34%, with 15/15 wins; peak RSS is 50.75 MiB, or 37.73%, lower. ON roughly matches fresh OFF: paired ratio 1.002662286, bounds [0.997012758, 1.005840682], six wins. The material regression is 54→1: paired slowdown 6.84% versus accepted and 6.59% versus OFF, zero wins against either. These are separate fresh captures from the inherited qualified 8.022165 ms accepted result.

## Implementation and scope

A private native cfg selects one hash pass with compact original-row/count records and existing row classes. Full scalar values and usize counts are materialized only after exact admission; records are released before stable output packing. The compact dictionary checks all four words through borrowed immutable input. First-occurrence order, zeros, cutoffs, sample policy and query independence remain. No point copy, key table, duplicate plan, new pool, unsafe code or field operation is added.

The clean private snapshot has 176 files: 172 of 175 originals are exact; grouping.rs, core Cargo.toml and Cargo.lock have reversible changes, plus one child. The native-only hashbrown 0.16.1 dependency has default features disabled and inline-more enabled. All 17 field files remain exact and their 33-test qualification is inherited. The original grouping allocation forecast covers the new record lifetimes; this is not a minimum-RAM or pressure qualification.

The decision to run this new protocol evaluates whole-request latency and memory despite a phase-level 3% failure. It does not waive the old constructor gate or adopt a favorable subset. All nine latency rows are retained.

## Complete request measurements

Ryzen 9 9950X, 16 physical cores and 32 logical CPUs, with 32 workers pinned to CPUs 0–31. Fifteen fixed rotated/reversed case and variant blocks compare the exact accepted executable, fresh OFF and ON. Each primary process times 151 requests after three primes; other circuits time 31 after three primes. Loading/preparation are outside latency and inside lifetime RSS. Fresh solving, randomized proving, SDK verification and serialization are timed. Ratios pair process medians within blocks and need not equal ratios of displayed medians.

|Circuit|Accepted ms|OFF ms|ON ms|ON/accepted ratio|Bounds|Wins / 15|ON/OFF ratio|Bounds|Wins / 15|
|---|---:|---:|---:|---:|---|---:|---:|---|---:|
|1x2|8.152022|8.026144|8.033347|0.986585469|[0.982801006, 0.989434219]|15|1.002662286|[0.997012758, 1.005840682]|6|
|2x2|11.394626|11.246055|11.204142|0.985478284|[0.978740241, 0.987480726]|15|0.997700119|[0.990721022, 0.999285283]|12|
|4x2|16.055527|15.896628|15.852074|0.980976894|[0.971488284, 0.991348219]|13|0.997537947|[0.984935619, 1.006718429]|10|
|8x2|24.230379|24.140120|24.083721|0.990825027|[0.978295408, 1.004178434]|11|1.003431497|[0.983408725, 1.010786172]|7|
|12x2|33.869702|33.798232|33.531005|0.988790955|[0.972234688, 0.997432915]|12|0.989964448|[0.983978200, 0.999252091]|13|
|16x2|40.583149|40.040498|39.670771|0.981256539|[0.969811705, 0.990448580]|12|0.992542959|[0.987736447, 1.005067291]|11|
|24x2|66.708818|66.117965|66.813065|1.002252084|[0.995927166, 1.006511642]|6|1.012114989|[1.000766463, 1.019269990]|3|
|merge-24x1|65.574555|64.976160|65.470053|1.001421527|[0.990179617, 1.004613548]|7|1.008458765|[1.003798039, 1.011527809]|1|
|merge-54x1|162.474643|162.128890|172.728370|1.068378645|[1.059385016, 1.071481907]|0|1.065869552|[1.059408609, 1.069339369]|0|

|Original eight aggregate versus|Ratio|Bounds|Wins / 15|
|---|---:|---|---:|
|OFF|0.998636422|[0.995014629, 1.002273267]|9|
|accepted|0.989047179|[0.987302759, 0.990663458]|15|

The original-eight aggregate improves 1.095% versus accepted. Its 0.136% improvement estimate versus OFF has bounds spanning parity and only nine wins, so that gain check fails. Every relative RSS check passes. Peak RSS is lower than accepted on the eight original circuits and slightly higher on 54→1; this is an observed tradeoff, not a universal RAM bound.

## CPU time and memory

RSS is the maximum process lifetime high-water mark across blocks, including loading and primes, in MiB. It is not incremental workspace or minimum RAM. CPU time sums user and system time per request.

|Circuit|Accepted CPU ms|OFF CPU ms|ON CPU ms|Accepted peak RSS MiB|OFF peak RSS MiB|ON peak RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2|183.147|181.467|183.006|134.50|136.25|83.75|
|2x2|262.781|261.287|262.192|265.00|273.00|217.50|
|4x2|420.633|417.945|411.535|314.75|323.50|290.75|
|8x2|671.127|669.236|663.882|517.50|502.50|479.00|
|12x2|936.225|931.854|927.711|713.25|727.00|657.00|
|16x2|1129.327|1127.102|1114.580|814.75|825.75|787.00|
|24x2|1901.230|1892.220|1922.740|1282.84|1297.41|1221.75|
|merge-24x1|1811.650|1805.034|1817.468|1180.02|1180.72|1135.04|
|merge-54x1|4457.806|4442.136|4673.598|1885.01|1875.35|1897.42|

## Declared checks and calibration

Primary ON/accepted and ON/OFF require upper paired ratio <1 and at least 10 wins. Both original-eight equal-log aggregates require upper <1 and at least 10 wins. Every circuit's ON/control upper ratio must be ≤1.02 against both controls, and observed ON peak RSS must be ≤1.05× each control. The memory allowance is relative, not a fixed architecture RAM cap. Ranks 4–12 of 15 give nominal 96.484375% individual median coverage under independent/exchangeable blocks, not a familywise guarantee. No gain floor, exclusion, extension, layout/seed selection, threshold change or unchanged rerun is applied.

OFF/accepted intervals within factor 1.03 are reported as calibration diagnostics in this new protocol, not a performance gate. Both exact-artifact comparisons remain required. A control difference limits source-level causal attribution; it does not erase the directly paired candidate/accepted observation. Earlier failed calibration gates and the accepted build's documented physical-16 engineering exception remain unchanged.

Failed performance checks:

- `1x2-w32/ON/OFF`
- `merge-54x1-w32/ON/OFF`
- `merge-54x1-w32/ON/accepted`
- `original8/ON/OFF/gain`

Calibration diagnostics outside factor 1.03:

None.

## Correctness and automatic profiling

Fresh OFF/ON pass 124/127 unfiltered native core tests and eight/eight prover tests. Three native child tests adapt the parent constructor's collision, whole-output, cutoff, immutable-input and allocation-bound checks. The parent algorithm is unchanged apart from result/reservation ownership and the private dispatch. No new field suite is counted.

All 27 preflight exports pass independent Gnark verification before latency. All 405 latency processes precede six separate FIFO-gated 199 Hz cycles:u recordings: primary at 16 and 32 workers, accepted/OFF/ON, 251 requests after three primes. CPU IDs and timestamps are explicit. Physical-16 profiles are diagnostic, not a new latency qualification. All recordings precede ownership transfer, the remaining 411 independent verifications, owner-user decoding and lost-record audits. No owned compiler/helper/hash/archive or second CPU workload overlaps capture.

|Profile|Samples|Lost records|
|---|---:|---:|
|cycles-1x2-w16-OFF|5362|0|
|cycles-1x2-w16-ON|5322|0|
|cycles-1x2-w16-accepted|5292|0|
|cycles-1x2-w32-OFF|8939|0|
|cycles-1x2-w32-ON|9029|0|
|cycles-1x2-w32-accepted|9106|0|

All profiles have one sampled SDK process and zero lost records. Flat symbols and static assembly do not establish exclusive cache, allocator, spill or critical-path costs. All 900 workflow steps complete. A capture-helper preparation initially stopped before producing capture.rs because an exact-match transformation expected a different inherited string spelling. Correcting only that transformation preserves runtime source and the declared protocol; no successful build/test/capture is repeated. The initial source and failure are archived.

The trial totals **20,802 internally verified SDK requests and 438 fresh independent Gnark exports**, with 876 proof/public members. Every retained proof is 164 bytes, fixture/public-bound and distinct from all 21,259 prior proof identities. The ledger is **21,697** after verification. Source, receipt and proof archives round-trip byte-exactly.

Publication initially stopped before creating the report because the inherited collector had verified the expanded proof set but had not persisted its hash ledger. An offline bookkeeping repair checks all 876 inventory files against their hashes and writes the union of the 21,259 prior and 438 already-verified new identities. It adds no prover run or Gnark verification and changes no measurement or gate. [Repair source](artifacts/close-ledger.rs), [repair receipt](artifacts/ledger-repair.json), [initial publication failure](artifacts/initial-publication-failure.txt).

## Supplied M5 comparisons and decision

M5 CPU/Metal values are supplied historical references, not newly measured Apple results. ON times below are experimental full-proof measurements, not adopted production performance.

|Circuit|ON ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.033347|37|4.606×|24|2.988×|3.7|
|2x2|11.204142|59|5.266×|30|2.678×|5.9|
|4x2|15.852074|109|6.876×|48|3.028×|10.9|
|8x2|24.083721|189|7.848×|74|3.073×|18.9|
|12x2|33.531005|284|8.470×|101|3.012×|28.4|
|16x2|39.670771|335|8.445×|124|3.126×|33.5|
|24x2|66.813065|528|7.903×|176|2.634×|52.8|
|merge-24x1|65.470053|507|7.744×|175|2.673×|50.7|
|merge-54x1|172.728370|—|—|—|—|—|

The candidate reaches 0/8 supplied CPU 10× latency targets in these measurements. The original objective remains active and unproven. There is no supplied M5 reference for 54→1.

The declared complete-request gate fails, so this implementation is not adopted. Every positive estimate, memory observation and failed check remains in the record; no favorable subset is promoted.

The next diagnostic should profile the frozen 54→1 executables to locate the extra CPU work before changing the algorithm again. The six primary profiles cannot attribute that large-circuit regression to grouping, allocation, MSM, FFT or scheduling.

Accepted production remains 175 files, with qualified local 1×2 8.022165 ms and 134 MiB peak RSS. The latest AWS 1×2 remains Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96. No new AWS, phone/ARM/TEE, forester 250/4999, pressure/swap, minimum-RAM or concurrent-throughput result is claimed. [Current local/AWS summary](../2026-10-29-current-cpu-summary/README.md).

[Protocol](artifacts/design.md), [all observations and checks](artifacts/analysis.json), [implementation](artifacts/representative.rs), [source audit](artifacts/source-audit.json), [source](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [publication review](artifacts/publication-review.json).
