# Mixed scalar and SIMD G1 streaming

The fixed eight-SIMD-plus-one-scalar schedule is slower in all twelve caller contexts and is not adopted. Each context has **0/15 winning blocks against both original controls**. Calibrated one- and sixteen-worker cases are approximately 20–28% slower; the thirty-two-worker paired estimates are 28–39% slower against OFF. Three thirty-two-worker control calibrations fail and remain in the result, so those estimates do not support a precise regression size. All twenty-four gain checks fail independently of calibration and RSS.

The candidate handles eight ordinary affine additions with the existing AVX-512 field backend and a ninth with the existing scalar backend. Eight vector prefix chains and one scalar prefix share a single inversion. The reverse pass interleaves scalar and vector products. Remaining points extend the scalar chain. No field representation, arithmetic primitive, point formula, key, lookup table, solver plan, thread pool policy or production dispatch changes. The new private inverse helper composes existing field operations. The component directly calls the real original or candidate streaming kernel. Owned calls use unsigned pre-signed points; borrowed calls carry signs in bucket indices. Candidate record construction and loop organization also differ from the original, so this result rejects this implementation without proving that every mixed arithmetic schedule is slower.

Fifteen fixed rotated/reversed blocks compare a separate original build, fresh OFF and ON in one candidate binary. Both final links use release optimization, native CPU, fat LTO and one codegen unit. Each process creates one independently evolving fixture per configured worker, runs three 128-batch primes, then 4,096 timed batches per fixture through Rayon. Every prime and the final timed state is checked against generator multiples calculated independently of the affine streaming formulas. Checks, fixture construction and resets are outside timing; dispatch completion is included. A final-state oracle is not a check of every intermediate batch state. Measured allocation counts are zero and scratch capacities stay stable. Inputs use valid distinct destinations/nonzero denominators, as required by this internal kernel; upstream exception handling is unchanged.

Ryzen 9950X, sixteen physical cores and thirty-two logical CPUs, shared host. Worker affinities are CPU 2 for solo, CPUs 0–15 for sixteen and CPUs 0–31 for thirty-two, with 512 KiB stacks. Fixtures are Rayon tasks; this is a complete streaming-call throughput experiment, not single-proof latency. Original source 176 is recovered exactly from both private snapshots after removing the three diagnostic additions. No field source changes.

The prospective gate requires each of the twelve ON/control upper bounds below one and at least 10/15 wins against both controls, each OFF/original interval within factor 1.03, and each maximum process RSS ON/control at most 1.05. Bounds are ranks 4–12 of fifteen ratios, nominal 96.484375% marginal coverage under independent exchangeable blocks, without familywise coverage. All observations and the original gate remain; no subset, parameter rescue, extension or unchanged reroll is used. There are **30 failed checks: 24 gain, 3 calibration, 3 RSS**.

|Caller/batch/workers|Original / OFF / ON µs per wave|ON/OFF ratio [bounds]|ON/original ratio [bounds]|ON max RSS MiB|
|---|---:|---:|---:|---:|
|owned-200-w1|4.050 / 4.050 / 4.971|1.226433 [1.225541, 1.229928]|1.227846 [1.225907, 1.229019]|2.50|
|owned-200-w16|4.597 / 4.648 / 5.775|1.242318 [1.238879, 1.246198]|1.257250 [1.247446, 1.263898]|3.75|
|owned-200-w32|7.470 / 6.790 / 9.404|1.361545 [1.094049, 1.607578]|1.357944 [1.185825, 1.443400]|5.00|
|owned-352-w1|6.650 / 6.650 / 7.957|1.195750 [1.195029, 1.197347]|1.195375 [1.194676, 1.198999]|2.50|
|owned-352-w16|7.626 / 7.643 / 9.363|1.225935 [1.223426, 1.231114]|1.228792 [1.226987, 1.231725]|5.50|
|owned-352-w32|13.650 / 11.508 / 15.267|1.280116 [1.069372, 1.368947]|1.119940 [1.080527, 1.375866]|8.50|
|borrowed-200-w1|4.210 / 4.206 / 5.285|1.255408 [1.253733, 1.256725]|1.254650 [1.252727, 1.259006]|2.50|
|borrowed-200-w16|4.795 / 4.793 / 6.126|1.277640 [1.274727, 1.281001]|1.278033 [1.273683, 1.281652]|4.00|
|borrowed-200-w32|8.115 / 6.921 / 9.538|1.394100 [1.351383, 1.431593]|1.379526 [1.150192, 1.399540]|5.00|
|borrowed-352-w1|6.929 / 6.926 / 8.454|1.220465 [1.219637, 1.222099]|1.220414 [1.219046, 1.222045]|2.50|
|borrowed-352-w16|7.936 / 7.957 / 9.869|1.241329 [1.234669, 1.243550]|1.242671 [1.240258, 1.245760]|5.50|
|borrowed-352-w32|11.662 / 12.021 / 16.296|1.328343 [1.094019, 1.612025]|1.351815 [1.113253, 1.616128]|8.50|

Record capacity per fixture falls from 40,000 to 39,040 bytes at batch 200 (−2.4%) and 70,400 to 68,800 bytes at 352 (−2.27%). This excludes points, fixtures, allocator state and workers. RSS is whole-process lifetime high-water, not incremental scratch or minimum RAM. The small RSS guard failures are retained; neither lower payload nor zero allocations establishes a full-prover memory benefit. All raw minor faults and context switches are in the caller receipts.

All 127 native core tests pass, including two new tests for the nine-chain inversion and complete generator oracles across 0/1/7/8/9/10/17/18/23/199/200/352/511 points, both caller modes and scratch reuse. The initial build failed before tests because the diagnostic fixture omitted a G1Projective import; the sole repair is retained alongside that original source/log. Existing field/portable/SDK qualification is inherited, not rerun or newly claimed.

All 540 latency processes precede six grouped hardware-counter panels and six fixed-16M-user-cycle profiles. The six counters have 100% running time. Independent review validates 33,856 samples, expected PID/TID/CPU membership, event attributes and no lost records; every profile exceeds 1,000 samples and has less than 0.05% unknown symbols. Sampling does not establish absolute cycle coverage or removable wall time.

|32-worker counter context|OFF / ON instructions per batch wave|OFF / ON cycles per batch wave|
|---|---:|---:|
|owned-200-w32|1631830 / 2564955|917396 / 1225504|
|borrowed-352-w32|2725957 / 4372286|1570057 / 2096023|

The roughly 57–60% instruction increase and 33% cycle increase support rejecting this schedule; they do not isolate a CPU-port, spill or cache cause. ON work is inlined into Caller::run, while OFF mostly appears in stream_g1_impl. The static-code selector therefore does not isolate a standalone ON function; its RSP operands are not dynamic spill counts.

All 588 caller processes pass output and allocation checks. The workflow has 606 successful commands plus one preserved failed offline decode: perf created root-owned files with mode 0600, so user decoding initially lacked access. Ownership was repaired for the six task-owned files and every byte hash matched before/after. The original failed receipt remains. Workflow resume reused all 588 successful captures and ran only the eighteen decoders; no timing/counter/profile capture was repeated. Source/protocol/executable/compiler/log pins were checked across capture, and the independent reader recomputes every paired gain/calibration/RSS decision.

Production remains the accepted 176-source implementation. The following are its previously qualified local single-proof results, not measurements of this rejected candidate. The current experiment produces zero SDK/Gnark proofs and leaves the independently verified proof ledger at 27,851. No AWS, phone, cold-start, huge-circuit, memory-pressure or concurrent-proof result is added.

|Shape|Accepted CPU ms|Peak RSS MiB|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|10× CPU target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|132.75|37|24|4.667×|3.027×|3.7|
|2x2|11.151932|275.00|59|30|5.291×|2.690×|5.9|
|4x2|16.086386|312.25|109|48|6.776×|2.984×|10.9|
|8x2|24.179453|517.25|189|74|7.817×|3.060×|18.9|
|12x2|33.884455|706.50|284|101|8.381×|2.981×|28.4|
|16x2|40.077381|842.00|335|124|8.359×|3.094×|33.5|
|24x2|65.850777|1300.50|528|176|8.018×|2.673×|52.8|
|merge24|65.060937|1168.19|507|175|7.793×|2.690×|50.7|

Merge54 remains 161.883899 ms / 1876.96 MiB, without a supplied M5 reference. The original CPU 10× objective remains unmet: 0/8 shapes meet the target. [Accepted source and qualification](../2026-10-30-g2-vector-sign-adoption176/README.md). The accepted physical-16 calibration exception remains documented there.

The evidence favors reducing arithmetic/instruction work rather than adding a scalar side stream to this kernel. The fixed 9-point recipe is closed; there is no candidate SDK promotion. A new mechanism needs its own work model and complete-call evidence before full-proof qualification.

[Protocol](artifacts/declaration.json), [all latency/CPU/RSS rows and caller receipts](artifacts/latency-analysis.json), [independent review, counters and profile bins](artifacts/review.json), [workflow](artifacts/plan.json), [source/compiler pins](artifacts/pins.json), [original source pins](artifacts/original-pins.json), [exact source snapshots](artifacts/source.tar.gz), [Rust helpers and driver](artifacts/workflow.tar.gz), [raw command receipts and build logs](artifacts/receipts.tar.gz), [counter files](artifacts/counters.tar.gz), [publication and recovery audit](artifacts/publication.json), [host](artifacts/host.txt). Executables, libraries, raw perf.data, private keys, scalars and witness inputs are not published.
