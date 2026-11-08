# Combined scaled prefixes and X-sum carry removal: SDK pilot

**The combined candidate passes the primary latency pilot.** Local 1×2 takes **7.869400 ms**, versus accepted **7.988685 ms** and fresh OFF **8.031154 ms**. Paired estimates favor the candidate by **1.617% against accepted** and **1.910% against OFF**, with both intervals below parity. This selects full-shape qualification; **nothing is adopted yet**, and the all-eight CPU 10× goal remains unmet.

Sub-8-ms results predate this experiment. The earlier uniform-prefix candidate measured **7.859756 ms**, while the same accepted executable has measured **7.915095 ms**. The new candidate's absolute median is about 9.6 microseconds above that earlier candidate median across separate runs. This experiment establishes a small gain against its contemporaneous controls; it does not establish a new absolute record or an improvement over the earlier candidate.

## Change and prior evidence

This is a new composition of two qualified changes: [scaled prefixes](../2026-11-07-uniform-prefix-native176/README.md) and [typed uncarried X sums](../2026-11-08-linear-carry-callers176/README.md). The first uses uniform R260 multiplication inside selected prefixes and restores the ordinary R256 slope. The second constructs an unnormalized sum only from canonical host X coordinates and subtracts it from the unchanged ordinary slope square. That shared boundary makes the combination algebraically compatible; fresh coupling tests and complete proofs test the resulting artifact.

Both changes are enabled together in ON and disabled in OFF. G1 uses the existing 1600-byte/alignment-64 record; G2's FAST borrowed finish uses two ephemeral sums. No new heap buffer, point/key table, key copy, retained solver plan, production pool or fixed RAM cap is added. Scalar tails, exceptions, input/key representation, batch geometry and resource forecasts keep their parent behavior. Unsupported native features retain their original fallback expressions.

The separate [uniform-prefix pilot](../2026-11-07-uniform-prefix-pilot176/README.md) had a positive estimate with an interval narrowly crossing parity. The separate carry caller screen failed its declared checks despite positive aggregate estimates. Those results remain unchanged and are not pooled here. This is the first measurement of their combined implementation, with newly coherent OFF/ON libraries and SDKs. No claim that isolated gains add is used for selection.

## Complete proof measurements

Ryzen 9 9950X, 32 logical workers, process affinity CPUs 0–31. Thirty-one fixed rotated/reversed blocks compare all three executables. Each process runs three primes and 151 measured complete SDK requests. Solving, fresh randomized proving, internal verification and serialization are timed; loading and primes are outside. All observations remain, without exclusions, extra blocks, subsets or unchanged rerolls.

|Arm|Median process mean, ms|Largest latency-process maximum RSS, MiB|
|---|---:|---:|
|Accepted|7.988685|135.75|
|Fresh OFF|8.031154|133.25|
|Combined ON|7.869400|135.00|

|Paired comparison|Median ratio|Fixed-rank interval|Wins /31|Result|
|---|---:|---:|---:|---|
|OFF / accepted|1.001151|0.998201–1.009151|14|Calibration passes|
|ON / OFF|0.980896|0.976305–0.986858|30|Gain passes|
|ON / accepted|0.983830|0.978613–0.987447|28|Gain passes|

Ratios pair process means within each block; they need not equal quotients of the displayed marginal medians. Bounds use zero-based ranks10/22, median15, of31 paired ratios. They assume exchangeable blocks and are not familywise guarantees. Both gain comparisons require upper<1 and at least21wins. OFF/accepted calibration must fit factor1.03. The RSS screen uses the larger control peak×1.05+1024KiB, not an architectural memory cap. All seven selection checks pass, including diagnostic quality. No minimum positive-gain floor is applied.

Process RSS includes key loading and primes; these observations are not minimum RAM or device/pressure qualification. The maximum ON value is below accepted and above fresh OFF. Equal scratch payload does not establish equal whole-process peak memory.

## CPU counters and profiles

All93latency processes finish before six diagnostic processes, all independent Gnark verifiers and all offline analysis. Counters use31warm requests; profiles use251, each after three primes with FIFO acknowledgments gating the measured region. Sampling is user cycles, period16million,512mmap pages. No owned build/helper/hash/analyzer/decoder/verifier/publication or other benchmark overlaps any recording, including preflights. Exclusive host use is not established.

|Counter per request|OFF|ON|ON / OFF|
|---|---:|---:|---:|
|User cycles|816,391,264|797,474,508|0.976828|
|Instructions|1,387,791,982|1,362,028,797|0.981436|
|Generic cache misses|2,823,625|2,715,713|0.961783|
|Branch misses|910,400|894,783|0.982846|

All four events report100%running time. These single panels support a small reduction in complete-proof work, not a repeated-counter confidence interval or a causal cache/DRAM/spill diagnosis.

Accepted/OFF/ON profiles contain12,301/12,282/12,060samples, zero LOST records, at most1.854%unknown symbols and qualifying SDK-main/32-worker ownership. FFT leaf presence and actual binary/MMAP/PID/TID/CPU/period bindings pass. Flat samples are not exclusive wall-time fractions. Detailed statistics and decoded profiles remain in the evidence archive.

Actual newly linked SDKs are each fully decoded once. The signed borrowed FAST G1 function has4438→4437instruction sites and800→800IFMAs; borrowed FAST G2 has8601→8489sites and1450→1450IFMAs. All14selected owned/borrowed/mode bodies are retained. Static counts include alternate paths and do not establish executed instructions, dynamic spills or a whole-call-stack bound. No static-instruction speed gate is used for this new composition.

## Qualification and provenance

The182-file snapshot starts from the exact179-file qualified uniform-prefix snapshot and adds three exact carry/oracle children with recorded reversible edits. Reversing those edits recovers all179parent files; all176accepted production hashes remain unchanged. Native OFF and ON each pass two focused coupling tests:23,472checked bucket states, all sign masks, full/tail boundaries, canonical/inverse/store modes and reuse. Each also checks53worker executions across1/4/16/32pools with512KiB stacks and stable scratch capacities. The127unchanged parent core tests per arm are filtered and inherited, as are the independent parent field qualifications.

Seven native workflow steps build coherent eight-library OFF/ON SDKs and pass a generic noASM/noIFMA compile with both experimental cfgs enabled. Actual final links use pinned Rust1.98.1, native optimization, fatLTO and one codegen unit. The private field append is not claimed newly generator-consistent; production integration must address that before adoption.

Fresh preflight covers all nine shapes and all three arms at32workers, plus primary at one and16workers:33processes,132internally verified requests and33fresh independent Gnark exports. Timing adds93latency and six diagnostic processes,15,186SDKrequests and99new exports. All132exported proofs are independently verified, bound to their image/public input/verification key, and distinct from prior inventory and one another. Canonical unique-proof inventory advances35,314→35,347→**35,446**. Unexported requests are internally verified; they are not counted as independent Gnark exports.

The independent reader checks native/preflight/capture receipts and stream hashes, source/binary/library pins, all proof bindings and inventory unions, and reconstructs paired timing estimates. It inherits detailed PMU/profile parsing and does not replay a request, native test, decoder or verifier. The132exports and15,318new SDKrequests are this study's totals, not new accepted production evidence for every shape's latency.

Preparation failures are preserved: the initial source auditor treated an unchanged binary fixture asUTF8; continuation reused the completed snapshot and byte-hashed unchanged files. A later plan selector used an incorrect inherited generic-step name; only pending plan generation continued. The codegen executable initially collided with its output-directory name and was moved before the first decoder ran. No arithmetic, measured protocol, data, successful build/test/capture or selection check changes for these repairs.

## Required next step and M5 comparisons

The passing primary pilot selects a new prospective all-eight plus merge54/primary16performance campaign, portability/resource checks and generated-source integration before adoption. No fresh all-shape performance matrix is claimed by the one-request preflight. The candidate remains private, and the original all-eight CPU10×goal remains active,0/8.

The candidate's measured primary is **4.702× the supplied M5 CPU reference** and **3.050× the supplied Metal reference**. The inherited/fresh accepted table below preserves every required shape; M5 references use different hardware. Only primary is freshly measured in this pilot.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2, fresh accepted|7.988685|37 /4.632×|24 /3.004×|3.7|
|2×2, inherited|11.241463|59 /5.248×|30 /2.669×|5.9|
|4×2, inherited|15.933816|109 /6.841×|48 /3.012×|10.9|
|8×2, inherited|24.234311|189 /7.799×|74 /3.054×|18.9|
|12×2, inherited|33.855565|284 /8.389×|101 /2.983×|28.4|
|16×2, inherited|40.071563|335 /8.360×|124 /3.094×|33.5|
|24×2, inherited|65.917462|528 /8.010×|176 /2.670×|52.8|
|Merge24×1, inherited|65.305023|507 /7.764×|175 /2.680×|50.7|

Accepted merge54 remains162.240109ms without an M5 reference. Earlier accepted primary7.915095/7.928660/8.010122ms used the same accepted executable; inter-run variation is not an adopted code regression. No new AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM or concurrent-proof result is added.

[Design](artifacts/design.md),[source recovery](artifacts/source-audit.json),[native qualification](artifacts/native-qualification.json),[actual codegen](artifacts/codegen.json),[protocol](artifacts/protocol.json),[latency analysis](artifacts/latency-analysis.json),[diagnostics](artifacts/diagnostic-analysis.json),[review](artifacts/review.json),[publication](artifacts/publication.json),[evidence archive](artifacts/evidence.json.gz),[checksums](SHA256SUMS). The archive preserves UTF8 source deltas/children/helpers, complete raw measurements, counters/decoded profiles, pins/plans and receipts. Rawperf, executable/library/key/image/witness/proof/public payloads, unchanged full source and private ledgers are excluded.
