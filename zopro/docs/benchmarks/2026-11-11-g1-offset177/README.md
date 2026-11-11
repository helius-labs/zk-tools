# G1-only affine offsets: full SDK comparison

The candidate is **not adopted**. Both primary 32-worker gain checks fail; the other 29 latency, CPU and RSS checks pass. At 32 workers, accepted/OFF/ON process-mean medians are **7.799795 / 7.852464 / 7.856330 ms**. Paired ON/accepted is **1.009317 [1.003931, 1.027342]**; ON/OFF is **0.998346 [0.925876, 1.001169]**. The latter does not establish a gain. All backing copies and outcomes remain included.

The [earlier combined G1/G2 trial](../2026-11-08-affine-redc-offset-mask-callers176/README.md) remains failed. Its favorable G1 observations motivated a separate candidate and fresh protocol; no subset became an adoption result. This screen is now closed, without sample extensions, exclusions or revised gates. Production is unchanged.

## Complete-proof results

Local Ryzen 9950X, 32 fixed workers on CPUs 0–31, except the separate primary 16-worker row below. Thirty paired blocks; each consumer has three independently written byte-identical executable files, ten blocks per copy per context. Values are medians of process arithmetic means. Primary processes use 101 measured requests; other shapes use 31, after three primes. Fresh assignment, solving, blinding, proving, SDK verification and serialization are included. Loading and export are excluded from latency.

M5 references are supplied historical measurements, not a fresh matched-platform run. Speedup columns list **accepted / candidate**. RSS lists maximum lifetime peaks, including loading and primes; these are not minimum RAM.

|Shape|Accepted ms|OFF ms|ON ms|Peak RSS MiB accepted / ON|M5 CPU / Metal ms|CPU speedup accepted / ON|Metal speedup accepted / ON|CPU 10× limit ms|
|:---|---:|---:|---:|:---|:---|:---|:---|---:|
|1x2|7.799795|7.852464|7.856330|131.75 / 129.75|37 / 24|4.744× / 4.710×|3.077× / 3.055×|3.7|
|2x2|10.996449|11.029784|11.045560|279.75 / 268.25|59 / 30|5.365× / 5.342×|2.728× / 2.716×|5.9|
|4x2|16.023394|16.016715|16.164684|312.75 / 310.00|109 / 48|6.803× / 6.743×|2.996× / 2.969×|10.9|
|8x2|24.421907|24.407962|24.344066|505.50 / 510.50|189 / 74|7.739× / 7.764×|3.030× / 3.040×|18.9|
|12x2|34.170884|34.160318|34.241138|713.00 / 721.25|284 / 101|8.311× / 8.294×|2.956× / 2.950×|28.4|
|16x2|40.248514|40.385181|40.147881|853.00 / 835.50|335 / 124|8.323× / 8.344×|3.081× / 3.089×|33.5|
|24x2|66.491489|66.408064|66.358255|1319.60 / 1322.75|528 / 176|7.941× / 7.957×|2.647× / 2.652×|52.8|
|merge-24x1|65.769007|65.933755|65.774718|1190.38 / 1194.27|507 / 175|7.709× / 7.708×|2.661× / 2.661×|50.7|

At 16 workers, 1×2 accepted/OFF/ON is **9.147203 / 9.219746 / 9.172213 ms**, with lifetime peaks **86.00 / 87.75 / 87.00 MiB**. Merge 54×1 at 32 workers is **163.623967 / 163.250007 / 163.663848 ms**, with peaks **2015.49 / 1967.18 / 2031.59 MiB**; no M5 reference was supplied for that shape. These are fresh local observations. They add no AWS or Aeglos measurement. **All eight CPU 10× targets remain unmet (0/8).**

## Decision and executable backings

The prospective screen requires primary 32-worker latency upper < 1 against both accepted and OFF, other nine latency uppers ≤ 1.02 against accepted, ten CPU uppers ≤ 1.02 and ten RSS guards ON ≤ accepted × 1.05 + 1 MiB. There is no minimum gain floor. Other OFF comparisons are diagnostic, not extra vetoes. Bounds use 30 sorted ratios at ranks 9/20, with the average of ranks 14/15 as median; nominal coverage is **95.722605% marginal**, not simultaneous or a stationarity guarantee. Two integer coverage models and a separate selection-based statistics reader agree.

The predeclared backing-spread flags trigger: primary 32-worker between-copy median ratios span factors **1.0250 against accepted** and **1.0980 against OFF**. Three copies do not guarantee stable performance.

|Copy pair|Primary32 paired ON/accepted median|Primary32 paired ON/OFF median|
|---|---:|---:|
|0|1.005054|1.004759|
|1|1.029726|0.915120|
|2|1.004567|1.000120|

These strata are descriptive, with no per-copy confidence claim. Neither the faster copy nor the apparently large ON/OFF gain in pair 1 qualifies the arithmetic. The screen establishes no useful primary gain; it does not isolate a source-only cause for every observed gap.

## Automatic CPU profiles

A separate frozen panel measures primary 32-worker on all three retained OFF/ON copy pairs. Each block uses the same copy for plain, grouped counters and cycle sampling, 251 requests after three primes. All recording precedes decoding and verification. Four observer guards pass (median instrumented/plain ratios 1.0038–1.0099), as do all six counter and six profile-quality checks. The six profiles have **77,648 samples**, zero LOST records, maximum unknown share 1.864%, and checked actual executable/PID/TID/CPU attribution. Counter running coverage is at least 99.9%. These guards do not establish unbiased observation.

|Copy|OFF / ON instructions per proof, millions|OFF / ON cycles per proof, millions|OFF / ON G1 stream samples|OFF / ON G2 borrowed samples|
|---|:---|:---|:---|:---|
|0|1370.700 / 1366.054|802.960 / 813.681|2638 / 2589|2007 / 2033|
|1|1371.217 / 1365.679|901.853 / 815.891|2674 / 2617|3564 / 2017|
|2|1370.392 / 1364.838|807.141 / 804.450|2593 / 2609|2017 / 2038|

ON executes about 0.34–0.41% fewer total instructions, but cycles do not improve consistently. OFF copy 1's extra residence is concentrated in G2 borrowed apply: its G2 samples are much higher than OFF copies 0/2, while G1 samples are similar. Those OFF executables have identical bytes. This is evidence of a backing-associated measurement effect, not a demonstrated hardware mechanism. Equal sampling periods/windows aid comparison but do not make symbol samples exclusive wall time or removable latency. One profile block per copy is not a precise per-copy performance estimate.

## Correctness, resources and provenance

The G1-only candidate stores canonical bx instead of xsum and folds coordinate offsets into multiplication normalization. Five-field G1 scratch remains 1,600 bytes with the same allocation structure. G2, inversions, FFT, reduction helpers and scheduling retain accepted source. There are no new key tables, retained plans, production pools or architectural RAM caps. 130 native core tests, two prover unit tests, six cubic tests and 97 generic core tests passed in preparation; source review checks 19 inherited field inputs and reverses G1 changes to accepted hashes. The initial portable report reader's 102-versus-97 count error and repair remain documented in the retained build review; no test was rerun.

Preflight/main/profile complete **47,880 SDK calls, 45,045 measured samples and 945 distinct fresh Gnark-verified exports**. Public/image bindings, proof freshness, worker placement, command/stream receipts and all pinned inputs pass. Canonical count advances 52,000 → 52,027 → 52,927 → **52,945**. All nine executable-copy SHA/device/inode/mode identities remain unchanged. The independent main reader reconstructs 900 process means and all 31 decisions from raw output using a separate rank implementation; it is a same-agent cross-check, not an external audit.

Accepted SDK SHA256: `c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`. OFF: `90f1d8fbd8bd533daa4fcd572f66764a71266de378c17112f4074d5814ace4c3`. ON: `0db7b2530991766c7d5b6436b89d759a9f258690aa60f625bc70ea5bc265108e`. OFF/ON are coherent Rust 1.98.1 znver4 fat-LTO consumers from the frozen G1-only source. All successful builds, tests and captures are retained without replay.

No phone/ARM/TEE, huge 250/4999 circuit, swap/pressure, minimum-RAM or concurrent-throughput qualification is added. The host was shared and frequency was not locked. Profiling and analysis did not overlap timing. No AWS resources were created. No push or publication is approved.

## Next work and retained evidence

Keep this exact screen closed and the candidate unadopted. Before another tiny arithmetic experiment, investigate why byte-identical executable backings change G2 cycles. A bounded read-only examination of mapped code/constant page placement is a distinct possible diagnostic; it has not been run here. Do not infer a cache-coloring cause or adopt a fastest-file workaround from these observations. The broader ALL8 CPU 10× objective remains active.

[Full decisions](artifacts/assessment.json), [independent CPU/Metal comparisons](artifacts/independent-main-review.json), [screen protocol](artifacts/screen-design.md), [profile results](artifacts/profile-result.json), [profile protocol](artifacts/profile-protocol.md), [resource rows](artifacts/resource-rows.json), [build review](artifacts/build-review.json), [source review](artifacts/source-review.json), and [artifact index](artifacts/final-artifacts.json) retain the evidence. Archives contain raw driver/verification receipts, public proof exports, samples, decoded profiles and helper sources; no private key/image/witness payload or executable library is included. Raw perf.data and exact SDKs remain local with hashes. Reproduction requires the pinned dependencies and private inputs.

Private roots: `/home/ubuntu/src/zopro-perf-work/g1-offset177/` and `/home/ubuntu/src/zopro-perf-work/g1-offset-profile177/`. All work is local only.
