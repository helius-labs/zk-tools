# Ready-root inversion sharing: latency and CPU comparison

**The combined latency gate failed; no broker or production change is selected.** At 16 workers the optimistic shared bulk loses 26–27%; at 32 it wins 2.3–2.6% before request and handoff costs. Summed inversion-stage CPU time falls about 12×/31×. These are distinct results, and neither is a full-proof speedup.

The ready-root sharing implementation is closed without a broker or production adoption. Its predeclared latency gate fails six of fifteen checks: two16-worker nonregression, two16-worker gain and both threaded aggregate gain checks. All three control calibrations pass. All187commands, frozen analyzer and independent reviewer succeed on their first executions. No unchanged capture, build, qualification or decoder is rerun.

At16workers, shared/controlA is1.262530353 [1.256405866,1.269018437], zero wins; shared/controlB1.268336109 [1.260881284,1.273280129], zero wins. The optimistic bulk is26–27% slower. Median waves are511.515/510.645ns for the controls and647.507ns shared. At32, the genuine positive is preserved: shared/controlA0.974467150 [0.969001636,0.978047195] and shared/controlB0.976690830 [0.967967915,0.984943860], both15/15wins, about2.3–2.6% lower paired latency. Median waves are758.775/755.848ns versus737.976ns. These paired ratios differ from ratios of unpaired medians.

The equal-log16+32 paired aggregate is1.109214978 [1.104988715,1.111959375] againstA and1.114780024 [1.106302646,1.118460295] againstB, zero wins in both. The solo path is the identical individual-inversion fallback and passes nonregression. No positive-gain floor was imposed, and no worker count, block or result is dropped. The32-worker gain does not rescue the predefined combined gate.

Summed CPU time improves substantially: at16, about7.909microseconds/wave forA versus0.647shared, roughly12.2× lower; at32,23.009versus0.742microseconds, roughly31.0× lower. Median busy CPUs move15.44/30.40 to about1.0. Six PMU panels have100%coverage. Shared/controlA user-cycle ratios are about0.0862/0.0348 at16/32 and instruction ratios0.0772/0.0434. These are inversion-stage work observations, not server proof-throughput improvements or full-proof speedups. Independent inversions were already parallel, so reducing aggregate work did not guarantee lower wave latency.

This is favorable to the coordinator: all roots are already available, other workers and its SMT sibling are parked, and it pays no request publication, leader election, polling, queue or result-handoff cost. Actual packing/unpacking, wrapping output sums and main/worker start/end barriers are timed. Input address streams also differ: independent workers read columns of the public matrix, while the coordinator reads rows; this is not a same-locality measurement of inversion latency. No cache-level cause is inferred from generic cache-miss counts. It is not a mathematical lower bound on all asynchronous algorithms. There is no measured coordination-cost budget or complete-proof arrival model here; do not invent one from the narrow32-worker margin. The experiment closes this implementation before broker engineering, not every possible sharing strategy.

All156driver processes have zero timed allocation, exact public-fixture hashes, pinned TID/CPU/active-worker identities and validated sums. Equal-round hashes match across arms; unequal-duration PMU/profile sums are checked by the immutable driver against scalar references outside the clock, with identical-duration controls compared directly. Full canonical output/Arkworks wrapper qualification is inherited, not rerun. The benchmark fixture is256KiB of public roots, not an inverse lookup or prover table.

All six profiles qualify:49,708fixed-period samples, all on active workers, unknown fractions below0.08%, zero lost events, exact period and executableMMAP/FORK/TID/CPU ownership. Minor faults are roughly0.000008–0.000023per wave. Process RSS peaks2.0–2.5MiB include this small fixture and thread setup, not a proof key, full-proof peak or minimum RAM. There is no page-fault bottleneck evidence for this workload. The qualification's2,240-byte conceptual32-worker service state is still a sizeof calculation, not an implemented safe service or actual peak.

No proof/SDK/Gnark/AWS/Aeglos, huge/mobile/pressure/swap or throughput measurement is added. Production176/SDK6d83, local1×2 7.928660ms, inherited132.75MiBRSS and the30,846-proof ledger are unchanged. All eight CPU10× requirements remain active/unmet0/8, with both supplied M5CPU andMetal comparisons retained.


## Scope and frozen method

The [arithmetic qualification](../2026-11-06-root-sharing-qualification176/README.md) and its accepted field library are inherited without another native build, mathematical population, decoder or suite run. Driver `40efd381…` executes identical individual-inversion code for control-a and control-b in the SAME executable. They are repeated controls, not differently compiled production references or accepted proof SDK calls. Shared uses one existing benchmark worker on CPU2, with all other workers parked. Solo uses the exact individual fallback. No production worker/service is added.

The public input matrix has 256 rows of 32 canonical Fq roots (256 KiB), including deterministic full-width hashed roots and boundary rows. It is input data, not a precomputed inverse lookup. Individual workers traverse columns; the coordinator traverses rows. Different address streams, parked peers and omitted request handling make this favorable to the shared calculation; they prevent treating it as a universal inversion-latency bound. No cache-level cause is inferred from the generic counter.

All threads exist before timing, pinned CPU2 solo, physical cores0–15 at16 and CPUs0–31 at32, each with a512 KiB configured stack. Three256-wave primes precede sum reset. Main/worker start/end barriers, arithmetic, source packing/unpacking and output sums are timed. Fixture generation and individual-inverse reference/checksum validation are outside the clock. Every process requires zero timed allocation. The four wrapping-u64 sums per root are not a cryptographic proof; the wrapper's canonical full-output Arkworks qualification is independently bound.

The Rust/xtask protocol was frozen before any driver clock:9preflights and1strict checker,135latency processes (15paired blocks×3worker counts×3labels),6grouped counter processes,6cycle profiles and30offline ownership/decode/metadata/event/loss commands. All187commands and156driver outputs succeeded once. Context and label order rotates/reverses. Each latency process runs262144waves. No pilot, extra blocks, favorable topology, changed gate, batch-width/layout/attribute sweep or unchanged reroll is used. All latency completes before diagnostics, with no competing owned compute/build/hash/package workload during capture.

Intervals use sorted zero-based ranks3/7/11 of15 paired ratios; coverage is marginal, not a simultaneous family-wise guarantee. All controlB/A intervals must lie inside[0.97,1.03]. Every shared/control interval has nonregression upper≤1.01. At16and32, each shared/control comparison and the equal-log16+32 aggregate require upper<1 and≥11wins. These are15timing checks; no minimum positive-gain floor. All three calibrations pass. Six latency checks fail and are retained.

## Complete timing and resource results

ns/wave is the time to finish one root inversion per worker, not proof latency or proof throughput. CPU time sums the process's active threads; sharing deliberately parks its peers. RSS is the maximum process peak across the15blocks, including this small fixture and thread setup. It is not a full-proof peak, actual stack use or minimum RAM.

|Workers|ControlA ns/wave|ControlB ns/wave|Shared ns/wave|Shared/A [interval], wins|Shared/B [interval], wins|ControlB/A [interval], wins|
|---|---:|---:|---:|---|---|---|
|1|470.975639|470.787521|469.962986|0.997849882 [0.995618810, 1.002420811], 9/15|0.999531940 [0.992773149, 1.001621532], 9/15|1.001014522 [0.996234456, 1.003834952], 6/15|
|16|511.514530|510.644932|647.507061|1.262530353 [1.256405866, 1.269018437], 0/15|1.268336109 [1.260881284, 1.273280129], 0/15|0.996036663 [0.991470370, 1.001307870], 10/15|
|32|758.774902|755.847973|737.975876|0.974467150 [0.969001636, 0.978047195], 15/15|0.976690830 [0.967967915, 0.984943860], 15/15|0.999534194 [0.986174672, 1.006714308], 8/15|

|Threaded aggregate|Shared/control [interval], wins|
|---|---|
|control-a|1.109214978 [1.104988715, 1.111959375], 0/15|
|control-b|1.114780024 [1.106302646, 1.118460295], 0/15|

|Workers|A / B / shared CPU µs per wave|A / B / shared median busy CPUs|A / B / shared peak RSS MiB|A / B / shared minor faults per wave|
|---|---|---|---|---|
|1|0.470981598 / 0.470779419 / 0.469955444|0.999981338 / 0.999971521 / 0.999973185|2.500000000 / 2.500000000 / 2.500000000|0.000019073 / 0.000019073 / 0.000019073|
|16|7.909080505 / 7.888481140 / 0.647014618|15.437432121 / 15.456474356 / 1.000007887|2.000000000 / 2.000000000 / 2.250000000|0.000007629 / 0.000007629 / 0.000022888|
|32|23.009391785 / 23.010620117 / 0.742233276|30.400664436 / 30.456312587 / 1.000226407|2.250000000 / 2.250000000 / 2.500000000|0.000019073 / 0.000019073 / 0.000015259|

All failed checks:

- `w16/shared/control-a/nonregression`
- `w16/shared/control-a/gain`
- `w16/shared/control-b/nonregression`
- `w16/shared/control-b/gain`
- `threaded/shared/control-a/gain`
- `threaded/shared/control-b/gain`

## Counters and automatic profiles

Panels cover16/32workers and all3labels after all latency. Fixed FIFO-controlled cycles:u sampling period is16,000,000. Controls profile2,097,152waves, shared16,777,216; counts were fixed from active-core structure/prior budgets before clocks. Stat uses one quarter. Each of four grouped user events requires≥90%coverage; all six panels have100%. Per-wave normalization uses actual counts. Differing-duration shared output sums validate against individual references inside the pinned driver after the clock; equal-duration control hashes also match directly. There is no diagnostic speed or proof-throughput claim.

|Panel|User cycles/wave|Instructions/wave|Cache misses/wave|Branch misses/wave|
|---|---:|---:|---:|---:|
|stat-w16-control-a|41646.614655|129895.651953|0.015853882|23.502162933|
|stat-w16-control-b|41512.012453|129895.629667|0.013507843|19.717376709|
|stat-w16-shared|3588.618720|10027.372749|0.022343636|1.865203381|
|stat-w32-control-a|116757.719570|259858.344652|0.021478653|83.076583862|
|stat-w32-control-b|118100.479067|259858.351601|0.033813477|98.610546112|
|stat-w32-shared|4061.820370|11269.473625|0.003086567|0.914457083|

All6profiles qualify:≥1,000samples, unknown≤5%, zeroLOST, exact period, executableMMAP/FORK/PID/TID/CPU ownership and≥95%active-worker samples. All49,708samples are on declared active workers. Controls have all workers active; shared only the coordinator. Other owned main/parked-thread samples remain explicitly counted. No recording is extended or re-decoded for packaging.

|Profile|Samples|Unknown share|Active-worker share|Other owned samples|Qualified|
|---|---:|---:|---:|---:|:---:|
|cycles-w16-control-a|5424|0.03687%|100.00000%|0|true|
|cycles-w16-control-b|5409|0.01849%|100.00000%|0|true|
|cycles-w16-shared|3741|0.05346%|100.00000%|0|true|
|cycles-w32-control-a|15361|0.05208%|100.00000%|0|true|
|cycles-w32-control-b|15509|0.07737%|100.00000%|0|true|
|cycles-w32-shared|4264|0.04690%|100.00000%|0|true|

## Decision and continuation

The32-worker latency win and much lower inversion-stage CPU work are preserved. They do not establish a profitable service after publication/ownership/waiting costs, a complete-proof arrival model or the required all-context gain. Current leaf scratch accounting assumes no nested fork while a leaf waits; arbitrary job helping would need a new lifetime/admission design. No broker, new pool, wait window or32-only rescue is selected from this result. It does not prove every possible asynchronous sharing method impossible.

The [next stage](artifacts/next.md) requires a genuinely different whole-path algorithm, checked against primary sources and the existing negative history, before another native implementation. Prepared verification lines/public-input windows, fixed blinding windows and prior final-exponent studies were found already present; they are not new optimizations. No new candidate is currently selected. All eight original CPU10× targets remain active.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.





## Storage and artifacts

Raw profiles and every original workflow file remain at `/home/ubuntu/src/zopro-perf-work/root-sharing-waves176/perf/` and `runs/`. The six raw recordings and twelve sample/metadata decode stdout streams have local byte/SHA indexes. All other workflow files, including every latency output, command, exit and stat CSV, are in the verified UTF8 archive. Host-public excludes the unrelated process listing; its original local file stays pinned. No executable/library blobs, private key/image/witness/proof bytes, inverse-result table or third-party full text is embedded. No capture, successful suite or decode is repeated for publication.

[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [driver](artifacts/driver.rs), [bulk wrapper](artifacts/bulk.rs), [native qualification](artifacts/qualification-review.json), [symbols](artifacts/profile-symbols.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decode index](artifacts/profile-stream-index.json), [publication](artifacts/publication.json), [checksums](SHA256SUMS). No fresh SDK/proof/Gnark/AWS/Aeglos/huge/mobile/pressure/swap/minimum-RAM/throughput result.
