# Identical-binary fresh-process control

The corrected local control stays within every existing calibration bound. This run does not reproduce the larger control differences seen in previous candidate comparisons. It does not establish zero variation or prove that code changes caused those earlier differences.

There is no prover optimization or adoption here. The original all-eight CPU10× goal remains unmet. This is a measurement control needed before interpreting small performance differences.

## Protocol and disclosed repair

Local Ryzen9950X,16physical/32SMT cores,32workers,CPUs0–31. Both neutral labels a/b use exactly the same SDK path/inode/SHA256, accepted source, managed environment, key images and fixture requests. The only differing argument is the output directory. The executable SHA256 is `03e1260e89068c2bec37a2471c34e93628a9f081903e1b7eee9cab9c306aaf3d`. No prover or dependency rebuild occurs.

Each series contains all nine shapes × two labels × three rotated/reversed fresh-process blocks, with 31 timed requests after three primes: 54 latency processes, 1,836 SDK calls and 54 exports. After all latency measurements, two primary 199 Hz profiles, with 51 timed requests after three primes, add 108 SDK calls and two exports. Each complete series totals 1,944 SDK calls and 56 distinct Gnark exports. Fresh witness solving, randomness, proving, verification and serialization stay timed; loading and preparation are excluded.

The initial series violated isolation: its controller compiled the small checkpoint helper after timing began. The observed command took0.3388seconds including compile/checkpoint work, while progress showed the first block's merge54process. Exact per-proof overlap is unknown. That entire series is retained and marked nonconforming for timing; its bound crossings are not presented as clean baseline evidence.

Before the corrected series, every helper was compiled. The corrected controller then runs capture and postcapture analysis sequentially, with no owned build/profile/archive overlap during latency. Binary, settings, counts, ordering algorithm, primes and analysis bounds remain identical. Only output directories, immutable helper pins and the prior-proof set change. All56initial exports are included in corrected freshness checks. No corrected sample is excluded or repeated. [Repair declaration](artifacts/isolation-repair.md), [protocol equality check](artifacts/corrected-run/protocol-repair.json).

## Corrected control results

Each label's latency is the median of its three process medians. Paired b/a ratios are computed within blocks and may differ from ratios of displayed medians. Values below1mean labelb happened to be faster; there is no algorithm difference. The bounds are the same as the previous experiments: factor1.05 per block and1.03 for the paired median.

| Shape | a ms | b ms | Block b/a ratios | Paired median b/a | Paired geometric mean b/a |
|---|---:|---:|---|---:|---:|
|1x2|8.600057|8.611779|0.998143 / 1.001363 / 0.992509|0.998143|0.997332|
|2x2|11.555575|11.675460|1.016269 / 1.010375 / 0.984422|1.010375|1.003593|
|4x2|15.990403|16.217802|0.991422 / 0.983070 / 1.022148|0.991422|0.998740|
|8x2|24.345417|24.317866|0.993888 / 0.994042 / 1.023548|0.994042|1.003730|
|12x2|34.641517|33.899064|0.976749 / 0.990872 / 0.981255|0.981255|0.982941|
|16x2|40.705957|40.785367|0.985001 / 1.031261 / 1.001951|1.001951|1.005890|
|24x2|67.542436|68.388142|1.014071 / 1.012521 / 0.993039|1.012521|1.006498|
|merge-24x1|67.216960|67.190340|0.977751 / 1.010303 / 0.999604|0.999604|0.995793|
|merge-54x1|176.415503|175.910365|0.995556 / 0.999791 / 0.984314|0.995556|0.993199|

Across all27case/block pairs, b/a geometric mean=0.998610. Corrected bound crossings:

- None.

These observations do not permit discarding earlier failures, changing a gate after seeing a candidate result, or treating a tiny apparent gain as established. They describe one accepted binary on this local machine. A C7a conclusion requires target-specific evidence.

## Within-process observations

First5/last5 ratios use the timed samples after the existing3primes. A falling sequence can be compatible with warmup or scheduling changes; it does not establish their cause or a sufficient priming count. [All process observations](artifacts/corrected-run/analysis.json) retain min/max, median absolute deviation, CPU per proof, faults, switches and whole-process RSS. Involuntary switches alone are not proof of external preemption.

| Corrected primary process | Median ms | First5 ms | Last5 ms | Last/first | CPU ms/proof | Involuntary switches/proof |
|---|---:|---:|---:|---:|---:|---:|
|b0 /a|8.433534|8.723531|8.388819|0.961631|195.936|298.03|
|b0 /b|8.417874|8.570872|8.427272|0.983246|197.067|188.16|
|b1 /a|8.600057|8.659850|8.564280|0.988964|199.427|534.23|
|b1 /b|8.611779|8.953995|8.440877|0.942694|198.751|389.42|
|b2 /b|8.645182|8.735352|8.458079|0.968259|198.212|321.77|
|b2 /a|8.710435|8.888061|8.663437|0.974727|198.705|592.29|

Host/CPU-policy/cgroup snapshots are outside request timing. Root cgroup cpu.max is unavailable; the corrected supplemental snapshots inspect the controller's visible cgroup ancestry. These are not per-request or guaranteed child-process quota attribution, and no zero-throttling cause/absence is inferred from missing fields. No CPU policy, cgroup limit or affinity setting changes beyond the declared child taskset.

## Baseline and supplied M5 references

Pooled baseline is the median of the six corrected process medians per shape, combining the two identical labels. These are baseline observations, not a new speedup. M5CPU/Metal values are supplied historical references; no54→1 reference is supplied. RSS is the maximum whole latency process across both labels, including loading/setup/primes, not minimum RAM.

| Shape | Pooled baseline ms | Maximum RSS MiB | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|---:|
|1x2|8.605918|114.25|37 /24|4.30× /2.79×|
|2x2|11.653303|262.75|59 /30|5.06× /2.57×|
|4x2|16.104102|319.75|109 /48|6.77× /2.98×|
|8x2|24.331641|521.75|189 /74|7.77× /3.04×|
|12x2|34.489283|699.75|284 /101|8.23× /2.93×|
|16x2|40.745662|850.00|335 /124|8.22× /3.04×|
|24x2|67.995615|1294.50|528 /176|7.77× /2.59×|
|merge-24x1|67.203650|1195.62|507 /175|7.54× /2.60×|
|merge-54x1|176.181338|1982.41|not supplied|—|

## Verification, retained initial data and scope

Both complete series total **3888SDK calls,112distinct independently Gnark-verified proof exports and224proof/public members**. The corrected series alone is1944SDK/56exports. Fixture public hashes,164byte proof lengths, source/executable pins and distinct randomized identities pass. Prior5714→initial5770→corrected5826. No new field/core test-suite claim is made for the unchanged accepted binary.

The initial nonconforming series is retained under `artifacts/performance` and `artifacts/analysis.json`. Its apparent bound crossings:

- 8x2-w32/b2 b/a=0.9428577930563165
- 8x2-w32 paired median b/a=0.9604170219569903

Four profiles across both series contain13953samples, zero loss and oneSDKPID each. Corrected profiles contain6579samples; no profile timing is used as a latency comparison. Inlined symbol categories are not exclusive stage CPU or critical-path fractions. All170accepted source files and the SDK remain unchanged. No candidate is adopted, no cloud resources are created, and no new Aeglos, phone, TEE, memory-pressure, giant-circuit or concurrent-throughput measurement is added. The all-eight CPU10× goal remains unmet.

[Corrected analysis](artifacts/corrected-run/analysis.json), [corrected raw rows](artifacts/corrected-run/performance/rows.json), [frozen corrected config](artifacts/corrected-run/perf-plan.json), [initial analysis](artifacts/analysis.json), [combined proof inventory](artifacts/combined-proof-inventory.json), [closeout](artifacts/closeout.json) and [artifact index](artifacts/artifact-index.json) retain the evidence. Binaries, large keys and raw perf.data remain local with hashes.
