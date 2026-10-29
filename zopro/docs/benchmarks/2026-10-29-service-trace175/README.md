# Sparse service trace on accepted175

Low-overhead 32-worker qualification: **true** (calibration true, overhead true; ON/OFF paired median 0.994664524957014). This diagnostic adopts no optimization. All nine saved main graphs and one preflight graph are complete with zero drops.

## Observed requests

| Mode | Median ms | Whole-process maximum RSS MiB |
|---|---:|---:|
| accepted | 7.946014 | 108.25 |
| off | 8.174155 | 123.00 |
| on | 8.130542 | 123.00 |

Last query counts under instrumentation: `{"g2":4,"kh":5}`. This names the latest query boundary; it is not a removable critical-path duration or a causal estimate of uninstrumented latency.

| Query | Median query elapsed ms | Covered serial CPU ms | Pre-input elapsed ms | Median/max available-to-window start ms |
|---|---:|---:|---:|---:|
| a | 5.302849 | 20.560135 | 0.736909 | 0.978440/3.131606 |
| g2 | 5.336163 | 57.825522 | 2.051028 | 0.104366/1.612491 |
| kh | 5.251282 | 50.066703 | 0.000762 | 0.002414/0.004779 |

OFF/accepted block ratios: `[1.028711376546782,1.0039638833458011,1.0368751615099858]`; ON/OFF: `[0.994664524957014,1.0406489436802666,0.989425568287241]`. Calibration requires every block within factor 1.05 and paired median within factor 1.03; ON/OFF median overhead must be at most 1.05. Thresholds and all observations are retained. Failure leaves graphs descriptive of the instrumented schedule.

Passing these checks does not make the trace unbiased. ON/OFF ratios vary around parity; the negative median overhead is not a prover speedup. RSS values are specific to these drivers and runs, including recorder/snapshot and allocator effects. They do not replace the accepted full-scope resource measurements.

## Covered service

These are independent medians across the nine saved main graphs; columns are not expected to sum to a median query total.

| Query | Feeds CPU ms | Indexed leaves CPU ms | Weighted leaves CPU ms | Tail CPU ms | Phi CPU ms | Merge CPU ms |
|---|---:|---:|---:|---:|---:|---:|
| A | 13.847487 | 0.675473 | 4.901225 | 1.123263 | — | — |
| K/H | 40.900567 | — | 7.958439 | 1.308002 | — | — |
| G2 | 38.672602 | 11.712342 | 4.254485 | 0.245921 | 1.577839 | 1.431849 |

Feed scopes include digit reading, insertion/collision handling and the existing streaming arithmetic. They do not isolate arithmetic from memory or bookkeeping. The observations favor studying the shared streaming service before more bucket-reduction layouts. K/H and G2 both finish last in this small fixed set; neither query alone defines every request's tail. Their availability-to-start intervals are not idle time: a waiting worker can execute another query's work. The G2 pre-input interval includes grouping, decomposition, waiting and helped work, so it cannot be treated as removable scalar-preparation latency.

## Scope and coverage

Current source 175 includes balanced K/H and borrowed A/G2 paths. Query roots cover canonical A, fused K/H and exclusive G2, including grouping. Explicit parent IDs cross new window/part and merge/group/indexed/weighted callbacks. Serial CPU is recorded only at nonforking feeds, pre-normalization tails, phi maps and leaf callbacks. It includes allocation, data movement and recorder work; it is not a field-cycle count. Input markers are mathematical entry/readiness boundaries, not enqueue timestamps. Solving, FFT, scattering, assembly, verification and uncovered normalization remain outside covered serial service.

Fixed 1,024 events of 80 bytes per worker/caller bound recorder payload to 2.579 MiB at 32 workers plus metadata and snapshots. Driver resets/snapshots are outside request samples but can affect context; loop CPU includes that observer work. All 31 observations and drop counts remain; graphs only at 0/15/30 by prior declaration. No buffer growth, exclusions, sample extension or successful rerun. These allocations belong only to the diagnostic.

Fresh 126 core and eight prover tests pass, including two recorder fidelity/drop tests. All 17 field files are exact; 33 field tests are inherited. Eight cfg-only originals plus one new child recover all 175 original bytes; 167 originals stay exact. All actual arithmetic, field, scheduler, scalar split, key and plan logic is unchanged. Semantic preflight requires actual canonical/borrowed A, balanced 20-window K/H, 11-window phi2 G2 and consistent parent/containment/serial clocks before main capture. No nested serial spans or serial overlaps per worker are accepted.

The original static namespace audit also selected seven data/TLS symbols; their objdump outputs contain headers only. Those entries are retained and separated from text symbols in the [audit review](artifacts/code-audit-review.json). Runtime semantic coverage, rather than those data-symbol sizes, establishes that the intended hooks executed.

## CPU and Metal references

The all-eight comparison below is inherited from accepted175, not newly measured here. No instrumented speedup or candidate all-shape performance qualification is claimed.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.022165 | 37.0 | 4.612× | 24.0 | 2.992× | 3.7 |
| 2x2 | 11.335969 | 59.0 | 5.205× | 30.0 | 2.646× | 5.9 |
| 4x2 | 15.945544 | 109.0 | 6.836× | 48.0 | 3.010× | 10.9 |
| 8x2 | 24.090485 | 189.0 | 7.845× | 74.0 | 3.072× | 18.9 |
| 12x2 | 33.844855 | 284.0 | 8.391× | 101.0 | 2.984× | 28.4 |
| 16x2 | 39.931557 | 335.0 | 8.389× | 124.0 | 3.105× | 33.5 |
| 24x2 | 66.233309 | 528.0 | 7.972× | 176.0 | 2.657× | 52.8 |
| merge-24x1 | 65.273224 | 507.0 | 7.767× | 175.0 | 2.681× | 50.7 |

Accepted source 175/SDK 20fde remains qualified 1×2 8.022165 ms/max 134 MiB and 54→1 162.250464 ms (no supplied M5 reference). The physical-16 baseline-calibration engineering exception remains. The all-eight CPU 10× goal remains active and unmet, 0/8; primary needs 3.7 ms. No new AWS/Aeglos, 16-worker, mobile/ARM/TEE, huge 250/4999, pressure/swap/minimum-RAM/concurrency/throughput result.

## Provenance

All 39 workflow steps complete: 346 SDK requests verify internally; 19 fresh 164-byte exports independently pass Gnark and exact image/public binding and freshness against 17,786 prior identities. Ledger 17,805; 38 proof/public members represent 19 proofs. No owned build/helper/hash/archive/other workload overlapped capture. Source/image/library/driver/helper pins close. [Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [semantic preflight](artifacts/semantic-preflight.json), [graphs](artifacts/graphs.json), [observations](artifacts/rows.json), [analysis](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json).

Publication review retains 11 whitespace findings: seven objdump data-symbol headers, three tool log endings and one inherited Cargo.toml blank line. No newly authored source finding. Report interpretation/spacing and the post-capture static-audit review were added after generation; original pinned artifacts remain exact and archive hashes were regenerated and checked.
