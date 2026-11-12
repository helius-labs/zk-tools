# Adopt bounded G2 tile batching on native IFMA builds

**The exact G2 batching candidate passes all 33 declared local performance and
resource checks and is adopted.** Primary 16-worker paired latency improves
1.72%, with lower aggregate CPU cost. The faster 32-worker configuration passes
nonregression, but its speedup remains inconclusive. This is a small improvement;
the all-eight CPU 10× objective remains unmet, **0/8**.

|Context|Previous REF ms|Adopted ON ms|Paired ON/REF [marginal interval]|Peak RSS MiB, REF / ON|
|---|---:|---:|---:|---:|
|1×2, 16 workers|9.097619|8.949599|0.982791 [0.976462, 0.987638]|78.00 / 77.75|
|1×2, 32 workers|7.804093|7.788384|0.996080 [0.989068, 1.004460]|95.75 / 98.25|
|2×2, 32 workers|10.984918|10.931734|0.995175 [0.989359, 0.998909]|239.50 / 243.50|

Displayed times are medians of 30 process means, each with 17 fresh timed
requests after three primes. Paired ratios need not equal ratios of displayed
medians. The 16-worker medians differ by 0.148 ms; CPU medians decrease from
116.355 to 112.186 ms per request. At 32 workers, CPU decreases from 179.002 to
173.359 ms. These are aggregate CPU costs, not proof latency.

## Change and qualification

The [previous SDK screen and profiles](../2026-11-11-bundle-sdk178/README.md)
selected this exact candidate. Independent G2 bucket trees share batch inversions
within the original 1,024-point leaf bound. Point order and group results remain
unchanged. Checked admission covers extra job metadata and worker arrays; denial
falls back before input mutation. No key tables, expanded solver plans, extra
full point arrays or production pool are introduced.

Both consumers use their original three independently written, byte-identical
executable backings. Thirty pairs per context distribute ten pairs to each
backing. Context order rotates and reverses, and consumer order alternates. All
eight supplied shapes and merge 54×1 use 32 workers; primary also uses 16.
Accepted REF is the sole adoption comparator. Earlier runtime-OFF evidence
continues to support the mechanism without adding another adoption veto.

The prospective checks require primary16 latency upper <1, the other nine
latency uppers ≤1.02, all ten CPU uppers ≤1.02, and all ten peak-RSS values
≤1.05×REF+1 MiB. Each primary16 backing median must also improve. All **33/33**
pass. Sorted paired-ratio ranks 10 and 21 of 30 give nominal **95.7226% marginal**
median coverage under independent exchangeable blocks; no joint-confidence claim
is made. Primary32 backing spread is below the declared diagnostic flag.
No samples, contexts or copies were dropped, extended or rerun.

Twenty-three preflight and 600 main processes complete **12,092 SDK calls**,
including primes, and **10,223 timed samples**. All **623 fresh exports** pass
independent Gnark verification and image/public bindings. SDK verification runs
for every request. Primary and 2×2 execute four indexed reductions per request;
the larger shapes execute one. Three complete SDK cases with optional
preparation ceiling zero execute no new reductions and still verify correctly.
This checks configured denial, not physical memory exhaustion.

Source-bound native core, generator/edge and Arkworks query checks are inherited.
Fresh release checks add **104 generator-oracle comparisons**, with admitted and
denied preparation, 1/4/16/32-worker pools, nested joins and four OS callers
sharing four workers. All **97 generic x86-64 no-IFMA/no-ASM core tests** pass.
Ten selector contexts preserve existing generic, opt-out, ARM, wasm and GPU
selection; actual native Cargo selects both new cfgs without explicit flags.
These are functional concurrency and build checks, not throughput or physical
ARM measurements.

## All-eight CPU and Metal comparison

Fresh local Ryzen 9950X measurements below use 32 fixed workers, Rust 1.98.1,
znver4 and accepted allocator settings. Supplied M5 references are from different
hardware and were not remeasured here.

|Shape|Previous REF ms|Adopted CPU ms|Adopted peak RSS MiB|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1×2|7.804093|7.788384|98.25|37 / 4.751×|24 / 3.082×|3.7|
|2×2|10.984918|10.931734|243.50|59 / 5.397×|30 / 2.744×|5.9|
|4×2|16.001763|15.927431|284.00|109 / 6.844×|48 / 3.014×|10.9|
|8×2|24.365934|24.403586|482.25|189 / 7.745×|74 / 3.032×|18.9|
|12×2|33.912533|33.697671|657.50|284 / 8.428×|101 / 2.997×|28.4|
|16×2|40.095191|39.853842|787.00|335 / 8.406×|124 / 3.111×|33.5|
|24×2|66.172646|66.120558|1227.17|528 / 7.985×|176 / 2.662×|52.8|
|merge 24×1|65.602837|65.714498|1119.08|507 / 7.715×|175 / 2.663×|50.7|

Merge 54×1 is 162.874366 / 162.933446 ms, REF / ON, with paired ratio
1.000060 [0.996364, 1.005051]. Peaks are 1825.17 / 1857.30 MiB. It has no supplied
M5 reference. Passing nonregression does not establish improvement in every row.
Complete latency, CPU, memory and backing comparisons remain in the artifacts.

RSS includes loading, primes and allocator retention; it is not minimum RAM.
The maximum additional reservation per indexed call is 537,880 bytes here, not
measured allocation or a whole-proof bound. All candidate timed loops report
zero major faults; this does not establish absence of startup paging. No new
large-circuit, phone, swap or physical-pressure result is claimed.

## Adoption and reproducibility

Previous REF is `c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`;
adopted native SDK is
`b646c7b8c1d8a26a0db45fb3f67c453dd616bd9f02e25c36192721ae02855abe`.
All 177 production source hashes are checked: 173 original files unchanged,
three changed parents and one new child. Runtime sources are exact to the
measured SDK; the additional production change registers and selects the cfgs
on native IFMA builds. The measured atomic admission observer is retained.
`ZOPRO_CPU_INDEXED_TILE_BUNDLES=0` disables the reducer. Removing the observer is
a separate candidate, not a prerequisite imposed on this qualified runtime.

Cargo default activation is type-checked, not a newly timed release SDK. Native
runtime measurements apply to the retained binary above. There is **no new C7a
or Aeglos comparison**, and no inference that this resolves the C7a gap.

Main recording commands total 620.370 seconds; independent verifiers total
72.129 seconds. Preflight recording totals 8.370 seconds. Release checks link
the concurrent caller in 5.203 seconds, check native selection in 5.978 seconds
and build/run generic tests in 92.013 seconds. These exclude preparation and
analysis. No owned build/helper/analysis work overlaps SDK recording.

Canonical exports advance **54,304 → 54,327 → 54,927**, at
`/home/ubuntu/src/zopro-perf-work/bundle-all178/proof-sha256-after-main.json`,
SHA-256 `48e866327435bbc85f7c8d0433115337d7d903481594f307bab4373c39ec3119`.
The separate result reader reproduces all 33 decisions from raw observations;
it is a same-agent verification, not an external audit. The next research
question is the elapsed opportunity for moving ready query work into solver
execution, as recorded in the [playbook](../../cpu-performance-playbook.md).
No push is authorized.

[Protocol](artifacts/performance-protocol.md), [all checks](artifacts/assessment.json),
[independent reader result](artifacts/independent-main-review.json),
[release checks](artifacts/release-review.json), [adoption decision](artifacts/decision.json),
[production manifest](artifacts/production-source-manifest.json),
[post-adoption check](artifacts/post-adoption-review.json),
[sources and receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
Private inputs, proof payloads, binaries and full ledgers remain local.
