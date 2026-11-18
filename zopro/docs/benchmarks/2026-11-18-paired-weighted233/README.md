# Paired independent weighted trees: full-proof screen

The exact candidate passes the declared short-screen rule. Primary latency
improves **0.94% at 16 workers and 1.15% at 32** versus accepted227, with
3/3 paired wins at each count and against both controls. This is a promising
small gain; adoption still requires all-shape/resource qualification and current
CPU diagnostics. Accepted production remains unchanged.

Two independent original 256-bucket weighted trees share each existing affine
batch inversion. Their joins, depth, correction nodes and output order remain
unchanged. A dense pair has 1,004 joins and eight batch levels, versus 1,004
joins and sixteen separate levels. Unlike the closed coarser 512-point tree,
this changes neither tree's arithmetic topology. No new field kernel is used.

The private snapshot has 188 files: 185 accepted files are byte-identical, one
parent has a reversible hook/module suffix, and two children contain the
implementation and tests. The native resource-schedule/IFMA guard selects
eligible lengths divisible by 512; other inputs use the original reducer.

Requested capacities for all paired jobs and partials must fit the existing
weighted-workspace forecast before allocation. Existing admission remains
conservative, without new OS queries or locks per weighted call. Two trees
retain 1,022 point slots; pair indices and record vectors have explicit checked
bounds. Native tests check actual capacities, both curves, inverse modes,
complete outputs and totals against generator-coefficient oracles, exceptional
points, immutable inputs, reuse, fallback/overflow, budget compatibility through
2^20, and actual 1/4/16/32-worker pools with 512 KiB stacks.

All 141 native core tests pass unfiltered. Generic noASM/noIFMA core compilation
passes. Field source is unchanged and inherits accepted227 qualification.
These checks do not establish physical pressure/swap, phone or throughput behavior.

## Matched SDK results

Local Ryzen 9 9950X, Rust 1.98.1, znver4, fat LTO, one codegen unit. REF is exact
accepted227; OFF is a fresh coherent link with the private cfg absent, ON enables
it. Three independently backed executable copies per arm participate, one per
matched block. Cases/arms rotate and reverse. Default stacks, fixed worker/main
affinities, allocator settings and the three accepted opt-ins match.

Each process makes three primes, then 251 primary, 51 2x2 or seven merge54 timed
SDK calls. The interval includes solving, randomized proving, internal
verification and serialization; loading/export checks are outside.

|Context|REF ms|OFF ms|ON ms|ON/REF paired median|Wins|ON/OFF paired median|Wins|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2-w16|8.611978|8.656883|8.530766|0.990569810|3/3|0.988946685|3/3|
|1x2-w32|7.448495|7.486430|7.440538|0.988529757|3/3|0.993267410|3/3|
|2x2-w32|10.435681|10.507233|10.427051|1.006093498|1/3|1.006157543|1/3|
|merge-54x1-w32|163.003353|162.269857|164.093070|1.011241792|1/3|1.011308001|1/3|

Times are medians of process means; paired medians are different estimators
from ratios of marginal medians. All blocks remain. The short screen shows
2x2 and merge54 paired penalties of 0.61% and 1.12% versus REF, respectively.
No shape is promoted or excluded to hide those observations. Three blocks
do not establish confidence intervals or a reliable general speedup.

|Context|ON/REF CPU-time ratio|Peak RSS MiB REF / OFF / ON|
|---|---:|---:|
|1x2-w16|0.975636567|82.000 / 82.750 / 83.250|
|1x2-w32|0.978224455|128.000 / 130.000 / 127.500|
|2x2-w32|0.992583143|286.500 / 286.750 / 291.250|
|merge-54x1-w32|1.007692520|1608.281 / 1632.391 / 1684.430|

All four OFF/REF median calibrations pass factor 1.03 and all RSS guards pass
ON <= each control * 1.10 + 8 MiB. RSS includes loading/primes, and is not
minimum RAM or a structural memory-saving claim. The candidate fits the
existing requested-capacity forecast; this does not predict allocator/stack RSS.

The primary screen requires median < 1 and at least 2/3 wins against both
controls at either worker count, with no minimum gain floor. Both counts pass.
All 39 exported proofs pass fresh independent Gnark verification: 5,160 SDK
calls, 5,043 timed records, 78 receipts and 156 stream hashes. Both readers
agree and both smoke readers pass before guarded main dispatch. Capture
precedes late verification/analysis, without owned CPU cojobs. No successful
capture, test configuration or verifier is replayed. All preparation/build/SDK
commands pass first attempt. The restored previous ON handle was unavailable;
its completed immutable workflow result is used without restarting it.

Linked code inspection confirms the paired implementation for both curves.
Entry-point byte/instruction counts exclude called helper bodies and are not
dynamic work, spills or speed predictions. No new CPU profile is claimed here.

```text
REF b70a9472fa64ac6b6b41b227ca64ddfcf61c6443879c9187331eae6d5e22c153
OFF ee7da7023e801505b7d34043f0293c35c316f8dc90b53a0d7158f9de3dffac9f
ON d523f49e8548620c4373a1ab4be2712d705709fdd7a590c5ed73e9aff97424e3
```

Canonical inventory advances 61,182 -> 61,221; SHA-256 `547616a37e6df105289816355ced4ce4915189dcd9c2ec15982a23161c43fafa`.

Preparation 530.88s; active SDK 65.18s; active Gnark 4.23s; protocol to final
review 613.62s. Both allowances pass. Filesystem/xtask durations are descriptive
and exclude earlier research/history.

Accepted all-eight CPU/Metal comparisons remain [228](../2026-11-17-scaled-prefix-all228/README.md).
The all-eight CPU 10x goal remains active and unmet, 0/8; these four contexts
do not qualify that goal. No new AWS measurement or production change occurs.
Current full revision of paper 2024/038 could not be retrieved (403); no unseen
paper algorithm or performance number is used.

[Assessment](artifacts/assessment.json), [protocol](artifacts/protocol.md),
[independent reader](artifacts/independent-main.json), [timing](artifacts/timing.json),
[sources/plans/build receipts](artifacts/source-and-plans.tar.gz), [checksums](SHA256SUMS).
Private keys, requests, witness/point payloads, proofs, full inventories, binaries
and raw disassembly are excluded. Local root:
`/home/ubuntu/src/zopro-perf-work/paired-weighted233`. Publication awaits approval.
