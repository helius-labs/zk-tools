# Preparation and SDK stage costs

On the local Ryzen 9950X with32 Rayon workers, the current accepted prover takes **8.563 ms** for1x2 and **24.330 ms** for8x2. This diagnostic finds a substantial serial grouping pass before the small G2 query starts arithmetic. It proposes no production change and adds no AWS result.

| Circuit | Accepted SDK ms | Diagnostic OFF ms | Diagnostic ON ms | Accepted maximum RSS MiB | Supplied M5 CPU / Metal ms |
|---|---:|---:|---:|---:|---:|
|1x2|8.562586|8.681129|8.635884|98.25|37 /24|
|8x2|24.329978|25.058232|25.222441|465.50|189 /74|

Times are medians of three fresh-process medians,17 requests after three primes, rotated/reversed accepted/OFF/ON order, CPUs0–31. Each request includes fresh witness solving, randomness, proving, verification and serialization. Loading/preparation is excluded. RSS covers the whole process, including load/setup/primes; it is not minimum RAM. M5 values are supplied historical references. These local times are separate from the [fresh C7a comparison](../2026-10-26-ready-c7a/README.md).

## Small-proof preparation

Each primary capture records the same stage labels and scalar sizes. The following are medians across51 traced requests. Bounds are inclusive and overlap; do not sum nested stages or independent medians into a synthetic request.

| Primary stage | Wall ms | Calling-thread CPU ms |
|---|---:|---:|
| Solve wires |1.189|1.168|
| Canonical conversion |0.070|0.068|
| Gather A/B scalars |0.123|0.123|
| Product FFT |0.607|0.602|
| A grouping policy,21006 scalars |0.821|0.715|
| A map allocation/build/destruction, inside grouping |0.541|0.541|
| G2 grouping policy,30089 scalars |1.351|1.295|
| G2 map allocation/build/destruction, inside grouping |1.096|1.092|
| G2 scalar split |0.339|0.339|
| G2 arithmetic after split |4.063|3.404|
| Randomized assembly |0.159|0.010|
| SDK verification |0.468|0.335|
| SDK serialization |0.012|0.012|

G2 query entry occurs around2.220 ms after the first recorded witness span, nearly immediately after query-join entry around2.217 ms. G2 arithmetic starts around3.930 ms. Grouping and scalar decomposition account for most of that local preparation interval. Both A and B grouping ultimately decline: no packing spans occur, and the arithmetic retains the original scalar count. This agrees with the earlier cardinality census:17454/21006 A and26142/30089 B nonzero classes exceed the active80-percent cutoff. The serial map pass therefore performs work before returning to the ungrouped path.

Thread CPU closely tracks wall time in the map spans, which contain no nested Rayon work. This supports investigating avoidable CPU work there. It does not establish the same durations on C7a or prove that removing grouping saves the full observed duration from end-to-end latency. The split and other parallel spans can include unrelated work executed by the calling Rayon worker while waiting; their thread CPU is not exclusive stage CPU or summed child CPU. The arithmetic interval is inclusive and is not directly comparable with Aeglos's MSM timer.

The8x2 path takes a different core route; its solve/FFT/query boundaries are not covered by the small-path instrumentation. Its recorded grouping spans include useful grouping and packing, so the small-case rejection opportunity cannot be generalized to all inputs. Missing spans mean absent instrumentation, not zero work.

## Qualification and next experiment

The recorder uses a fixed512-record,32768-byte atomic buffer, monotonic wall and thread CPU clocks, and same-thread RAII spans. It allocates/formats/exports outside the request timer. It performs bounded clock syscalls during traced requests. Four recorder tests cover parallel/nested spans, overflow, incomplete spans and panic reuse;113 unfiltered native core tests, two raw proof tests, six cubic tests and two analyzer tests pass. Field arithmetic is unchanged and its existing qualification is inherited.

The full timing qualification **fails** one8x2 calibration block: OFF/accepted1.052249 exceeds the declared1.05 per-block bound. Paired OFF/accepted medians are1.023389 and1.028188; paired ON/OFF medians are1.003934 and1.014748 for1x2/8x2. Every primary check passes individually; the full-matrix failure is retained. No timing is excluded or rerun, and diagnostic latency does not replace the accepted reference.

Four correctness preflights plus18 timing processes complete **376 SDK calls,22 distinct independently Gnark-verified proof exports and44 proof/public members**, with104 bounded snapshots. Fixture hashes,164-byte proof lengths, distinct fresh randomness and all pinned sources/inputs are checked against5216 preceding exports. All170 accepted source files and the accepted SDK remain unchanged. No cloud resources are created. Initial pin-schema/output-directory setup failures occur before build or SDK execution and are preserved.

Next: test a small bitmap as an exact lower bound on distinct nonzero scalars. Occupied bits cannot exceed distinct scalars; exceeding the existing grouping cutoff can safely decline without constructing a full map. Collisions only cause fallback. This differs from the previously rejected higher grouping-acceptance cutoff. Memory charging, real input decisions and complete uninstrumented SDK comparisons must be verified before adoption. No performance gain is claimed yet, and the all-eight CPU10x goal remains unmet.

[Declared protocol](artifacts/design.md), [full analysis](artifacts/analysis.json), [source edits](artifacts/edits.json), [proof inventory](artifacts/proof-inventory.json), [native test records](artifacts/native/result.json), [measurement jobs](artifacts/performance-jobs.json) and [artifact hashes](artifacts/artifact-index.json) retain the evidence. Source and executable hashes are pinned; executables and large keys remain local. Snapshot JSONs are compressed losslessly.
