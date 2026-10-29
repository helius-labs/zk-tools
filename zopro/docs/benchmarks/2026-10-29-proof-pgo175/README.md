# Current-source proof-only PGO: SDK pilot

**The primary latency pilot passes; broader qualification is required before adoption.** Candidate latency is 7.924730 ms versus 7.988771 ms accepted and 8.032753 ms fresh OFF. Paired improvements are 0.828% against accepted and 1.320% against OFF, with savings of 66.155 µs and 104.678 µs. The candidate wins 13/15 and 15/15 blocks; both gain bounds and control calibration pass. This is a small verified primary gain, not a 2× breakthrough or all-eight qualification. Production remains unchanged.

## Compiler change and controls

All 175 prover source files are byte-exact to accepted production. This candidate uses one fresh proof-only profile to guide the current compiler; there are no new arithmetic algorithms, key tables, point copies, plans, memory caps or runtime tuning. The previous 22 October PGO study used older source with a 13.6 ms baseline, two preparation profiles and primary weighting; its positive estimates and failed old gate remain recorded. No old profile or rejected source candidate is reused.

Rust 1.98.1 / LLVM 22.1.8 generate and consume the profile. Explicit Cargo target keeps host build scripts outside target instrumentation. GEN uses atomic counter updates; its instrumentation can perturb scheduling and is not claimed negligible. USE has no profiling runtime symbols/sections or instrumentation buffers. Separate target directories and immutable hash-named libraries prevent accidental mixing. Accepted SDK is `20fde19f…`, fresh plain `2e3634f0…`, USE `144621f1…`; plain differs from accepted and must pass calibration. The new link passes the baseline cfg flags explicitly; this difference is recorded without assigning it an exclusive timing cause.

The workflow follows the [rustc PGO guide](https://doc.rust-lang.org/rustc/profile-guided-optimization.html) and [LLVM profile tools](https://llvm.org/docs/CommandGuide/llvm-profdata.html). The [LLVM 22 dump implementation](https://github.com/llvm/llvm-project/blob/release/22.x/compiler-rt/lib/profile/InstrProfilingFile.c) explains the one-dump behavior used to stop collection before exports. Upstream is a reference, not asserted byte-identical to the installed runtime. Actual tool hashes, profiles and counter audits are retained.

## Fixed training and provenance

One process per supplied shape, including 54→1, uses 32 workers on CPUs 0–31. Mmap loading, SDK construction and three complete priming requests precede counter reset. Eleven fresh SDK requests follow reset, then one profile dump occurs before proof/public exports and log formatting. All nine captures finish before 99 independent Gnark verifications and merging. Nine disjoint raw profiles are combined with equal weights, without a primary weight, cap variant, profile choice or training extension. Training times are not speed evidence.

The 126 SDK requests verify internally, including 27 excluded primes. All 99 recorded 164-byte proof exports independently pass Gnark and exact image/public binding and freshness. Ledger 18,670→18,769. The merged profile contains 6,229 function records and audits exactly 99 SDK proving calls, zero key loads and zero SDK constructions. The count checks use the selected records' maximum block counts, cross-checked with the fixed request/export counts; generic loop maxima are not treated as function-call counts. Nine raw profiles and the 2.5 MiB merged profile are retained byte-exact.

Two parser tests pass: one reconstructs the prior real IR-counter format and one checks zero/full-u64 counts. The old counter log is only a parser fixture, not a training input. Current-source native plain and USE each freshly pass all 33 field, 124 core and 8 prover tests. Existing arithmetic inputs are reused across these distinct compiler builds; no new mathematical-case count is claimed.

The linked audit retains nonzero training records for the hot G1 stream, borrowed G2 apply, balanced K/H feed and SDK proving function. No matching hot missing-profile or mismatch warning appears in the normal consumer library/link logs. All missing-function warnings, including unseen test and driver functions, remain archived. This warning/symbol check is not full optimization-remark coverage. PGO can inline away a standalone definition; symbol absence alone is not proof that work disappeared. [Training audit](artifacts/training-audit.json), [linked audit and warnings](artifacts/code-audit.json), [source audit](artifacts/source-audit.json).

## Complete verified latency pilot

Ryzen 9950X, 16 physical / 32 logical CPUs; 32 workers pinned to CPUs 0–31. All nine shapes × three variants pass native preflight. Fifteen fixed rotated/reversed primary blocks each measure 151 complete SDK requests after three primes. Loading is excluded; witness solving, fresh randomized proving, SDK verification and serialization are included. Medians below need not reproduce within-block paired estimates.

|Variant|Median ms|CPU ms/request|Median/max RSS MiB|M5 CPU speedup|M5 Metal speedup|
|---|---:|---:|---:|---:|---:|
|accepted|7.988771|180.976|129.00/133.50|4.632×|3.004×|
|OFF|8.032753|181.528|130.75/133.25|4.606×|2.988×|
|ON|7.924730|178.232|130.25/132.25|4.669×|3.028×|

RSS is process high-water including loading/primes, not minimum required RAM. Smaller observed peaks do not establish a memory-pressure or mobile improvement.

|Paired latency comparison|Ratio|Rank interval|Wins / 15|Saving µs|
|---|---:|---:|---:|---:|
|1x2-w32/OFF/accepted|1.003080634|0.998415298–1.011906179|4|-24.346|
|1x2-w32/ON/OFF|0.986795194|0.982345646–0.993194738|15|+104.678|
|1x2-w32/ON/accepted|0.991719002|0.987876367–0.996250474|13|+66.155|

The predeclared pilot requires both ON/control upper ratios below one with at least 10/15 wins, and OFF/accepted intervals inside [1/1.03,1.03]. Every condition passes. Ranks 4–12 of 15 have nominal 96.484375% individual coverage conditional on independent exchangeable blocks, without a familywise guarantee. There is no minimum positive-gain floor, exclusion, extension, successful rerun, profile choice or subset promotion. Passing permits the next qualification stage; it does not establish all-eight performance, compatibility, memory-pressure behavior, adoption or the 10× objective. [Design](artifacts/design.md), [all observations](artifacts/analysis.json), [candidate inputs for qualification](artifacts/candidate-manifest.json).

## CPU diagnostics

All 45 latency captures precede their Gnark verifications. Three FIFO-gated four-event stat runs measure 31 requests each, and three separate 199 Hz user-cycle profiles measure 251 each, after three excluded primes. All six diagnostics finish before verification/decoding. Events run at least 99.9%; each profile has one sampled process and zero lost records. No owned build/helper/hash/archive/other CPU work overlaps training or performance captures. The host is shared; exclusivity is not claimed.

|Variant|Cycles/request, million|Instructions/request, million|Cache misses/request, million|Branch misses/request, million|
|---|---:|---:|---:|---:|
|OFF|828.865|1420.320|2.978|1.201|
|ON|828.385|1365.130|3.089|1.206|
|accepted|831.678|1419.322|2.974|1.198|

|Profile|Samples|Lost|
|---|---:|---:|
|cycles-1x2-w32-OFF|8967|0|
|cycles-1x2-w32-ON|9130|0|
|cycles-1x2-w32-accepted|9100|0|

Original decoded streams and raw/demangled symbol weights remain available; raw perf data stay local with hashes. Static code, single counter runs and sampled periods do not isolate a register, cache, scheduling or inlining cause, nor removable wall time. Inlined work can move to enclosing symbols.

## Verification and preparation correction

The performance pilot completes 7,902 internally verified SDK requests and 78 fresh independent Gnark exports, including all-nine preflight and diagnostics. Proof/image/public bindings, 164-byte sizes and uniqueness against 18,769 prior identities pass; ledger is 18,847. Across training and performance: 8,028 SDK requests, 177 new independent proofs and 354 proof/public members.

Phase A completes 9 native steps, training 109, USE qualification 6 and performance 165. The archive retains 289 receipts / 1,156 original streams, all 175 sources, all proof/public members, raw/merged PGO data and counters, with checked hashes and gzip round trips. These are separate workflows, not one native receipt. The linked audit ran directly with observed exit zero and retained `audit.log`. [Closeout](artifacts/closeout.json).

The first performance preparation stopped before creating identities or a run plan because it expected the old `native-flags.txt` filename. The controller was corrected to read `control-flags.txt`, then preparation passed before the first capture. The generator produces the preserved initial controller; the one-filename repair record reconstructs the final version. Initial sources, original helper hash and observed direct failure are retained. No source arithmetic, compiler flags, profile, SDK, measurement gate, successful build/test/proof or timing capture was changed or repeated. [Failure](artifacts/initial-capture-controller/failure.json), [repair](artifacts/initial-capture-controller/repair.json).

## Inherited all-eight CPU and Metal comparison

These accepted-production times use the supplied historical M5 references. Only primary latency above is newly measured for the candidate; all-nine native preflights qualify correctness, not other-shape latency.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted primary remains 8.022165 ms / 134 MiB maximum RSS; 54→1 remains 162.250464 ms without a supplied M5 reference. Its full performance gate remains false with the recorded physical16 calibration engineering exception; final artifact gate remains true. No candidate physical16, portable/mobile/TEE, pressure/swap/minimum-RAM, concurrency, 250/4999, AWS or Aeglos qualification is claimed.

Next is all-eight/54 latency and physical16 qualification, followed by required compatibility/resource/final-artifact checks if those pass. There is no production adoption yet. The all-eight CPU 10× goal remains active and unmet; this candidate primary is about 4.67× the supplied M5 CPU reference and still above its 3.7 ms target.
