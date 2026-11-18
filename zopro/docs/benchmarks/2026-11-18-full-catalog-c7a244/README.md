# Full pinned Zopro catalog versus retained Aeglos on C7a

**Merge54 at 96 cores: Zopro 124.31 ms versus Aeglos
173.65 ms**, or 28.4% lower median latency.
Maximum lifetime RSS is **1.83 GiB versus
12.71 GiB**.

At equal 32-core counts, all 41 individual paired intervals favor current Zopro.
At 96 cores, 19 favor Zopro, 20 favor Aeglos and two are inconclusive. Zopro has
lower maximum lifetime RSS in all 82 equal-core comparisons. These are measured
results for the retained binaries and declared configurations; interval coverage
is marginal, without a simultaneous or familywise guarantee.

Primary 1×2 remains faster with Aeglos at its best observed count:
8.242 ms at 96 workers, versus
10.846 ms for Zopro at 32. At equal 96 workers,
Zopro is 12.691 ms. Taking each engine's lowest
median among 32/96 gives Zopro lower latency on 23/41 shapes. This descriptive
selection does not establish the optimal worker count.

This is a broader measurement of the existing adopted implementation, with no
new optimization or adoption decision. All 1,640 remote exports pass server
Gnark verification and host reverification. The separate offline reader checks
all 10,496 timed records, 4,920 execution receipts and 9,840 stream hashes.
The instance, tagged volumes, temporary bucket/objects and secret parameter
files are removed and checked.

## Catalog coverage

The previous nine cases were the historical performance subset. This run covers
**every entry in the pinned 41-key Zopro catalog: 38 confidential-transfer shapes
and three merges**, including output widths 4/8/16, larger inputs through 49×2,
and merge8/24/54. All use key release `proving-keys/3a1c88f90f609eb1`.

The broader mobile manifest has 130 keys across additional families. The current
server manifest has 124 keys and a different release prefix. Those additional
families/releases are outside this comparison. [Coverage inventory](artifacts/coverage.json)
records the exact selected keys, manifest hashes and unmeasured mobile entries.
This report does not label all 130 mobile keys as benchmarked.

All 41 shapes have matching Gnark keys, Zopro images and requests, with public
witness bindings. Nine existing pairs are reused; 32 additional pairs are restored
from pinned artifacts. Additional requests come from the matching release's
isolated fixture helpers. Every exported proof is verified against the
corresponding image's Gnark verification key and expected public witness.

## Reading the tables

Latency is the median of nine fresh-process means. Each main process runs three
primes and seven timed requests. CPU is reported process user+system CPU time per
timed request, summarized by the median across those processes. It includes
synchronization and runtime work; it is not useful-arithmetic utilization.

RSS is the maximum lifetime process peak across the preflight and nine main
processes, including key loading, preparation and primes. It is not minimum RAM.
Ratios below one favor Zopro. Paired ratios use matched block means, so they can
differ from ratios of displayed marginal medians.

Bounds use zero-based sorted ranks 1/7 from nine block values: nominal 96.09375%
marginal median coverage under independent continuous-block assumptions. There
is no multiplicity correction. All observations are retained; no extension,
exclusion, faster backing selection or worker-count rescue is applied.

## 32 physical cores

|Shape|Zopro ms|Aeglos ms|Paired Z/A [bounds]|CPU ms Z/A|Maximum RSS MiB Z/A|
|---|---:|---:|---:|---:|---:|
|1×2|10.846|12.073|0.8928 [0.8730, 0.9112]|228.6 / 327.9|64.9 / 412.0|
|1×4|13.241|15.419|0.8621 [0.8442, 0.8821]|281.3 / 420.4|150.9 / 518.6|
|1×8|14.733|16.886|0.8711 [0.8202, 0.8869]|332.3 / 468.7|205.0 / 621.0|
|1×16|18.022|19.795|0.9030 [0.8808, 0.9176]|406.3 / 569.0|230.5 / 768.7|
|2×2|14.665|18.891|0.7763 [0.6779, 0.7985]|321.4 / 535.9|173.5 / 716.7|
|2×4|15.422|20.180|0.7661 [0.6786, 0.8026]|335.1 / 554.7|180.2 / 742.0|
|2×8|17.704|24.960|0.7040 [0.6957, 0.7206]|426.1 / 738.4|224.5 / 940.1|
|2×16|21.854|28.272|0.7842 [0.7559, 0.7877]|534.0 / 824.5|265.1 / 1131.8|
|3×2|18.160|26.468|0.6827 [0.6720, 0.6921]|444.9 / 789.9|195.4 / 1075.2|
|3×4|18.796|27.500|0.6766 [0.6450, 0.6970]|460.1 / 808.0|212.0 / 1104.8|
|3×8|20.175|29.039|0.6913 [0.6719, 0.7149]|503.5 / 857.1|247.8 / 1190.4|
|4×2|19.658|30.864|0.6347 [0.6257, 0.6484]|496.5 / 912.9|227.2 / 1256.7|
|4×4|20.633|31.520|0.6558 [0.6484, 0.6608]|519.5 / 924.8|250.3 / 1254.3|
|4×8|21.920|32.847|0.6699 [0.6636, 0.6860]|560.3 / 994.4|291.0 / 1333.2|
|4×16|24.758|36.277|0.6845 [0.6741, 0.6989]|636.1 / 1083.3|336.0 / 1445.2|
|5×2|21.657|34.949|0.6162 [0.5995, 0.6332]|562.5 / 1049.1|267.1 / 1432.6|
|5×4|22.099|35.398|0.6229 [0.6175, 0.6326]|575.1 / 1073.8|280.6 / 1430.3|
|5×8|25.128|45.286|0.5513 [0.5431, 0.5649]|666.2 / 1388.2|340.8 / 1759.8|
|5×16|27.987|48.446|0.5828 [0.5764, 0.5917]|740.7 / 1465.7|390.0 / 1911.7|
|6×2|24.635|47.152|0.5228 [0.5070, 0.5294]|659.1 / 1437.5|346.2 / 1871.0|
|6×4|25.267|47.194|0.5341 [0.5215, 0.5402]|674.3 / 1448.2|342.2 / 1880.7|
|6×8|26.677|49.348|0.5469 [0.5319, 0.5633]|712.2 / 1492.8|387.6 / 2048.4|
|8×2|26.891|54.724|0.4916 [0.4729, 0.5103]|739.3 / 1664.7|409.6 / 2271.1|
|8×4|28.020|55.160|0.5051 [0.4956, 0.5170]|759.9 / 1687.5|415.9 / 2334.0|
|8×8|29.668|56.778|0.5246 [0.5118, 0.5315]|800.7 / 1732.2|436.3 / 2404.0|
|8×16|32.161|60.104|0.5364 [0.5300, 0.5447]|873.1 / 1828.8|477.4 / 2527.5|
|12×2|36.019|86.747|0.4161 [0.4097, 0.4233]|990.3 / 2666.8|580.0 / 3364.5|
|12×4|36.092|87.338|0.4127 [0.4066, 0.4213]|996.2 / 2675.7|561.2 / 3390.0|
|12×8|37.869|88.971|0.4272 [0.4201, 0.4309]|1046.3 / 2725.0|594.3 / 3524.1|
|16×2|41.253|102.715|0.4044 [0.3905, 0.4116]|1159.8 / 3166.8|671.6 / 4297.9|
|16×4|42.574|104.681|0.4025 [0.4001, 0.4173]|1186.6 / 3204.7|736.2 / 4313.1|
|16×8|44.408|106.430|0.4169 [0.4011, 0.4279]|1239.6 / 3248.1|736.7 / 4406.6|
|24×2|60.617|172.286|0.3504 [0.3425, 0.3704]|1728.0 / 5207.6|1051.8 / 6384.7|
|24×4|61.136|171.073|0.3604 [0.3553, 0.3656]|1751.4 / 5196.5|1118.6 / 6355.4|
|32×2|74.195|202.193|0.3701 [0.3612, 0.3842]|2123.2 / 6142.0|1348.2 / 7969.2|
|40×2|84.861|234.445|0.3623 [0.3544, 0.3702]|2412.9 / 7133.0|1524.6 / 9322.2|
|48×2|133.607|331.241|0.4052 [0.3875, 0.4138]|3722.4 / 10176.0|1707.5 / 11859.3|
|49×2|136.375|340.267|0.3995 [0.3839, 0.4187]|3767.2 / 10336.3|1754.0 / 12130.6|
|Merge 8×1|27.772|52.206|0.5318 [0.5169, 0.5401]|737.9 / 1601.6|404.3 / 2210.4|
|Merge 24×1|62.367|166.388|0.3747 [0.3704, 0.3875]|1680.6 / 5039.1|1000.8 / 6173.9|
|Merge 54×1|141.866|347.579|0.4088 [0.3941, 0.4201]|3602.8 / 10671.7|1836.9 / 12543.8|

## 96 physical cores

|Shape|Zopro ms|Aeglos ms|Paired Z/A [bounds]|CPU ms Z/A|Maximum RSS MiB Z/A|
|---|---:|---:|---:|---:|---:|
|1×2|12.691|8.242|1.5129 [1.4655, 1.5811]|694.1 / 523.8|60.3 / 458.1|
|1×4|16.145|10.009|1.6063 [1.5593, 1.7017]|923.4 / 665.2|139.0 / 583.0|
|1×8|17.626|10.822|1.6221 [1.5976, 1.6803]|1053.4 / 736.4|193.8 / 684.1|
|1×16|21.017|12.424|1.7108 [1.6577, 1.7205]|1237.2 / 884.7|237.1 / 815.1|
|2×2|17.170|12.008|1.4432 [1.3830, 1.4894]|1000.2 / 825.6|164.5 / 805.3|
|2×4|17.674|12.446|1.3999 [1.3863, 1.4652]|1025.6 / 871.1|165.3 / 814.6|
|2×8|20.312|15.285|1.3190 [1.3011, 1.3858]|1261.7 / 1147.7|213.3 / 1017.0|
|2×16|24.332|17.126|1.4231 [1.3823, 1.4448]|1592.7 / 1265.1|277.8 / 1217.4|
|3×2|19.827|16.149|1.2281 [1.1781, 1.2570]|1332.0 / 1204.6|191.5 / 1156.8|
|3×4|20.882|16.554|1.2671 [1.2392, 1.2777]|1370.4 / 1255.0|206.3 / 1198.2|
|3×8|21.846|17.683|1.2410 [1.2302, 1.2577]|1481.9 / 1322.7|238.0 / 1265.3|
|4×2|21.641|18.404|1.1606 [1.1426, 1.1978]|1487.6 / 1408.6|232.0 / 1343.3|
|4×4|22.915|19.016|1.1873 [1.1687, 1.2154]|1526.4 / 1401.1|244.8 / 1363.0|
|4×8|24.071|19.686|1.2265 [1.1941, 1.2357]|1650.5 / 1499.3|269.5 / 1454.2|
|4×16|26.440|21.661|1.2143 [1.1892, 1.2788]|1797.0 / 1608.4|316.7 / 1577.7|
|5×2|23.888|21.135|1.1197 [1.1101, 1.1787]|1695.7 / 1549.0|266.7 / 1533.2|
|5×4|24.684|21.381|1.1444 [1.1221, 1.1879]|1733.3 / 1590.3|275.6 / 1559.8|
|5×8|27.686|26.555|1.0508 [1.0119, 1.0554]|1982.0 / 2129.7|323.8 / 1925.3|
|5×16|29.796|27.907|1.0796 [1.0599, 1.0932]|2100.4 / 2306.9|387.5 / 2060.4|
|6×2|26.922|27.515|0.9712 [0.9624, 0.9747]|1993.2 / 2205.2|319.2 / 1993.2|
|6×4|27.577|27.462|1.0018 [0.9640, 1.0268]|2045.7 / 2257.5|334.1 / 1998.8|
|6×8|29.193|28.429|1.0200 [0.9892, 1.0608]|2104.3 / 2329.9|347.8 / 2160.1|
|8×2|28.993|31.265|0.9316 [0.9052, 0.9391]|2160.4 / 2593.8|387.1 / 2446.0|
|8×4|29.803|31.595|0.9517 [0.9224, 0.9677]|2221.9 / 2596.5|400.6 / 2478.5|
|8×8|31.192|32.488|0.9586 [0.9362, 0.9771]|2290.7 / 2675.2|417.4 / 2593.5|
|8×16|33.694|34.491|0.9644 [0.9367, 0.9913]|2442.4 / 2803.6|477.7 / 2755.1|
|12×2|36.451|48.906|0.7421 [0.7129, 0.7540]|2591.2 / 4143.4|553.4 / 3677.6|
|12×4|36.302|48.746|0.7484 [0.7320, 0.7544]|2609.5 / 4120.4|544.9 / 3664.0|
|12×8|37.406|49.518|0.7509 [0.7380, 0.7689]|2682.4 / 4242.1|583.5 / 3729.8|
|16×2|41.881|56.862|0.7350 [0.7233, 0.7404]|2878.6 / 4862.6|694.0 / 4596.9|
|16×4|41.576|57.504|0.7191 [0.7065, 0.7343]|2913.5 / 4899.4|683.0 / 4610.3|
|16×8|43.148|58.396|0.7374 [0.7251, 0.7448]|2956.9 / 4979.8|722.0 / 4676.9|
|24×2|57.474|87.786|0.6561 [0.6440, 0.6612]|3823.6 / 7458.8|1104.0 / 6746.2|
|24×4|58.192|88.126|0.6629 [0.6553, 0.6744]|3854.5 / 7452.8|1129.1 / 6716.8|
|32×2|70.443|103.069|0.6804 [0.6708, 0.7094]|4301.4 / 8696.2|1365.5 / 8364.3|
|40×2|80.084|118.791|0.6785 [0.6694, 0.6983]|4741.8 / 10048.4|1609.6 / 9718.6|
|48×2|106.554|168.105|0.6387 [0.6228, 0.6579]|6078.5 / 14286.4|1832.1 / 12394.3|
|49×2|108.890|168.865|0.6474 [0.6330, 0.6680]|6150.4 / 14444.9|1802.9 / 12546.5|
|Merge 8×1|30.914|30.251|1.0346 [1.0160, 1.0404]|2299.4 / 2502.6|371.2 / 2348.4|
|Merge 24×1|62.182|84.655|0.7345 [0.7201, 0.7482]|3924.4 / 7249.1|1017.8 / 6589.4|
|Merge 54×1|124.312|173.648|0.7217 [0.7120, 0.7321]|6475.2 / 14961.3|1873.3 / 13011.8|

## Core scaling

|Engine|96 cores lower latency|96 cores higher latency|Inconclusive|
|---|---:|---:|---:|
|Zopro|8|27|6|
|Aeglos|41|0|0|

These classifications use each shape's paired 96/32 interval. Aeglos benefits
from 96 workers across this catalog. Current Zopro's scaling depends on the
circuit; small cases generally benefit from fewer workers, while several larger
cases improve at 96. The next diagnostic target remains the small-circuit C7a
critical path: useful arithmetic, synchronization and available parallel work.
Aggregate CPU time alone identifies no particular scheduler, cache or allocation
cause. No new PMU profile was captured.

## Supplied M5 references

Apple timings are supplied historical references; no Apple machine was remeasured.
Speedup pairs are for Zopro at 32/96 AWS workers. All 16 context checks fail the
declared CPU 10× upper-bound criterion; the original objective remains **0/8**.
The other 33 shapes have no supplied reference.

|Shape|M5 CPU/Metal ms|CPU speedup 32/96|Metal speedup 32/96|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1×2|37 / 24|3.41 / 2.92|2.21 / 1.89|3.7|
|2×2|59 / 30|4.02 / 3.44|2.05 / 1.75|5.9|
|4×2|109 / 48|5.54 / 5.04|2.44 / 2.22|10.9|
|8×2|189 / 74|7.03 / 6.52|2.75 / 2.55|18.9|
|12×2|284 / 101|7.88 / 7.79|2.80 / 2.77|28.4|
|16×2|335 / 124|8.12 / 8.00|3.01 / 2.96|33.5|
|24×2|528 / 176|8.71 / 9.19|2.90 / 3.06|52.8|
|Merge 24×1|507 / 175|8.13 / 8.15|2.81 / 2.81|50.7|

## Binary and execution provenance

Current SDK:
`d523f49e8548620c4373a1ab4be2712d705709fdd7a590c5ed73e9aff97424e3`.
Its 188-file implementation was adopted at `b164b9776b`; this is the exact
experiment-233 forced-cfg SDK qualified in 234/235. The normal native selector
integration was separately validated, but its normally selected SDK was not
newly linked or timed here. Production source hashes remain unchanged.

Retained Aeglos Sorted:
`eab9fbfd4b94aeef9ced3ce3e3ede21a37ab70ad38b217720f1afd06fceba99b`,
source `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`. Later upstream versions are
outside this measurement. This comparison does not include previous Zopro build
194; the [earlier matched comparison](../2026-11-18-adopted-c7a239/README.md) reports
that separate result.

One native eu-north-1 c7a.24xlarge ran AMD EPYC 9R14, with 96 physical cores,
one thread per core, one socket/NUMA node and Linux 7.0.0-1014-aws. Fixed process
masks, individual Zopro worker placement, default SDK stacks and the previous
allocator policy are retained. All three independently written byte-identical
backings per engine participate via block modulo three.

The matrix has 164 engine/shape/count contexts. All 164 preflight captures precede
their 164 Gnark verifications and both reader gates. Nine rotated/reversed matched
blocks then produce 1,476 main captures, all before their 1,476 late verifications.
Only the confirmed AWS command was polled during capture, with passive clock
observations and no owned CPU cojobs. No capture, cryptographic verifier or
scientific result reader was replayed.

Zopro timing covers solving, quotient work, fresh blinding, proof construction,
serialization and SDK verification. Aeglos retains its `verified_ms` boundary.
Loading, key preparation, pool creation, primes and exported-file writes are
outside proof latency and inside lifetime RSS. Cold first-proof latency,
concurrent-server throughput, ARM/phone behavior, huge forester circuits and
physical memory pressure are not measured.

## Run duration and preparation

The remote command ran November 18, 2026, 12:51:47–13:49:13 UTC:
**57 min 26.385 s**; its systemd unit reports 57 min 26.347 s.
The 820 Zopro capture commands sum to 525.799 s,
and the 820 Aeglos commands to 2571.109 s.
Their timed-request samples sum to 200.989 s and
344.543 s, respectively.

Those command durations include fresh-process setup, three primes, timed calls,
export and harness work. They are not isolated setup measurements or exclusive
CPU/critical-path fractions. [Inclusive command times](artifacts/inclusive-command-times.json)
separates that broader boundary from proof latency. This comprehensive sweep
should not replace focused candidate screens for every small kernel experiment.

Local qualification first produced 82 verified exports from 328 SDK calls at
four workers, outside the AWS performance dataset. The remote dataset adds
1,640 exports and 15,416 calls. Total new exports: 1,722; total SDK calls: 15,744.
Host reverification adds no distinct proofs.

Before any SDK call, the nine-case draft was expanded to all 41 keys and the
main-process count was prospectively set to seven timed requests, retaining nine
blocks. A fixture-helper build needed one missing type import; one initial asset
plan rejected dotted step names, and a generator invocation hit the existing
workflow lock before execution. Successful corrected plans are retained.
The 7,872,015,075-byte package uses encrypted multipart-capable transfer, with
an isolated lifecycle amendment after local qualification; no SDK, input,
sampling or timing boundary changed. None of these preparation fixes replayed
successful scientific work.

## Memory and observation limits

The native unit's 30 GiB/no-swap limit is infrastructure headroom. Its recorded
peak is 13,946,118,144 bytes. Unit and system.slice memory high/max/oom counters
remain zero; root memory-event fields are unavailable. All 820 Zopro processes
report zero measured-loop major faults. All 820 Aeglos processes lack that field;
absence is preserved as null.

Unit/parent `cpu.max` and `cpuset.cpus.effective` reads are unavailable.
Root cpuset reports 0–95; root `cpu.max` is also unavailable. Read errors and
ancestor observations are retained, without inferring unlimited CPU or the
absence of throttling. The terminal systemd summary's 512 KiB peak contradicts
the retained unit peak and SDK RSS. Tables use the SDK per-process observations.

No minimum-RAM, swap-fallback, universal memory bound or concurrent throughput
claim follows from this sweep. Additional memory/RSS and command-time observations
are in the artifacts.

## Saved results and cleanup

One instance was launched and its termination verified. Tagged EBS volumes are
absent. Temporary objects, incomplete multipart uploads, their private bucket
and secret transport parameter files are removed. Cleanup completed before
this report was written. No ECR image was published.

Canonical inventory: **64,133**, at
`/home/ubuntu/src/zopro-perf-work/full-c7a244/host-remote/proof-sha256.json`,
SHA256 `4ebd9384d39f83322d132500dc2abc2440992baff22d308509d1ccc326c8d240`.
Complete private bench root: `/home/ubuntu/src/zopro-perf-work/full-c7a244`.
The separate offline reconstruction is a same-agent cross-check, not an external
audit; it runs no prover or cryptographic verifier again.

[All results and paired bounds](artifacts/results.json),
[sanitized per-process samples](artifacts/processes.json.gz),
[independent review](artifacts/independent-review.json),
[host verification](artifacts/host-review.json),
[execution receipts](artifacts/receipts.json.gz),
[cgroup observations](artifacts/cgroup-resources.json),
[coverage](artifacts/coverage.json), and [sources/plans](artifacts/source-and-plans.tar.gz).
Key/request files, proof/public bytes, full proof inventories, executables
and raw PID/TID/address streams are excluded.
