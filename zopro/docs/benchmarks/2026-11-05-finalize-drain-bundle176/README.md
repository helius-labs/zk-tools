# Low-memory finalization and queue-drain composition

The fixed composition improves primary32 against both controls, but **fails the declared full-matrix gate and is not adopted**. Primary ON measures **7.741618ms**, versus7.946445ms accepted and7.928521ms freshOFF. Paired ON/accepted improves2.875% (ratio0.971254, bounds[0.968506,0.975602]); ON/OFF improves2.565% (0.974347,[0.968656,0.978906]). Both comparisons win15/15blocks.

Two checks fail: physical16 fresh-control calibration and original-eight aggregate gain versus accepted. All individual context non-regression checks pass, as do the CPU-counter/profile qualification checks. The primary improvement is retained as measured evidence; it does not make the full gate pass or replace accepted performance. No original-eight CPU10×target is achieved.

## Fixed implementation and scope

This combines two previously implemented methods selected before new data: [GLV rerandomized final assembly](../2026-10-28-rerandomized-glv-sdk/README.md) and [guarded affine queue draining](../2026-10-29-queue-drain175/README.md). Both had positive fresh-build comparisons but failed their earlier complete qualification. Those failures remain unchanged; no component speedup is transferred or assumed additive. This is a new composed current-source SDK with one runtime selector, not a new arithmetic formula or a repeat of an unchanged standalone experiment. The compact weighted tree and late-M3geometry are excluded.

The finalizer reuses the exact previously tested signed scalar decomposition and endomorphism tables to shorten G1/G2 doubling chains. Guarded queue draining reuses original `take_from_queue` and affine batches when the existing SIMD-width eligibility guard permits, otherwise preserving scalar projective fallback. OFF andON use the same executable; default isOFF. The original timing body is byte-exact after reversing its main-function rename. The wrapper prints only selected configuration after that body returns; it is not a per-proof path counter.

No Window field, capacity, scalar grouping, MSM window geometry, FFT, solver, random sampler, proof distribution, key/image format or resource forecast changes. Field source is exact. There is no added proving-key table, duplicate key/solver plan, pool or global RAM cap. GLV image-array payloads512bytesG1/3072bytesG2are per proof, inherited from the parent method, and exclude compiler frames and other temporaries. No pressure/minimum-RAM or complete stack bound is claimed.

## Complete SDK matrix

Ryzen9950X:32logical CPUs0–31 for all9shapes, plus primary16physical CPUs0–15. Accepted is exactSDK `6d83…`; freshOFF/ON share one optimized native/fat-LTO/one-CGU SDK. Fifteen fixed rotated/reversed process blocks cover all10contexts×3arms. Each process runs3priming requests, then151timed primary32,301primary16, or31other requests. Timing includes witness solving, fresh randomized proving, SDK verification and serialization; loading/primes are excluded. Reported latency is median of15process medians. Paired ratios need not equal quotients of displayed medians.

|Context|Accepted ms|OFF ms|ON ms|ON/accepted median [bounds]|ON/OFF median [bounds]|
|---|---:|---:|---:|---:|---:|
|1x2-w32|7.946445|7.928521|7.741618|0.971254 [0.968506, 0.975602]|0.974347 [0.968656, 0.978906]|
|2x2-w32|11.171887|11.280951|11.200652|1.001990 [0.995200, 1.010039]|0.992431 [0.986288, 0.998685]|
|4x2-w32|16.192375|16.162028|16.207322|1.007925 [1.000268, 1.018970]|1.007108 [0.989705, 1.013633]|
|8x2-w32|24.115064|24.367030|24.389915|1.016260 [1.000197, 1.028010]|1.012816 [0.989554, 1.023890]|
|12x2-w32|33.887411|34.015655|33.932557|1.001805 [0.991969, 1.017553]|0.998135 [0.978982, 1.011123]|
|16x2-w32|40.253937|40.185885|40.196812|0.999626 [0.995382, 1.009165]|1.004851 [0.993393, 1.010508]|
|24x2-w32|66.033924|66.545265|66.388553|1.005370 [1.002478, 1.010629]|0.997324 [0.990305, 0.999653]|
|merge-24x1-w32|65.500856|65.505612|64.998533|0.995270 [0.987647, 0.999407]|0.994815 [0.988354, 1.008500]|
|merge-54x1-w32|161.708661|161.936375|162.157848|1.001036 [0.998008, 1.013825]|1.003248 [0.996225, 1.008988]|
|1x2-w16|9.554908|9.379266|9.263118|0.966654 [0.937293, 1.024293]|0.984418 [0.960924, 1.002702]|

|Original-eight equal-log aggregate|Median|Lower|Upper|Wins/15|Gain check|
|---|---:|---:|---:|---:|---|
|ON/accepted|0.999632|0.997276|1.003664|9|false|
|ON/OFF|0.996577|0.992890|0.999100|13|true|

The original-eight gain versusOFF is0.342%and passes its declared bound; versusaccepted the estimate is0.037%with bounds spanning parity and9/15wins, so it fails. Physical16OFF/accepted bounds[0.962040,0.989375] fall below the lower calibration threshold1/1.03. This is a faster fresh control outside the declared range, not evidence of a candidate slowdown or a newly adopted OFF optimization. Primary16ON/OFF also has a wide interval spanning parity; no strict16-worker gain is established.

Calibration requires every OFF/accepted interval inside[1/1.03,1.03]. Every context ON/control upper bound must be≤1.03; primary32 and original-eight aggregate additionally require upper<1 and≥10/15wins againstBOTHcontrols. Bounds are ranks4–12of15, nominal96.484375%marginal under independent/exchangeable blocks, without familywise coverage. All observations, including failed calibration and unfavorable contexts, remain. No sample extension, threshold waiver, component/shape selection or unchanged reroll follows. Passing this screen would only select further resource/compatibility/final-artifact qualification; it would not directly establish adoption or the10×goal.

## CPU service, memory counters and automatic profiles

|Context|Accepted CPU ms|OFF CPU ms|ON CPU ms|Accepted / OFF / ON process RSS MiB|
|---|---:|---:|---:|---:|
|1x2-w32|179.562|179.511|175.284|134.75 / 135.25 / 133.50|
|2x2-w32|258.659|260.735|261.261|276.50 / 274.50 / 272.25|
|4x2-w32|424.668|422.521|426.290|311.25 / 316.25 / 320.75|
|8x2-w32|668.913|674.954|675.011|514.25 / 504.75 / 508.50|
|12x2-w32|939.641|944.465|945.140|716.00 / 727.00 / 711.50|
|16x2-w32|1124.093|1127.414|1126.084|819.50 / 817.50 / 831.25|
|24x2-w32|1890.078|1905.124|1909.834|1290.05 / 1292.58 / 1309.00|
|merge-24x1-w32|1812.938|1816.613|1802.186|1164.71 / 1174.96 / 1183.26|
|merge-54x1-w32|4424.162|4438.786|4437.218|1846.91 / 1837.47 / 1857.09|
|1x2-w16|115.900|117.899|114.387|92.00 / 92.00 / 92.00|

CPU is summed user+system service per request, summarized across process means. RSS is maximum lifetime rusage counter among latency processes, including loading and primes. Earlier host audits found these counters can be below page-table snapshots; they are diagnostics, not minimum RAM, a rigorous lifetime peak, or a memory-pressure qualification. No requested counter ratio exceeds the diagnostic1.05warning threshold. No page-table resource matrix or concurrent throughput measurement is added.

All450latency processes finish before6grouped PMU panels and6separate16-million-user-cycle profiles, primary16/32acrossall3arms. Stats use31requests, profiles251, each after3primes excluded byFIFO. All measurements finish before remaining Gnark verification/decode/archive; no owned compilation, hashing, helper, verifier or other workload overlaps latency/profiles. The host is shared; exchangeability remains an assumption.

|Panel|Instructions M/request|Cycles M/request|Cache misses M/request|Profile samples|Unknown share|Qualified|
|---|---:|---:|---:|---:|---:|---|
|w16/accepted|1344.294|541.246|1.840343|8606|1.476%|true|
|w32/accepted|1389.096|816.393|2.702352|12407|1.588%|true|
|w16/OFF|1340.318|554.739|1.850916|8772|1.368%|true|
|w32/OFF|1383.099|813.471|2.684777|12401|1.814%|true|
|w16/ON|1300.807|548.857|1.855309|8503|1.305%|true|
|w32/ON|1340.210|789.786|2.876511|11930|1.903%|true|

At32workers, instructions drop1,383.099→1,340.210million and cycles813.471→789.786million versusOFF in the single diagnostic panels. Primary CPU service also falls179.511→175.284ms/request. Cache misses increase in that panel, so no cache/DRAM cause is inferred. These panels are single observations, not confidence intervals or a mechanism decomposition. All six profiles pass sample/unknown/CPU/one-executable-PID/TID/mmap/FORK/period/zero-loss checks; flat symbol weights include inlined work and retain unvalidated absolute-period coverage. The GLV-name matcher sees zero selected period in every profile; this narrow attribution does not measure zero execution. Source/configuration and correctness are checked independently. No claim assigns the gain to either ingredient separately.

## Correctness and provenance

OFF andON each pass128unfiltered core tests,2raw-image tests and6cubic prover tests. Three inherited queue tests adapt only their expected runtime selector condition and cover pending/queue boundaries, identity/opposite/doubling cases, signed destinations and complete generator-sum outputs. Existing rerandomization exponent/zero-A/zero-delta/prepared/unprepared oracles run in both modes. Field qualification is inherited, not rerun. The native link command explicitly contains-O,native CPU,fatLTO andoneCGU before capture.

All30preflight exports pass independent pinned Gnark and configuration checks before timing. All34,698SDKrequests verify internally. All492fresh164-byte exports pass independent Gnark, exact public/image bindings, common VK identity per shape and uniqueness against29,461prior proofs. Canonical distinct proof ledger becomes**29,953**. Exported public proofs are counted once; priming/requests are not independent export counts. No successful test/capture/verifier is repeated, and no repair is needed.

The private179-file snapshot has170of176originals byte-exact, six reversible parents andthreechildren. Reversing changes recovers all176accepted originals; field files, Window fields/capacity forecasts and required key data remain exact. Both algorithm children match the previous runtime sources; this is composition evidence rather than a new formula claim. The original SDK timing body recovers byte-exact. Independent review checks314pins,1,024successful command receipts/stream hashes, source recovery, raw sample medians/all paired gates/aggregate failures,492unique proofs and separately reparsed profile symbol counts.

The full gate remainsfalse and production is unchanged. The primary32gain is a useful result within this measured scope; the fixed combination is closed without adoption, ingredient tuning or a favorable-subset follow-up. No CPU10×target is met.

## Fresh comparisons against supplied M5 references

The M5 CPU/Metal values are supplied historical references. These rows use this experiment's ON observations, not adopted production timings.

|Shape|ON ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.741618|37 / 4.779x|24 / 3.100x|3.7|
|2x2|11.200652|59 / 5.268x|30 / 2.678x|5.9|
|4x2|16.207322|109 / 6.725x|48 / 2.962x|10.9|
|8x2|24.389915|189 / 7.749x|74 / 3.034x|18.9|
|12x2|33.932557|284 / 8.370x|101 / 2.976x|28.4|
|16x2|40.196812|335 / 8.334x|124 / 3.085x|33.5|
|24x2|66.388553|528 / 7.953x|176 / 2.651x|52.8|
|merge-24x1|64.998533|507 / 7.800x|175 / 2.692x|50.7|

Merge54ON162.157848ms has no supplied M5reference. AcceptedSDK remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`, with inherited primary7.928660ms and all-eight baseline in the accepted manifest; the newly matched accepted rows above are diagnostic observations, not replacement baseline qualification. This is now the latest full-SDK measurement. No new AWS/Aeglos, phone/ARM/TEE,250/4999, pressure/swap/minimum-RAM, resource matrix or concurrent-proof throughput result is added. All-eight verified CPU10×goal remainsactive0/8.

## Artifacts

[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [setup](artifacts/setup.rs), [selector](artifacts/selector.rs), [driver](artifacts/driver.rs), [native plan](artifacts/native-plan.json), [link plan](artifacts/link-plan.json), [capture plan](artifacts/plan.json), [identities](artifacts/identities.json), [all results](artifacts/analysis.json), [raw summary rows](artifacts/rows.json), [profiles](artifacts/profile-symbols.json), [reviewer](artifacts/review.rs), [review](artifacts/review.json), [proof inventory](artifacts/proof-inventory.json), [ledger](artifacts/proof-sha256-after.json), [all command receipts](artifacts/receipts.json.gz), [PMU CSV](artifacts/counters.json.gz), [source](artifacts/source.tar.gz), [source member hashes](artifacts/source-members.json), [public proofs](artifacts/public-proofs.tar.gz), [proof member hashes](artifacts/public-proofs-members.json), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS). Source179/publicproof984regular member sets and extracted bytes are hash-exact; both JSONgzip archives roundtrip exactly. Executables,libraries,rawperf,private actual keys/images/witnesses/scalar/point arrays are excluded.
