# Compact native weighted bucket trees

The complete caller gate is **false**, with eight control-calibration failures. The native candidate is **not adopted**. Both 32-worker curve aggregates improve against both controls, and every nonregression and requested-capacity check passes; those positives remain evidence within this component scope. They are not a verified proof-latency gain.

Against fresh OFF, the G1 aggregate improves **4.015%** and G2 **5.296%**, each winning all 15 blocks. Against the original diagnostic build, the estimates are **9.689%** and **2.601%**. However, the controls themselves differ systematically: G2 OFF is about 3.7–6.6% slower in several cells, while G1 OFF is about 5.5–5.7% faster at 32 workers. All six G2 calibrations and both G1 32-worker calibrations fail the original factor-1.03 interval requirement. The gate is not rescued by the favorable candidate estimates.

|Curve / control|32-worker aggregate ON/control ratio [bounds], wins|
|---|---|
|G1/OFF|0.959850 [0.951156, 0.961888], 15/15|
|G1/frozen|0.903108 [0.899977, 0.905637], 15/15|
|G2/OFF|0.947042 [0.945252, 0.951110], 15/15|
|G2/frozen|0.973993 [0.967781, 0.980815], 15/15|


This targets weighted bucket reductions. The previous [compact indexed-tree SDK experiment](../2026-10-31-compact-sdk176/README.md) targeted unweighted trees and remains unadopted. The earlier [coarser weighted groups](../2026-10-22-universal-coarse-weighted-sdk/README.md) and [window-local weighted schedule](../2026-10-22-universal-window-local-weighted-sdk/README.md) changed task geometry. Here the original 256-bucket group topology and arithmetic identity stay intact; only the private group implementation changes. No production entry dispatches to it.

For adjacent pairs Cj=B2j+B2j+1, preserve W(B)=2W(C)−sum(B2j). At a level with S equal-length segments, copy the first segment's even positions to a tail outside all old inputs, reduce the S old segments using one shared inverse, then place the saved correction segment after their compacted outputs. Coordinates remain in compact native four-word storage while segment length exceeds eight. All inverse factors exist before forward output compaction. The existing complete reducer performs the final three levels, followed by the original doubling/subtraction identity and total output.

Reject an initial identity or a zero norm before that level writes. The input stays immutable, so a late rejection discards native buffers before complete original fallback. The new group uses one coordinate buffer plus prefix/norm records and a small complete tail. It adds no lookup table, transformed/copied key, duplicate solver plan, pool, synthetic RAM ceiling, field multiplication, C or assembly backend. Reused lane movement/zero/compact helpers and their two tests are byte-exact from the preceding native compact-tree study. The IFMA generator passes its consistency check.

Both private snapshots contain 178 files. Five original parents have reversible diagnostic/helper insertions; 171 original files remain exact in each, and source recovery checks all 176 accepted files twice. Arithmetic, original point kernels, weighted_sum, solver, routing and schedules recover byte for byte. The control compilation omits the native cfg and calls original weighted_sum. Candidate OFF and ON are identical executable copies with a runtime selector. Actual final links use optimization level 3, native CPU, fat LTO and one codegen unit. Source identity does not guarantee identical code placement or inlining across control builds; the calibration difference's exclusive cause is unresolved.

Qualification passes **37 native field tests**, **128 native core tests** and **two independent integer-model tests**. The field suite includes the two unchanged lane-helper tests. The core suite adds G1/G2 weighted-and-total checks across every power of two from 1 through 256, identities, doubling, opposites, a late-cancellation family, input immutability and repeated OFF/ON transitions; another test covers reuse on 1/4/16/32-worker pools with 512 KiB stacks. The integer model imports no field/curve code and reproduces every original coefficient and overwrite bound. No new portable, full-prover, phone or pressure qualification follows.

Ryzen 9950X, 16 physical cores / 32 logical CPUs, shared host. Twelve contexts cover G1/G2, 128/256 buckets and 1/16/32 workers. Workers pin individually to CPU 2, CPUs 0–15 or CPUs 0–31 and use 512 KiB stacks; fixtures may move between workers. Each process uses three checked eight-round primes and 4,096 timed complete weighted groups per fixture. Input import, all arithmetic, complete finishing and dispatch/wait are timed; fixture construction, pool creation and reference checks are excluded. Inputs alternate between two preconstructed full-width random generator-multiple families with different worker seeds. They are synthetic dense groups, not a changing witness stream or captured proof distribution.

All 588 caller processes match independent weighted-generator and total references, with identical output digests across arms, zero timed allocations, stable capacities and zero timed fallbacks. Every prime and final state is checked; intermediate timed results are not each independently checked.

|Context|Original / OFF / ON µs per wave|ON/OFF [bounds], wins|ON/original [bounds], wins|OFF/original calibration|
|---|---:|---|---|---|
|G2-128-w1|24.167 / 25.754 / 22.850|0.887098 [0.885452, 0.888935], 15/15|0.946733 [0.942946, 0.948094], 15/15|1.066066 [1.063802, 1.067898], 0/15|
|G2-128-w16|28.019 / 29.475 / 26.027|0.882322 [0.878078, 0.885910], 15/15|0.929758 [0.927430, 0.935333], 15/15|1.054719 [1.045814, 1.061651], 0/15|
|G2-128-w32|39.494 / 41.004 / 39.600|0.967208 [0.962687, 0.970746], 15/15|1.003673 [1.000726, 1.007922], 3/15|1.036942 [1.035269, 1.040000], 0/15|
|G2-256-w1|34.438 / 36.197 / 32.001|0.885109 [0.878882, 0.887388], 15/15|0.929486 [0.924654, 0.932187], 15/15|1.050333 [1.048559, 1.055093], 0/15|
|G2-256-w16|40.846 / 42.280 / 36.828|0.869875 [0.867193, 0.873478], 15/15|0.901635 [0.898288, 0.909652], 15/15|1.039353 [1.029586, 1.048773], 1/15|
|G2-256-w32|59.569 / 60.547 / 56.302|0.928102 [0.922295, 0.934940], 15/15|0.947327 [0.927963, 0.962712], 15/15|1.023111 [1.009242, 1.035040], 3/15|
|G1-128-w1|12.983 / 12.661 / 12.290|0.970888 [0.967589, 0.972527], 15/15|0.948495 [0.942620, 0.950779], 15/15|0.976506 [0.971020, 0.978957], 15/15|
|G1-128-w16|14.274 / 14.013 / 13.567|0.969924 [0.966942, 0.971935], 15/15|0.950603 [0.948701, 0.951871], 15/15|0.980346 [0.977020, 0.982883], 15/15|
|G1-128-w32|21.349 / 20.116 / 19.508|0.967800 [0.962075, 0.978135], 15/15|0.914606 [0.909313, 0.920247], 15/15|0.943411 [0.930386, 0.957654], 15/15|
|G1-256-w1|18.643 / 18.326 / 17.435|0.951505 [0.949358, 0.952864], 15/15|0.935206 [0.933414, 0.937465], 15/15|0.982949 [0.981256, 0.986681], 15/15|
|G1-256-w16|20.713 / 20.539 / 19.415|0.947075 [0.944348, 0.950736], 15/15|0.938517 [0.935924, 0.940890], 15/15|0.990280 [0.988140, 0.991695], 14/15|
|G1-256-w32|30.955 / 29.071 / 27.493|0.942862 [0.936061, 0.954054], 15/15|0.890943 [0.884035, 0.897606], 15/15|0.944935 [0.930472, 0.948461], 15/15|


Each curve's equal-log aggregate over both 32-worker sizes requires upper ON/control ratio below one and at least 10/15 wins against both controls. Every context upper ON/control must be at most 1.03; every OFF/original interval must fit factor 1.03. Ordered ranks 4–12/15 have nominal 96.484375% marginal coverage under independent exchangeable blocks, without familywise guarantees. No gain floor, exclusions, extension, favorable subset, threshold change, parameter sweep or unchanged reroll occurs.

Failed checks: `["G2-128-w1/OFF/frozen/calibration","G2-128-w16/OFF/frozen/calibration","G2-128-w32/OFF/frozen/calibration","G2-256-w1/OFF/frozen/calibration","G2-256-w16/OFF/frozen/calibration","G2-256-w32/OFF/frozen/calibration","G1-128-w32/OFF/frozen/calibration","G1-256-w32/OFF/frozen/calibration"]`.

|Context|Original / OFF / ON CPU ms per wave|Original / OFF / ON maximum-RSS counter MiB|Original / OFF / ON retained scratch bytes per fixture|
|---|---:|---:|---:|
|G2-128-w1|0.024167 / 0.025754 / 0.022849|2.75 / 2.75 / 2.75|`44032 / 44032 / 41728`|
|G2-128-w16|0.422862 / 0.447035 / 0.401464|3.00 / 3.00 / 3.25|`44032 / 44032 / 41728`|
|G2-128-w32|1.217690 / 1.257035 / 1.225844|3.25 / 3.25 / 3.50|`44032 / 44032 / 41728`|
|G2-256-w1|0.034438 / 0.036197 / 0.032002|2.75 / 2.75 / 2.75|`85312 / 85312 / 75328`|
|G2-256-w16|0.611474 / 0.637879 / 0.568587|3.50 / 3.50 / 3.50|`85312 / 85312 / 75328`|
|G2-256-w32|1.830320 / 1.877928 / 1.741964|4.25 / 4.25 / 4.25|`85312 / 85312 / 75328`|
|G1-128-w1|0.012984 / 0.012662 / 0.012291|2.25 / 2.50 / 2.50|`29632 / 29632 / 23872`|
|G1-128-w16|0.220195 / 0.215836 / 0.209363|2.75 / 2.75 / 2.75|`29632 / 29632 / 23872`|
|G1-128-w32|0.651044 / 0.614489 / 0.599579|2.75 / 3.00 / 3.00|`29632 / 29632 / 23872`|
|G1-256-w1|0.018644 / 0.018326 / 0.017436|2.50 / 2.75 / 2.75|`59328 / 59328 / 43392`|
|G1-256-w16|0.318800 / 0.316150 / 0.300035|3.00 / 3.00 / 3.00|`59328 / 59328 / 43392`|
|G1-256-w32|0.928767 / 0.901858 / 0.848602|3.50 / 3.25 / 3.25|`59328 / 59328 / 43392`|


Requested retained scratch decreases from 29,632 to 23,872 bytes for G1/128, 59,328 to 43,392 for G1/256, 44,032 to 41,728 for G2/128 and 85,312 to 75,328 for G2/256. The separate [work model](artifacts/work-model.json) reproduces every observed context capacity. For 256 buckets both paths perform 502 ordinary additions and eight shared inversions; for 128, 247 additions and seven inversions. This changes data movement and execution cost, not the mathematical addition count.

Those capacities exclude fixture/key arrays, inline fields, compiler stack and allocator overhead. Recorded maximum-RSS counters are diagnostic: earlier page-table audits found undercounts on this host. They are not accurate resident peaks or minimum RAM. No fresh page-table sampler, hard memory envelope, swap or pressure result is claimed.

All 540 latency processes precede six grouped four-event counter panels and six separate fixed-16M-user-cycle profiles. Counter panels run 16,384 groups per worker and profiles 65,536 after three checked primes excluded through FIFO acknowledgements. No owned build, helper, hashing, archive, analyzer or second workload overlaps capture. Perf ownership changes occur only after recording and preserve its hash.

|Counter panel|Cycles per wave|Instructions per wave|Cache misses per wave|Branch misses per wave|Profile samples|
|---|---:|---:|---:|---:|---:|
|stat-G1-256-w32-OFF|4004082|8450602|3.204|683.635|16313|
|stat-G1-256-w32-ON|3847822|7486027|3.663|297.956|15740|
|stat-G1-256-w32-frozen|4168012|8422394|3.347|879.587|17194|
|stat-G2-256-w32-OFF|8100075|15655963|101.465|803.089|33286|
|stat-G2-256-w32-ON|7633372|13948527|146.129|192.604|31316|
|stat-G2-256-w32-frozen|7791131|15647876|149.604|741.703|31915|


At size 256, ON retires about **11.4% fewer instructions for G1** and **10.9% fewer for G2** than OFF. Cycles fall about **3.90%** and **5.76%**. These are descriptive single panels, not paired uncertainty bounds or a measured attribution to one instruction class. Generic cache events do not identify bandwidth or a cache-level cause. All events run 100%.

The six profiles contain **145,764 samples**, zero lost records, at least 15,740 samples per profile and unknown share at most 0.693%. Expected process, registered worker TID, mapping/FORK, allowed CPU and fixed-period evidence is checked. Flat symbol attribution includes inlined work; it is not removable proof wall time. [Linked-code records](artifacts/codegen.json) include all observed weighted diagnostic/reducer specializations, including constructors and checks outside timing. Static sites/RSP operands are not dynamic spills or comparable exclusive function costs when inlining boundaries differ.

The first read-only reviewer stopped because a broad helper text replacement changed its expected period from 16,000,000 to 15,880,000. Capture used the correct fixed period throughout. The initial reviewer/source/log remain; review-v2 changes only that literal and passes all 618 commands, 588 outputs, raw statistics, profile symbol bins, capacity comparisons and gate recomputation. No workload or successful native test is rerun. A separate pre-capture preparation-log pin finalization and draft helper corrections are documented in [planning notes](artifacts/planning-notes.md). Build-OFF/ON logs refer to one candidate build, not two builds.

No SDK request, proof export or Gnark verification is added; the canonical distinct-proof ledger remains **28,349**. Accepted source 176 and SDK SHA-256 `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661` stay unchanged. The following CPU/Metal comparisons are inherited accepted full-proof observations and supplied M5 references, not timings of this prototype. The accepted adoption retains its documented physical16 calibration engineering exception and false original full-performance gate.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37.0|4.667×|24.0|3.027×|3.7|
|2x2|11.151932|59.0|5.291×|30.0|2.690×|5.9|
|4x2|16.086386|109.0|6.776×|48.0|2.984×|10.9|
|8x2|24.179453|189.0|7.817×|74.0|3.060×|18.9|
|12x2|33.884455|284.0|8.381×|101.0|2.981×|28.4|
|16x2|40.077381|335.0|8.359×|124.0|3.094×|33.5|
|24x2|65.850777|528.0|8.018×|176.0|2.673×|52.8|
|merge-24x1|65.060937|507.0|7.793×|175.0|2.690×|50.7|


The all-eight CPU 10× objective remains **active and unmet, 0/8**. Accepted primary is 7.928660 ms against a 3.7 ms CPU target. Historical merge54 is 161.883899 ms without a supplied M5 reference. AWS/Aeglos comparisons predate source 176; no new AWS resource or measurement occurs. No mobile/ARM/TEE, huge250/4999, concurrent-throughput or minimum-RAM claim follows.

This component provides a useful direction, with an unpassed calibration gate and no proof-speed claim. A distinct next hypothesis is to preserve the current groups but combine their small final frontiers: sixteen full 256-bucket groups currently invoke 48 late shared inversions; one joint three-level finish could use three and vectorize scalar tails. This is a structural count only, not an implemented algorithm or predicted speedup. It would add frontier ownership, movement, admission and task costs that need complete measurement. The current candidate is not rerun or adopted on the basis of that hypothesis.

[Protocol](artifacts/design.md), [full results](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [independent review](artifacts/review.json), [profile symbols](artifacts/profile-symbols.json), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [publication audit](artifacts/publication.json). Archives: [source snapshots](artifacts/source.tar.gz), [Rust workflows](artifacts/workflow.tar.gz), [command receipts](artifacts/receipts.tar.gz), [counter panels](artifacts/counters.tar.gz), [disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS). Executables/libraries/raw perf.data and private inputs stay local with hashes. Root: `/home/ubuntu/src/zopro-perf-work/weighted-tree176`.
