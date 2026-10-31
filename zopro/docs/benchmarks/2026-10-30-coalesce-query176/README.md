# Sparse full-scalar coalescing: native complete-query trial

**Final-driver compiler correction (31 October 2026):** These query drivers used an extern-only `rustc_flags` override, omitting the default optimization/LTO profile while their backend libraries remained native release builds. The [compiler-profile audit and correction](../2026-10-31-sdk-link-flags176/README.md) documents the limitation. Original data and failed gates remain; this correction promotes no candidate and changes no accepted proof timing.

**The query gate fails; no adoption.** The primary equal-log A/B2 aggregate at 32 workers is 1.045199 with interval [1.037001, 1.061052] and 0/15 winning blocks: 4.52% slower by the paired estimate. Single-worker A improves 7.05%, and 16-worker A improves 1.10%; those gains are retained below. Primary B2 slows at every tested worker count. Production source 176, the accepted SDK and proof ledger of 27,353 remain unchanged.

The earlier [operation-count screen](../2026-10-30-bounded-coalesce176/README.md) predicted 3.329%/3.104% fewer primary A/B2 generic fill/top pairs after group formation. This trial implements the proposal and measures its full query cost. Count savings alone did not predict latency at high worker counts. These measurements cover queries, without a new proof or AWS run.

## Mechanism and memory

Reuse the original full scalar classification when its final population cutoff declines grouping. Preserve sampling, memory admission, capacity policy, stable class IDs, zeros and cutoff. The existing arrays then support grouping within consecutive 1,024-row tiles inside original point partitions. Combine only equal entire nonzero scalars with at least two active original digits; ignore identity operands. Packed count/representative/group tags reuse existing count records, with a 2 KiB u16 link array and up to 512 eight-byte tile records. Reduce all selected groups with the existing complete affine group kernel, then retain sorted skip rows and representative-point records. Release original classification before curve reduction. Admitted global grouping bypasses this path, as do unsupported inputs.

Windows pass original contiguous runs to existing borrowed batch loops, substitute only derived repeated sums and omit their other rows. No singleton point array, retained key lookup table or duplicate solver plan is created. G2 retains original high/low flush boundaries and Phi application. Narrow tops use the original indexed reducer through a mapped getter. Width 12, A's one part, B2's two parts, weighting, Horner steps, field arithmetic, inverses and exceptional arithmetic remain. The private query wrappers reconstruct window scheduling; this trial measures the resulting implementation as a whole. It does not isolate the effect of saved additions.

| Primary query | Derived sums | Skipped rows | Retained overlay payload |
|---|---:|---:|---:|
| A |227|583|18,676 bytes|
| B2 |325|849|47,596 bytes|

Actual record alignment makes these payloads larger than the prior screen's hypothetical 17,768/46,296 bytes. They exclude original inputs, classification/formation scratch, allocator overhead, stacks and all other query workspace. Optional checked reservations conservatively charge new metadata, formation workspace and possible buffer-reallocation overlap using the actual duplicate count. Allocation/admission failure falls back to the already-classified raw route. There is no fixed universal RAM ceiling. These payloads are not process peaks or minimum RAM requirements. All 12 contexts observe the intended route and exact counts: primary sparse; 2×2 original admitted grouping; OFF original query wrapper.

## Complete-query latency

AMD Ryzen 9 9950X, 16 physical cores/32 logical CPUs. Individually pinned custom Rayon workers use 512 KiB stacks: CPU 2 for one worker, 0–15 for 16 workers, and 0–31 for 32 workers. Both coherent builds use the same driver bytes and canonical point/scalar fixtures. Timers include global classification, optional preparation, point formation, ordinary/top fills, component/part merges, weighting, normalization, allocation/drop and pool dispatch/join. Exclusive B2 word reset is charged identically inside both timers. Loading, pool creation and independent oracle checks are excluded. Every timed result is checked afterward.

Fifteen fixed rotated/reversed paired blocks follow 24 preflights. Each latency process has three checked primes, 20 timed queries and one checked post. The 360 latency processes finish before separate diagnostics. No owned build/hash/archive/analysis workload overlaps capture; exclusive host use is not claimed.

| Shape/query | Workers | OFF ms/query | Candidate ms/query | Paired candidate/OFF | Bounds | Wins/15 |
|---|---:|---:|---:|---:|---:|---:|
|1x2/A|1|14.652497|13.625729|0.929455|[0.928528,0.930814]|15|
|1x2/A|16|1.542336|1.525153|0.989042|[0.982056,0.995992]|14|
|1x2/A|32|1.194199|1.238235|1.022120|[1.015956,1.054967]|1|
|1x2/B|1|37.232785|37.517195|1.007235|[1.006277,1.008684]|0|
|1x2/B|16|3.472723|3.652988|1.046506|[1.037039,1.053679]|0|
|1x2/B|32|3.070181|3.259344|1.067370|[1.039913,1.078800]|0|
|2x2/A|1|14.542076|14.633808|1.007358|[1.005179,1.009123]|0|
|2x2/A|16|2.157401|2.160639|1.002133|[0.992228,1.015117]|6|
|2x2/A|32|1.772073|1.742513|0.966114|[0.897302,1.002044]|10|
|2x2/B|1|39.815454|39.754591|0.998249|[0.997033,0.999584]|13|
|2x2/B|16|4.006636|4.045700|1.010505|[0.991006,1.029101]|6|
|2x2/B|32|3.561757|3.613325|1.012722|[0.988869,1.026852]|6|

Gate requirements were declared before capture: the primary equal-log A/B2 ratio at 32 workers must have upper bound <1 and at least 10 wins, with every context's upper bound ≤1.03. The aggregate and three context bounds fail: primary A-w32, B-w16 and B-w32. Ranks 4–12 of 15 provide nominal 96.484375% marginal median intervals, without familywise coverage. Marginal time medians need not divide into paired estimates. There is no minimum gain floor, exclusion, extension, favorable subset, parameter sweep or unchanged rerun. The 2×2 bypass observations measure separate linked wrappers/classifier code; their changes do not establish coalescing gains. No previously frozen complete-query caller exists for this diagnostic API, so OFF uses fresh original-source bodies. This is not accepted-SDK calibration.

## CPU service, faults and process memory

| Shape/query | Workers | OFF/candidate CPU ms/query | OFF/candidate peak RSS MiB | OFF/candidate median minor faults/query | OFF/candidate median voluntary switches/query |
|---|---:|---:|---:|---:|---:|
|1x2/A|1|14.652/13.625|10.25/10.25|0.10/0.35|2.00/2.00|
|1x2/A|16|16.930/15.647|17.25/19.00|22.85/26.00|65.95/63.20|
|1x2/A|32|24.899/23.077|24.00/24.50|30.05/39.30|83.20/77.40|
|1x2/B|1|37.232/37.516|15.75/16.25|0.10/0.60|2.00/2.00|
|1x2/B|16|44.899/45.207|26.75/26.00|33.60/27.45|120.90/112.05|
|1x2/B|32|70.769/70.941|32.75/32.50|42.45/31.30|202.00/209.60|
|2x2/A|1|14.542/14.634|13.75/13.50|0.10/0.10|2.00/2.00|
|2x2/A|16|17.804/17.857|22.25/22.25|25.60/25.95|61.95/62.35|
|2x2/A|32|27.250/27.110|27.50/28.50|52.45/53.45|91.15/89.60|
|2x2/B|1|39.814/39.754|27.75/28.00|0.10/0.10|2.00/2.00|
|2x2/B|16|48.122/48.330|43.25/43.75|28.10/25.70|113.25/116.50|
|2x2/B|32|76.142/75.915|54.25/46.25|29.50/25.95|194.40/196.70|

A reduces aggregate CPU service at 32 workers even as latency worsens. B2's CPU service is nearly unchanged while latency increases. Total work and latency therefore move differently in this implementation. Additional serial preparation, sparse-span overhead, task shape and arithmetic batch changes are possible contributors; these observations do not isolate their causal costs. Faults are per-process minor faults over the warm timed loop; they do not establish swap reads or a RAM capacity failure. RSS is process high-water sampled at loop end and includes loading/primes. It does not measure a complete prover peak, incremental overlay cost or minimum RAM. Changes are mixed, including the 2×2 bypass.

## Automatic warm counters and profiles

After all latency measurements: eight FIFO-gated grouped stat processes, then eight separate `cycles:u` recordings with a fixed 16-million-cycle period. Each uses primary A/B2 at 16/32 workers, with 200 timed queries after three excluded primes. All four grouped events qualify at 100% reported coverage. These are descriptive panels, without paired counter confidence intervals.

| Query/workers/arm | Cycles M/query | Instructions M/query | Generic cache misses M/query | Branch misses M/query |
|---|---:|---:|---:|---:|
|A/16/control|85.868|208.530|0.241472|0.138591|
|A/16/candidate|78.829|191.746|0.220009|0.136241|
|A/32/control|115.203|214.512|0.303775|0.164387|
|A/32/candidate|105.145|197.221|0.286870|0.160945|
|B/16/control|223.507|550.401|0.868562|0.269236|
|B/16/candidate|225.938|548.707|0.749499|0.319765|
|B/32/control|318.041|568.538|1.280208|0.319830|
|B/32/candidate|320.575|567.753|1.248271|0.374304|

At 32 workers, A uses 8.73% fewer cycles and 8.06% fewer instructions in the separate counter panel; B2 cycles increase 0.80% and instructions fall 0.14%. B2 branch misses increase 17.03%, while generic cache misses fall 2.49%. None identifies a cache level, DRAM bandwidth or an exclusive critical-path fraction. The low retained payload does not guarantee lower latency.

| Profile | Samples | Unknown share | Lost |
|---|---:|---:|---:|
|A/16/control|1063|1.129%|0|
|A/16/candidate|978|0.204%|0|
|A/32/control|1414|0.849%|0|
|A/32/candidate|1287|1.010%|0|
|B/16/control|2782|0.863%|0|
|B/16/candidate|2820|0.993%|0|
|B/32/control|3973|1.007%|0|
|B/32/candidate|3990|1.579%|0|

All 18,307 samples have the fixed 16-million-cycle period, expected executable PID, registered worker/caller TID, worker FORK/task evidence, matching binary MMAP and permitted CPU. All recordings have zero lost records. Candidate A-w16 has 978 samples and is retained with that limitation. Profiles remain dominated by the same stream/apply, reduction and inverse kernels; G2 `coalesce_run` is visible. Symbol shares include inlined work and are not removable wall time. Absolute sampling coverage is not validated.

## Validation and retained failures

The 177-file private snapshot has 170 byte-identical original files, six append-only parents with exact prefix recovery, and one private child. All 176 production files and field arithmetic remain unchanged. Three newly executed focused native tests pass (125 unaffected core tests filtered): original-classifier equivalence, mapped-span point ownership, complete weighted-generator results, optional-allocation denial/fallback, identity/cancellation/doubling exceptions, ragged tile/partition boundaries, and repeated 1/4/16/32-worker queries on 512 KiB stacks. Existing field/core qualification is inherited. This trial does not qualify portable targets, full proofs, huge circuits or concurrent proofs.

All four real populations bind to independent complete Arkworks MSM oracles. Two new A references were needed because the older A ordinary-window bucket checks omit top and final weighting. B2 full references are reused byte-for-byte. No new witness solve, proof or Gnark export occurs. All 400 caller processes check three primes, every timed result and one post: 12,024 complete weighted-query outputs. Inputs stay unchanged, routes/counts/payloads stay stable, and worker affinity is checked. Source, library, driver, oracle, fixture and protocol pins match.

An initial symbol audit expected an out-of-line preparation symbol, which had been inlined; it stopped before any caller ran. Only the audit expectation changed to the linked classifier symbol; runtime routes provide activation evidence. The initial helper/log are retained. The native five-step, consumer four-step and capture 440-step workflows each finish once. Eight planned ownership commands affect only generated `perf.data` before 32 read-only decoders. No capture, test or oracle is repeated.

The first offline reviewer incorrectly assumed `link-sdk` plan entries contain `argv`, as command entries do. Its failed source/log remain; the corrected reviewer checks generated rustc arguments, library/driver identities and flags separately. This bookkeeping repair changes no capture or candidate. Successful review independently checks 449 raw command/exit/stream receipts, 1,505 pin entries, original source recovery, all route/payload/output receipts, paired wall/CPU/RSS statistics, all counter values, every sample and symbol bin, exact executable PID/TID/CPU/MMAP/FORK evidence and the failed gate. It reruns no query, oracle or proof.

## Accepted proof comparisons — inherited, unchanged

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge 24→1|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Accepted merge 54→1 is 161.883899 ms without a supplied M5 reference. Accepted primary process RSS is 132.75 MiB. The [adoption report](../2026-10-30-g2-vector-sign-adoption176/README.md) retains its physical-16 calibration engineering exception and qualification scope. No new memory or platform result follows.

Latest AWS primary remains historical source 175 on c7a.24xlarge: Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96, each campaign's best tested configuration. That does not compare current source 176. No AWS resources were created here.


All eight CPU 10× targets remain unmet. No SDK/full-proof trial follows this failed query gate. This exact sparse overlay is closed for adoption at high worker counts; its solo/16-worker A gains are retained as evidence. A future trial needs a distinct measured operation or dependency reduction. No new AWS resources, Aeglos comparison, minimum RAM, phone/ARM/TEE, swap, huge-circuit or throughput result follows.

[Design](artifacts/design.md), [prospective capture protocol](artifacts/capture-design.md), [all observations](artifacts/analysis.json), [symbol profiles](artifacts/symbols.json), [independent review](artifacts/review.json), [source audit](artifacts/source-audit.json), [binary identities](artifacts/identities.json), [linked-code check](artifacts/codegen.json), [fixture/oracle provenance](artifacts/fixtures.json). Archives: [private source](artifacts/source.tar.gz), [helper/protocol sources](artifacts/workflow.tar.gz), [all raw command receipts and counter CSVs](artifacts/receipts.tar.gz), [disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS).

Private point/scalar/key arrays, executables, libraries and perf.data stay local with retained hashes. No witness-derived output point is published. [Accepted implementation and qualification](../2026-10-30-g2-vector-sign-adoption176/README.md).
