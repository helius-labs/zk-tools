# First segmented common-Z SDK trial

**This integration does not establish a proof-speed gain and is not adopted.** The local 1×2 candidate measures 8.006460 ms, accepted production 8.002770 ms and fresh OFF 8.020467 ms. Both paired candidate intervals cross 1.0 and both win-count checks fail. Control calibration, correctness, memory screening and automatic profile quality pass. Production remains unchanged.

This tests the exact G1 segmented common-Z finish from the [earlier caller trial](../2026-11-07-common-z-callers176/README.md) at its first actual SDK boundary. The earlier trial's four calibration and six nonregression failures remain unchanged. Before preparation, builds or clocks, the new design explicitly overrode only its no-SDK-after-failure selection rule. All 30 same-binary caller nonregression checks had passed and small eligible callers improved, which justified gathering direct proof evidence; neither fact implied proof-level qualification. No old measurement is rerun or relabeled as passed.

The finish applies only to existing G1 unweighted task frontiers with at most 16 live points and some group of at least three. It shares denominator products and a projective scale across groups, normalizes final outputs once, and preserves odd carries, zero/singleton groups and transactional identity/equal-x fallback. Weighted one-level reduction, G2 and stream-add arithmetic retain their original behavior. The runtime option is read once in Scratch::default. There is no new field arithmetic, key lookup table, key copy, retained solver plan, heap scratch, production pool or allocator policy. Fixed stack arrays and compiler spills remain implementation costs.

All 176 current production inputs are checked: 175 private copies are exact, one msm.rs parent reverses exactly, and three old child files retain their arithmetic bytes; only an include path becomes local. Earlier diagnostic adapters are omitted. Field/generated arithmetic source remains exact; Cargo creates metadata-distinct coherent libraries, so byte-identical library binaries are not claimed. Existing field math qualification is inherited. Both eight-library SDKs retain native O3, fat LTO and one codegen unit. Sixty new linked functions are decoded once: three candidate G1 reducer consumers reach the original wrapper, which reaches the segmented kernel; the control has no such route. Stream G1/G2 IFMA site counts remain 800/1,450. Static route inspection is not a speed measurement.

Native qualification passes seven steps, including 127 core, two prover and six cubic tests, both SDK links and generic/noASM/noIFMA compilation. The offline qualification reader initially tried to decode an unchanged binary fixture as UTF-8; its failed source/binary/log are retained, and only that reader was corrected. A capture-helper build needed a larger macro recursion limit for protocol JSON; its original source and repair record remain. Successful builds, tests, decoders and captures are not repeated for bookkeeping.

Twenty-two fresh preflight exports pass Gnark verification across both SDKs and all nine shapes at 32 workers, plus primary at one/16 workers. Their 88 SDK requests establish correctness, not comparative speed. The native core tests explicitly enable the candidate. Preflight outputs and all proof/public/image/verifying-key bindings and freshness are checked by a separate reader.

Ryzen 9 9950X, 16 physical cores/32 logical CPUs, shared host. The fixed primary pilot runs 31 rotated/reversed blocks of accepted, fresh OFF and candidate on CPUs 0–31, 151 fresh requests after three primes per process. Timing includes solving, fresh proof randomness, proving, internal verification and serialization; loading is excluded. All 93 latency processes finish before three PMU panels and three cycle profiles. All 99 Gnark verifiers and 15 ownership/decode/quality commands follow recording. No owned build/helper/hash/analysis/report/push overlaps any recording, including preflights. No observations are excluded, extended or repeated.

|Comparison|Median paired ratio [fixed rank interval]|Wins / 31|
|---|---:|---:|
|Candidate / fresh OFF|0.997445585 [0.996131325, 1.001050926]|19|
|Candidate / accepted|0.999396831 [0.997366072, 1.006749094]|16|
|Fresh OFF / accepted|1.002627760 [0.997302364, 1.005900045]|14|

The prospective rule uses zero-based sorted ratio ranks 10/15/22 of 31 process means. Both candidate upper bounds must be below 1.0, with at least 21 winning blocks against each control; there is no minimum-gain floor. Fresh OFF/accepted must stay within factor 1.03. Four gain/win-count checks fail. Calibration passes, so this result is not rejected merely because a fresh control moved outside the declared range. This fixed integration closes without an all-eight candidate campaign, adoption or a threshold/layout/compiler/worker/host rescue.

|Arm|Median process mean ms|Maximum process RSS MiB|User CPU ms/request|System CPU ms/request|Minor faults/request|
|---|---:|---:|---:|---:|---:|
|accepted|8.002770|135.25|174.679|5.003|129.132|
|control|8.020467|135.50|174.262|5.073|130.132|
|candidate|8.006460|136.75|174.228|5.088|128.874|

RSS includes loading, priming and process high-water behavior. The candidate passes the comparison screen of max(accepted, OFF) × 1.05 + 1 MiB. These are not minimum-RAM, memory-pressure, swap, phone or concurrent-proof measurements. Minor faults are serviced without disk I/O; these counts do not establish a latency cause.

|Arm|User cycles/request|Instructions/request|Cache misses/request|Branch misses/request|Profile samples|Common-Z samples|
|---|---:|---:|---:|---:|---:|---:|
|accepted|811705746|1387914049|2720127|904574|12246|0|
|control|809943005|1388661479|2687517|897433|12257|0|
|candidate|810821887|1389452532|2738293|891345|12353|2|

All three profiles qualify: 36,856 samples, zero LOST, unknown shares below 1.8%, and 100% of samples on the assigned execution threads. The assigned set is the SDK main thread plus 32 Rayon threads recovered from task metadata, within taskset CPUs 0–31. No fixed per-thread CPU placement is claimed. Counters run at 100% coverage. Sampling uses cycles:u, fixed period 16,000,000, 512 mmap pages and acknowledged FIFO enable after primes / disable before final output checks. PID/TID/CPU/MMAP and complete raw symbols are retained.

Two candidate samples land in common_z_finish::arithmetic::segments; neither control has common-Z samples. This confirms runtime execution. That small flat sample count does not measure branch-hit frequency, successful speculative finishes, critical-path contribution, or the speedup available from removing the work. The earlier twice-as-fast sixteen-point caller does not predict a twice-as-fast proof. One PMU panel/profile per arm is diagnostic, not a replicated causal experiment.

The capture adds 15,186 fresh SDK requests and 99 independently verified exports. Including preflights, this integration adds 15,274 requests and 121 fresh verified exports, extending the canonical inventory from 35,050 to 35,171. Both capture analysis and the independent reader pass on first execution and reconstruct all 213 commands, 99 SDK outputs/Gnark bindings, 31 paired blocks and seven selection checks. There are 626 frozen capture input pins. All raw samples and failed checks remain available.

The latest accepted primary is slightly above 8 ms. The same unchanged SDK previously measured 7.915095/7.928660, 8.018344 and 7.995682 ms locally; the cause of variation between runs remains unconfirmed. The other seven shapes below are **inherited** from the [earlier full SDK campaign](../2026-11-07-fused-epoch-sdk176/README.md), with no new matched common-Z timing for them. Supplied M5 references use different hardware.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.002770|37.0|24.0|4.623×|2.999×|3.7|
|2x2|11.241463|59.0|30.0|5.248×|2.669×|5.9|
|4x2|15.933816|109.0|48.0|6.841×|3.012×|10.9|
|8x2|24.234311|189.0|74.0|7.799×|3.054×|18.9|
|12x2|33.855565|284.0|101.0|8.389×|2.983×|28.4|
|16x2|40.071563|335.0|124.0|8.360×|3.094×|33.5|
|24x2|65.917462|528.0|176.0|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507.0|175.0|7.764×|2.680×|50.7|

The original verified CPU 10× goal remains active and unmet, **0/8**. Accepted merge54 remains inherited at 162.240109 ms without an M5 reference. No AWS/Aeglos, 250/4999, ARM/mobile/TEE, pressure/swap or minimum-RAM result is added. Production stays at accepted SDK 6d83.

The [evidence archive](artifacts/evidence.json.gz) retains prospective plans/pins, helpers, changed-source text, native/portable receipts, linked disassembly, raw SDK latency outputs, full decoded profiles, PMU CSVs and the full independently reviewed latency analysis. Decompression produces relative paths mapped to {utf8,bytes,sha256}; every extracted payload is verified. The public latency view omits only the duplicated per-job rows, whose exact bytes remain archived; publication metadata distinguishes the raw reviewed-analysis hash from the public-view hash. Binaries/libraries, raw perf data, proving images, proof/public payloads, private ledgers and full source copies are excluded, with identities retained.
