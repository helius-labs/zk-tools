# Zopro CPU benchmark report

29 October 2026. Snapshot of completed measurements at repository commit `e3b8cb4db8a9d2608e9a3c7f39dee420eaa95925`.

The accepted Zopro build proves **1×2 in 8.022 ms locally**, with **134 MiB observed peak RSS**. On AWS c7a.24xlarge, its best tested 1×2 result is **11.916 ms**, compared with **7.959 ms for Aeglos Sorted**. Across the nine AWS circuits, Zopro has lower best-count latency on six and lower observed peak RSS on all nine. Aeglos leads the three smallest circuits.

The latest complete SDK experiment reduces observed 1×2 peak RSS by **37.73%**, from **134.50 to 83.75 MiB** in its fresh comparison. It improves paired latency by 1.34% versus accepted, but does not establish a gain versus fresh OFF and makes 54→1 **6.84% slower** versus accepted. Its declared gate fails, so production remains unchanged. The CPU 10× objective against the supplied M5 references remains unmet on all eight reference circuits.

## Measurement conditions

Local measurements use a Ryzen 9 9950X, 16 physical cores, 32 logical CPUs and 128 GiB RAM. The accepted local table uses 32 workers. AWS measurements use one c7a.24xlarge with 96 physical cores and no SMT, testing 32, 64 and 96 workers. The AWS container has a 96-CPU quota, a 30 GiB memory limit, no swap and disabled networking; this memory setting is not an architecture requirement.

The latency tables report medians of 15 process medians after three priming proofs. Local processes measure 151 requests for 1×2 and 31 for other circuits; AWS processes measure 101 and 17 respectively. Each timed request includes fresh witness solving, fresh randomized proving, SDK verification and serialization. Loading and preparation are outside latency but included in lifetime process RSS. Peak RSS is the maximum high-water mark across the corresponding processes, not minimum RAM or incremental workspace.

AWS Zopro uses the accepted 175-file source compiled for Zen 4; the local executable targets the Ryzen machine. Aeglos uses the Sorted engine at commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, with prepared-key cache hits checked. Results apply to these configurations.

## Accepted local results

The M5 CPU and Metal values are supplied historical references. They were not remeasured in these runs. Speedup is reference latency divided by Zopro latency.

|Circuit|Zopro ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.022|134.00|37|4.612×|24|2.992×|3.7|
|2x2|11.336|275.00|59|5.205×|30|2.646×|5.9|
|4x2|15.946|315.00|109|6.836×|48|3.010×|10.9|
|8x2|24.090|512.50|189|7.845×|74|3.072×|18.9|
|12x2|33.845|707.25|284|8.391×|101|2.984×|28.4|
|16x2|39.932|826.25|335|8.389×|124|3.105×|33.5|
|24x2|66.233|1328.75|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273|1175.50|507|7.767×|175|2.681×|50.7|
|54→1|162.250|1831.87|—|—|—|—|—|

There is no supplied M5 reference for 54→1. Local 1×2 needs approximately 54% lower latency to reach its 3.7 ms CPU target.

The accepted build's original full performance gate remains **false** because its physical-16 fresh-control calibration interval exceeded the declared range. Adoption used an explicit engineering exception; all candidate gain/nonregression checks and the separate final-artifact gate passed. Later experiments do not inherit that exception. [Adoption decision](../2026-10-29-balanced-kh-adoption/README.md), [final-artifact qualification](../2026-10-29-balanced-kh-production/README.md).

## AWS comparison with Aeglos

Each displayed latency is that engine's fastest median among 32, 64 and 96 workers. RSS comes from the selected count. This is a descriptive selection across the measured matrix; it does not prescribe a universal worker count. The underlying report retains every same-worker comparison and paired interval.

|Circuit|Zopro ms|Workers|Aeglos ms|Workers|Zopro peak RSS MiB|Aeglos peak RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2|11.916|32|7.959|96|120.97|480.14|
|2x2|15.895|32|11.822|96|230.36|811.11|
|4x2|19.993|32|18.492|96|276.21|1351.90|
|8x2|26.898|64|31.531|96|450.82|2491.09|
|12x2|34.862|64|48.580|96|648.76|3714.61|
|16x2|38.983|64|57.136|96|766.20|4612.36|
|24x2|54.034|64|86.667|96|1225.00|6758.14|
|merge-24x1|57.133|64|84.350|96|1109.38|6608.05|
|merge-54x1|122.397|64|173.236|96|1877.03|13115.29|

For 54→1, this is 122.397 ms and 1.83 GiB peak RSS for Zopro versus 173.236 ms and 12.81 GiB for Aeglos. Lower RSS alone does not explain latency differences. [Complete AWS measurements, configuration and artifacts](../2026-10-29-proof-pgo-c7a175/README.md).

## CPU scaling

The AWS 1×2 sweep shows the principal remaining gap:

|Workers|Zopro latency ms|Aeglos latency ms|Zopro CPU ms per proof|Aeglos CPU ms per proof|
|---|---:|---:|---:|---:|
|32|11.916|11.868|256.128|328.248|
|64|11.995|8.774|420.399|430.446|
|96|13.778|7.959|704.247|513.394|

At 32 workers the measured medians are close. Aeglos latency falls as workers increase, while Zopro consumes more CPU time without a latency improvement. Zopro's sampled epoch/scheduler residence rises from 14.09% at 32 workers to 61.48% at 96. Those flat CPU sample categories are not exclusive wall-time costs or directly removable fractions.

The measurements support investigating useful parallel work, critical paths and coordination. They do not establish RAM capacity as the sole bottleneck. The container records zero memory/swap/OOM event deltas. Its 39 CPU-throttled periods cover the entire workflow and cannot be attributed to one timed engine. The tables do not establish minimum RAM, behavior under memory pressure, mobile/ARM/TEE performance or concurrent proof throughput.

## Recent experiments

All results below remain experimental; the accepted implementation is unchanged. Full-proof comparisons use their own fresh accepted controls, so their medians should not be substituted into the earlier nine-circuit table. The latest experiment has not been measured on AWS.

### Complete SDK proofs with representative grouping

The new constructor hashes each input once using compact representative-row/count records, then copies full scalar values only after admission. It preserves query independence and adds no persistent key table, duplicate execution plan or point-array copy. Its earlier component gate failed despite primary constructor gains of 21.5% and 23.3%; the separately declared SDK experiment evaluates the complete latency and memory tradeoff across all nine circuits. [Constructor measurements](../2026-10-29-representative-groups175/README.md), [complete SDK measurements](../2026-10-29-representative-sdk175/README.md).

The SDK comparison uses 15 fixed rotated/reversed blocks at 32 workers on local CPUs 0–31. Accepted is the exact production executable; OFF is a fresh build with the experimental path disabled; ON enables it. Each process times 151 primary or 31 other requests after three primes, using the same timed scope as the accepted table. RSS includes process loading and preparation.

|Circuit|Accepted ms|OFF ms|ON ms|Accepted peak RSS MiB|OFF peak RSS MiB|ON peak RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.152022|8.026144|8.033347|134.50|136.25|83.75|
|2x2|11.394626|11.246055|11.204142|265.00|273.00|217.50|
|4x2|16.055527|15.896628|15.852074|314.75|323.50|290.75|
|8x2|24.230379|24.140120|24.083721|517.50|502.50|479.00|
|12x2|33.869702|33.798232|33.531005|713.25|727.00|657.00|
|16x2|40.583149|40.040498|39.670771|814.75|825.75|787.00|
|24x2|66.708818|66.117965|66.813065|1282.84|1297.41|1221.75|
|merge-24x1|65.574555|64.976160|65.470053|1180.02|1180.72|1135.04|
|merge-54x1|162.474643|162.128890|172.728370|1885.01|1875.35|1897.42|

Ratios pair process medians within each block and therefore need not equal ratios of the displayed medians. Lower is faster. The original-eight aggregate gives each reference circuit equal weight in log latency.

|Paired comparison|Latency ratio|Bounds|Wins / 15|
|---|---:|---|---:|
|1x2 ON/accepted|0.986585469|[0.982801006, 0.989434219]|15|
|1x2 ON/OFF|1.002662286|[0.997012758, 1.005840682]|6|
|Original eight ON/accepted|0.989047179|[0.987302759, 0.990663458]|15|
|Original eight ON/OFF|0.998636422|[0.995014629, 1.002273267]|9|
|54→1 ON/accepted|1.068378645|[1.059385016, 1.071481907]|0|
|54→1 ON/OFF|1.065869552|[1.059408609, 1.069339369]|0|

Primary and original-eight gain checks require an upper ratio below 1 and at least 10 wins against both controls. Every circuit must have an upper ratio at most 1.02 and maximum RSS at most 1.05 times each control. Ranks 4–12 of 15 provide nominal 96.484375% individual median coverage under independent/exchangeable blocks, without a familywise guarantee.

Four speed checks fail: primary ON/OFF, original-eight aggregate ON/OFF, and both 54→1 nonregression comparisons. Every relative RSS check passes. All fresh-control calibration intervals fall within factor 1.03; calibration is diagnostic in this experiment. No thresholds or included circuits changed after measurement.

The original-eight aggregate improves 1.095% versus accepted, while its OFF comparison spans parity. Memory falls on all original eight circuits but rises slightly on 54→1. On 54→1, CPU time increases from 4,457.806 to 4,673.598 ms per request versus accepted, approximately 4.8%; the existing primary-only profiles cannot locate that additional work. This implementation is not adopted. The lower primary RSS remains a useful lead for a revised design.

### Other recent results

|Experiment|Measured result|Decision|
|---|---|---|
|[Signed differences in complete G2 batches](../2026-10-29-signed-g2-caller175/README.md)|Borrowed 200-pair batches improve 2.42% solo and 2.26% at 16 physical cores, but regress 1.87% at 32 logical workers. All calibration and nonregression checks pass.|Aggregate gain checks fail; no SDK integration or adoption.|
|[Two-pass deferred grouping](../2026-10-29-deferred-groups175/README.md)|Primary constructors improve about 44%, but the all-input constructor aggregate is 20.87% slower than original.|Component gate fails; no SDK integration. Superseded experimentally by the one-pass constructor above.|
|[Direct signed CIOS difference](../2026-10-29-signed-difference175/README.md)|Affine expression 7.38% faster versus fresh ordinary code, 31/31 wins; four-expression aggregate 0.45% faster. Dependent chain regresses 2.89%, scaled product 3.81%.|Generic component gate fails three nonregression checks. No SDK proof timing or adoption.|
|[Borrowed GLV fused K/H](../2026-10-29-glv-kh-borrowed175/README.md)|Full-proof candidate 8.481 ms versus accepted 8.140 ms; paired regression 4.27%, zero wins in 31 blocks. Peak RSS 137.00 versus 133.75 MiB.|Not adopted; both gain checks fail, calibration passes.|
|[Shared witness classification](../2026-10-29-shared-wire-sdk175/README.md)|Candidate 9.401 ms versus accepted 8.130 ms; paired regression 15.90%. Peak RSS 99.25 versus 136.75 MiB.|Not adopted for speed; lower observed RSS retained as a memory tradeoff lead.|
|[Index-only grouped dictionary](../2026-10-29-swiss-groups-sdk175/README.md)|Candidate 8.012 ms versus accepted 8.031 ms; paired gain interval spans parity. Peak RSS 102.50 versus 133.50 MiB.|Not adopted; accepted-relative gain and fresh-control calibration checks fail.|
|[Early-X arithmetic](../2026-10-29-early-x-sdk175/README.md)|Caller aggregate improves 0.320%; full proof is 8.068 ms versus accepted 8.040 ms.|Not adopted; full-proof gain checks fail.|
|[Proof PGO on AWS](../2026-10-29-proof-pgo-c7a175/README.md)|All-eight aggregate approximately flat at 32 workers, 5.704% slower at 64 and 4.997% slower at 96.|Not adopted; 17 nonregression checks fail.|

The signed-difference helper adds no heap buffer, persistent key table or point copy. Its BigInt model passes 30,084 exact cases, and two native tests pass 172,544 chained lane comparisons against Arkworks. The affine-expression paired candidate/fresh ratio is 0.926221229 with bounds [0.925461631, 0.927912346]. These expression measurements omit inversion, packing, bucket construction and scheduling, so their gain cannot be applied directly to proof latency.

## Conclusions and next experiment

Zopro's accepted approach combines vector field arithmetic, batched affine MSM additions, borrowed point operands, scalar endomorphism decomposition and balanced fused K/H windows. Its observed AWS memory advantage is useful, while Aeglos retains the small-circuit latency lead in the measured matrix. None of the recent SDK experiments above passes its declared complete-request performance gate.

The next selected diagnostic profiles the frozen accepted, OFF and ON 54→1 executables, collecting CPU counters and sampled code paths before further changes. It should locate whether the extra CPU work lies in grouping, allocation, MSM, FFT or scheduling. That diagnostic is prepared but has no completed capture at this snapshot; it does not qualify a new latency result. The six completed SDK profiles cover only primary 1×2 at 16 and 32 workers, so they cannot explain the large-circuit regression.

Further implementation work should preserve independent query readiness and bounded temporary memory without large per-key lookup expansions. A revised design needs complete-request measurements across the circuit matrix; constructor gains and low RSS alone do not establish a production improvement.

## Verification and source records

The accepted final-artifact qualification passes 124 native core and eight prover tests, 101 portable core and eight prover tests, 28,000 SDK requests and 505 distinct independent Gnark proof exports. The latest AWS comparison adds 37,272 SDK requests and 1,248 distinct independent exports across its three engines. All benchmark-owned resources from that AWS run were removed.

The representative SDK trial passes 124/127 native core tests and eight/eight prover tests for OFF/ON. It completes 20,802 internally verified SDK requests, 438 fresh independently Gnark-verified proof exports and six profiles with one sampled process and zero lost records each. Independently verified preflights precede latency; all latency captures precede profiles; all captures finish before the remaining independent verifications and offline analysis. No owned compiler, analysis helper or second CPU workload overlaps capture. Its 900 workflow steps complete. The historical distinct-proof ledger is **21,697**. The report itself adds no timing or verification samples.

The [detailed current summary](../2026-10-29-current-cpu-summary/README.md) retains the wider experiment history. This snapshot includes [full-precision data](data.json), [supporting source hashes](source-hashes.json), and [file checksums](SHA256SUMS). Source-hash paths are relative to the repository's `zopro/` directory. Each linked experiment preserves its declared checks, failed checks, compiler flags, source/binary identities, process results and verification receipts.
