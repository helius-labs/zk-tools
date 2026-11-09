# Requested-live storage, allocator retention and proof RSS

The diagnostic finds substantial free allocator storage alongside high process RSS. In all six merge24 observations, accepted and combined builds have exactly the same requested-memory highwater above their loaded baseline: **186,762,535 bytes**. Merge54 likewise matches at **385,519,303 bytes** in all six observations. The earlier RSS-screen failure remains a failure; these instrumented observations do not waive it or establish its cause.

After 51 proofs, every one of the 30 processes has only **1,775 more logically outstanding requested bytes** than after priming. The driver retains the last SDK response; the diagnostic does not attribute that difference by callsite. End-of-run glibc free-arena storage reaches roughly **795–858 MiB on merge24** and **985–1,118 MiB on merge54**, while current Rust requested storage is about 128 and 267 MiB. This distinguishes retained allocator space from currently outstanding Rust allocations. It is not a minimum-RAM estimate or a universal no-leak claim.

The study completes **1,620 internally verified SDK requests**, **30 fresh unique Gnark-verified exports**, **1,710 phase snapshots** and **1,530 diagnostic request records**. Accepted and combined field/core/prover/SDK libraries are reused unchanged. No production optimization, allocator policy or pool is introduced. The parent [full performance comparison](../2026-11-09-prefix-short-sdk186/README.md) stays closed and not adopted with all nine failed checks. [Raw review](artifacts/review-result.json), [prospective review](artifacts/prospective-review.json).

## Instrumentation and scope

A Rust global allocator wrapper forwards pointers, layouts, allocation/zeroing, realloc and free operations to the original prover allocator. Fixed atomic counters record successful requested bytes, logical outstanding bytes and a cumulative highwater that is never reset. A failed allocation leaves ownership and live counts intact. Realloc counts its logical requested-size change; temporary physical overlap and allocator metadata are excluded. Atomic operation linearization is not a physical-allocation timeline, and a multi-counter snapshot is not atomic.

The driver captures glibc `mallinfo2`, Linux currentRSS/HWM/anonymous/file/VM/thread counts and rusage after pool setup, mapping, SDK load, each prime and each subsequent proof. It reserves **41,280 bytes** for typed observer records before loading. Output formatting occurs after collection. Glibc accounting includes metadata/tcaches and omits mapped keys, stacks and private backend mappings; it is not the same accounting domain as Rust requested bytes or RSS. `/proc` RSS and rusage highwater can disagree; both raw values are retained.

Three fixed process blocks cover accepted REF and combined BOTH on 1×2, 8×2, 16×2, merge24 and merge54, with 32 workers and three primes plus 51 requests each. The exact inherited driver placement uses one global Rayon pool/default stacks, worker i on CPU i and N+1 tasks checked before/after. Retained malloc settings match the parent; no `malloc_trim`, `mallopt`, global scheduler change or synthetic RAM cap is applied. The host `sched_schedstats` knob is 0, so a zero runqueue-wait counter would not prove absence of waiting; no such claim is made. All 30 recordings finish before the independent verifiers. [Design](artifacts/design.md), [protocol](artifacts/protocol.json), [driver](artifacts/driver.rs).

**All durations and CPU observations here are instrumented diagnostics, not calibrated latency results.** Shared atomics and inter-proof snapshots perturb scheduling and allocator behavior. There is no adoption/performance gate in this study, no new M5 speedup estimate and no unchanged performance reroll.

## Requested storage and retention

The table gives medians across three processes per cell. Requested highwater is cumulative from process start; live and free/RSS columns use phase-boundary snapshots. Units are MiB. These do not measure a minimum live working set.

|Cell|Loaded live|After primes live|End live|Requested highwater|Glibc free, primed→end|Current RSS, primed→end|Rusage HWM at end|
|---|---:|---:|---:|---:|---:|---:|---:|
|16x2/BOTH|74.231|74.231|74.232|173.249|259.571 → 561.028|502.793 → 811.094|804.500|
|16x2/REF|74.231|74.231|74.232|173.031|270.584 → 612.678|511.914 → 859.492|852.500|
|1x2/BOTH|6.037|6.037|6.039|21.909|34.696 → 98.367|63.508 → 124.902|119.000|
|1x2/REF|6.037|6.037|6.039|21.882|32.533 → 94.768|61.547 → 120.965|115.000|
|8x2/BOTH|47.833|47.833|47.835|110.700|166.646 → 364.253|302.859 → 506.945|499.750|
|8x2/REF|47.833|47.833|47.835|109.929|166.337 → 392.426|301.113 → 533.652|527.250|
|merge-24x1/BOTH|128.252|128.252|128.254|306.363|429.232 → 812.422|773.980 → 1177.898|1170.082|
|merge-24x1/REF|128.252|128.252|128.254|306.363|451.877 → 842.327|793.418 → 1199.445|1193.312|
|merge-54x1/BOTH|267.108|267.108|267.110|634.768|564.908 → 1027.524|1412.105 → 1872.074|1936.520|
|merge-54x1/REF|267.108|267.108|267.110|634.768|529.241 → 1098.984|1375.832 → 1947.918|2011.309|

The loaded-baseline subtraction removes the small absolute difference between arm processes; it is an accounting comparison, not a memory-screen adjustment. Requested highwater can vary on smaller shapes and its first observed final value can occur in a prime or a later proof. All phase frames remain available rather than treating every cumulative maximum as a new per-proof peak. [All cells and process observations](artifacts/cells.json).

Allocation traffic remains high. The 1×2 observer records about 5,636 successful allocations plus 5,638 reallocations per SDK request and roughly 91.96 MiB of cumulative requested size. Merge54 records roughly 23,950 allocations plus 18,860 reallocations and about 1,610 MiB/request. The figures count requested sizes, not copied bytes, peak RAM or memcpy bandwidth. Current live storage returns near loaded state even as free arena space grows. Removing allocation calls alone is not a speedup argument: the earlier inline-index experiment (`2026-10-22-universal-inline-pairs-sdk`) removed about half the small-proof calls without establishing a latency win, and the [packed G1 records](../2026-10-27-g1-raw-scratch/README.md) regressed. Those results remain closed.

## Saved slow-run evidence

A separate offline reader retains all 830 latency processes from the uninstrumented parent campaign. In its slow 8×2/BOTH/block0, the fixed first 25 samples average **23.779600 ms** and the remaining 26 average **32.722837 ms**; the control halves are **23.932607/23.774989 ms**. This is a sustained burst, not one isolated sample. The whole-process mean is 28.338897 versus 23.852253 ms, while CPU service is 668.074784 versus 670.884863 ms/request. Aggregate CPU service over wall falls from 28.123 to 23.572 cores, with zero major faults and more involuntary switches. This is consistent with reduced parallel utilization or waiting, but does not identify a scheduler, allocator or memory cause.

The earlier merge24 highest-RSS BOTH/block8 has an ordinary 65.024884 ms process mean. The RSS maximum and worst latency event therefore cannot simply be assumed to be the same event. All slow observations remain in the failed original gate; neither the half split nor this new accounting creates an exclusion or retry. [All 830 rows and 50 cells](artifacts/offline.json).

## Validation and continuation

A first allocator self-check failed because its maximum-size test did not form a valid aligned layout. Before any SDK invocation, the test was corrected to the maximum valid aligned size, also respecting the realloc input contract. The old source and failed receipts are retained. Both accepted and combined wrappers then pass alignment 64, zeroing, growth/shrink, crossing 128 KiB, failed-realloc ownership/payload preservation and final live-count restoration. The capture reports zero allocation failures in all 30 proof processes. The initial build plus corrected four-step build contribute five successful build/self-check commands and one failed self-check; all 60 capture/verifier commands succeed.

Before the first SDK request, reader volume accounting was corrected to sum inside-SDK request deltas separately from observer activity, and parent image/request SHA bindings were strengthened. Prior reader sources/binaries and pins are retained. The prospective review passes all 48 pins, all 10 cells and 30 caller/verifier bindings. The independent result reader checks 60 receipts, raw 1,710 frames/1,530 requests, all 30 unique proof/image/public/VK bindings, observer capacity, zero allocator failures and both exact merge highwater identities. No successful scientific capture is repeated. [Exact text evidence](artifacts/evidence.json.gz).

The next optimization still needs a distinct complete consumer/resource argument. This accounting suggests investigating where repeated short-lived growth and arena retention arise, while also reducing dominant ordinary G1/G2 work. It does not justify changing malloc settings, relaxing the old RSS rule, promoting only favorable shapes, or retrying rejected inline/packed layouts unchanged. No replacement candidate is selected. Full all-eight/merge54 performance, correctness, resource/admission, portable and concurrency qualification must precede adoption. [Continuation loop](artifacts/next.md).

## Prior CPU and Metal comparison, inherited

These are the earlier unadopted combined build's full SDK measurements, **not timings from this instrumented diagnostic**. M5 references use different hardware. The primary had a small gain, but its complete-scope gate failed. No new CPU/Metal performance conclusion is added here.

|Shape|Prior candidate ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.753327|37 / 4.772×|24 / 3.095×|3.7|
|2x2|10.841685|59 / 5.442×|30 / 2.767×|5.9|
|4x2|15.687119|109 / 6.948×|48 / 3.060×|10.9|
|8x2|23.928185|189 / 7.899×|74 / 3.093×|18.9|
|12x2|33.528082|284 / 8.471×|101 / 3.012×|28.4|
|16x2|39.407196|335 / 8.501×|124 / 3.147×|33.5|
|24x2|65.774403|528 / 8.027×|176 / 2.676×|52.8|
|merge-24x1|65.270789|507 / 7.768×|175 / 2.681×|50.7|

The original all-eight CPU 10× goal remains active and unmet, **0/8**. Production's 176 members/accepted SDK 6d83 are unchanged. Canonical independent exports advance 39,578→39,608. No new AWS/Aeglos, mobile, huge-circuit, pressure/swap/minimum-RAM or throughput result is claimed. Capture 85103 is terminal with exit code 0 and its handle is cleared; no recording or owned cloud resource is live.

Private root: `/home/ubuntu/src/zopro-perf-work/proof-lifetime-census186`. [Publication checks](artifacts/publication.json), [checksums](SHA256SUMS). The archive preserves UTF8 helpers, raw counters, verified receipts and pins. Executables/libraries, unchanged source snapshots, key/witness/proof payloads and the private full ledger stay local.
