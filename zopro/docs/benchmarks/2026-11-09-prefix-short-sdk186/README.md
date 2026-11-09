# Scaled G2 prefixes with the short forward map: complete proofs

The combined candidate measures **7.753327 ms** for local 1×2 at 32 workers, versus accepted **7.852740 ms** and fresh control **7.855035 ms**. Its primary paired gain passes against all four controls. The complete-scope decision is **failed: 62/71 checks pass**, including a failed merge memory screen. **This exact composition is closed and not adopted. The CPU 10× goal remains unmet, 0/8.**

The run completed 71,050 internally verified SDK requests and 910 fresh, unique proof exports independently verified with Gnark. The raw review validates all 1,895 capture/offline receipts, placement/task observations, 100 pairwise contrasts and all 71 decisions. [Results](artifacts/results.json), [raw review](artifacts/review-result.json), [prospective review](artifacts/prospective-review.json).

## Implementation and controls

The candidate combines [qualified scaled prefixes and typed uncarried X sums](../2026-11-08-uniform-carry-default-performance183/README.md) with the [qualified short forward map](../2026-11-09-negative-phi-short176/README.md). Three affine beta loops change; projective maps, scalar splitting, window geometry, order, queues, tails, grouping and admission remain unchanged. Canonical points are exported before the short map. It adds no heap buffer, retained table or pool; fixed-size SIMD temporaries and inherited short-factor constants remain. Source-level allocation invariance does not guarantee equal process RSS. [Native composition qualification](../2026-11-09-prefix-short-composition186/README.md), [source audit](artifacts/source-audit.json).

All five consumers use the exact same inherited driver/support and fixed worker placement: one global Rayon pool with default stacks, worker i on CPU i, main on the broad inherited mask. N+1 tasks, worker identities and affinity are checked before and after every process. This is a common benchmark condition, **not an adopted production placement default**. REF reuses accepted libraries; CONTROL freshly builds accepted-source core/dependents; PREFIX reuses the qualified default183 libraries; SHORT changes only the three affine map loops; BOTH uses the qualified composition186 field/core with coherent dependents. All 12 build/link commands pass. Qualified unchanged libraries, mathematical models and the 167-query native qualification are reused without replay.

The component arms are controls. Their earlier failed performance decisions remain failed. No unchanged reroll, post-result subset, exception, parameter/worker/threshold change or production switch is selected here. [Build design](artifacts/build-design.md), [protocol](artifacts/protocol.json), [full design](artifacts/design.md), [sealed inputs](artifacts/closing-inputs.json).

## Complete-proof latency

Numbers are medians of process means, in ms. Primary 1×2/32 uses 31 matched blocks of 151 requests; the other nine contexts use 15 blocks, with 151 requests for 1×2 at 16 workers and 51 otherwise. Each process first makes three full priming requests. All witness solving, proving and internal verification are in the SDK call; loading/pool setup and exported-proof checks are outside the clock. The fixed context/arm rotation and reversal were declared before any SDK preflight. No observation was excluded or extended.

|Context|REF|CONTROL|PREFIX|SHORT|BOTH|
|---|---:|---:|---:|---:|---:|
|1x2-w32|7.852740|7.855035|7.829355|7.813607|7.753327|
|2x2-w32|11.026811|10.983210|10.919432|10.905297|10.841685|
|4x2-w32|15.829917|15.703195|15.779489|15.799796|15.687119|
|8x2-w32|24.163935|23.868937|23.957634|23.986654|23.928185|
|12x2-w32|34.087049|33.768338|33.566637|33.744541|33.528082|
|16x2-w32|40.010922|39.833872|39.476671|39.821221|39.407196|
|24x2-w32|66.255362|66.108620|65.809652|66.214650|65.774403|
|merge-24x1-w32|65.178861|65.631975|64.985525|65.521287|65.270789|
|merge-54x1-w32|162.381111|162.812802|162.216157|162.995206|163.314315|
|1x2-w16|9.198357|9.210306|9.071617|9.152316|8.967592|

Primary 1×2/32 paired ratios below 1 favor BOTH. The bootstrap intervals and the separately declared rank intervals are both retained; the rank criteria determine the primary gain decision. All four gains, win counts, nonregression and primary calibration checks pass.

|BOTH versus|Paired geometric ratio [bootstrap95%]|Declared rank interval|Wins|
|---|---:|---:|---:|
|REF|0.986673077 [0.983786772, 0.989625209]|[0.983411350, 0.990502602]|29/31|
|CONTROL|0.986075125 [0.983111643, 0.989148911]|[0.983931037, 0.990139837]|29/31|
|PREFIX|0.990222987 [0.988397716, 0.992158679]|[0.988339496, 0.993101695]|29/31|
|SHORT|0.991728747 [0.988959629, 0.994416722]|[0.990301295, 0.994811853]|27/31|

At 16 workers, BOTH measures **8.967592 ms** versus REF **9.198357 ms**; paired BOTH/REF is 0.976946581 [0.974065501, 0.980172448], 15/15 wins. These small-proof gains do not establish a twofold improvement.

Original-eight equal-log aggregates, with shape-stratified bootstrap intervals:

|BOTH versus|Equal-log ratio [bootstrap95%]|Required aggregate gate|
|---|---:|---|
|REF|0.991057976 [0.986401800, 0.995966386]|pass|
|CONTROL|0.994691822 [0.990266712, 0.999590964]|pass|
|PREFIX|0.999651118 [0.996195944, 1.003954686]|descriptive control|
|SHORT|0.995446460 [0.991804288, 1.000002125]|descriptive control|

The aggregate estimates assume exchangeable matched blocks independently resampled within each shape. Cross-shape/shared-host dependence and familywise coverage are not established. All 100 contrasts, raw ratios and primary rank intervals remain in [contrast data](artifacts/contrasts.json).

## Failed checks and memory

The full gate fails these nine unchanged checks:

- `8x2-w32/BOTH/REF/nonregression`
- `8x2-w32/BOTH/CONTROL/nonregression`
- `8x2-w32/BOTH/PREFIX/nonregression`
- `8x2-w32/BOTH/SHORT/nonregression`
- `16x2-w32/CONTROL/REF/calibration`
- `16x2-w32/BOTH/CONTROL/nonregression`
- `16x2-w32/BOTH/PREFIX/nonregression`
- `16x2-w32/BOTH/SHORT/nonregression`
- `merge-24x1-w32/BOTH/RSS`

For 8×2, BOTH/CONTROL is **1.017568126 [1.000186620, 1.044116737]**: its upper bound exceeds the declared 2% nonregression limit. At 16×2, CONTROL/REF has **[0.969012580, 1.001379101]**, outside the lower calibration bound 1/1.03, and three candidate nonregression intervals also exceed 1.02. Every slow observation remains included; this report does not label those observations noise or infer a cause.

Merge24 peak RSS is **1287.679688 MiB** for BOTH versus REF **1219.500000** and CONTROL **1223.234375**. The declared `max(REF, CONTROL) × 1.05 + 1 MiB` limit is **1285.396094 MiB**, exceeded by **2.283594 MiB**. The source adds no new heap buffer, but the measured peak fails; allocation overlap, scheduling and allocator retention are possible hypotheses, not established causes.

|Context|REF MiB|CONTROL MiB|PREFIX MiB|SHORT MiB|BOTH MiB|
|---|---:|---:|---:|---:|---:|
|1x2-w32|132.750|133.000|133.500|133.250|134.750|
|2x2-w32|299.500|301.750|294.000|301.750|299.250|
|4x2-w32|329.750|331.250|333.750|335.750|325.750|
|8x2-w32|547.250|533.500|534.500|543.750|547.000|
|12x2-w32|765.750|750.500|753.750|750.000|760.750|
|16x2-w32|874.500|859.250|871.500|882.500|885.750|
|24x2-w32|1368.000|1368.750|1374.250|1406.750|1370.031|
|merge-24x1-w32|1219.500|1223.234|1233.000|1203.855|1287.680|
|merge-54x1-w32|2021.754|2040.566|2060.055|2134.965|2028.547|
|1x2-w16|88.750|87.500|86.750|88.250|89.000|

These are maxima of process-lifetime RSS across latency runs, including image loading, key state, priming and workers. They are not minimum RAM, private allocation counts or pressure/swap/device qualification. Merge54 also retains a latency negative: BOTH/REF **1.008321783 [1.003005794, 1.013506565]**, 4/15 wins, even though it passes the 2% nonregression rule. No M5 reference is supplied for merge54.

## CPU work and profiles

At 1×2 with 32 workers, median user+system CPU service falls from **179.579662 to 174.475159 ms/request** while process RSS rises from 132.75 to 134.75 MiB. Minor faults fall from 127.966887 to 126.728477/request in this context; larger contexts can increase, so this is not a general page-fault improvement claim. A fault count does not by itself establish a DRAM or swapping bottleneck.

One automatic PMU panel per context/arm gives the following primary32 counts per request:

|Arm|Instructions, millions|User cycles, millions|Cache misses, millions|Branch misses, millions|
|---|---:|---:|---:|---:|
|REF|1386.976852|814.403632|2.688246|0.894659|
|CONTROL|1385.615567|807.343613|2.682404|0.870838|
|PREFIX|1360.264271|795.514644|2.664367|0.884654|
|SHORT|1373.964356|802.415509|2.654976|0.905984|
|BOTH|1347.481476|788.560192|2.698585|0.890286|

All 15 profiles qualify: **219005 samples**, zero LOST, 100% assigned execution samples and unknown shares no greater than **1.891708%**. All 60 PMU event readings report 100% scheduled. The dominant primary samples still include ordinary G1 streaming, borrowed G2 apply, epoch handling and inversion. These are flat sample shares, not exclusive wall time, removable fractions or causal critical-path evidence. [Diagnostic data](artifacts/diagnostic-analysis.json).

## M5 CPU and Metal references

The following are observations of the **unadopted BOTH candidate** at32 workers under the common fixed placement. M5 timings are supplied references from different hardware. They are not fresh Metal measurements.

|Shape|Candidate ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.753327|37 / 4.772×|24 / 3.095×|3.7|
|2x2|10.841685|59 / 5.442×|30 / 2.767×|5.9|
|4x2|15.687119|109 / 6.948×|48 / 3.060×|10.9|
|8x2|23.928185|189 / 7.899×|74 / 3.093×|18.9|
|12x2|33.528082|284 / 8.471×|101 / 3.012×|28.4|
|16x2|39.407196|335 / 8.501×|124 / 3.147×|33.5|
|24x2|65.774403|528 / 8.027×|176 / 2.676×|52.8|
|merge-24x1|65.270789|507 / 7.768×|175 / 2.681×|50.7|

**0/8** reach the CPU 10× point targets. Historical accepted sub8ms results remain valid: standard1×2 **7.915095 ms**, later standard **8.075143 ms** on the same accepted code, and a separate earlier fixed-placement **7.869302 ms**. The current accepted fixed observation **7.852740 ms** belongs to this campaign. No cross-campaign code regression or production record is inferred. [Previous accepted comparison](../2026-11-09-paired-top-query176/README.md).

## Review, decision and continuation

The capture handle 55421 is terminal with exit code 0. All 807 sealed source/artifact/reader pins remain exact. All 50 preflights, 830 latency processes, 30 diagnostics, 910 Gnark verifiers and 75 profile-offline commands complete. Every export is unique against the prior 38,668-proof ledger; the canonical count is now 39,578. Production's 176 members and accepted SDK 6d83 remain unchanged. No AWS/Aeglos, mobile, huge-circuit, pressure/swap/minimum-RAM or throughput measurement is added.

The original frozen raw reviewer rejected approximately one-ulp differences when comparing computed floating-point objects. It and its executable/pins are preserved. A separate postcapture reader changes only numeric-result equality to a 1e-11 relative/absolute tolerance; integer, boolean and string checks and every performance threshold remain unchanged. It independently rebuilds raw means/peaks, the 100 contrasts and all 71 decisions and confirms 910 unique Gnark bindings and worker/task observations. Statistical functions and profile parsing are shared, not independent audits of those algorithms. No measurement was replayed. The preparer's console said 14 build steps; the authoritative plan/result contain 12. Exact helpers and receipts are retained in the [text evidence archive](artifacts/evidence.json.gz).

The primary gain is useful evidence that this composition reduces small-proof latency. The full gate failure closes this exact recipe without adoption, exception or component-subset rescue. Next investigate a distinct substantial reduction in ordinary G1/G2 work and live bucket/scratch lifetimes, using the saved profiles and slow/RSS rows. No replacement is selected. New native work needs a complete consumer/resource argument and a new prospective comparison; full correctness/resource/admission/portable/concurrency qualification must precede adoption. The original all-eight CPU 10× objective remains active. [Continuation loop](artifacts/next.md).

Private root:`/home/ubuntu/src/zopro-perf-work/prefix-short-sdk186`. [Publication checks](artifacts/publication.json), [checksums](SHA256SUMS). The archive includes UTF8 helpers, deltas, pins, raw timing/verification/profile receipts and analysis; executable/library blobs, raw perf data, key/witness/proof bytes, the private full proof ledger and unchanged source snapshots are excluded. No recording is live.
