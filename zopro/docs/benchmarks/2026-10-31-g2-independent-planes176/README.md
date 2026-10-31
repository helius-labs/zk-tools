# Independent G2 component and range fills

**The complete-query gate fails; this candidate is not adopted.** Primary B2 with 32 workers is 3.00% slower than fresh OFF: paired ratio 1.030017 [1.010084, 1.050175], 0/15 wins. It also fails the original frozen-caller gain check. The accepted prover and its 7.928660 ms primary proof result remain unchanged.

## Change and scope

The private source-176 candidate retains the two existing point ranges inside each of the two phi scalar components. Four independent ordinary bucket sets replace two sets that each feed high then low. The two high partitions merge first; phi applies once to that merged bucket set. The two low partitions merge, then high and low merge before the original weighted reduction and Horner recombination. Width 12, scalar decomposition, narrow indexed top, grouping, field arithmetic and original borrowed pending/exception handling remain.

This differs from the [shared two-plane state](../2026-10-28-dual-plane-g2-174/README.md) and [two component tasks over full inputs](../2026-10-29-component-partition175/README.md). It removes repeated bucket transforms and exposes more independent fill work, while adding bucket initialization, normalization and merging. A generic point-addition saving was not assumed. No transformed input cache, persistent key table, duplicate solver plan, point-array copy or extra pool is introduced.

One checked conservative full-candidate scratch reservation covers four per-window output sets, all four top-parent envelopes and per-worker leaf scratch using existing typed workspace formulas. The original proof forecasts remain charged. The token stays live through every consumer. Denial before decomposition leaves scalar words unchanged and selects the original path. There is no new fixed RAM cap; admission accounting and process RSS are not hard memory or minimum-RAM guarantees. The diagnostic route counter records actual entry after admission; ON takes route 1 for every measured B2 query, with no silent fallback. A retains route 0. Metadata payload zero denotes no overlay payload, not zero temporary bucket storage.

## Fixed query measurements

Ryzen 9 9950X, 16 physical cores/32 logical CPUs. Frozen is the [corrected optimized original-query caller](../2026-10-31-sdk-link-flags176/README.md), SHA 0ba4cd4e…; OFF and ON share one fresh release binary, SHA ec1f0d11…. The final driver source is exact across controls. Its actual compiler arguments include -O, native CPU, fat LTO and single codegen unit, with SHA2 passed through additional_rustc_flags. Backend libraries are coherent native release builds.

All four original point/scalar fixtures and full independent Arkworks references are reused exactly. Custom Rayon workers have 512 KiB stacks and individual CPU affinity: CPU 2 solo, CPUs 0–15 or 0–31. Complete query timing includes classification, scalar reset for B2, grouping, decomposition, fills, narrow top, merges, weighting, Horner, normalization, allocation/drop and dispatch/join. Loading, pool creation and oracle checking are outside. Every timed output is retained and checked afterward, alongside three checked primes and one checked post; points remain unchanged.

Fifteen fixed paired blocks rotate/reverse cases and variants. Each process has 20 timed queries. All 540 latency processes finish before diagnostics. Ranks4–12 give nominal 96.484375% marginal median bounds under exchangeable independent blocks, without familywise coverage. The gate requires primary B2/32 and the primary equal-log A/B2 aggregate to improve against both controls (upper<1 and at least 10/15 wins), every context upper≤1.03 against both, and every OFF/frozen interval within factor1.03. No gain floor, excluded observation, favorable subset, extension, parameter sweep or unchanged reroll.

Times below are medians of process means. Ratios use within-block pairs and need not equal ratios of marginal medians. Bounds and wins are shown for both candidate comparisons.

|Context|Frozen ms|OFF ms|ON ms|ON/OFF ratio [bounds], wins|ON/frozen ratio [bounds], wins|
|---|---:|---:|---:|---|---|
|1x2-A-w1|14.587451|14.546907|14.563149|1.000253 [0.998222, 1.002881], 6/15|0.998354 [0.994588, 1.000537], 11/15|
|1x2-A-w16|1.534813|1.534665|1.531154|0.997385 [0.993495, 1.002743], 11/15|0.996377 [0.992864, 1.001018], 9/15|
|1x2-A-w32|1.206281|1.201572|1.212182|1.004524 [0.986559, 1.032784], 6/15|1.004892 [0.972804, 1.018262], 7/15|
|1x2-B-w1|37.338788|37.696657|38.066930|1.010741 [1.008782, 1.012285], 0/15|1.020255 [1.019281, 1.022166], 0/15|
|1x2-B-w16|3.682039|3.668813|3.715807|1.015280 [1.006521, 1.034611], 2/15|1.013596 [1.007785, 1.030405], 2/15|
|1x2-B-w32|3.280211|3.254439|3.377978|1.030017 [1.010084, 1.050175], 0/15|1.037972 [0.993984, 1.045443], 5/15|
|2x2-A-w1|14.571473|14.527647|14.525416|1.000272 [0.997394, 1.003024], 7/15|0.995184 [0.991979, 1.000151], 11/15|
|2x2-A-w16|1.885629|1.879090|1.876900|1.003537 [0.977980, 1.036736], 7/15|0.995371 [0.992363, 1.003311], 9/15|
|2x2-A-w32|1.545825|1.529042|1.529703|1.005781 [0.979368, 1.040364], 7/15|0.973718 [0.964059, 1.018033], 11/15|
|2x2-B-w1|39.978123|40.209015|40.472197|1.007159 [1.005900, 1.009596], 0/15|1.012625 [1.011947, 1.014758], 0/15|
|2x2-B-w16|4.206502|4.236150|4.364761|1.030241 [1.022980, 1.032940], 0/15|1.034970 [1.029652, 1.044442], 1/15|
|2x2-B-w32|3.897543|3.859740|4.029958|1.051468 [1.024383, 1.069966], 1/15|1.045327 [1.024723, 1.076543], 0/15|

Primary equal-log A/B2 at 32: ON/OFF **1.024899 [1.004861, 1.034240], 3/15**; ON/frozen **1.010802 [0.995065, 1.022498], 5/15**. All declared checks and both control ratios remain in the machine-readable analysis and independent review. Calibration fails for 2×2 A at 16/32; those unchanged-A contexts remain in the gate. Even apart from calibration, primary B2 loses every paired block against fresh OFF and fails its gain check. The small negative-control positive estimates are not algorithm gains.

## CPU and memory

|Context|Frozen/OFF/ON CPU ms/query|Frozen/OFF/ON max process RSS MiB|
|---|---:|---:|
|1x2-A-w1|14.587/14.546/14.563|9.50/9.50/9.50|
|1x2-A-w16|17.005/16.944/16.956|18.25/18.75/18.25|
|1x2-A-w32|25.024/24.958/25.000|27.25/24.25/24.00|
|1x2-B-w1|37.338/37.696/38.066|15.50/15.50/15.50|
|1x2-B-w16|44.928/45.037/46.643|27.00/27.75/33.25|
|1x2-B-w32|70.637/70.210/78.167|34.25/34.25/40.25|
|2x2-A-w1|14.571/14.528/14.524|12.75/12.75/12.75|
|2x2-A-w16|17.651/17.661/17.615|22.00/23.00/22.50|
|2x2-A-w32|26.357/26.315/26.300|30.00/30.00/29.25|
|2x2-B-w1|39.978/40.208/40.472|27.25/27.25/27.25|
|2x2-B-w16|48.230/48.379/50.685|43.75/36.50/49.75|
|2x2-B-w32|76.723/76.086/86.364|60.50/60.75/67.50|

Primary B2/32 CPU rises 70.210→78.167 ms/query, while maximum lifetime process RSS rises 34.25→40.25 MiB versus OFF. At 2×2 B2/32 these are 76.086→86.364 ms and 60.75→67.50 MiB. RSS includes input loading, expected arrays and primes; it is not a full prover peak or minimum RAM. Raw minor faults and context switches remain available. No cause is inferred from fault counts or RAM capacity alone.

## Automatic counters and profiles

After all latency, 12 FIFO-gated four-event counter panels and 12 separate fixed 16-million-user-cycle profiles run 200 queries after 3 excluded primes. Every event reports 100% running coverage. These single panels are descriptive, without paired counter confidence intervals.

|Panel|Cycles M/query|Instructions M/query|Cache misses M/query|Branch misses M/query|Samples|
|---|---:|---:|---:|---:|---:|
|stat-1x2-A-w16-candidate|85.447|207.435|0.234030|0.137398|1058|
|stat-1x2-A-w16-control|85.307|207.259|0.239267|0.138424|1063|
|stat-1x2-A-w16-frozen|85.542|207.354|0.239797|0.137241|1061|
|stat-1x2-A-w32-candidate|113.544|212.784|0.282477|0.166050|1412|
|stat-1x2-A-w32-control|113.396|212.619|0.284875|0.168035|1435|
|stat-1x2-A-w32-frozen|114.449|213.070|0.294418|0.165145|1413|
|stat-1x2-B-w16-candidate|231.647|548.251|0.874732|0.326603|2893|
|stat-1x2-B-w16-control|224.229|550.587|0.732599|0.264772|2797|
|stat-1x2-B-w16-frozen|223.601|549.781|0.744392|0.265215|2792|
|stat-1x2-B-w32-candidate|351.934|570.494|1.479963|0.382067|4377|
|stat-1x2-B-w32-control|319.864|568.562|1.060219|0.310373|4013|
|stat-1x2-B-w32-frozen|324.074|568.420|1.231465|0.312950|4010|

All 28,324 samples have expected PID, registered caller/worker TID, pinned worker CPU, binary MMAP, worker FORK evidence and fixed period; every recording has zero loss, at least 1,058 samples and less than1.31% unknown share. B2/32 cycles rise 319.864→351.934 million/query versus OFF; instructions rise 568.562→570.494 million. At 16 workers instructions fall slightly while cycles rise. More leaves and fewer transforms do not establish less CPU service or faster completion.

The candidate profile has more samples in Window::new and the separate G2 prefix-merge kernel. This is consistent with additional bucket initialization and merges, but symbol samples are affected by inlining and interleaving. The trial does not isolate a cache, synchronization, allocator, clock or dependency cause. Profile shares are not removable wall-time fractions, local CPI or absolute sampling-coverage evidence.

## Qualification and evidence

The private snapshot has 177 files: 171 byte-exact originals, five reversible parents and one new child. Independent reverse edits recover all 176 original sources. All field source is exact and inherits its qualification. The native run passes 128 unfiltered core tests, including 24 complete generator-sum cases across 1/4 workers, signed/full-width scalar families and lengths 1024/1025/4097/8193. Separate ordinary/top bucket oracles cover identities, equal/opposite points and signs. Admission denial preserves words; checked-cost boundaries and overflow are tested. Both raw-image prover tests pass. No fresh field, cubic-feature, portable or complete-proof qualification is claimed.

Source review corrected an unqualified type name before native compilation. An initial analysis-helper link omitted the demangler extern argument and was corrected before capture. Successful native tests, library freeze, final link and complete capture run once; no measured executable or threshold changes during the trial.

All 665 workflow commands complete: four native steps, one final link and 660 capture/decode steps. The 600 caller processes check 18,036 complete query outputs against existing independent full-MSM references. There is no new solve, oracle execution, SDK proof request, Gnark export or verification count. The proof ledger remains 27,353. Independent review checks 1,298 pin entries, original/private source recovery, actual compiler commands, every raw command/exit/stream receipt, every paired wall/CPU/RSS distribution and gate, all counter values and every profile bin and thread/mapping/loss receipt. No query or proof is rerun by review.

The candidate is closed as a negative result: repeated transforms were structurally removed, but the combined implementation increased service time and failed latency checks. Its unchanged recipe is not selected for full-proof timing. A next candidate needs a distinct reduction in dominant useful work; adding task parallelism alone has not established the required jump.

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

The original all-eight CPU 10× goal remains active and unmet, 0/8. No new full-proof speedup, AWS/Aeglos, mobile/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput result follows. No AWS resources were created. No production source or accepted binary changes.

[Protocol](artifacts/capture-design.md), [design](artifacts/design.md), [all observations](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [identities](artifacts/identities.json), [independent review](artifacts/review.json), [symbols](artifacts/symbols.json), [linked code](artifacts/codegen.json). Archives: [complete private source](artifacts/source.tar.gz), [workflow/helper source](artifacts/workflow.tar.gz), [raw command receipts and counter CSVs](artifacts/receipts.tar.gz), [selected disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS).

Local root: `/home/ubuntu/src/zopro-perf-work/g2-independent-planes176`. Executables, libraries, point/scalar fixtures, curve outputs and raw perf.data remain local; their hashes are retained.
