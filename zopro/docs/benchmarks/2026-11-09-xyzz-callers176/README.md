# Complete sparse XYZZ MSM comparison

**The fixed candidate is rejected.** All 12 context medians are slower against fresh OFF. The observed four-case 32-worker aggregate ON/OFF is **1.037470 [1.034627, 1.040556]**, or 3.747% slower, with 0/15 paired block wins. ON/REF is **1.034296 [1.030834, 1.036512]**, 0/15 wins. Two control-calibration checks also fail, so the experiment does not establish globally qualified control agreement or a precise causal effect. No code is adopted and no full SDK follow-up is selected.

The candidate replaces only sparse residual Jacobian accumulators with the previously qualified XYZZ representation. The exact qualified runtime retains ordinary affine batches, queue order, point order, window geometry, Phi mapping and weighted reduction. Its checked operation-owned reservation remains in the measured path and the unchanged required-workspace forecast is held. REF is accepted source with identical benchmark-only exports; OFF is the new static abstraction with Jacobian; ON enables optional XYZZ. Three final consumers reuse the original native field1200, native/O3/fat-LTO/one-CGU, the same Rayon dependency and allocator.

The four fixtures are complete balanced G1 K/H (30,529+32,768 points), uniform G1 K/H at width12 with the same points, ordinary G1 (21,006 points, width12, one part), and G2 Phi (15,044 points, width12, two parts). Inputs are deterministic synthetic valid points and full-width scalars. An untimed Arkworks helper generates complete canonical input hashes and full MSM references; each process regenerates native inputs and checks that hash before primes. These are qualification/census dimensions, not newly captured private proof inputs.

One operation per loop uses a real 1/16/32-worker pool internally, with 512 KiB stacks and fixed affinity CPU2/0–15/0–31. Time includes Window construction, admission, normal affine work, sparse exceptions/doubling, Phi, weighted tree, final affine normalization and intermediate destruction. G2's fresh mutable scalar copy is included. Fixture/pool/oracle creation, three validated primes and final equality/hash are excluded. This is a complete MSM screen, not full proof latency or concurrent-proof throughput.

|Caller /workers|REF ms|OFF ms|ON ms|ON/OFF [bounds]; wins|ON/REF [bounds]; wins|
|---|---:|---:|---:|---|---|
|g1-balanced-w1|34.683665|34.510073|34.996837|1.013957 [1.012068, 1.016811]; 0/15|1.009485 [1.005724, 1.010805]; 1/15|
|g1-balanced-w16|3.523282|3.513014|3.572649|1.015798 [1.010159, 1.021171]; 0/15|1.014491 [1.010815, 1.021285]; 0/15|
|g1-balanced-w32|2.454278|2.460654|2.505432|1.019153 [1.005742, 1.034522]; 1/15|1.021344 [1.003943, 1.030469]; 1/15|
|g1-uniform-w1|37.186249|37.079340|37.343291|1.006766 [1.005868, 1.007356]; 0/15|1.003412 [1.002101, 1.004291]; 0/15|
|g1-uniform-w16|3.529063|3.526128|3.599581|1.023019 [1.016876, 1.024436]; 0/15|1.018377 [1.014330, 1.028993]; 0/15|
|g1-uniform-w32|2.523287|2.540618|2.589879|1.022293 [1.013333, 1.027987]; 1/15|1.029420 [1.013880, 1.036013]; 1/15|
|g1-ordinary-w1|14.545681|14.449609|14.598940|1.009888 [1.008130, 1.012472]; 0/15|1.004262 [1.002199, 1.007046]; 1/15|
|g1-ordinary-w16|1.332288|1.321984|1.387850|1.046037 [1.043387, 1.052221]; 0/15|1.041340 [1.036930, 1.048004]; 0/15|
|g1-ordinary-w32|0.944025|0.943965|1.009047|1.071448 [1.051764, 1.083811]; 0/15|1.060069 [1.045905, 1.076013]; 0/15|
|g2-phi-w1|22.522719|22.861946|23.166128|1.013171 [1.011568, 1.014077]; 0/15|1.028021 [1.027237, 1.028989]; 0/15|
|g2-phi-w16|1.910347|1.944149|2.005798|1.030619 [1.025699, 1.034455]; 0/15|1.059272 [1.043495, 1.066445]; 0/15|
|g2-phi-w32|1.628227|1.625384|1.682481|1.036952 [1.024087, 1.041503]; 0/15|1.034026 [1.017804, 1.046796]; 2/15|

Ratios below 1 are faster. Fixed 15 paired blocks rotate/reverse context and arm order; each process measures 15 complete MSMs. Bounds are ordered ranks 4..12 of 15 block ratios (nominal 96.484375% marginal median coverage under independent exchangeable blocks, without a simultaneous guarantee). These are paired estimates, so they need not equal ratios of arm medians. No context or positive block is excluded.

The 94 declared checks have 15 failures: two of 12 control calibrations,11 of 24 nonregressions and both aggregate gain checks. Ordinary G1 at 32 has OFF/REF lower0.970454 below 1/1.03; G2 Phi at16 has upper 1.041130 above 1.03. All 24 RSS screens and 32 PMU/profile quality checks pass. Balanced G1 at 32 has valid calibration yet ON/OFF bounds wholly above 1, so the negative conclusion is not based only on the two failed calibrations. No threshold, curve, worker, layout, profile-length or unchanged-run rescue is applied.

|32-worker caller|REF /OFF /ON maximum RSS MiB|
|---|---:|
|g1-balanced-w32|27.75 /28.25 /27.75|
|g1-uniform-w32|30.00 /30.00 /30.00|
|g1-ordinary-w32|19.00 /18.25 /18.50|
|g2-phi-w32|19.50 /20.00 /20.00|

RSS includes fixtures, point construction, runtime and primes; it is not incremental proof scratch or minimum RAM. Allocation-audited preflights retain request/reallocation counts and cumulative requested bytes, which are not a live-memory peak. The sparse point payload is 128 vs 96 bytes in G1 and 256 vs 192 in G2; optional reservation bounds are not allocations. The rejection is on latency, not the declared RSS screen.

All 691 serialized workflow commands succeed first science execution: 38 allocation/denial preflights, one checker,540 latency processes,16 PMU panels,16 profiles and80 offline commands. There are610 actual caller processes. All latency finishes before diagnostics, and all recording finishes before offline decode/ownership/event/loss checks. During recording only the confirmed live workflow handle is polled; no owned build, helper, analyzer, hash, report or push overlaps. PMU panels use256 MSMs; separate profiles use1024, selected prospectively.

All 16 PMU groups meet 99.9% running. All 16 profiles qualify:188,075 samples, minimum 5,407, maximum unknown share1.258%, zero LOST, with binary/PID/TID/CPU/task/MMAP/event/period bindings. XYZZ type namespaces occur in all eight ON profiles, confirming changed Window execution; many samples include ordinary feed code with that type. Single PMU panels show only roughly 0.01–0.24% fewer instructions, with mixed cycle/cache results and more branch misses. They do not isolate a critical path, admission cost, doubling, register pressure or a cache-level cause. The standalone MSM's greater CPU cost at 32 workers also does not establish a new full-proof scheduling result.

The frozen analysis and a separate same-agent reader check 981 pins, all 691 command/stream receipts,610 caller results,176 unchanged production hashes, exact qualified runtime beneath common shims, all independent fixture hashes and 94 recomputed checks. The original reader stopped on a 1ulp JSON summary round-trip difference; its source and executable are preserved. The repaired reader changes only equality against serialized summaries to a 2e-12 relative+absolute tolerance; raw-data thresholds, scope and gate decisions are unchanged. No benchmark repeats. The earlier consumer link failure was repaired by moving Arkworks to the untimed oracle and retaining all three successful core builds; see [preparation](artifacts/preparation-note.md) and [reader repair](artifacts/reader-repair.json).

This coordinate change removes too little total work to justify adoption. Continue with distinct bulk arithmetic or dependency reductions in accepted proof profiles. There are no new SDK requests, proofs, Gnark exports or production changes; canonical verified inventory remains 43,730. The full CPU10× objective remains active and unmet, **0/8**. The following accepted local 32-worker proof results and supplied M5 CPU/Metal references are inherited, not new measurements.

|Shape|Accepted CPU ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.829286|37.0 /4.726×|24.0 /3.065×|3.7|
|2x2|10.979335|59.0 /5.374×|30.0 /2.732×|5.9|
|4x2|15.803969|109.0 /6.897×|48.0 /3.037×|10.9|
|8x2|23.944017|189.0 /7.893×|74.0 /3.091×|18.9|
|12x2|33.703579|284.0 /8.426×|101.0 /2.997×|28.4|
|16x2|39.858478|335.0 /8.405×|124.0 /3.111×|33.5|
|24x2|66.186709|528.0 /7.977×|176.0 /2.659×|52.8|
|merge-24x1|65.489902|507.0 /7.742×|175.0 /2.672×|50.7|

Accepted primary proof RSS remains 133.50 MiB; merge54 remains 162.541995 ms without a supplied M5 reference. Earlier 7.803404 ms and latest 7.829286 ms used the same accepted SDK. No new AWS/Aeglos, physical ARM/mobile, huge-circuit, pressure/swap, minimum-RAM or concurrent-proof throughput evidence is added.

[Full analysis](artifacts/analysis.json), [independent review](artifacts/review-result.json), [CPU diagnostics](artifacts/diagnostic-summary.json), [source audit](artifacts/source-audit.json), [identities](artifacts/identities.json), [Arkworks references](artifacts/oracle.json), [protocol](artifacts/protocol.md), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Private root: /home/ubuntu/src/zopro-perf-work/xyzz-callers176.
