# Native joint-ring G2 query experiment

**Rejected.** The primary G2 query at 32 workers is **60.24% slower than fresh OFF** and **49.79% slower than optimized FROZEN**. Peak query-process RSS rises from **34.25 MiB to 145.75 MiB**. This candidate is closed without a full SDK trial or adoption. Production and accepted proof timings remain unchanged.

The [work screen](../2026-11-06-g2-joint-ring-screen176/README.md) and [queue/tree census](../2026-11-06-g2-joint-ring-census176/README.md) found fewer modeled additions. Native counters confirm fewer instructions, but cycles and cache misses increase. Profiles show substantial window construction and merging work. These observations do not isolate one cause or establish a RAM capacity limit.

## Complete-query measurements

Ryzen 9950X, 16 physical cores and 32 logical CPUs. B queries for 1×2 and 2×2 run at 1, 16 and 32 workers; primary A at 32 workers is an unchanged negative control. Solo pins CPU 2; wider pools pin CPUs 0 through workers−1, with 512 KiB worker stacks. Each process primes three times, times 17 queries, then checks once more. Times below are medians of 15 process medians. Ratios use matched blocks and need not equal ratios of the displayed times.

|Query / workers|FROZEN ms|OFF ms|ON ms|ON/OFF ratio [bounds]|Wins/15|OFF/ON maxRSS MiB|
|---|---:|---:|---:|---|---:|---:|
|1x2-A-w32|1.186676|1.182418|1.178861|0.991838 [0.981398, 1.012018]|9|24.00/22.25|
|1x2-B-w1|37.321429|36.855125|37.375031|1.014079 [1.013455, 1.016153]|0|15.50/20.00|
|1x2-B-w16|3.634236|3.425734|4.661321|1.357452 [1.348538, 1.377735]|0|28.50/105.00|
|1x2-B-w32|3.254992|3.022413|4.863001|1.602409 [1.577926, 1.641565]|0|34.25/145.75|
|2x2-B-w1|39.937509|39.116179|39.999295|1.022168 [1.021945, 1.024797]|0|27.25/31.50|
|2x2-B-w16|4.186928|3.904696|5.503978|1.400238 [1.373136, 1.418641]|0|43.50/119.25|
|2x2-B-w32|3.788007|3.476790|5.784437|1.680651 [1.641713, 1.700678]|0|53.50/153.50|

The aggregate for both shapes at 32 workers is ON/OFF 1.630628 [1.609504, 1.670860], with 0/15 wins; ON/FROZEN is 1.510786 [1.490109, 1.526090], also with 0/15 wins. All gain conditions fail. Nonregression also fails.

OFF/FROZEN calibration fails in all four B contexts at 16 or 32 workers: fresh OFF is 5.8–8.4% faster than the optimized frozen API control. That discrepancy remains unresolved; it establishes neither an accepted-build improvement nor its cause. The candidate is slower than both controls in these contexts, so the discrepancy does not change the rejection. All observations remain in the analysis.

The protocol declares 15 rotated/reversed blocks, all seven contexts and three variants. Median intervals use sorted ranks 4–12 of 15: nominal 96.484375% marginal coverage under independent, exchangeable blocks, without a familywise guarantee. All OFF/FROZEN intervals must fit [1/1.03, 1.03]; every ON/OFF and ON/FROZEN upper ratio must be ≤1.02. Primary B32 and the geometric aggregate of both B32 contexts each require an upper ratio below 1 and at least 10/15 wins against both controls. No gain floor, exclusions, extensions, changed margins, subset selection or parameter/layout/worker reroll. [Full protocol](artifacts/protocol.md).

Each query includes original grouping, candidate admission and digit preparation, fills, merges, coordinate trees, deallocation and output normalization. Scalar reset copies remain inside every B timer. Parsing and worker construction are outside. Every output matches the inherited complete Arkworks reference hash, and every point input remains unchanged. Actual 2×2 queries include admitted grouping; the earlier raw 2×2 model did not represent that runtime population. Peak RSS includes loading, primes and allocator retention. It is not minimum RAM, full-proof memory or pressure/concurrency qualification.

## Counters and automatic profiles

All 315 latency processes finish before six FIFO-gated, four-event PMU panels, followed by six separate profiles with a fixed 16-million `cycles:u` period. Each diagnostic measures 200 queries after three excluded primes. Ownership, event, loss and decoding checks run after all captures finish. No owned build, hash, archive, analyzer, verifier or second workload overlaps recording.

|Diagnostic|Instructions/query M|Cycles/query M|Generic cache misses/query M|Branch misses/query M|IPC|
|---|---:|---:|---:|---:|---:|
|stat-1x2-B-w16-OFF|549.556|221.173|0.739|0.267|2.485|
|stat-1x2-B-w16-ON|507.657|271.881|1.953|0.524|1.867|
|stat-1x2-B-w32-OFF|567.426|316.244|1.052|0.311|1.794|
|stat-1x2-B-w32-ON|533.231|525.234|3.530|0.585|1.015|
|stat-2x2-B-w32-OFF|596.798|345.645|1.254|0.320|1.727|
|stat-2x2-B-w32-ON|566.106|599.622|3.860|0.605|0.944|

All stat events run 100% of the measurement interval. Primary32 instructions decrease about 6%, while user cycles increase 66% and generic cache misses increase 3.36×. Median minor faults per query increase from 86.7 to 790.1; median system CPU time increases from 1.86 to 5.56 ms, and total CPU time from 69.88 to 113.76 ms. Counter panels are separate observations, not paired latency estimates. Generic cache events do not identify a cache level. Minor faults are not disk reads and do not establish swapping or insufficient installed RAM. User-cycle counts aggregate logical threads, including SMT.

|Profile|Samples|Unknown share|Qualified|
|---|---:|---:|---|
|cycles-1x2-B-w16-OFF|2760|0.725%|true|
|cycles-1x2-B-w16-ON|3331|1.531%|true|
|cycles-1x2-B-w32-OFF|3964|0.984%|true|
|cycles-1x2-B-w32-ON|6551|1.343%|true|
|cycles-2x2-B-w32-OFF|4283|1.191%|true|
|cycles-2x2-B-w32-ON|7486|1.509%|true|

The six profiles contain 28,375 samples, zero lost records and verified PID, worker TID/CPU pairs, period and executable MMAP bindings. Primary32 ON flat samples include 10.23% in `Window::new`, 11.57% in the bucket-merge iterator wrapper and 12.15% in the owned G2 prefix kernel. OFF has 41.07% in the borrowed G2 kernel. Shares include inlining and surrounding work. They are not exclusive critical-path time or removable latency; absolute event-period coverage is not validated. [Full symbols](artifacts/symbols.json).

## Implementation and memory

The recipe uses radix 128, 19 ring windows and two original point-range parts. Six scalar units map to 2,731 integer representatives in three signed planes, with 8,193 G2 bucket slots per part. A 64 KiB circuit-independent integer residue map and fixed signed-axis routing live in read-only executable data. No per-key point table, transformed proving-key copy or retained duplicate plan is built. Codes occupy 38 bytes per input scalar. The implementation merges parts, applies φ to completed unit planes, merges representatives, then reuses signed coordinate grouping and weighted trees padded to 128 entries. Field representation, scalar splitting, point arithmetic, borrowed streaming queues and complete exceptional paths remain unchanged.

Buckets need 1 MiB of coordinate payload per part, versus 256 KiB originally. Live planes, owned merge operands, initialization, axes and allocation add costs beyond generic-add counts. The profiles and PMU data are consistent with this larger working set and data movement, but do not isolate a memory-level bottleneck. No smaller-plane or different-width follow-up is selected by this experiment.

Before scratch allocation, a checked optional reservation covers all 19×2 suspended parent outputs, indexed-top inputs and results, codes, axis outputs, routing data and worst nonforking leaf scratch per worker, in addition to existing proof forecasts. Suspended parents are not bounded by worker count. Overflow, unsupported geometry/kernel or denied admission selects the original borrowed fallback before changing inputs. This conservative reservation is not actual allocated memory or a pressure guarantee. The inherited driver's `retained_payload_bytes` field records the reservation, not a measured retained allocation.

## Native qualification and preserved preflight failure

The initial private hook covered only the nongrouped return. It passed 125 OFF and 129 ON core tests, and all 21 preflight processes returned correct complete outputs. The activation checker then stopped before any latency block because grouped 2×2 bypassed the hook. No speed decision used those preflight clocks. The original source, libraries, executables, 21 caller receipts and failed checker remain preserved.

The successor moves the same optional hook into the shared `g2_phi` entry after the existing geometry guard, covering eligible raw and grouped inputs. Arithmetic, tables, width, part count, resource formulas and performance thresholds remain unchanged. A new generator test with 8,192 inputs and 1,536 classes requires grouped route 314 through the complete-query API at 1, 4, 16 and 32 workers, preserving original inputs. The changed integration passes 125 OFF and 130 ON unfiltered core tests on its first native workflow, then builds and freezes coherent libraries. Four earlier native cases are applied to the changed integration, alongside one new case: five new cases total, not nine independent tests. Unchanged field qualification is inherited.

Both snapshots contain 178 files. Of 176 originals, 171 are byte-exact; all originals recover by reversing five diagnostic, dispatch and visibility edits. Generated constants are identical across attempts and are not regenerated. Each new core links with its own coherent field and dependencies. All final query drivers have identical source and verified `-O`, native CPU targeting, fat LTO and one codegen unit. The frozen control is the existing optimized `query-release` build `0ba4…`; earlier unoptimized coalesce/late-M3 executables are excluded.

Seven unchanged frozen preflight receipts are reused with exact binary, input, command and stream bindings. New artifacts receive 14 first preflights; all required B ON routes activate 314. The 372-step shared workflow records 341 new callers plus seven reused preflights, producing 348 qualified rows. It checks 9,168 query invocations, including 35 reused calls; 9,133 are new shared calls. The initial attempt adds 105 calls, giving 9,238 calls across both attempts without counting reused data twice. These are component queries, not SDK proofs.

Initial helper failures are retained: a missing formatter-trait import in constant generation, a moved-path borrow in compiler pinning, and an unsupported per-link environment field rejected before any link. Narrow repairs precede each first successful execution; no successful native test/library/freeze, link or capture is repeated for bookkeeping. Initial schema/source/log copies and the compiler-environment revision remain in receipts. The closeout reviewer initially normalizes omitted link flags as empty instead of the tool's default optimized flags; correcting that bookkeeping check reuses every existing result.

Independent closeout checks 1,793 pin occurrences, 409 successful workflow commands and the preserved failing activation checker. Source recovery, native suites, coherent libraries, final link flags, query/oracle/activation/affinity receipts, raw PMU counts and exact profile worker CPU bindings pass. It independently reconstructs the 17-query process medians, paired ranks, all performance gates and RSS/CPU summaries. [Review](artifacts/review.json). No native result is adopted and no successful recording is rerun.

## Accepted proof comparison, inherited

No new full SDK latency, serialized proof or Gnark export is added. The canonical distinct-proof ledger remains 30,708. There is no new AWS/Aeglos, ARM/mobile/TEE, 250/4999, pressure/swap/minimum-RAM or concurrent-throughput result. The all-eight CPU 10× goal remains active and unmet: 0/8. Accepted local proof timings at 32 workers and supplied historical M5 CPU/Metal references follow. These are inherited measurements.

|Shape|Accepted ms|M5 CPU ms|M5 Metal ms|CPU/local|Metal/local|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37|24|4.667×|3.027×|3.7|
|2x2|11.151932|59|30|5.291×|2.690×|5.9|
|4x2|16.086386|109|48|6.776×|2.984×|10.9|
|8x2|24.179453|189|74|7.817×|3.060×|18.9|
|12x2|33.884455|284|101|8.381×|2.981×|28.4|
|16x2|40.077381|335|124|8.359×|3.094×|33.5|
|24x2|65.850777|528|176|8.018×|2.673×|52.8|
|merge-24x1|65.060937|507|175|7.793×|2.690×|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference. Accepted SDK SHA-256 stays `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`.

[Analysis](artifacts/analysis.json), [protocol](artifacts/protocol.md), [source snapshots](artifacts/source-snapshots.hex.json.gz), [exact receipts and helpers](artifacts/receipts.json.gz), [review source](artifacts/review.rs), [checksums](SHA256SUMS). Source archives preserve 356 members, including tracked binary fixtures, through byte-exact hex encoding. Receipt members round-trip exact UTF-8 bytes, lengths and hashes. Runtime point/scalar fixtures, private keys/images/witnesses, executables, libraries and raw `perf.data` stay local with hashes. Reproduction requires the pinned toolchain and dependencies, two private B fixtures and the pinned A negative control. Command arguments, environments, sources and inherited complete-MSM oracle identities are retained. Private roots: `/home/ubuntu/src/zopro-perf-work/g2-joint-ring-native176` and `/home/ubuntu/src/zopro-perf-work/g2-joint-ring-shared176`.

The [raw-profile inventory](artifacts/raw-profile-inventory.json) binds the six preserved local perf.data files to their lengths, SHA-256 hashes, exact executable hashes and actual PIDs. Inventory creation occurs after recording and adds no capture or decode.
