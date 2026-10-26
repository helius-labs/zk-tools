# Exact grouping lower bound on C7a

The unchanged bitmap candidate does not pass the declared C7a adoption gate. Production remains unchanged. This completes the target-specific follow-up to the [local bitmap screen](../2026-10-26-grouping-lower-bound/README.md), with current accepted Zopro, candidate OFF/ON and pinned Aeglos Sorted on the same dedicated c7a.24xlarge.

The [proposed-adoption report](PROPOSED-ADOPTION.md) describes the resulting speed and memory comparison if we accept the candidate, including the qualification limits.

## Request latency

EPYC9R14,96physical cores without SMT,30GiB container. Three rotated/reversed fresh-process blocks;31timed requests for1x2 and17 for other shapes, after3primes. All engines use compact firstN CPU taskset masks. Accepted and candidate target znver4; OFF/ON use the same executable with an explicit runtime0/1. Aeglos uses its retained prepared-key policy and budget. Witness solving, fresh randomness, proving, SDK verification and serialization are timed; loading/preparation is excluded. No owned build, archive, validation or profile overlaps cloud latency measurements.

Each cell is the median of three process medians in milliseconds.

| Circuit | Workers | Accepted Zopro | Candidate OFF | Candidate ON | Aeglos Sorted | ON/OFF paired geometric mean |
|---|---:|---:|---:|---:|---:|---:|
|1x2|32|12.369411|12.340488|12.377347|11.886016|1.000004|
|1x2|64|12.630590|12.865719|12.095798|8.819515|0.944125|
|1x2|96|14.144541|14.873470|12.845416|8.006822|0.863675|
|2x2|64|17.204238|17.648451|17.809364|13.126056|1.007647|
|2x2|96|18.489178|18.869297|19.367080|11.555503|1.009373|
|4x2|64|20.987779|21.598432|21.103947|20.915727|0.992257|
|4x2|96|23.279178|23.457791|23.108750|18.461442|0.997761|
|8x2|64|27.416094|27.224324|27.456417|35.869768|1.002188|
|8x2|96|29.307536|29.694693|30.365974|31.115283|1.028815|
|12x2|64|34.644841|36.135480|36.084750|57.470874|1.006855|
|12x2|96|38.527998|38.077085|38.457207|48.775704|1.009647|
|16x2|64|39.187621|40.134890|40.089777|65.155543|1.006298|
|16x2|96|43.981256|43.126691|43.655485|56.886177|1.011302|
|24x2|64|56.891591|58.353688|57.358674|106.957620|0.984799|
|24x2|96|62.747293|61.675081|61.797498|87.358076|0.988577|
|merge-24x1|64|60.697836|60.724196|59.219785|104.052996|0.978822|
|merge-24x1|96|66.109811|66.240022|67.935210|84.883804|1.019339|
|merge-54x1|64|125.893127|127.859627|126.333445|208.796586|0.993766|
|merge-54x1|96|133.271174|136.282686|136.173609|173.728938|1.001213|

Primary32 is a descriptive scaling control. Qualification requires primary improvements at both64/96 and nonregression of every other target case. The complete [analysis](artifacts/analysis.json) retains all block ratios, resource observations, profiles and failures. Best-tested worker counts below are descriptive and are not an adopted worker-selection policy.

## Best tested configurations and supplied M5 references

Whole-process RSS includes loading/setup/primes. It is neither incremental scratch nor minimum RAM. Summed CPU includes coordination and is not an operation count. M5CPU/Metal values are supplied historical references. Candidate ratios do not establish adoption.

| Circuit | Accepted ms / workers | Candidate ms / workers | Aeglos ms / workers | Accepted / candidate / Aeglos RSS MiB | Supplied M5 CPU / Metal ms | Candidate CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|
|1x2|12.369 /32|12.096 /64|8.007 /96|101.6 /69.3 /464.9|37 /24|3.06× /1.98×|
|2x2|17.204 /64|17.809 /64|11.556 /96|220.0 /221.5 /808.5|59 /30|3.31× /1.68×|
|4x2|20.988 /64|21.104 /64|18.461 /96|269.8 /270.6 /1349.1|109 /48|5.16× /2.27×|
|8x2|27.416 /64|27.456 /64|31.115 /96|478.4 /476.2 /2458.0|189 /74|6.88× /2.70×|
|12x2|34.645 /64|36.085 /64|48.776 /96|639.8 /669.2 /3710.5|284 /101|7.87× /2.80×|
|16x2|39.188 /64|40.090 /64|56.886 /96|788.8 /793.3 /4622.7|335 /124|8.36× /3.09×|
|24x2|56.892 /64|57.359 /64|87.358 /96|1194.7 /1262.3 /6705.9|528 /176|9.21× /3.07×|
|merge-24x1|60.698 /64|59.220 /64|84.884 /96|1107.9 /1147.8 /6569.8|507 /175|8.56× /2.96×|
|merge-54x1|125.893 /64|126.333 /64|173.729 /96|1883.1 /1916.1 /12991.8|not supplied|—|

0 of8 candidate best-tested rows reach the supplied CPU10× reference. The complete all-eight objective remains unmet; no configuration is adopted from this result. Concurrent throughput, phone/TEE overhead, memory pressure, swap and giant circuits are unmeasured here.

## Declared gate

The [frozen protocol](artifacts/design.md) requires primary64/96 ON/OFF and ON/accepted≤0.98 with at least two block wins each, every other target shape≤1.02 against both controls, original-eight aggregate ratios≤0.995 at each worker count, and calibration OFF/accepted within1.03 paired and1.05 per block. No result is excluded or rerun. Failures:

- 1x2-w96 b0 OFF/accepted=1.0558596941475644
- 1x2-w96 paired OFF/accepted=1.044390553217669
- 2x2-w64 b1 OFF/accepted=1.0589654357048803
- 2x2-w96 b2 OFF/accepted=1.0609440937945984
- merge-54x1-w96 b0 OFF/accepted=1.06602353978521
- primary_failures: none.
- 2x2-w64 ON/accepted=1.0442025470967888
- 2x2-w96 ON/accepted=1.0386335157387603
- 8x2-w96 ON/OFF=1.0288146606012518
- 8x2-w96 ON/accepted=1.0357750238169534
- 12x2-w64 ON/accepted=1.0357576739919927
- 16x2-w64 ON/accepted=1.0268360139694352
- merge-24x1-w96 ON/accepted=1.020361675236845
- merge-54x1-w96 ON/accepted=1.03063281317805
- w64 ON/OFF=0.9901636914506211, ON/accepted=1.0092822694889096
- w96 ON/OFF=0.9897331156720165, ON/accepted=0.9965339471486925

## Algorithm, profiles and verification

This is the exact local candidate: for8192≤n<32768 with the existing80percent grouping cutoff, a temporary16–64KiB bitmap counts occupied hash bits. Occupancy cannot exceed distinct nonzero scalars; exceeding the existing cutoff safely rejects full grouping. Collisions only cause fallback. The bitmap is freed before full grouping allocations and fits within the existing forecast classes-vector phase after the existing checked reservation. No large lookup table, new proving math, changed cutoff or scheduler change.

The previously source-pinned accepted znver4 SDK and its33field/109core/raw2/cubic6 qualification are reused. The identical candidate source receives a fresh znver4 build and113unfiltered core tests, including4exact-bound/collision/cardinality/memory tests, plus raw2/cubic6 and the generated-IFMA check. Field source is unchanged; accepted target field qualification is inherited. Exact-container local preflight completes144SDK36Gnark across all nine shapes and four engines before publication or launch. Cloud completes5064latency SDK calls and432profile SDK calls with236distinct exports. Combined **5640SDK calls,272distinct Gnark exports and544proof/public members**. Every export independently verifies again locally after transport across9pinned keys. Repeat verification is counted once. Fixture public hashes,164byte proofs, fresh randomized identities and source/image/executable hashes are checked against5340 preceding exports; next prior set5612.

Eight separate199Hz cycles profiles follow all latency measurements, primary32/96 for each engine. They are descriptive aggregate user-cycle evidence, not request acceptance timings. Compiler inlining and generic symbol categories do not isolate stage CPU or permit treating a missing symbol as zero work. Aeglos MSM timing has different boundaries from previous Zopro arithmetic traces.

| Case / engine | Profile samples | Lost records | Epoch + scheduler category % | Grouping category % |
|---|---:|---:|---:|---:|
|1x2-w32/accepted|4931|0|12.465|0.821|
|1x2-w32/aeglos|3618|0|0.000|0.000|
|1x2-w32/off|4487|0|14.036|0.771|
|1x2-w32/on|5396|0|11.533|0.000|
|1x2-w96/accepted|19656|0|62.767|0.247|
|1x2-w96/aeglos|6152|0|0.000|0.000|
|1x2-w96/off|17079|0|64.373|0.214|
|1x2-w96/on|15920|0|63.492|0.035|

The direct pinned perf6.8 executable passes a cycles:u availability check before cloud SDK measurements, avoiding the previously observed kernel-wrapper mismatch. Source and iteration counts do not change. Whole-workflow cgroup before/after counter deltas are in the [closeout receipt](artifacts/closeout.json); they include verification/profile work and are not per-request measurements.

## Cleanup and artifacts

The owned instance is terminated. Its encrypted EBS volume, temporary ECR tag, private transfer object and bucket are confirmed absent; the presigned URL parameter file is deleted. No temporary benchmark instance remains live in the region. Production services, networking and IAM policies are unchanged. [Cleanup receipt](artifacts/cleanup-complete.json).

All170 accepted production files and accepted SDK remain unchanged. [Full analysis](artifacts/analysis.json), [raw request/profile rows](artifacts/remote-results/output/measure/performance/rows.json), [exact container manifest](artifacts/context/manifest.json), [build pins](artifacts/build-pins.json), [candidate source edits](artifacts/candidate-source-edits.json), [proof inventory](artifacts/proof-inventory.json), [posttransport Gnark log](artifacts/post-transport-gnark.log) and [artifact index](artifacts/artifact-index.json) retain the evidence. Large keys, binaries and raw perf.data remain local with hashes; decoded profiles and proof exports are archived. Private transport parameters are excluded.
