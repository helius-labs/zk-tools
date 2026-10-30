# Bounded native G1 column tiles

**The complete-filler gate fails; no adoption.** The32-worker aggregate is1.003706[0.994900,1.028885],5/15winning blocks. The interval crosses parity and five of six context nonregression checks fail. The favorable1x2estimate remains in the table; it is not promoted independently. Production source176, accepted proof latency and ledger27,353 are unchanged.

The prior all-column/circular-wave implementation remains rejected. This follow-up changes the active-state lifetime and topology: process at most25cohorts/200partial segments to completion, export them, then reuse their native storage for the next tile. Replace comparison sorting with a stable eight-bin partition, preserving the exact descending-population/ascending-ID order. It removes the global CSR wave schedule and all-segment native coordinate allocation. Field/range helpers, segment size8, point formula, exception fallback and final partial-merge topology remain exact.

## Count and memory tradeoff

The changed-path layout test checks sizeof(Column)=704 and sizeof(Record)=1024 on this target. Capacities bound native columns to17,600bytes and records to25,600bytes per active filler. Exported partial G1points, indices, maps, allocator overhead, merge workspace and compiler frames remain additional. This is not a43.2KB total memory bound or minimumRAM claim. The full partial-point output array remains live alongside each active tile.

Stable partition and tiled scheduling preserve the cohort order and linear vector-update/addition totals on all42ordinary rawA windows. Local tile boundaries add linear inversion stages:

| Raw shape | Original ordinary additions including merges | Prior global linear waves | New tiled linear waves | Change |
|---|---:|---:|---:|---:|
| 1x2 | 348545 | 1641 | 1704 | +3.839% |
| 2x2 | 636516 | 2884 | 2952 | +2.358% |

These are structural nonexceptional stage counts, not measured inversion costs. Real curve exceptions/cancellations and final merging can differ. The prior count model's49%logical-codec saving is not a latency estimate. No segment/tile-size sweep is performed: eight points per segment and25cohorts per tile derive from the existing native lane and200-entry batch geometry.

## Complete ordinary-filler measurements

Identical driver bytes compare the byte-exact original borrowed filler with the fresh tiled caller. Each pass fills all21ordinary width12windows for raw pre-group Ainputs; these are not automatically the final grouped-query populations. Timed scope includes histogram/partition/scatter, tile construction, all field/range/exception work, export/partial merge, allocation/drop and pool dispatch/wait. Loading, pool/oracle construction, narrow top, final weighting, solving and proof verification are excluded. **These are filler timings, not proof latency.**

Fifteen fixed rotated/reversed paired blocks use3primes,20timed passes and1post-check per process at1/16/32workers on the shared Ryzen9950X. Whole-process affinity is CPU2/0–15/0–31; workers may migrate within it. No additional worker pool or blocking worker barrier is introduced. No owned build/hash/archive/analysis workload overlaps captures; exclusive host use is not claimed.

| Raw shape | Workers | OFF ms/pass | Tiled ms/pass | Paired tiled/OFF | Bounds | Wins/15 |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 1 | 10.571355 | 10.919078 | 1.034345 | [1.030973, 1.038579] | 0 |
| 1x2 | 16 | 1.051783 | 1.127794 | 1.065972 | [0.958427, 1.078646] | 5 |
| 1x2 | 32 | 0.855039 | 0.833633 | 0.987295 | [0.958536, 1.013042] | 10 |
| 2x2 | 1 | 16.702504 | 18.980610 | 1.137064 | [1.135269, 1.139285] | 0 |
| 2x2 | 16 | 1.646595 | 1.939937 | 1.181024 | [1.141274, 1.187176] | 1 |
| 2x2 | 32 | 1.338826 | 1.405726 | 1.041074 | [1.021998, 1.082461] | 2 |

Primary equal-log aggregate across both32-worker shapes requires upper ratio<1 and at least10wins; every context upper must be≤1.03. The aggregate fails both conditions. Only1x2-w32passes its context bound; its gain interval still crosses parity. Physical16medians are higher, with2x2clearly slower and1x2uncertain. Ranks4–12/15 give nominal marginal96.484375%conditional coverage without family-wise guarantees. Marginal medians need not divide into paired estimates. There is no minimum gain floor, exclusion, block extension, favorable subset or unchanged reroll. The old all-column failed gate remains false; the two implementations are separate studies, not a paired head-to-head attribution.

## CPU service and sampled process memory

| Raw shape | Workers | OFF CPU ms/pass | Tiled CPU ms/pass | OFF/tiled peak sampled RSS MiB | OFF/tiled median minor faults/pass |
|---|---:|---:|---:|---:|---:|
| 1x2 | 1 | 10.571 | 10.919 | 14.25 / 14.50 | 0.15 / 0.15 |
| 1x2 | 16 | 11.665 | 13.283 | 18.75 / 27.25 | 8.30 / 42.60 |
| 1x2 | 32 | 15.966 | 19.974 | 19.75 / 28.75 | 17.00 / 36.35 |
| 2x2 | 1 | 16.702 | 18.981 | 17.00 / 17.75 | 0.10 / 0.10 |
| 2x2 | 16 | 18.046 | 22.413 | 20.00 / 33.75 | 7.05 / 57.95 |
| 2x2 | 32 | 23.817 | 32.856 | 19.25 / 36.50 | 17.80 / 24.30 |

RSS is the process high-water sampled at the timed-loop end, including loading, expected arrays and primes. It is not a prover peak, incremental workspace or minimumRAM. The bounded native tile does not remove other partial/index/merge allocations; total observed process memory remains higher than OFF. Fault counts do not isolate a latency cause.

## Integrated warm counters and profiles

All latency precedes4grouped stat panels and4separate FIFO-gated cycles:u recordings. Each measures200passes after3excluded primes atprimary1x2w16/32. All four events run100%. Values below are one descriptive panel per context, not paired counter confidence intervals.

| Workers / arm | Cycles M/pass | Instructions M/pass | Generic cache misses M/pass | Branch misses M/pass |
|---|---:|---:|---:|---:|
| 16 / control | 60.289 | 145.890 | 0.124463 | 0.117864 |
| 16 / candidate | 67.618 | 152.499 | 0.749935 | 0.152161 |
| 32 / control | 76.656 | 150.445 | 0.144217 | 0.125938 |
| 32 / candidate | 91.671 | 156.736 | 0.807905 | 0.165544 |

At32workers, tiled has about4.18%more instructions and5.60×generic cache misses than matched OFF. Bounding native coordinate state alone does not establish less total work or good overall memory behavior. The counters do not isolate a cache level, RAM bandwidth, input-gather cost or causal fraction of the latency.

| Profile | Samples | Unknown share | Lost |
|---|---:|---:|---:|
| 16 / control | 684 | 0.439% | 0 |
| 16 / candidate | 753 | 0.133% | 0 |
| 32 / control | 727 | 0.275% | 0 |
| 32 / candidate | 938 | 1.279% | 0 |

All3,102samples have the fixed16Mperiod, one expected executablePID per recording, sampledTIDs matched to actual workerFORK records and allowed CPUbounds. All profiles have fewer than1,000samples and zero lost records. Symbol bins include inlined work; sampled-period shares are not exclusive costs or removable wall time. Absolute period-to-wall coverage is not validated. The linked candidate contains no comparison-sort path for the column filler.

## Qualification and retained preparation details

The isolated178-file snapshot changes only the column child relative to the qualified previous candidate:177files, including all field helpers, are byte-identical. The125unaffected core tests and35base+2field-helper tests are inherited. Four changed-path tests run now (125filtered): the previous3generator/exception/threaded tests against the new filler plus a new stable-partition,1600/1601/3201-point tile-boundary and native-layout test. All pass. No standalone prover/portable or fullSDK qualification is inferred.

Exact prior independent Arkworks real-point arrays and saved scalar fixtures are reused by hash, with no new solve or oracle execution. Every process checks all43,008buckets per shape in3primes and1post-check; timed intermediate outputs are black-boxed/dropped but not individually checked. All200caller processes pass:6,012ordinary-fill passes including primes/checks,126,252individual windows and34,406,400bucket comparisons. Zero new SDK proof requests or independent exports; ledger27,353unchanged.

A preparation-utility character/string typo was corrected before its first execution. A template-whitespace assertion then stopped helper generation; initial utility sources, partial audit and assertion log are retained. The repaired audit is identical, and no native test or measurement had run. The subsequent4-step native build/test/freeze/link workflow and220-step capture workflow each complete on their first executions. Four planned ownership commands make only the generated perf.data readable before16read-onlydecoders; there is no failed decoder or capture rerun.

Independent review checks all raw200command/exit/stream receipts, six latency/CPU/RSS cells, paired distributions/failed gate, four grouped counters, every sample/symbol bin and executablePID/workerTID/CPUownership. It rechecks178private/176production sources, immutable libraries, unchanged oracle bindings and cached scalar words against actual input fixtures. It executes no tests, solve, model, capture or verifier again.

## Accepted proof comparison — inherited, unchanged

| Shape | Zopro ms | Peak RSS MiB | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | 10× CPU target ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.928660 | 132.75 | 37 | 4.667× | 24 | 3.027× | 3.7 |
| 2x2 | 11.151932 | 275.00 | 59 | 5.291× | 30 | 2.690× | 5.9 |
| 4x2 | 16.086386 | 312.25 | 109 | 6.776× | 48 | 2.984× | 10.9 |
| 8x2 | 24.179453 | 517.25 | 189 | 7.817× | 74 | 3.060× | 18.9 |
| 12x2 | 33.884455 | 706.50 | 284 | 8.381× | 101 | 2.981× | 28.4 |
| 16x2 | 40.077381 | 842.00 | 335 | 8.359× | 124 | 3.094× | 33.5 |
| 24x2 | 65.850777 | 1300.50 | 528 | 8.018× | 176 | 2.673× | 52.8 |
| merge24 | 65.060937 | 1168.19 | 507 | 7.793× | 175 | 2.690× | 50.7 |

The all-eight 10× CPU objective remains unmet: **0/8** targets achieved. The primary accepted result is 7.928660 ms against a 3.7 ms target. The inherited 54-1 merge result is 161.883899 ms / 1,876.96 MiB peak RSS, without a supplied M5 reference. Latest AWS comparisons predate accepted source176; this study changes none of them.


No new AWS/Aeglos, phone/ARM/TEE, huge-circuit, pressure/swap, concurrent-proof or minimumRAM result follows. This exact bounded-tile implementation is closed by the failed gate. No fullSDK trial or source promotion follows.

[Design](artifacts/design.md), [capture protocol](artifacts/capture-design.md), [all observations](artifacts/analysis.json), [structural counts](artifacts/counts.json), [profiles](artifacts/symbols.json), [independent review](artifacts/review.json), [linked code](artifacts/codegen.json), [source audit](artifacts/source-audit.json). Archives: [private source](artifacts/source.tar.gz), [workflow/helper sources](artifacts/workflow.tar.gz), [all command receipts and raw counter CSVs](artifacts/receipts.tar.gz), [linked disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS).

Raw key/scalar/point/expected arrays, executables, libraries and perf.data stay local with recorded hashes. No witness-derived point outputs are published. Related: [previous all-column rejection](../2026-10-30-g1-columns176/README.md), [accepted production](../2026-10-30-g2-vector-sign-adoption176/README.md).
