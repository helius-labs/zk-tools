# Fused epoch handle access in complete SDK proofs

The first optimized full-SDK trial establishes **no gain**. The candidate measures **8.043005 ms** on 1×2, versus **8.028460 ms fresh OFF** and **8.018344 ms accepted**. Paired candidate/OFF is 0.999186 [0.994622, 1.003655]; candidate/accepted is 1.005235 [1.001722, 1.008654]. The full declared screen fails. Production remains unchanged and this exact runtime integration is closed.

## Change and scope

The candidate imports the previously qualified private Crossbeam epoch 0.9.21/deque 0.8.8 change: combine the single-job steal's is_pinned/fence/pin operations inside one thread-local with_handle callback. The original nested SeqCst fence, unpinned epoch CAS, guard lifetime/drop, steal CAS and collection cadence remain. Victim selection, retries, sleep/wake policy, pools, task graph, cryptographic arithmetic, key representation and resource policy retain their existing source. No table, key copy, retained plan or new persistent allocation is introduced. Compiler layout and stack use can change.

This trial is an explicit engineering exception to the earlier [query-component stop rule](../2026-10-31-fused-epoch-pin176/README.md), selected before the new builds and clocks. That query's final driver accidentally omitted its intended release profile; its positive fresh comparison and failed calibration/gate remain recorded. Its result is not promoted or rerun. This is the first test of that integration in the actual optimized SDK. The [compiler-profile correction](../2026-10-31-sdk-link-flags176/README.md) remains separate evidence.

All 176 accepted production identities remain exact. In the private snapshot, 174 originals remain byte-identical; only Cargo patch/lock metadata change, and both originals recover exactly. All 43 imported dependency files match historical build pins; 25 Rust files also match the sources used by the successful inherited dependency tests. Existing dependency lifetime/unwind/TLS-destruction qualification and field qualification are inherited, not rerun. Five historical ignored doctests are not passes. New serial integration qualification passes **125 core and 2 prover-library tests**. Generic/no-ASM/no-IFMA compilation passes; this is not phone or ARM execution.

Both SDKs use coherent eight-library builds and actual final O3/native/fat-LTO/one-CGU links. The new linked control has two steady-state handle checks across single-steal and its pin helper; the candidate has one with inlined pinning. Required fence/CAS/drop operations remain visible. Single-steal grows 356→785 bytes; the control additionally calls a 489-byte pin helper. These local sizes are not whole-program size or stack bounds. Both signed borrowed G2 consumers retain 51,862 bytes, 8,591 instruction sites and 1,450 IFMA sites.

## Fixed complete-proof measurements

Ryzen 9 9950X, 16 physical cores/32 logical CPUs, shared host. Fifteen fixed rotated/reversed blocks compare unchanged accepted SDK `6d83`, fresh OFF `861a` and candidate `6640`. All nine shapes use 32 workers on CPUs 0–31; primary also uses 16 workers on CPUs 0–15. Each process primes three requests, then measures 151 primary or 31 other requests. Latency includes fresh solving, randomized proving, internal verification and serialization; key loading is excluded. No owned build, analysis, hashing, report or other benchmark overlaps recording.

Times below are medians of process means. Ratios are geometric means of paired block ratios, with fixed 20,000-resample bootstrap intervals; they need not equal ratios of displayed medians.

|Context|Accepted ms|OFF ms|Candidate ms|Candidate/OFF [95%]|Wins/15|
|---|---:|---:|---:|---|---:|
|1x2-w32|8.018344|8.028460|8.043005|0.999186 [0.994622, 1.003655]|7|
|2x2-w32|11.241463|11.294369|11.280064|0.999014 [0.994717, 1.003359]|8|
|4x2-w32|15.933816|16.122477|16.212378|1.005572 [0.998116, 1.012345]|4|
|8x2-w32|24.234311|24.240419|24.237384|0.998890 [0.993439, 1.004681]|9|
|12x2-w32|33.855565|34.151532|33.892037|0.996021 [0.985368, 1.008257]|12|
|16x2-w32|40.071563|40.257905|40.144250|0.999037 [0.990944, 1.007835]|9|
|24x2-w32|65.917462|66.169615|66.201829|1.002701 [0.997122, 1.008872]|7|
|merge-24x1-w32|65.305023|65.455125|65.222997|0.997845 [0.992677, 1.002763]|8|
|merge-54x1-w32|162.240109|161.559386|162.460137|1.004442 [1.000709, 1.007661]|4|
|1x2-w16|9.488278|9.638419|9.684371|0.985300 [0.935064, 1.031300]|9|

The original-eight aggregate is **0.999779 [0.996876, 1.002492] versus OFF**, 7/15 wins, and **1.004035 [1.001448, 1.006742] versus accepted**, 2/15. The candidate/OFF comparison does not establish either a gain or regression. The accepted comparison is slower. Primary gain and both aggregate-gain checks fail. 4×2 fails candidate/accepted nonregression. Primary16 fails control calibration and both nonregression ceilings; its wide intervals remain in the evidence and do not establish a precise performance sign. Eight failed checks are retained.

The prospective selection requires primary32 and all-eight aggregate upper ratios below 1 against both controls, every context's upper ratio ≤1.02 and every fresh/accepted calibration within factor 1.03. No minimum gain floor, excluded blocks, extensions, subset promotion or unchanged retry is used. No new implementation is selected by a favorable point estimate.

## CPU profiles and memory observations

All nine PMU panels and nine cycle profiles pass the declared quality checks: at least 99.9% counter running time, at least 1,000 samples/profile, unknown ≤5%, zero LOST events, recorded process/thread/CPU/MMAP ownership and FIFO boundaries. Total profile samples are **90,419**. Flat sampled counts include inlining and sampling effects; they are not exclusive costs, retired instructions or critical-path latency.

At primary32, symbols under `crossbeam_epoch` fall from 1,219 OFF samples to 170 candidate samples, while Stealer/steal symbols rise 34→983. The combined descriptive group changes 1,253→1,153 and whole scheduler categories 1,628→1,543. Pinning was inlined into steal: the disappearing `with_handle` entry does not represent eliminated pinning work or a corresponding proof-time saving. G1/G2 borrowed functions remain the largest sampled arithmetic consumers.

One-shot primary32 PMU panels record OFF→candidate instructions 1.387170→1.376030 billion/request, cycles 816.455→816.181 million/request and branch misses 899,592→935,420/request. Instructions decrease without a demonstrated latency gain. These panels are not replicated causal estimates, and generic cache misses do not identify a memory level or DRAM bottleneck.

|Context|Accepted / OFF / candidate maximum recorded process RSS, MiB|
|---|---:|
|1x2-w32|135.00 / 135.00 / 133.50|
|1x2-w16|89.00 / 89.00 / 90.00|
|24x2-w32|1317.79 / 1292.00 / 1292.86|
|merge-54x1-w32|1801.40 / 1861.21 / 1838.37|

Primary32 median minor faults/request are 130.636/130.225/127.967, accepted/OFF/candidate. RSS includes loading, primes and allocator history; these observations are not minimum RAM, a complete lifetime peak, pressure/swap or concurrent-proof qualification. Fault counts do not measure fault service time or disk paging. All context CPU/RSS/fault summaries are retained.

## Verification and provenance

The six-step native workflow, 44-step preflight and 981-step measurement/verification workflow all complete. The independent reader checks **1,031 workflow receipts**, eight saved codegen-command receipts, 2,474 pins, source recovery, **26,514 raw measured SDK samples**, all 32 paired contrasts and bootstrap intervals, 30 context/arm resource summaries and 36 raw PMU event rows. Its separate one-step xtask run succeeds. It does not replay native tests, cryptographic verification or decoders.

Preflight contributes 88 SDK requests/22 fresh independent Gnark exports; the campaign adds 27,918 requests/468 exports. Total **28,006 requests and 490 unique fresh exports** pass image/public/VK/proof bindings and canonical freshness checks. Canonical verified exports advance 34,439→34,461→**34,929**. Proofs are 164 bytes/public 44 bytes. Keys, requests, witnesses, proof/public payloads, executables, libraries and raw binary perf files stay local with hashes.

The first preparation helper used a nonexistent historical review filename after copying source and before builds/tests/clocks. Its continuation verifies existing copied bytes, consumes the actual review.json and completes only unstarted preparation. The original helper and failure note remain. No successful native test, build, capture, model or verifier is repeated for bookkeeping.

## Supplied M5 comparisons

These are fresh local accepted/candidate observations against supplied historical timings on different hardware. Ratios do not establish equal hardware efficiency. Earlier sub-8 ms observations use the same accepted binary; this trial adopts no slower replacement.

|Shape|Accepted ms|Candidate ms|M5 CPU ms|M5 Metal ms|CPU speedup accepted/candidate|Metal speedup accepted/candidate|CPU10× limit ms|
|---|---:|---:|---:|---:|---|---|---:|
|1x2|8.018344|8.043005|37|24|4.614× / 4.600×|2.993× / 2.984×|3.7|
|2x2|11.241463|11.280064|59|30|5.248× / 5.230×|2.669× / 2.660×|5.9|
|4x2|15.933816|16.212378|109|48|6.841× / 6.723×|3.012× / 2.961×|10.9|
|8x2|24.234311|24.237384|189|74|7.799× / 7.798×|3.054× / 3.053×|18.9|
|12x2|33.855565|33.892037|284|101|8.389× / 8.380×|2.983× / 2.980×|28.4|
|16x2|40.071563|40.144250|335|124|8.360× / 8.345×|3.094× / 3.089×|33.5|
|24x2|65.917462|66.201829|528|176|8.010× / 7.976×|2.670× / 2.659×|52.8|
|merge-24x1|65.305023|65.222997|507|175|7.764× / 7.773×|2.680× / 2.683×|50.7|

Accepted and candidate meet **0/8** and **0/8** CPU targets. The original verified all-eight CPU 10× objective remains active and unmet. Merge54 has no supplied M5 reference. This trial adds no AWS/Aeglos, mobile/ARM/TEE, large 250/4999, pressure/swap, minimum-RAM or concurrent-throughput comparison.

[All context results](artifacts/results.json), [independent review](artifacts/review.json), [decision](artifacts/decision.json), [CPU/Metal data](artifacts/m5-comparisons.json), [profile attribution](artifacts/profile-attribution.json), [full analysis, raw receipts, frozen source/helpers/plans/pins and inherited dependency evidence](artifacts/evidence.tar.gz), [archive member hashes](artifacts/archive-index.json), [checksums](SHA256SUMS). The compact results omit only the raw-row array; the archive retains the complete original JSON and streams byte-for-byte. Canonical ledgers and excluded binary/private payloads remain local with hashes.

Private benchmark root: `/home/ubuntu/src/zopro-perf-work/fused-epoch-sdk176`.
