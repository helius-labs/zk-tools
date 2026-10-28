# G2 baseline runtime-layout diagnostic

**The tested prefix-buffer placements do not reproduce or explain the previous 3× control gap.** Natural placement and fixed page offsets 0/2048 have nearly equal latency here. No prover change is adopted. The old packing candidate and its failed calibration remain closed; these diagnostic timings do not replace its observations.

One newly instrumented baseline executable runs through two equal-length hardlink aliases with identical bytes and inode. Field/core/prover arithmetic comes exclusively from the previous study's frozen accepted-arithmetic libraries. No field, core or production allocator source is rebuilt or changed. The Rust observation wrapper records allocation layout outside clocks and changes only backing for requests matching the small G2 prefix record size/alignment in the forced modes.

## Unprofiled observations

Seven fixed rotated/reversed blocks cover G2 at 200/352 pairs in three placement modes and a natural G1-owned/200 control. Two aliases give 98 fresh processes. Each uses CPU 2, one active caller, three 1,024-call primes with independent checks/reset, then 16,384 timed complete caller calls. Values below are medians of process means across both aliases, in µs per batch.

| Caller | Pairs | Natural | Page offset 0 | Page offset 2048 |
|---|---:|---:|---:|---:|
| G2 | 200 | 7.836895 | 7.835083 | 7.826023 |
| G2 | 352 | 13.230685 | 13.221800 | 13.228230 |
| G1 owned | 200 | 4.113877 | — | — |

| G2 pairs | Forced offset | Alias | Paired forced/natural median | Marginal min–max bounds |
|---|---:|---|---:|---|
| 200 | 0 | a | 0.999097069 | [0.992227730, 1.002676780] |
| 200 | 0 | b | 1.000968588 | [0.997524094, 1.004135606] |
| 200 | 2048 | a | 0.998562807 | [0.996459569, 1.002229024] |
| 200 | 2048 | b | 0.998627933 | [0.996259278, 1.001139962] |
| 352 | 0 | a | 0.999915504 | [0.998346579, 1.002148513] |
| 352 | 0 | b | 0.998244106 | [0.996563394, 1.001335886] |
| 352 | 2048 | a | 1.001025584 | [0.997961594, 1.002594150] |
| 352 | 2048 | b | 0.998649871 | [0.997814476, 1.001955138] |

Small positive estimates are retained, including about 0.14% lower 200-pair latency at offset 2048. Every placement interval includes parity. This establishes no reproducible placement speedup and supplies no basis for choosing a production padding rule. Bounds are the min/max of seven paired block ratios, nominal 98.4375% marginal coverage under independent exchangeable blocks, with no familywise guarantee. No gain gate, sample exclusion, extension or parameter sweep was used.

| Caller/context | Alias b/a median | Marginal min–max bounds |
|---|---:|---|
| g1/200/m0 | 0.999696644 | [0.956905685, 1.041932168] |
| g2/200/m0 | 0.999111542 | [0.996891970, 1.002951822] |
| g2/200/m1 | 1.001015895 | [0.997786913, 1.011952843] |
| g2/200/m2 | 0.999835078 | [0.997225091, 1.001320486] |
| g2/352/m0 | 1.001182081 | [0.999781608, 1.002380293] |
| g2/352/m1 | 0.999661874 | [0.998329458, 1.001910712] |
| g2/352/m2 | 1.000115760 | [0.997624156, 1.001523950] |

G2 alias results stay close here. The G1 negative control retains a wider roughly −4.3% to +4.2% paired range, so this is not a claim of universally stable measurement. The changed harness, longer priming, common executable and controlled placements distinguish this study from the earlier failed control comparison. The original anomaly's cause remains unresolved; it is not assigned to prefix placement, compiler behavior, RAM capacity or the old candidate.

## Allocation evidence and correctness

All 110 timing/stat/cycle processes report zero timed allocations and pass independent weighted-generator output checks. Digests match across modes and aliases for each size/round scope. Every G2 process records exactly one live prefix-sized, 64-byte-aligned buffer: 16,000 bytes for 200 pairs, 28,160 for 352. Natural timing buffers consistently occupy page offsets 3072/1024 respectively; forced modes record exactly 0 or 2048. Stack/fixture offsets vary across processes and are preserved in the results. The fixed 512-event journal does not overflow. It is disabled before clocks, and live buffer maps are reconstructed afterward.

Forced requests use a bounded pointer registry, separate backing, and head/tail canaries. Ownership-based deallocation remains valid across mode changes; resize copies the smaller live extent and preserves old ownership if allocation fails. The diagnostic reserves 4,128 extra backing bytes per intercepted request. This overhead and the fixed observation journal are not a proposed prover policy, new universal memory limit, measured RAM minimum or heap saving. Matching requests during setup can also be intercepted; the live-buffer assertion checks the actual clock-time scope.

Four native probe tests pass: two new allocator/caller tests cover both sizes, offsets, zeroing, guards, resize transitions and real G2/G1 generator oracles; two inherited perf-helper tests check FIFO behavior. Complete field/core suites are inherited through the exact frozen libraries, not rerun. Initial preparation expected only the two new tests; the first checker stopped before aliases or captures when it saw four passing tests. A versioned count-only repair retained the original successful suite and probe binaries. No successful test or capture was repeated.

## Hardware counters and profiles

After all 98 unprofiled captures, six separate FIFO-gated perf-stat processes use the group `{cycles:u,instructions:u,cache-misses:u,branch-misses:u}`; six separate 199 Hz cycles:u profiles follow. Each performs 262,144 calls after three 1,024-call primes. All twelve complete before decoding, hash or archive work. No owned build/helper/hash/archive/other benchmark overlaps captures. An unrelated host ceremony job is recorded; no exclusive-host claim is made.

All four grouped stat counters report 100% running time. Counts below are per batch from one stat process per context; they are descriptive observations, not confidence intervals.

| G2 pairs | Mode | Cycles/batch | Instructions/batch | IPC | Cache misses/batch | Branch misses/batch |
|---|---|---:|---:|---:|---:|---:|
| 200 | natural | 41254.765 | 109427.313 | 2.652477 | 0.006935 | 14.854084 |
| 200 | offset 0 | 41175.148 | 109427.323 | 2.657606 | 0.002846 | 13.728401 |
| 200 | offset 2048 | 41202.101 | 109427.324 | 2.655868 | 0.003178 | 14.376740 |
| 352 | natural | 69592.878 | 184968.832 | 2.657870 | 0.005989 | 16.323341 |
| 352 | offset 0 | 69506.322 | 184950.832 | 2.660921 | 0.006607 | 14.695591 |
| 352 | offset 2048 | 69552.380 | 184959.832 | 2.659288 | 0.010967 | 14.840931 |

The instruction and cycle counts are similar across tested placements. Generic cache-miss counts are retained without assigning them to a particular cache level or causal stall. Counters include the enabled loop and small control/accounting edges. Profiling may alter natural heap placement; actual profile layouts are recorded rather than assumed equal to unprofiled layouts.

| Cycle capture | Samples | Sampled PIDs | Lost |
|---|---:|---:|---:|
| cycles-g2-200-m0 | 407 | 1 | 0 |
| cycles-g2-200-m1 | 407 | 1 | 0 |
| cycles-g2-200-m2 | 405 | 1 | 0 |
| cycles-g2-352-m0 | 770 | 1 | 0 |
| cycles-g2-352-m1 | 777 | 1 | 0 |
| cycles-g2-352-m2 | 770 | 1 | 0 |

Total samples: 3536. Every cycle capture has one sampled PID and zero lost records. Most sampled period falls in G2 prefix application, which includes inline packing/arithmetic; scalar Fq inversion is also visible. These shares are not exclusive opcode costs or removable proof wall time. Raw perf.data files stay local with hashes; decoded observations are archived.

The preceding six-counter availability check left one event uncounted on a short command. The four-counter group used here was verified available before the design froze; no post-result event substitution occurred. This check is separate from the kernel experiment. Source, protocol, helpers, exact compiler, libraries, both alias paths and executable hashes were pinned before capture and pass afterward. No ELF executable was edited; all baseline library/executable inspections are read-only.

## Accepted comparison (inherited)

The full all-eight 10× CPU objective remains active and unmet, 0/8. These are the accepted 172-source full-proof observations, not this diagnostic's proof measurements; M5 CPU/Metal times are supplied references.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | M5 CPU / accepted | M5 Metal / accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |

Accepted historical 1x2 peak RSS: 134.50 MiB; primary CPU target: 3.7 ms. Merge54: 161.341835 ms / 1910.1875 MiB peak, with no supplied M5 reference. The accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates.

No new SDK/Gnark proof was generated; the independently verified ledger remains 12,983. There is no new complete-MSM/proof, all-eight/54, pressure/concurrency, mobile/ARM/TEE, giant-circuit or AWS/Aeglos result. No cloud resource was created.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [identity audit](artifacts/identity-audit.json), [library provenance](artifacts/library-provenance.json), allocator/probe source and full raw logs are archived. The [preceding packing report](../2026-10-28-direct-ref-pack/README.md) retains its original failed controls and binary-restoration record.

Next: leave the unproven page-placement hypothesis unadopted. Return to reducing work or dependencies in the dominant point-addition kernels, using this common-image diagnostic structure for future component controls. Do not rerun the closed direct-input candidate or select a padding offset from these small estimates.
