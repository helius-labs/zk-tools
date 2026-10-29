# Index only scalar grouping with grouped probing

**The component gate passes; production is unchanged.** All 18 real A/B input contexts improve with 31/31 paired wins. The equal-log constructor aggregate ratio is **0.440682 [0.439767, 0.441014]**, about 55.93% lower latency versus the source-extracted original in this compiled context. This selects a separate complete-proof trial; it is not a prover speedup or adoption.

## Implementation and control continuity

The dictionary uses pinned `hashbrown` 0.16.1 `HashTable<u32>` with full-value equality and rehash callbacks into the existing owned scalar values. It stores class indices and retains grouped control-byte probing. This replaces the failed scalar linear-probe algorithm from [index-groups175](../2026-10-29-index-groups175/README.md). The complete original/candidate builder hook file is byte-identical to that experiment; only the dictionary implementation changes.

The dependency is compiled offline with default features disabled and inline-more enabled. It is [MIT OR Apache-2.0](https://github.com/rust-lang/hashbrown/tree/v0.16.1); source files and the resolved crate checksum are pinned. It already appears in the repository lockfile, but is not currently a direct production core dependency. The standalone private package does not change production Cargo files. No new unsafe code is written; hashbrown retains its internal unsafe implementation.

This preserves zeros, stable first-occurrence class/value order, final cutoff, scalar arithmetic and all existing output metadata. No key-point table, copied execution plan, new pool or input-sized point copy is introduced. Existing frozen accepted source 175 libraries, native targeting, fat LTO and one codegen unit are reused. Sampling and resource admission precede both extracted constructors and are excluded from their timers.

The freshly compiled original differs in absolute timing from the preceding experiment despite identical builder source. Primary A original is 0.281796 ms here versus 0.166538 ms previously; B is 0.434695 versus 0.257059 ms. These are different compiled contexts and captures, not paired calibration. No exclusive host/compiler cause is established. Therefore the 55.93% constructor comparison cannot be transferred to accepted prover latency. A full-SDK accepted/fresh-OFF/ON comparison is required. [Continuity audit](artifacts/control-continuity.json).

## Real input constructor results

Ryzen 9950X, main thread CPU2. For all nine fixtures, frozen accepted raw-key and freshly serialized solver-image scalar solutions agree. A/B use canonical words for the first two shapes and Montgomery words for the remaining shapes, following the production entry geometry. All 18 samples admit construction; both primary populations subsequently decline at the unchanged full-cardinality cutoff. Private scalar arrays are never persisted.

Thirty-one fixed blocks rotate/reverse context order and alternate variants. Each cell averages eight separately timed constructors after three primes per context/variant. Matching block/call variants share fixed AHash seeds without seed selection. The constructor timer includes dictionary allocation, hashing, insertion, growth, map destruction and successful output construction. Returned metadata checks/destruction occur afterward. Every prime and timed output matches the independent ordered-map oracle in every field. These are single-thread constructor timings, not full query/proof costs or concurrency measurements.

|Input|Original ms|Candidate ms|Paired ratio|Bounds|Wins/31|
|---|---:|---:|---:|---|---:|
|1x2/A|0.281796|0.120778|0.428564|[0.418406, 0.431993]|31|
|1x2/B|0.434695|0.193477|0.443800|[0.439241, 0.450176]|31|
|2x2/A|0.535107|0.224028|0.419161|[0.415776, 0.421520]|31|
|2x2/B|0.752742|0.327922|0.434297|[0.432660, 0.436361]|31|
|4x2/A|0.920848|0.384979|0.418065|[0.416818, 0.419589]|31|
|4x2/B|1.372375|0.575215|0.420477|[0.417395, 0.423944]|31|
|8x2/A|1.733047|0.731784|0.423802|[0.422276, 0.424252]|31|
|8x2/B|2.606224|1.079224|0.414915|[0.413476, 0.416257]|31|
|12x2/A|2.680999|1.236814|0.460708|[0.458965, 0.464183]|31|
|12x2/B|3.985463|1.798113|0.452037|[0.449291, 0.453308]|31|
|16x2/A|3.504583|1.590720|0.454172|[0.450684, 0.455222]|31|
|16x2/B|5.161180|2.172498|0.420000|[0.417205, 0.422447]|31|
|24x2/A|5.252113|2.363501|0.451390|[0.446088, 0.455395]|31|
|24x2/B|8.195780|3.653980|0.444810|[0.444048, 0.446865]|31|
|merge-24x1/A|5.009984|2.238705|0.446317|[0.444613, 0.447759]|31|
|merge-24x1/B|7.750285|3.439946|0.444286|[0.442994, 0.445942]|31|
|merge-54x1/A|11.796956|5.560298|0.471653|[0.470377, 0.472207]|31|
|merge-54x1/B|18.814827|9.367645|0.497688|[0.496550, 0.500076]|31|

All declared checks pass: both primary upper ratios<1 with at least 21/31 wins, every upper ratio≤1.03, and the all 18 aggregate upper<1 with at least 21 wins. Bounds use ordered paired ranks 10–22 of 31, nominal 97.055% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusions, sample extensions, threshold changes, seed/layout selection or unchanged rerolls occur.

## Correctness, memory and CPU profiles

The three reused algorithm oracles execute against the new dictionary: deliberately identical hashes with wraparound/growth, full four-word equality/stable metadata against independent BTreeMap cases, and conservative dictionary-payload arithmetic. Two inherited FIFO tests also pass; five tests total, unfiltered. This is new implementation coverage, not newly invented test cases or a fresh full core/field/SDK qualification. Native test and benchmark links pass on their first execution. The initial dependency-workflow plan lacks the required source-pins field; the schema is corrected before any dependency build or test. Its initial plan is archived.

All 8,928 timed constructors and 108 primes finish before two separate FIFO-gated 199 Hz cycles:u primary-B profiles, 8192 calls each after checked primes. Each profile has one sampled PID and zero lost records. Individual profile outputs are dropped; primes and a post-profile call are checked. All captures precede owner-user decoding and loss checks.

|Profile|Samples|Lost|
|---|---:|---:|
|original|832|0|
|candidate|309|0|

Flat period weights and emitted common-build disassembly are retained. They do not isolate a cache/probe/compiler cause or represent removable proof wall time. No owned helper/build/hash/archive or second benchmark overlaps captures.

Each dictionary entry uses four class-index bytes plus control storage/group padding. Both growth buffers fit the existing conservative grouping-map forecast in tested integer calculations; output arrays remain unchanged. This is an allocation-payload bound, not measured per-variant RSS savings, allocator overhead or minimum RAM. Whole-process RSS includes all nine key loads/solves and cannot be assigned to one dictionary or proof.

## Accepted prover and supplied M5 references

No SDK proof or independent Gnark export is produced; the proof ledger remains 20,881. Production source 175 and SDK 20fde remain unchanged. Inherited complete-proof timings and supplied historical M5 references follow; these are not candidate full-proof measurements. Accepted full-performance-gate failure/physical 16 exception remains recorded; final-artifact gate remains true.

|Shape|Accepted ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|134.00|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|275.00|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|315.00|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|512.50|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|707.25|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|826.25|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|1328.75|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|1175.50|507|7.767×|175|2.681×|50.7|
|54→1|162.250464|1831.87|—|—|—|—|—|

Original verified all-eight CPU 10× goal remains active/unmet 0/8. Latest [AWS comparison](../2026-10-29-current-cpu-summary/README.md) remains 11.916 ms Zopro versus 7.959 ms Aeglos for 1×2 at each engine's fastest tested worker count. No new AWS, mobile/ARM/TEE, forester 250/4999, memory-pressure, swap, minimum-RAM or concurrent-throughput result is claimed. Next: qualify this exact mechanism in clean full SDK builds and compare accepted/fresh-OFF/ON before further adoption work.

[Protocol](artifacts/design.md), [all blocks and profiles](artifacts/analysis.json), [source and receipt archive](artifacts/source-and-receipts.json.gz), [closing pins](artifacts/pins.json), [dependency lockfile](artifacts/dependency-Cargo.lock).

## Full proof follow up

The [SDK pilot](../2026-10-29-swiss-groups-sdk175/README.md) measures 8.012 ms ON versus 8.031 ms accepted, with a 22.313 µs paired saving estimate and bounds spanning parity. Fresh OFF fails calibration. Observed ON peak RSS is 102.50 MiB versus 133.50 MiB accepted. The full-proof gate fails and the candidate is not adopted; the component result remains within its original scope.
