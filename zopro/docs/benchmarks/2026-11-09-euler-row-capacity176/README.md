# Euler solver row capacity

The private candidate reserves actual constraint rows for the canonical Euler path, retaining full-domain reservation for the coset path. It adds no table, pool or retained workspace. Production remains unchanged.

The 216 new native row cases pass with zeroed/uninitialized storage across 1, 16 and 32 workers, preserving slots, optional C, reloaded plans, invalid-witness rejection, incomplete-plan fallback and Euler products. All 26 SDK preflights pass independent Gnark verification. The complete matched campaign runs 273 fresh latency processes: 13 conditions × 3 variants × 7 rotated/reversed blocks, with 3 primes/process. Frozen accepted REF, fresh OFF and candidate ON share fixed worker placement. Four PMU panels and four cycle profiles follow all latency runs. Every exported proof is independently verified and unique against the prior 40,162-hash ledger: 307 new exports, canonical 40,469. Instrumented times are excluded from latency results.

|Shape|Workers|REF ms|OFF ms|ON ms|ON/OFF ratio|95% block interval|Winning blocks|
|---|---:|---:|---:|---:|---:|---|---:|
|12x2|32|33.930932|33.828696|33.998178|1.006002|[0.988166, 1.018522]|2/7|
|16x2|32|40.013619|39.904111|39.682335|0.991224|[0.981478, 1.000010]|5/7|
|1x2|1|94.716052|93.939520|93.541820|0.998491|[0.996281, 1.001331]|5/7|
|1x2|16|8.968458|8.932010|8.931298|1.012029|[0.998903, 1.032399]|3/7|
|1x2|32|7.699867|7.706800|7.696830|0.998830|[0.995379, 1.002477]|5/7|
|24x2|32|66.023007|65.969515|65.740113|0.996986|[0.989581, 1.003201]|3/7|
|2x2|32|11.038591|10.905069|10.896874|0.997148|[0.990257, 1.004186]|4/7|
|4x2|32|15.929492|15.711732|15.804507|1.000304|[0.990644, 1.009731]|2/7|
|8x2|32|24.033392|23.781146|23.818267|1.004034|[0.994957, 1.016585]|4/7|
|merge-24x1|32|65.216469|65.466804|65.227277|0.999444|[0.990550, 1.008906]|4/7|
|merge-54x1|1|1503.837662|1513.057438|1507.433015|0.998434|[0.994886, 1.001923]|4/7|
|merge-54x1|16|172.687431|173.343754|173.447330|1.000965|[0.993232, 1.007934]|3/7|
|merge-54x1|32|161.524872|161.621946|161.022702|0.994952|[0.979551, 1.010065]|3/7|

The four gain checks fail; all 13 calibration checks and all nonregression and RSS checks pass. No optimization is adopted. The protocol contains no minimum gain cutoff; gains must be distinguishable from measured block variation. All observations are retained without rerolls.

The paired primary ON/OFF estimate is 0.117% faster, with a 95% interval spanning 0.462% faster to 0.248% slower. Merge54 is 0.505% faster by paired geometric ratio, with an interval spanning 2.045% faster to 1.007% slower. Neither establishes a reliable gain. Primary actual geometry is 30,922 rows / 32,768 domain; merge54 is 1,178,199 / 2,097,152. The eliminated reservation is 59,072 bytes per row buffer for primary and 29,406,496 bytes for merge54; those are requested virtual capacities, not RSS savings.


|Shape|Workers|OFF peak RSS MiB|ON peak RSS MiB|OFF minor faults/proof|ON minor faults/proof|
|---|---:|---:|---:|---:|---:|
|12x2|32|684.750|651.750|3554.8|3315.6|
|16x2|32|760.250|757.500|3504.9|3702.2|
|1x2|1|34.250|34.250|0.0|0.0|
|1x2|16|80.750|81.750|131.1|129.4|
|1x2|32|117.500|116.500|299.8|293.0|
|24x2|32|1192.484|1178.449|8218.8|6953.9|
|2x2|32|235.750|237.000|1861.3|1793.6|
|4x2|32|278.500|270.000|1601.5|1489.8|
|8x2|32|463.500|457.000|2298.4|2459.4|
|merge-24x1|32|1103.887|1075.250|5138.6|4924.9|
|merge-54x1|1|1166.590|1166.586|34789.0|34789.0|
|merge-54x1|16|1683.215|1639.086|21321.7|21873.4|
|merge-54x1|32|1859.398|1788.137|23460.3|22541.8|

All four cycle profiles have zero lost records and all four PMU panels report 100% running. Instructions per proof are essentially unchanged (primary −0.016%, merge54 +0.026%). Minor faults vary with allocator/process state: the merge54 PMU panel increases from 668,130 to 951,033 faults over 31 proofs even though its uninstrumented per-process median is lower. No consistent fault reduction is established. The unchanged control primary profile places 20.35% of aggregate user cycles in G1 streaming addition, 15.68% in borrowed G2 addition and 7.34% in scalar Fq inversion. These are aggregate CPU shares, not wall-time critical-path fractions. [Demangled profile leaves](artifacts/profile-leaves.json).

The analysis helper initially failed to compile because a JSON summary was moved before later diagnostic branches; cloning that analysis value fixed it before the timing campaign started. The original helper source is retained in the archive. No prover build, preflight, timing process or recording was repeated.

Requested capacity is not physical memory. RSS is a whole-process high-water mark including key loading; it does not isolate scratch lifetimes or establish minimum RAM, pressure, mobile or huge-circuit behavior. Minor faults are process counters for the observed loop; profiles are single diagnostic panels and cannot establish causal time savings alone. No new AWS or Aeglos result is claimed.

[Analysis, resource counters and profile quality](artifacts/analysis.json), [declared protocol](artifacts/protocol.json), [source changes](artifacts/source-audit.json), [proof bindings](artifacts/proof-inventory.json).

Production is unchanged. The historical accepted primary result is 7.857170 ms; the fresh accepted REF measurement here is 7.699867 ms under this campaign’s seven-block protocol. The fresh REF comparisons against both supplied M5 references follow; no shape reaches the CPU 10× target:


|Shape|Fresh accepted REF ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.699867|37|24|4.805×|3.117×|3.7|
|2x2|11.038591|59|30|5.345×|2.718×|5.9|
|4x2|15.929492|109|48|6.843×|3.013×|10.9|
|8x2|24.033392|189|74|7.864×|3.079×|18.9|
|12x2|33.930932|284|101|8.370×|2.977×|28.4|
|16x2|40.013619|335|124|8.372×|3.099×|33.5|
|24x2|66.023007|528|176|7.997×|2.666×|52.8|
|merge-24x1|65.216469|507|175|7.774×|2.683×|50.7|

The separate review initially used exact equality for a recomputed floating-point fault rate and failed on a one-ULP JSON parse difference; a 10⁻¹² relative comparison tolerance resolves that representation check. Both helper originals are retained. The numerical selection gates are unchanged.

[Review](artifacts/review.json), [continuation](artifacts/next.md), [source and receipt archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Binary libraries, raw stack recordings, proof payloads and the full proof ledger remain private.
