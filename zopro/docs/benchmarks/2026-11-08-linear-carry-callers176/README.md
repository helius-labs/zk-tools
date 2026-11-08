# Deferred X-sum carry propagation in complete batch callers

**The fixed uniform implementation is not adopted and does not select an SDK trial.** The full 24-context aggregate improves, but the declared screen fails 14 checks: five control calibrations, seven candidate nonregression checks and both G1 aggregate gain checks. The positive G2 result is retained. Production source 176 and SDK 6d83 remains unchanged, with local1×2 approximately 7.9–8.0 ms.

G2's four-size 32-worker paired aggregate is1.42% faster than fresh OFF and1.23% faster than frozen, winning14/15 blocks against each. ALL24 improves0.95% against OFF and1.34% against frozen. G1's32-worker gain remains unestablished: its interval spans parity against both controls. A failed upper-bound check is not proof of a slowdown of that size. These are synthetic complete-addition caller results, not proof latency.

## Change

The exact [qualified field type](../2026-11-08-linear-carry-fusion176/README.md) keeps Ax+Bx as five unnormalized limb sums until subtracting it from square(slope). Internal signed carries can be−2; the original2p correction produces exactly the original normalized representative. Uncarried digits never reach IFMA or an ordinary field export. This eliminates an intermediate carry pass without removing any multiplication.

G1 replaces only its scratch X-sum type, two constructors and reverse subtraction under a private native guard. Its record remains 1,600 bytes, alignment 64. Both native FAST modes qualify because imported host X anchors and all canonical bucket writes are below p. Other FAST-controlled arithmetic stays original. G2 uses two ephemeral sums only in FAST signed-borrowed reverse X; FAST=false remains original. Both scratch-building branches, signs, Y arithmetic, prefixes, inversion, tails, key/bucket storage and allocation sites retain their operations. Unsupported guards and OFF retain the original expressions. No LUT, key copy, retained plan, new production pool, fixedRAM cap or C/ASM rewrite is added.

Kernel contracts remain finite ordinary points, distinct destinations and nonzero denominators. The original complete exception dispatcher is unchanged and is not newly qualified by this direct-kernel matrix. The179-file snapshot has 171 exact originals and five reversible parents plus three children; all 176 originals recover. Field source and its already-qualified native library are reused byte-exact without a new field build or old field-suite replay.

## Complete caller measurements

AMD Ryzen 9 9950X, 16 physical cores and 32 logical CPUs, shared host. Frozen is the immutable paired-coordinate-store176/caller-OFF original-arithmetic diagnostic caller, not the production SDK. Fresh OFF/ON share the qualified field library and coherent core artifacts. Final links use nativeO3/fatLTO/oneCGU/debug1. The inherited standard driver differs only in the compile-time experiment selector. Diagnostic Rayon workers have512 KiB stacks, soloCPU2 or CPUs0–15/0–31.

Public generator fixtures change bucket points on every update. Three checked 128-round primes and reset precede8,192 measured batches per worker. Timers include dispatch, borrowed-reference construction and all addition/inversion work; setup/reset/final checks are outside. Every process matches independently accumulated generator coefficients, zero timed allocations, stable/equal scratch capacities and assigned-workerCPU checks. This is a synthetic caller workload, not arbitrary SDK input distribution, an individual-task service time or concurrent-proof throughput. Intermediate timed states are not each individually checked.

All 24 contexts and three arms run in15 fixed rotated/reversed blocks. Ratios pair within a block; displayed median times can have a different quotient. Bounds use zero-based ranks3/11, median7, of15ratios. Marginal bounds do not imply simultaneous familywise coverage. Every observation is retained, without exclusions, extensions, subsets, unchanged rerolls or compiler/layout/worker rescue.

|Aggregate|ON/OFF median [bounds], wins|ON/frozen median [bounds], wins|
|---|---:|---:|
|G1, four sizes at32workers|1.008498 [0.961521, 1.045618], 6/15|0.988555 [0.944835, 1.034810], 10/15|
|G2, four sizes at32workers|0.985838 [0.978186, 0.993001], 14/15|0.987746 [0.969315, 0.991345], 14/15|
|All 24 contexts|0.990539 [0.980827, 0.996455], 12/15|0.986635 [0.979993, 0.991355], 14/15|

|Context|Frozen µs|OFF µs|ON µs|ON/OFF median [bounds], wins|ON/frozen median [bounds], wins|
|---|---:|---:|---:|---:|---:|
|G1-200-w1|4.236|4.237|4.217|0.994724 [0.992771, 0.996146], 15/15|0.996221 [0.993943, 0.998082], 15/15|
|G1-200-w16|4.796|4.833|4.790|0.992300 [0.989736, 0.995380], 13/15|0.998403 [0.993817, 1.003854], 9/15|
|G1-200-w32|6.912|6.894|6.930|1.010341 [0.991252, 1.173895], 7/15|1.000456 [0.873855, 1.181238], 7/15|
|G1-201-w1|4.351|4.351|4.332|0.995284 [0.993042, 0.996744], 14/15|0.995337 [0.994457, 0.998253], 14/15|
|G1-201-w16|4.927|4.937|4.916|0.994951 [0.990693, 1.001205], 11/15|0.997708 [0.989110, 1.000489], 10/15|
|G1-201-w32|7.093|8.032|7.190|1.011535 [0.990522, 1.051029], 5/15|1.010466 [0.991709, 1.175198], 6/15|
|G1-352-w1|6.966|6.962|6.934|0.995330 [0.994613, 0.996228], 15/15|0.995368 [0.993867, 0.995931], 15/15|
|G1-352-w16|7.956|7.981|7.922|0.990760 [0.990170, 0.993682], 15/15|0.993768 [0.990778, 0.996537], 15/15|
|G1-352-w32|12.541|11.596|11.515|0.990315 [0.982823, 1.010259], 11/15|0.960804 [0.792891, 1.027018], 11/15|
|G1-353-w1|7.073|7.073|7.035|0.994574 [0.994251, 0.996121], 15/15|0.995170 [0.994086, 0.996462], 15/15|
|G1-353-w16|8.101|8.092|8.049|0.994285 [0.992719, 0.999139], 13/15|0.995372 [0.991893, 0.996695], 13/15|
|G1-353-w32|11.780|11.816|11.810|1.000290 [0.982926, 1.014192], 7/15|0.999686 [0.815283, 1.010531], 8/15|
|G2-200-w1|7.701|7.700|7.563|0.981851 [0.980940, 0.983014], 15/15|0.982984 [0.981451, 0.983717], 15/15|
|G2-200-w16|8.935|8.958|8.790|0.980413 [0.978106, 0.983937], 15/15|0.983666 [0.979799, 0.986695], 15/15|
|G2-200-w32|14.418|14.338|14.759|0.990936 [0.981215, 1.044316], 9/15|0.989587 [0.936846, 1.040694], 8/15|
|G2-201-w1|8.060|8.087|7.872|0.973054 [0.972067, 0.974269], 15/15|0.977008 [0.974792, 0.978168], 15/15|
|G2-201-w16|9.377|9.408|9.152|0.972951 [0.971369, 0.974827], 15/15|0.977185 [0.973226, 0.979108], 15/15|
|G2-201-w32|14.967|14.899|14.585|0.982787 [0.934570, 0.989283], 13/15|0.971380 [0.962267, 0.989338], 13/15|
|G2-352-w1|13.034|13.031|12.801|0.982815 [0.982297, 0.983422], 15/15|0.983378 [0.982163, 0.983946], 15/15|
|G2-352-w16|15.219|15.341|14.965|0.975455 [0.973215, 0.980160], 15/15|0.983931 [0.981780, 0.986588], 15/15|
|G2-352-w32|25.026|24.909|24.239|0.971545 [0.966559, 0.986120], 15/15|0.975958 [0.966104, 0.981949], 15/15|
|G2-353-w1|13.382|13.398|13.109|0.978943 [0.978097, 0.979802], 15/15|0.979612 [0.979165, 0.980640], 15/15|
|G2-353-w16|15.620|15.719|15.301|0.974303 [0.971537, 0.977874], 15/15|0.979888 [0.978825, 0.982365], 15/15|
|G2-353-w32|25.546|25.229|25.025|0.985297 [0.982091, 0.992677], 13/15|0.978779 [0.976194, 0.988988], 14/15|

The declared78checks require every OFF/frozen interval withinfactor1.03, every candidate/control nonregression upper≤1.02, and each curve's32-worker aggregate plus ALL24 to have upper<1 and≥10/15wins against both controls. There is no minimum gain floor. The five calibration failures are G1/201/352/353 at32workers and G2/200/201 at32workers. All seven failed nonregression checks are also32-worker cases. Full failure list and positive estimates remain in [analysis](artifacts/full-analysis.json) and [review](artifacts/full-review.json). The aggregate positives do not override failed individual checks or establish a full-proof gain.

A separately declared seven-block necessary early screen precedes this matrix: both curves/200points/one32workers,12preflights,84latencycalls,4PMUpanels and4profiles. It passes its necessary rejection rules and quality checks, selecting the full matrix. Its G1/32 frozen calibration fails and remains recorded. Early data are not pooled with the full matrix or treated as a repeatable gain claim.

## Automatic profiles, counters and resources

All 1,080 full-matrix latency processes precede18 PMU panels and18 profiles; all90ownership/decode/event/loss operations follow recordings. FIFO acknowledgment gates exclude primes and checks. Profiles use cycles:u,16million-cycle period,512mmap pages and2097152solo/65536threaded rounds; stat runs onequarter as many. All full profiles qualify:118,473 samples, zero LOST, unknown≤5%, assigned-worker and target shares≥95%, each≥1000samples. All grouped counters have≥99.9%scheduled coverage. Targets include the actual original kernels/inverse and both frozen/fresh caller namespaces.

|PMU context|ON/OFF retired-instruction ratio|ON/OFF user-cycle ratio|
|---|---:|---:|
|G1-200-w1|0.996166|0.995673|
|G1-200-w32|0.996174|1.006047|
|G1-352-w32|0.995849|0.995368|
|G2-200-w1|0.984489|0.981837|
|G2-200-w32|0.984475|0.987016|
|G2-352-w32|0.983905|0.980547|

G1 retires about0.38–0.42% fewer instructions, G2 about1.55–1.61% fewer. G1's200/32 panel consumes slightly more cycles despite fewer instructions. These single diagnostic panels support small work removal; they do not identify a causal cache/DRAM limit, critical path or replicated latency saving. Early profiles separately qualify with23,042 samples. No owned build/helper/hash/analysis/decoder/verifier/publication overlaps any recording, including preflights.

Every timed allocation count is zero and observed scratch capacities match controls. At200/201/352/353points, G1 record payload capacities follow the same1600-byte record and original tail storage; G2 prefix/tail capacities remain unchanged. The JSON retains per-process RSS, CPU service, faults and switches. Reported getrusage high-water counters are not exact lifetime peaks or minimumRAM, and no stack/pressure/device claim follows from equal heap payloads.

Actual linked G1/G2 functions retain800/1450IFMA sites. Whole-function instruction rows change4435→4416 and8600→8517; static counts include cold/alternate paths and are not dynamic spills or speed estimates. Each of the two final binaries is fully decoded once; selected functions and hashes are archived.

## Qualification and retained failures

OFF and ON each pass two new focused native tests:23,472 checked point states, all 256 sign masks at eight points, full/tail/boundary lengths0–511, owned/signed-borrowed G1, borrowed G2, canonical modes, scalar/vector inversion, scratch policies and repeated updates. Source points stay unchanged. Each arm also qualifies 53 workers across1/4/16/32 pools on512 KiB stacks with stable capacities.125 unchanged core tests per arm are filtered and inherited, as are existing field/prover suites. Generic noASM/noIFMA core compilation passes; no ARM/mobile runtime is measured.

The initial workflow completes six core-build/test steps, then caller linking fails because the inherited link search path omits the reused transitive field library. A three-step continuation adds only that dependency directory to both links and performs the generic compile. Passed builds/tests are not repeated. Initial link command, stderr and repair are retained. The first full-plan preparation then lacks protocol.md after successfully saving host.json; the repaired preparation uses that exact saved host record. Full target names include both actual caller namespaces before the first full recording. One postcapture invocation used the unsupported `analyze` subcommand and performed no analysis; the actual `finish` mode then passes once. Preparation/mode failure notes describe tool-transcript observations where separate raw streams were not saved.

Independent early/full readers reconstruct all matched ratios/gates and validate command/environment/exit/stream bindings, outputs/resources, PMU and profile ownership/quality. Full review passes all1279receipts/1188callers/78timing checks as data validation while retaining the failed adoption gate. Native qualification checks original-source recovery and exact inherited field identity. Successful tests, verifiers, captures and decoders are not rerun for bookkeeping.

Across early and full captures,1292checked caller processes execute221852032kernel batches including primes; these are not proofs. Native 9 successful commands, codegen4, early125 and full1279 provide1417successful workflow receipts, plus the retained failed link. No new SDK request or exported proof is created; canonical35314 remains unchanged.

## Accepted prover comparison and next action

All proof timings below are inherited: latest primary accepted control8.010122ms and seven earlier full-campaign observations. Earlier7.915095/7.928660/7.995682ms and8.002770ms used the same accepted executable; the inter-run cause remains unconfirmed without an adopted regression. Supplied M5 values use different hardware.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.010122|37 /4.619×|24 /2.996×|3.7|
|2×2|11.241463|59 /5.248×|30 /2.669×|5.9|
|4×2|15.933816|109 /6.841×|48 /3.012×|10.9|
|8×2|24.234311|189 /7.799×|74 /3.054×|18.9|
|12×2|33.855565|284 /8.389×|101 /2.983×|28.4|
|16×2|40.071563|335 /8.360×|124 /3.094×|33.5|
|24×2|65.917462|528 /8.010×|176 /2.670×|52.8|
|Merge24×1|65.305023|507 /7.764×|175 /2.680×|50.7|

Merge54 remains 162.240109 ms without a supplied M5 reference. The original all-eight verified CPU10×goal remains active and unmet,0/8. No new SDK, AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, minimumRAM or service-concurrency result is claimed.

Close this fixed uniform carry-fusion implementation without a favorable subset or unchanged reroll. Retain the qualified arithmetic, G2 gain and small retired-work reduction. They do not supply the requested twofold proof improvement. The next research step needs a distinct reduction in dominant MSM work, with complete cost and caller evidence; no new runtime candidate is selected by this report.

[Design](artifacts/design.md),[full protocol](artifacts/full-protocol.md),[native qualification](artifacts/qualification.json),[source recovery](artifacts/source-audit.json),[actual codegen](artifacts/codegen.json),[early analysis](artifacts/early-analysis.json),[early review](artifacts/early-review.json),[full analysis](artifacts/full-analysis.json),[full review](artifacts/full-review.json) and[publication](artifacts/publication.json) accompany[the evidence archive](artifacts/evidence.json.gz). The gzip JSON map stores relative-name UTF8 payloads with lengths and hashes. It includes raw caller streams, PMU CSV, full decoded profiles, plans/pins, source deltas/helpers and receipts. Full disassemblies, binaries/libraries/rawperf, unchanged full source, actual keys/images/witnesses/proofs/public arrays and full private ledgers are omitted; compiled identities/full-decode hashes remain bound. [Archive index](artifacts/archive-index.json) and[checksums](SHA256SUMS) cover the publication.
