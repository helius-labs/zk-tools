# Paired independent weighted trees: all-shape qualification

The exact233 SDK passes the declared qualification; **70/70** performance
and resource checks pass. Both raw readers agree. Accepted227 remains unchanged.

Current CPU diagnostics and final exact source/resource review remain required
before adoption. Small gains have no minimum-size cutoff.

This retains the [233 candidate](../2026-11-18-paired-weighted233/README.md),
its exact SDK, nine independently backed REF/OFF/ON executable copies, driver
and options. No relink or runtime source change occurs. REF is accepted227b70;
OFF is the fresh233 link with the candidate cfg absent. Each backing participates
in ten blocks. Independent original256 weighted trees share batch inversions;
requested capacities fit existing weighted-workspace admission. No lookup table,
key copy, new pool or memory-policy cap is introduced.

## Complete-proof time and memory

Local Ryzen9 9950X, Rust1.98.1, znver4, fatLTO, one codegenunit. Thirty matched
blocks per context, three primes,151 timed primary or17 other requests per
process. Values are medians of process means in ms; RSS is the maximum
process-lifetime peak in MiB, including loading and priming. The SDK interval
includes solving, randomized proving, internal verification and serialization.
Loading/export checks are outside. Default stacks and fixed worker/main
affinities, allocator settings and accepted opt-ins match233.

|Context|REF ms|OFF ms|ON ms|Peak RSS MiB REF / OFF / ON|
|---|---:|---:|---:|---:|
|1x2-w16|8.717740|8.772815|8.640740|85.00 / 83.25 / 81.75|
|1x2-w32|7.500146|7.521683|7.460937|128.25 / 129.75 / 128.75|
|2x2-w32|10.552026|10.641541|10.585262|240.00 / 237.25 / 237.25|
|4x2-w32|15.681038|15.775320|15.728386|280.00 / 290.25 / 287.00|
|8x2-w32|23.811288|24.076027|23.830724|466.75 / 472.50 / 474.75|
|12x2-w32|33.605192|34.118947|33.533216|654.50 / 665.25 / 662.25|
|16x2-w32|39.672879|39.682264|39.522878|773.00 / 776.25 / 773.50|
|24x2-w32|65.979043|66.346191|65.939965|1216.50 / 1227.50 / 1253.50|
|merge-24x1-w32|65.193085|65.382529|65.349162|1096.02 / 1124.91 / 1095.58|
|merge-54x1-w32|162.530596|163.374746|163.135118|1864.73 / 1848.43 / 1945.46|

## Paired primary results

|Context / control|Latency median [rank interval]|CPU-time median [rank interval]|
|---|---:|---:|
|1x2-w16 / REF|0.991083278 [0.988653631, 0.992568685]|0.977449291 [0.976745466, 0.978515736]|
|1x2-w16 / OFF|0.986156055 [0.976647186, 0.991571648]|0.976264164 [0.972114553, 0.978680400]|
|1x2-w32 / REF|0.995564248 [0.992863251, 0.998344444]|0.979895495 [0.978093568, 0.981478065]|
|1x2-w32 / OFF|0.991808971 [0.989141320, 0.994724935]|0.979138371 [0.972769943, 0.981974558]|

Primary gain including all three backing medians against both controls:

- 1x2-w16: true.
- 1x2-w32: true.

All raw paired ratios remain in the assessment. Marginal times and paired-ratio
medians are different estimators. Sorted30 indices9/20 define rank intervals,
with95.7226% marginal coverage under exchangeable independent blocks; this is
not simultaneous confidence. Every context requires latency and CPU upper
ON/eachcontrol <=1.02 and OFF/REF median calibration within factor1.03. At least
one primary also needs latency upper<1 against both controls and median<1
within each backing. There is no minimum gain floor.

RSS guards require ON <= eachcontrol *1.10 +8MiB in every context, retaining all
increases. This is neither equal memory use nor minimum RAM/allocator/physical
pressure qualification. Existing checked requested-capacity admission remains
unchanged; new paired leaf geometry was bounded and tested in233.

All70 performance/resource checks pass.

## Correctness and provenance

Native141 core and native field qualification are inherited by exact source/
configuration identity from233/accepted227. Fresh native prover2+cubic6 and
generic noASM/noIFMA core102/prover2/cubic6 suites pass unfiltered. Unchanged
generic field12 qualification is inherited. New test targets preserve the SDK
and all original link inputs. No ARM/mobile or concurrent-throughput claim.

All930 fresh exports pass independent Gnark verification, with42240SDK calls
and39450timed records. Both smoke readers pass before guarded main launch.
Capture precedes late verification and analysis; no owned CPU cojob overlaps
any SDK capture. Neither successful capture nor test configuration is replayed.
The separate same-agent reader reconstructs930exports,1860receipts,3720stream
hashes, paired comparisons and the full ledger union. It is not an external audit.

Canonical61221 ->62151; SHA-256 `a757f8aba5e1a4192b5e486a1b18cb3eab0fc10b991a05e35a266c410e336a63`.

Preparation 267.39s; SDK active 1152.46s; Gnark active 111.29s; protocol-to-review
1570.15s. Descriptive filesystem/xtask timings and allowances are retained in
the timing artifact.

## Supplied M5 references

These are supplied different-hardware references, not new matched-machine
measurements. Merge54 has no supplied M5 reference.

|Shape|ON CPU ms|M5 CPU speedup|M5 Metal speedup|CPU10x target ms|Target passes|
|---|---:|---:|---:|---:|---|
|1x2|7.460937|4.959x|3.217x|3.7|false|
|2x2|10.585262|5.574x|2.834x|5.9|false|
|4x2|15.728386|6.930x|3.052x|10.9|false|
|8x2|23.830724|7.931x|3.105x|18.9|false|
|12x2|33.533216|8.469x|3.012x|28.4|false|
|16x2|39.522878|8.476x|3.137x|33.5|false|
|24x2|65.939965|8.007x|2.669x|52.8|false|
|merge-24x1|65.349162|7.758x|2.678x|50.7|false|

The full all-eight goal remains active and unmet, 0/8. No new AWS
measurement or production change occurs.

[Protocol](artifacts/protocol.md), [assessment](artifacts/assessment.json),
[raw review](artifacts/independent-main-review.json),
[qualification](artifacts/qualification-review.json), [timing](artifacts/timing.json),
[sources/plans/receipts](artifacts/source-and-plans.tar.gz), [checksums](SHA256SUMS).
Private inputs, key/point payloads, proofs, full inventories, binaries and raw
thread/address streams are excluded. Exact candidate source is archived in233.
Evidence is committed locally; publication awaits approval.
