# Parallel final carry normalization

**This implementation is slower and is not adopted.** The all-24-context aggregate is 6.00% slower than fresh OFF and 6.28% slower than the frozen accepted-source caller, with zero wins out of fifteen against either. G1 and G2 both regress. Correctness, equal scratch capacities, zero timed allocations and all eighteen profiles pass. No SDK integration is selected; production and accepted proof times remain unchanged.

The candidate changes the four final radix-52 carry steps after eight-lane CIOS multiplication. It extracts high/low parts independently, calculates residual carry generate/propagate masks, then applies masked unit corrections. The exact output digits match the original serial shift/add/mask sequence. Quotient generation, input products, Montgomery factors, modulus, fused range correction and 100/150 IFMA product counts remain. This differs from earlier paired Montgomery-quotient cancellation, split coefficient planes and coefficient pre-add. No variant of the rejected five-product block is selected.

For B=2^52 and raw coefficients below 2^58, h_i=floor(t_i/B), l_i=t_i mod B, u_j=l_j+h_(j−1). Each residual carry is one bit because h_i<64. With g_j=[u_j≥B] and p_j=[u_j=B−1], c1=g1, c2=g2∨(p2∧c1), c3=g3∨(p3∧c2). Bottom digits become l0, u1 mod B, (u2+c1) mod B and (u3+c2) mod B; the top is t4+h3+c3. The sixth scratch coefficient is unchanged. This identity removes a serial vector carry recurrence at the expense of extra comparisons, masks and temporaries; it does not establish a machine-speed gain.

The motivating saved dependency graph and instruction calibration are inherited from the [earlier audit](../2026-11-07-fq8-dependency176/README.md), not rerun. Its optimistic ordinary critical path ends in the carry sequence. Those graph scores are neither measured kernel latency nor removable proof time. This experiment tests the complete caller rather than promoting the graph prediction.

The exact integer model passes 113,920 raw tuples: 38,880 small-radix boundaries, 7,776 native-radix boundaries, 1,728 constructed carry chains and 65,536 deterministic random cases. It checks exact serial digits and an independent arbitrary-width represented integer. An additional 81 generate/propagate/kill combinations check the boolean prefix law for both incoming carries and all prefix lengths. Two focused native tests pass 75,040 raw tuples/375,200 normalized digits/75,040 preserved sixth coefficients, plus 47,232 complete field outputs and 47,232 canonical exports across Fq/Fr and R256/R260. Full products include lazy boundaries through 2p, random inputs and changing chains, with an independent BigUint REDC reference.

Native ON passes 126 core tests, including a focused signed/tail/reuse/small-stack caller test. OFF passes that new test with existing tests filtered. A generic/noASM/noIFMA compile check passes with the diagnostic adapter native-gated. The 35 old field tests are inherited and filtered; the new arithmetic cases above are fresh. This is not a new proof or mobile-runtime qualification. The initial focused field-test build lacked two config imports; the failed source/log are retained and only imports were added. The successful field library and generator check are reused. A helper's borrow error and initial model spacing repair occur before any timing. No successful build, oracle, decoder, test or observation is replayed for bookkeeping.

All 176 accepted parents reverse exactly: 172 private originals are byte-identical; field/generator and two diagnostic core include/export parents are reversible. Four child files are added. Production core function bodies, scratch types, resource planner, solver, keys, queues and schedules are unchanged. Fresh OFF and ON use separate matching field libraries and coherent optimized core links. The frozen accepted-source executable is reused unchanged and is not the production SDK.

Actual linked functions are decoded once per fresh arm. G1/G2 retain 800/1,450 IFMA sites. G1 instruction sites rise 4,425→4,533 and G2 8,590→8,771; ZMM stack-operand rows change 226→230 and 500→494. The OFF G1 diagnostic run function contains an inlined kernel; ON calls the separately emitted kernel. Thus compiler inlining is part of this fixed implementation, not held constant. Static sites/stack operands are not executed instructions, dynamic spill counts or stack/RAM bounds; no post-clock attribute/layout rescue follows.

Ryzen 9 9950X, 16 physical cores/32 logical CPUs, shared host. G1/G2 × 200/201/352/353 points × one/16/32 workers gives 24 contexts. Three checked 128-round primes and a reset precede 8,192 timed rounds per worker. Changing public-generator fixtures have independent full output checks after each prime and timed loop. Dispatch, reference construction, complete point updates and inversion are timed; setup/reset/checks are outside. Diagnostic 512 KiB stacks pin solo to CPU 2, sixteen workers to CPUs 0–15 and thirty-two to CPUs 0–31. These are caller times, not proof latency or arbitrary SDK distributions.

Seventy-two preflights and one checker precede fifteen fixed rotated/reversed blocks: 1,080 latency calls, then eighteen PMU panels and eighteen profiles, then all ninety ownership/decode/metadata/event/LOST operations. Total: 1,279 commands and 1,188 checked caller processes. No owned build/helper/hash/analysis/report/push overlaps any recording, including preflights. No observations are excluded, extended or repeated.

The prospective rule uses paired-ratio sorted zero-based ranks 3/7/11 of fifteen. All OFF/frozen intervals must fit factor 1.03; all candidate/control nonregression upper bounds must be ≤1.02. Each curve's four-size 32-worker aggregate and the all-24 aggregate must have upper <1 and at least ten wins against both controls. There is no gain floor. Marginal intervals are not simultaneous family-wise guarantees.

|Aggregate|Candidate / fresh OFF, fixed rank interval|Wins / 15|Candidate / frozen, fixed rank interval|Wins / 15|
|---|---:|---:|---:|---:|
|G1|1.064184714 [1.031009975, 1.123053494]|0|1.077599849 [1.060169149, 1.128562736]|0|
|G2|1.042146460 [1.033908012, 1.060222212]|0|1.050340916 [1.033576134, 1.060443316]|0|
|ALL|1.059975277 [1.055138474, 1.068308276]|0|1.062847393 [1.060151340, 1.069517149]|0|

The fixed study fails 59 of 78 timing checks: five calibrations, all 48 nonregression checks and all six aggregate gain checks. Calibration failures are G1 201/352/353 and G2 200/201 at 32 workers. They remain visible and limit those control comparisons; separately calibrated solo and sixteen-worker regressions also remain. Every context's candidate/OFF median exceeds one. This closes the uniform recipe without selecting a curve, worker subset or carry variant.

|Context|OFF µs/wave|Candidate µs/wave|Candidate / OFF, fixed rank interval|Wins / 15|
|---|---:|---:|---:|---:|
|G1-200-w1|4.238|4.525|1.067650813 [1.066812530, 1.069540755]|0|
|G1-200-w16|4.815|5.111|1.058772234 [1.055344246, 1.067082694]|0|
|G1-200-w32|6.899|7.314|1.060424698 [1.055861647, 1.102101901]|2|
|G1-201-w1|4.349|4.649|1.069647481 [1.067581073, 1.070907192]|0|
|G1-201-w16|4.971|5.241|1.057549393 [1.048535551, 1.062134448]|0|
|G1-201-w32|7.872|7.514|1.056769572 [0.929567812, 1.067724650]|6|
|G1-352-w1|6.959|7.459|1.071674624 [1.071316765, 1.073285092]|0|
|G1-352-w16|7.973|8.502|1.066189627 [1.063973969, 1.068865542]|0|
|G1-352-w32|11.586|12.493|1.083461049 [1.060089464, 1.289113138]|2|
|G1-353-w1|7.066|7.583|1.072353737 [1.071365544, 1.073075509]|0|
|G1-353-w16|8.090|8.629|1.065917745 [1.061505411, 1.069829548]|0|
|G1-353-w32|11.785|12.695|1.071292292 [1.060376008, 1.085148841]|3|
|G2-200-w1|7.704|8.132|1.055683370 [1.055558355, 1.056232360]|0|
|G2-200-w16|8.952|9.396|1.049666206 [1.046239963, 1.053035489]|0|
|G2-200-w32|14.345|15.038|1.048884291 [1.032664639, 1.066833750]|1|
|G2-201-w1|8.055|8.610|1.069370944 [1.067677636, 1.070243667]|0|
|G2-201-w16|9.335|9.951|1.066005671 [1.063449372, 1.067049819]|0|
|G2-201-w32|14.943|15.943|1.034204101 [0.999369534, 1.096471298]|4|
|G2-352-w1|13.032|13.790|1.057635464 [1.057190229, 1.058930402]|0|
|G2-352-w16|15.193|16.029|1.054187262 [1.053388909, 1.055319662]|0|
|G2-352-w32|24.918|26.120|1.048740626 [1.046104923, 1.051198931]|0|
|G2-353-w1|13.379|14.255|1.065192430 [1.064198742, 1.066899139]|0|
|G2-353-w16|15.606|16.570|1.062048855 [1.060092386, 1.063680616]|0|
|G2-353-w32|25.485|26.666|1.048174266 [1.043803167, 1.050675603]|0|

All final outputs agree across arms. Timed allocations are zero and retained record/tail capacities match exactly, including tails and workers. G1 capacities are 40,000/40,640/70,400/71,040 bytes per worker for the four sizes; G2 capacities are 16,000/17,280/28,160/29,440 bytes. Maximum observed candidate caller RSS is 18.75 MiB, including setup. Neither caller RSS nor equal buffers establishes full-prover peak, minimum RAM, pressure/swap, phone feasibility or concurrent-proof throughput. There is no table, key copy, retained solver plan, new pool or production allocator change.

|Diagnostic context|Instructions ON/OFF|Cycles ON/OFF|
|---|---:|---:|
|G1-200-w1|1.049153|1.070981|
|G1-200-w32|1.049131|1.051456|
|G1-352-w32|1.052577|1.067550|
|G2-200-w1|1.043681|1.056807|
|G2-200-w32|1.043647|1.053544|
|G2-352-w32|1.045011|1.058198|

These single panels show 4.4–5.3% more instructions and 5.1–7.1% more cycles. Cache-miss changes vary, including decreases with slower calls. The result does not isolate mask cost, register pressure, inlining or DRAM as an exclusive cause. The complete latency comparison determines rejection.

All eighteen profiles qualify: 121,716 samples, zero LOST, maximum unknown 0.104% and minimum target share 99.821%. Grouped counters meet 99.9% running coverage. Sampling uses cycles:u, fixed period 16,000,000, 512 mmap pages and acknowledged FIFO gating after primes/disable before output checks. Assigned workers and PID/TID/CPU/MMAP metadata are checked. Full symbols and PMU CSVs remain archived; flat samples are not critical-path wall time.

Both analyzers pass on first execution. The independent reader checks 421 pins, all 1,279 command receipts, 1,188 caller outputs, 78 timing checks and eighteen profiles. Its pass confirms reconstruction, not a passing performance gate. Native/model qualification checks sixteen successful receipts and retains the failed initial test build. Prepared/compiled readers and plans are pinned before clocks.

This trial creates no SDK requests, proofs or cloud resources. Canonical inventory stays 35,171. All proof results below are **inherited** from the [previous report](../2026-11-08-rank5-cross-raw176/README.md): latest matched 1×2 plus seven earlier full-campaign rows. The exact same accepted binary previously measured 7.915–7.929 ms locally; the latest 8.002770 ms is a later observation with unexplained variation, not an adopted code regression. Supplied M5 references use different hardware.

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

The original verified CPU 10× goal remains active and unmet, **0/8**. Merge54 remains inherited at 162.240109 ms without an M5 reference. No new AWS/Aeglos, giant-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM or concurrent-proof result is added. This implementation does not select an SDK trial or adoption.

The [evidence archive](artifacts/evidence.json.gz) retains prospective designs/plans/pins, changed-source/helpers, native/model/portable receipts, actual linked disassembly, raw caller outputs, complete decoded profiles, PMU CSVs and the complete independently reviewed analysis. Decompression yields relative paths mapped to {utf8,bytes,sha256}; all extracted payloads are verified. The public analysis omits only duplicated command/caller maps; complete original bytes stay archived. Binaries/libraries, raw perf data, keys/images/proof payloads, private ledgers and full source copies are excluded, with relevant identities retained.
