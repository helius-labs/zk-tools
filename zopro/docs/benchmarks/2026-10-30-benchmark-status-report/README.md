# CPU prover benchmark status

30 October 2026. Snapshot after the bounded G2 queue-scan native workflow, based on repository commit `e5cda2cbba597354226927fa515be8ad10dd6518`. This document consolidates existing measurements; writing it adds no benchmark samples or proofs.

Accepted Zopro **1×2 remains 8.022 ms locally with 134 MiB peak process RSS**. The latest AWS c7a.24xlarge comparison is **11.916 ms for Zopro versus 7.959 ms for Aeglos Sorted**, at each engine's fastest tested worker count. Zopro leads six of nine AWS circuits in that selection and uses less observed peak RSS on all nine. Aeglos leads the three smallest circuits. The supplied M5 CPU 10× target remains **unmet on all eight reference shapes**.

The latest queue-scan candidate has passed native tests, but has **no measured latency or peak prover RSS**. Its count model removes some projective work while adding affine updates, inversions and queue inspections. That tradeoff needs a complete-query measurement before any proof-speed claim.

## Scope and reproducibility

Local measurements use a Ryzen 9 9950X, 16 physical cores, 32 logical CPUs and 128 GiB RAM on a shared host. The accepted local table uses 32 workers. AWS measurements use one c7a.24xlarge with 96 physical cores without SMT, testing 32/64/96 workers. The network-disabled AWS container has a 96-CPU quota, a 30 GiB memory limit and no swap; that limit is a benchmark setting.

Accepted timings are medians of 15 process medians after three priming proofs. Local primary processes measure 151 requests and other shapes 31; AWS processes measure 101 and 17 respectively. Timed Zopro requests include fresh witness solving, randomized proving, SDK verification and serialization. Loading and preparation are excluded from latency and included in lifetime RSS. Peak RSS is the largest process high-water mark across the corresponding blocks; it does not establish minimum RAM or incremental proof workspace.

Production remains the accepted 175-file source. The local SDK executable SHA-256 is `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f`. AWS uses the accepted source compiled for Zen 4, a different executable. Aeglos uses the Sorted engine at commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, with prepared-key cache hits checked. Comparisons describe these tested configurations. M5 CPU and Metal values are supplied historical references, not fresh Apple measurements.

## Accepted local proof results

Speedup is reference latency divided by Zopro latency. Times are milliseconds; memory is MiB.

|Circuit|Zopro ms|Peak RSS|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.022|134.00|37|4.612×|24|2.992×|3.7|
|2x2|11.336|275.00|59|5.205×|30|2.646×|5.9|
|4x2|15.946|315.00|109|6.836×|48|3.010×|10.9|
|8x2|24.090|512.50|189|7.845×|74|3.072×|18.9|
|12x2|33.845|707.25|284|8.391×|101|2.984×|28.4|
|16x2|39.932|826.25|335|8.389×|124|3.105×|33.5|
|24x2|66.233|1328.75|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273|1175.50|507|7.767×|175|2.681×|50.7|
|merge-54x1|162.250|1831.87|—|—|—|—|—|

No M5 reference was supplied for 54→1. Primary 1×2 requires approximately 54% lower latency to reach 3.7 ms.

The accepted build's original full performance gate remains **false** because physical-16 fresh-control calibration exceeded its declared range. Adoption was an explicit engineering exception; candidate gain/nonregression checks and the separate final-artifact gate passed. Later experiments do not inherit that exception. [Adoption decision](../2026-10-29-balanced-kh-adoption/README.md), [final-artifact qualification](../2026-10-29-balanced-kh-production/README.md).

## Latest AWS comparison

Each latency is the fastest median among 32/64/96 workers for that prover; RSS comes from the selected count. This selection is descriptive. The source report retains all equal-worker paired comparisons.

|Circuit|Zopro ms|Workers|Aeglos ms|Workers|Zopro RSS MiB|Aeglos RSS MiB|
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

For 54→1, observed RSS is 1.83 GiB for Zopro and 12.81 GiB for Aeglos. These observations do not establish why latency differs. No newer AWS measurement exists for the candidates below. [Full AWS matrix, profiles and artifacts](../2026-10-29-proof-pgo-c7a175/README.md).

The 1×2 worker sweep shows the scaling difference directly:

|Workers|Zopro ms|Aeglos ms|Zopro summed CPU ms/request|Aeglos summed CPU ms/request|
|---|---:|---:|---:|---:|
|32|11.916|11.868|256.128|328.248|
|64|11.995|8.774|420.399|430.446|
|96|13.778|7.959|704.247|513.394|

Aeglos reduces latency as workers increase; Zopro consumes more CPU without reducing latency in this sweep. Useful parallel work, critical paths and coordination remain investigation targets. RAM capacity, cache behavior, syscalls and synchronization have not been isolated as the cause. Local SMT behavior cannot be transferred directly to AWS.

## Recent completed experiments

Candidate/control ratios below pair observations within fixed blocks; they need not equal ratios of displayed medians. Lower is faster. Bounds are nominal individual median intervals, not familywise guarantees.

|Experiment and scope|Key result|Decision|
|---|---|---|
|Combined cached-hash grouping and queue drain; full SDK|1×2 7.846378 ms, 87 MiB RSS; paired gain versus accepted 2.47%|Not adopted: 8×2 nonregression and physical-16 control-calibration checks failed|
|Bounded G2 delta; full SDK|1×2 8.168629 ms, 136.50 MiB RSS; ON/OFF 1.016085852 [1.009616275, 1.017233682], 6/31 wins|Rejected: primary and original-eight gain checks fail, with additional physical-16 failures|
|G2 prefix endpoints; real-operand batch replay|Primary ON/OFF 1.013109710 [1.011559551, 1.016175286], 2/31 wins|Rejected before SDK timing|
|Cold scalar fallback extraction; real-operand batch replay|Primary ON/OFF 1.010588058 [1.005824531, 1.015566998], 6/31 wins|Rejected before SDK timing; positive short-tail results retained|
|Direct trace-one symmetric square; synthetic arithmetic expression|Norm-affine SQUARE/OFF 1.045108395 [1.043746316, 1.046602792], 0/31 wins|Rejected before valid-curve integration; improved old basis still loses to ordinary arithmetic|

The combined candidate's lower RSS and primary gain are useful observations. Its failed whole-matrix gate remains recorded; an interval whose upper bound exceeds a tolerance does not establish a slowdown equal to that bound. [Combined SDK experiment](../2026-10-29-cached-drain175/README.md).

The bounded-delta caller had passed with a 0.405% aggregate improvement versus OFF, but the SDK integration did not transfer that improvement. Its full trial completed 55,788 internally verified requests and 972 fresh independent Gnark exports. [Complete SDK results](../2026-10-30-loose-delta-sdk175/README.md).

The endpoint and cold-tail timings measure batch replay, not proofs. Compact replay preserves selected arithmetic inputs but does not preserve complete key/bucket locality or proof scheduling. All failed checks and positive cells remain in the [endpoint report](../2026-10-30-g2-endpoints175/README.md) and [cold-tail report](../2026-10-30-g2-cold-tail175/README.md). The [trace-one report](../2026-10-30-trace-one-square175/README.md) retains all four arithmetic variants, tests and profiles.

## Current experiment: bounded G2 queue scan

The accepted borrowed G2 queue drain stops at the first busy tail entry. The candidate scans the bounded existing queue in descending order, removes free entries and leaves blocked entries for later. It retains batch200, the existing queue/scratch, scalar recoding and worker policy. It adds no heap buffer, point-array copy, per-key lookup table, duplicate execution plan or persistent cache. It changes queue order, so actual curve exceptions require qualification. This differs from the earlier flush-queue change in the combined candidate.

The ordinary noncancelling count model uses real scalar populations, source-extracted recoding and two independent field-token sum checks. It excludes exceptional curve behavior, the special top window, actual arithmetic costs and proof scheduling.

|Shape|Projective arrivals, old → scan|Final merges, old → scan|Batch inversions, old → scan|Busy checks, old → scan|
|---|---:|---:|---:|---:|
|1x2|1178 → 617|429 → 114|2605 → 2610|589924 → 598703|
|2x2|376 → 284|360 → 271|2688 → 2689|607172 → 607487|

For primary 1×2, 561 arrivals move to affine processing, about 0.10% of all modeled arrivals. Five additional inversions and 8,779 busy checks may offset the saved projective work. The necessary opportunity check passes; this is not evidence of a 2× gain. All modeled bucket sums, reconstruction checks and original capacity limits pass. [Count model, assumptions and source](../2026-10-30-g2-queue-scan-count175/README.md).

The new native workflow is complete in all 15 steps. The API-only baseline passes 124 core tests; fresh OFF and ON each pass 126. Each variant also passes eight prover tests, and its original SDK consumer links. New tests exercise a blocked tail hiding a free entry, actual bucket outputs, full/tail batches, signs, identities, doubling, cancellation and input immutability. All 17 field source files remain unchanged. These tests qualify the private implementation, not its performance.

At this snapshot, whole-query fixture/oracle execution and performance capture have **not started**. An earlier fixture-workflow launch was rejected by the host lock before any step ran. The independent Arkworks checker is compiled. There is no whole-query latency, calibration result, proof-speed gain, measured peak prover RSS or adoption decision for this candidate. [Native status and test receipts](artifacts/queue-scan-native.json).

The next planned measurement checks independent complete G2 sums before timing, then compares API-only baseline, OFF and ON in 31 fixed rotated/reversed blocks: primary at 1/16/32 workers and 2×2 at32. Counters and profiles follow latency. Only a passing complete-query comparison selects a separate complete SDK experiment with both controls, proof verification and memory checks; portable, resource and final-artifact qualification remain required before adoption.

## Conclusions and limits

Accepted performance has not changed. The most useful current evidence is the AWS scaling gap and the combined candidate's measured primary/RSS improvement, alongside its unresolved whole-matrix checks. Recent isolated arithmetic and layout improvements have repeatedly failed to transfer to complete proofs. The queue scan tests a small work-removal opportunity with existing storage; its speed remains unknown.

The retained distinct independently verified proof ledger is 23,971. This report creates no new requests, exports, captures or AWS resources. The latest native candidate adds tests and builds, not proof measurements.

Existing sample-period accounting failed 21/24 diagnostic panels. Profile attribution remains qualitative; sample shares are not exclusive costs, local CPI or removable proof wall time. [Worker counters](../2026-10-29-worker-work175/README.md), [sampling-accounting limits](../2026-10-29-sample-accounting175/README.md).

These measurements do not establish minimum RAM, memory-pressure/swap behavior, ARM/mobile/TEE performance, forester 250/4999 performance or concurrent throughput. No large per-key lookup table is proposed.

## Records

[Machine-readable accepted rows](data.json), [source hashes](source-hashes.json), [checksums](SHA256SUMS). Exact observations and failed checks remain in the linked experiment reports. The [earlier requested snapshot](../2026-10-30-current-benchmark-report/README.md) is preserved unchanged. The [rolling summary](../2026-10-29-current-cpu-summary/README.md) provides the broader experiment history.
