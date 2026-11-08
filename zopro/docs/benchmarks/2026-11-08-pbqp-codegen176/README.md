# PBQP allocation of accepted CPU arithmetic

**This compiler configuration is slower overall and is not adopted.** The all-24-context paired aggregate is 6.03% slower than fresh Greedy and 6.65% slower than the frozen accepted-source caller, with zero wins out of fifteen against either. The four-size G2 aggregate at 32 workers is 13.42% slower than fresh Greedy. G1's 32-worker interval includes parity. Correctness, equal scratch capacities, zero timed allocations and all eighteen profiles pass. No SDK integration is selected; accepted production remains unchanged.

The candidate changes final machine register allocation, retaining the accepted arithmetic and the same core bitcode and field library in both fresh arms. [LLVM's code-generator documentation](https://llvm.org/docs/CodeGenerator.html) describes PBQP as a cost-based register allocator and Greedy as the default. This study uses rustc 1.98.1, commit 48a229ceaefd4985c50990b14116b6d856af0985, with LLVM 22.1.8. Local compiler help exposes both allocators and allocation verification. No external speedup is imported.

Fresh OFF explicitly selects `-C llvm-args=--regalloc=greedy`; ON selects `-C llvm-args=--regalloc=pbqp`. Both enable `-C llvm-args=--verify-regalloc` at final fat-LTO links, retaining native O3, one codegen unit and debug1. The verifier is a compilation check. A compiled cfg checks only the diagnostic CLI arm label. PBQP applies across the final linked consumer rather than just one selected field multiply. There is no new production arithmetic, intrinsic, runtime memory allocator, flag, point table, copied key, retained plan or thread pool.

All 176 accepted source parents recover exactly: 174 private originals are byte-identical and two core parents only append a native diagnostic module/export. Two child files contain the inherited actual-kernel adapter and its focused test. Every field source and every original production core function body remains exact. Both caller executables reuse one newly compiled diagnostic core library with embedded bitcode and the exact existing accepted field1200 library. The existing frozen accepted-source caller is a contextual control, not the production SDK. Generator qualification is inherited by exact original source; it is not rerun.

Because allocation changes machine code, the original 35 native field tests and 126 core tests run under PBQP with fat LTO and allocation verification. This is fresh code-generation qualification using existing mathematical oracles, not an unchanged test replay. A focused OFF caller test also passes. The focused case covers signs, tails, scratch reuse and one/four/sixteen/thirty-two-worker pools with 512 KiB stacks. A generic/noASM/noIFMA compile check passes; it does not qualify PBQP on ARM or phone hardware. All ten qualification steps and subsequent readers succeed once. No successful build, test, decoder or recording is repeated for bookkeeping.

Actual new linked G1/G2 functions are decoded once per fresh arm. IFMA site counts remain 800/1,450. G1 instruction sites increase 4,425→4,534 and G2 8,590→8,923. ZMM stack-operand sites rise 226→575 and 500→1,015; all stack-operand sites rise 575→1,210 and 1,451→2,354. Function sizes rise 26,198→28,716 and 51,830→56,980 bytes. These static sites are not executed spill counts, stack peaks or a speed result. The dynamic panels below distinguish instruction retirement from static code size. No allocator/coalescing/scheduler/layout/attribute tuning follows clocks.

Ryzen 9 9950X, 16 physical cores/32 logical CPUs, shared host. Both curves, 200/201/352/353 points and one/16/32 workers give 24 contexts. Public changing generator-point fixtures have independent complete output checks after three 128-round primes and after 8,192 timed rounds per worker. Each clock includes dispatch, borrowed-reference construction, complete point arithmetic and inversion. Setup/reset/checks are outside timing. Diagnostic 512 KiB stacks pin solo to CPU 2, sixteen workers to CPUs 0–15 and thirty-two to CPUs 0–31. These are complete addition-caller times, not proof latency or arbitrary SDK input distributions.

Seventy-two preflights and a checker precede fifteen fixed rotated/reversed blocks: 1,080 latency calls, then eighteen PMU panels and eighteen cycle profiles, then ninety offline ownership/decode/metadata/event/LOST commands. Total: 1,279 commands and 1,188 checked caller processes. No owned build/helper/hash/analysis/report/push overlaps any recording, including preflights. No observations are excluded, extended or repeated.

The prospective rule uses paired sorted ratio ranks 3/7/11 of fifteen. All OFF/frozen intervals must fit factor 1.03; all candidate/control nonregression upper bounds must be ≤1.02. Each curve's four-size 32-worker aggregate and the all-24 equal-log aggregate must have upper <1 and at least ten wins against both controls. There is no gain floor. Marginal intervals are not simultaneous family-wise guarantees.

|Aggregate|PBQP / fresh Greedy, fixed rank interval|Wins / 15|PBQP / frozen, fixed rank interval|Wins / 15|
|---|---:|---:|---:|---:|
|G1|1.020277854 [0.960008423, 1.056089951]|5|1.015198243 [0.994546460, 1.100962992]|5|
|G2|1.134227035 [1.128027952, 1.138062596]|0|1.144855062 [1.136781561, 1.154035072]|0|
|ALL|1.060251452 [1.049944248, 1.068848646]|0|1.066511926 [1.062865146, 1.081869919]|0|

The study fails 61 of 78 timing checks: seven calibrations, all 48 nonregression checks and all six aggregate gain checks. Calibration failures are all four G1 sizes at 32 workers and G2 200 at one/16/32 workers. These remain visible and limit their frozen comparisons; neither they nor G1's uncertain 32-worker result establish a gain. Every context's PBQP/Greedy median exceeds one. The fixed configuration closes without a favorable curve/worker subset or another allocator recipe.

|Context|Greedy µs/wave|PBQP µs/wave|PBQP / Greedy, fixed rank interval|Wins / 15|
|---|---:|---:|---:|---:|
|G1-200-w1|4.234|4.337|1.024415619 [1.023008174, 1.025169846]|0|
|G1-200-w16|4.807|4.969|1.032865064 [1.028228794, 1.036514207]|0|
|G1-200-w32|6.975|7.266|1.041649888 [0.887870070, 1.067860964]|6|
|G1-201-w1|4.350|4.483|1.030095187 [1.029035307, 1.031204564]|0|
|G1-201-w16|4.936|5.117|1.036672268 [1.024914199, 1.039476646]|0|
|G1-201-w32|8.226|7.532|1.058114159 [0.903604921, 1.217480393]|6|
|G1-352-w1|6.964|7.130|1.024003682 [1.022802147, 1.024997362]|0|
|G1-352-w16|7.963|8.231|1.034818754 [1.032141777, 1.037126206]|0|
|G1-352-w32|11.703|12.288|1.054876740 [0.836148096, 1.065089876]|5|
|G1-353-w1|7.071|7.271|1.028071839 [1.026633181, 1.030021327]|0|
|G1-353-w16|8.101|8.378|1.034446577 [1.031820727, 1.037247255]|0|
|G1-353-w32|11.823|12.430|1.058433319 [0.858367041, 1.069681718]|4|
|G2-200-w1|7.959|8.319|1.044626153 [1.043476726, 1.046529210]|0|
|G2-200-w16|9.220|9.808|1.064134106 [1.062370338, 1.067523507]|0|
|G2-200-w32|14.694|16.688|1.124191982 [1.111658246, 1.148052583]|0|
|G2-201-w1|8.204|8.875|1.083566418 [1.081762173, 1.084769170]|0|
|G2-201-w16|9.521|10.451|1.097181980 [1.091942752, 1.100579011]|0|
|G2-201-w32|15.064|17.224|1.144024537 [1.129110711, 1.154090341]|0|
|G2-352-w1|13.284|14.114|1.061774338 [1.060173017, 1.062907052]|0|
|G2-352-w16|15.471|16.723|1.080715920 [1.078392455, 1.083195308]|0|
|G2-352-w32|24.927|28.124|1.128957689 [1.125731168, 1.134892119]|0|
|G2-353-w1|13.508|14.720|1.089370402 [1.086515250, 1.090485082]|0|
|G2-353-w16|15.743|17.395|1.104081499 [1.101873488, 1.106555899]|0|
|G2-353-w32|25.402|28.813|1.135828303 [1.131517708, 1.140565415]|0|

All output hashes/oracles match, with zero timed allocations and exact equal retained record/tail capacities. G1 capacities for the four sizes are 40,000/40,640/70,400/71,040 bytes per worker; G2 capacities are 16,000/17,280/28,160/29,440 bytes. Maximum observed PBQP caller RSS is 18.50 MiB, including setup. These buffers and caller RSS do not bound compiler-generated stack, full-prover peak, minimum RAM, pressure/swap, phone feasibility or concurrent-proof throughput. No memory-cap architecture change is introduced.

|Diagnostic context|Instructions PBQP/Greedy|Cycles PBQP/Greedy|
|---|---:|---:|
|G1-200-w1|0.993747|1.020808|
|G1-200-w32|0.993726|1.033197|
|G1-352-w32|0.993017|1.024919|
|G2-200-w1|1.029065|1.044031|
|G2-200-w32|1.029038|1.055324|
|G2-352-w32|1.030054|1.064913|

In these single panels, G1 retires about 0.6–0.7% fewer instructions despite its larger function and more static stack sites, while cycles rise about 2.1–3.3%. G2 instructions rise about 2.9–3.0% and cycles 4.4–6.5%. Cache-miss changes vary by context. This does not isolate spills, memory latency, instruction scheduling or register assignment as the exclusive cause. Static stack sites do not count dynamic spills. The complete paired caller comparison determines rejection; these results apply to this pinned configuration and workload.

All eighteen profiles qualify: 123,085 samples, zero LOST, maximum unknown 0.143% and minimum target share 99.844%. Grouped counters meet 99.9% running coverage. Sampling uses cycles:u, fixed period 16,000,000, 512 mmap pages and acknowledged FIFO enable after primes/disable before output checks. Assigned workers, PID/TID/CPU/MMAP and raw symbols are checked. Profiles are flat CPU observations, not critical-path wall-time fractions.

Both analyzers pass on first execution. The independent reader reconstructs 401 pins, all 1,279 command receipts, 1,188 caller outputs, 78 timing checks and eighteen profiles. Its pass confirms evidence consistency, not a passing performance gate. All readers/helpers/plans are compiled and pinned before any clock. Compilation timing is not part of the caller result.

This trial adds no SDK requests, proofs or cloud resources. Canonical inventory remains 35,171. All proof results below are **inherited** from the [previous report](../2026-11-08-normalize-prefix176/README.md), including its latest matched 1×2 and seven earlier full-campaign rows. The same accepted binary previously measured 7.915–7.929 ms locally; its latest 8.002770 ms is a later observation with unexplained variation, not an adopted code regression. Supplied M5 references use different hardware.

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

The original verified CPU 10× goal remains active and unmet, **0/8**. Merge54 stays inherited at 162.240109 ms without an M5 reference. No new AWS/Aeglos, giant-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM or concurrent-proof comparison is added. This fixed PBQP configuration does not select an SDK trial or adoption.

The [evidence archive](artifacts/evidence.json.gz) retains prospective designs/plans/pins, diagnostic source/helpers, native/portable receipts, actual linked disassembly, raw caller outputs, complete decoded profiles, PMU CSVs and the complete independently reviewed analysis. Decompression yields relative paths mapped to {utf8,bytes,sha256}; all extracted payloads are verified. The public analysis omits only duplicated command/caller maps; complete original bytes stay archived. Binaries/libraries, raw perf data, keys/images/proof payloads, private ledgers and full accepted source copies are excluded, with relevant identities retained. The compiler version is included; full local hidden-option help is identified by its pinned hash.
