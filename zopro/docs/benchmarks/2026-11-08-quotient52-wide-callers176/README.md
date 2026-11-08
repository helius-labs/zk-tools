# Overwide radix-52 quotient: complete point callers

**The variant does not establish a complete-caller improvement and is not adopted.** The all-24-context paired ratio versus the original OFF control is 1.000305 [0.995802, 1.019626], with 7/15 wins. The fixed gate fails 23/78 checks. All 1,188 caller outputs/allocation checks pass, all 18 profiles qualify, and the separate reader passes. No SDK trial or production change is selected.

This differs from the [rejected masked parent](../2026-11-08-quotient-mullo-callers176/README.md): the new native-qualified field keeps normal 52-bit quotients as full wrapping 64-bit products because every consumer truncates them. The mixed 48-bit mask remains. [Exact native/raw arithmetic and the all-input argument](../2026-11-08-quotient52-wide-native176/README.md) are inherited without replay. Removing the extra masks restores the original complete kernels' AND counts. It does not establish lower cycle cost.

Five control calibration failures occur at 32 workers: G1 sizes 200/201/352/353 and G2 size 200. Those comparisons remain inconclusive under the declared calibration rule; their large ranges are retained. There are also 12 nonregression and all 6 aggregate-gain failures. Every single-worker and 16-worker context calibrates. In all eight G2 contexts at those worker counts the candidate loses every ON/OFF block, with paired median slowdowns of 1.16–2.27%. G1 has small gains and losses that depend on size. No subset is promoted, observation excluded or capture rerun.

## Source and qualification

The 177-file private snapshot contains the original 176 files plus one existing diagnostic child. All 158 non-field runtime bodies recover exactly after removing the two existing append-only diagnostic suffixes; the 18 field members equal the qualified new field snapshot. G1/G2 fixture, driver, perf and test bytes are unchanged. No closed phase, descriptor, tree or offset runtime is imported. There is no new table, heap allocation, point/key copy, record, retained plan or production pool.

Five new commands build ON core, build/run its 127 unfiltered tests, link its complete caller, and compile coherent generic no_asm/no_ifma core guards. New ON checks include 39,960 sequential and 162,816 threaded generator-point comparisons, signs/tails/immutability/reset/reuse and 53 actual workers in pools 1/4/16/32 requesting 512 KiB stacks with nested joins. The original scalar arithmetic supplies the point oracle; these are not independently verified cryptographic proofs.

The original OFF artifact, its 127 tests and actual decodes are reused unchanged from the masked-parent campaign. They are inherited qualification, not fresh builds/tests. Both control binaries are executed afresh in this capture. ON core and driver explicitly link the same new qualified field artifact and original matching dependencies, with optimized flags preserved. The field artifacts are reused, not rebuilt. The frozen diagnostic caller is distinct from accepted proof SDK6d83.

One new symbol listing and four new function decodes cover the actual ON G1/G2 runs and production kernels. Original OFF decode evidence is reused; both actual run functions transfer to the kernels.

|Actual kernel|IFMA sites OFF to ON|VPMULLQ sites OFF to ON|AND sites OFF to ON|Instruction sites OFF to ON|RSP operand sites OFF to ON|
|---|---:|---:|---:|---:|---:|
|G1 streaming|800 to 760|0 to 40|167 to 167|4425 to 4392|591 to 592|
|G2 borrowed|1450 to 1390|0 to 60|346 to 346|8590 to 8533|1469 to 1472|

These are static instruction sites, not dynamic instruction totals, spill time or complete stack bounds. Total multiply count is unchanged. The normal-quotient representation and mixed-48 correctness are established; whole-caller performance is a separate question.

## Fixed comparison

Local shared Ryzen 9950X, 16 physical/32 logical CPUs. Affinity is CPU2 for one worker and CPUs0–15/0–31 otherwise. Both curves, sizes200/201/352/353 and 1/16/32 workers give 24 contexts. Fifteen fixed rotated/reversed process blocks compare frozen original source176, reused original OFF and new ON. Three checked 128-round primes/reset precede 8,192 changing measured batches per worker. Setup/output checks are outside the clock; complete arithmetic, inversion, dispatch and reference preparation are included.

All 72 preflights, one checker, 1,080 latency processes, 18 PMU panels, 18 fixed-period profiles and 90 offline commands complete: 1,279 commands, 1,188 callers and 199,292,928 worker-batch calls including primes. These counts are not proofs. During any recording only live handle57736 was polled; no owned helper/build/hash/decoder/analyzer/verifier/report/archive/push overlapped. Both analysis commands and the prospective command review pass on first execution.

|Aggregate|ON/OFF [fixed rank interval]|Wins/15|ON/frozen [fixed rank interval]|Wins/15|
|---|---:|---:|---:|---:|
|G1|0.968312333 [0.941418740, 1.052312200]|9|0.994875579 [0.919251663, 1.091704509]|8|
|G2|1.006862267 [0.998864012, 1.017654558]|5|1.008099267 [1.001529695, 1.013040443]|3|
|ALL|1.000305418 [0.995801714, 1.019625803]|7|1.005537962 [0.992261258, 1.018893111]|6|

G1/G2 aggregates combine four sizes at 32 workers; ALL combines all 24 contexts with equal log weights within each block. Pairing uses block ratios, whose median need not equal a quotient of raw-time medians. Fixed sorted ranks3/7/11 of15 give marginal intervals, not familywise coverage. Calibration requires reciprocal factor1.03; all48nonregression uppers must be <=1.02; both curve32 aggregates and ALL24 need upper<1 and >=10wins versus both controls. There is no positive gain floor.

|Context|OFF us/wave|ON us/wave|ON/OFF [interval]|Wins/15|Control calibrated|
|---|---:|---:|---:|---:|---|
|G1-200-w1|4.241|4.226|0.996604 [0.995785, 0.998531]|14|true|
|G1-200-w16|4.814|4.799|0.995831 [0.992951, 1.000881]|10|true|
|G1-200-w32|6.926|6.929|0.999340 [0.979124, 1.006785]|8|false|
|G1-201-w1|4.354|4.374|1.004367 [1.003468, 1.004772]|0|true|
|G1-201-w16|4.929|4.959|1.004527 [1.001666, 1.009442]|3|true|
|G1-201-w32|7.082|7.132|1.008134 [0.971734, 1.118625]|4|false|
|G1-352-w1|6.964|6.956|0.998270 [0.997121, 0.999423]|13|true|
|G1-352-w16|7.959|7.952|0.998392 [0.997348, 1.001754]|9|true|
|G1-352-w32|11.699|11.682|0.990663 [0.803786, 1.021199]|9|false|
|G1-353-w1|7.076|7.087|1.001670 [1.000818, 1.002319]|1|true|
|G1-353-w16|8.084|8.097|1.000942 [0.996928, 1.004264]|6|true|
|G1-353-w32|11.771|11.904|0.998799 [0.826724, 1.021192]|8|false|
|G2-200-w1|7.700|7.822|1.016304 [1.015414, 1.017352]|0|true|
|G2-200-w16|8.935|9.080|1.014791 [1.012742, 1.017937]|0|true|
|G2-200-w32|14.483|14.650|1.011831 [0.968829, 1.054705]|5|false|
|G2-201-w1|8.083|8.268|1.022670 [1.020797, 1.024128]|0|true|
|G2-201-w16|9.381|9.573|1.021268 [1.019338, 1.022674]|0|true|
|G2-201-w32|15.168|15.296|1.004557 [0.989550, 1.037268]|6|true|
|G2-352-w1|13.023|13.170|1.011595 [1.010398, 1.012807]|0|true|
|G2-352-w16|15.239|15.449|1.013209 [1.009145, 1.016779]|0|true|
|G2-352-w32|24.873|24.987|1.005000 [0.999886, 1.013911]|4|true|
|G2-353-w1|13.392|13.593|1.015523 [1.014075, 1.016597]|0|true|
|G2-353-w16|15.653|15.869|1.014158 [1.013190, 1.016655]|0|true|
|G2-353-w32|25.390|25.537|1.008637 [1.000776, 1.017787]|3|true|

## Automatic diagnostics and resource scope

All18profiles qualify:120,002samples, zeroLOST, minimum3013samples, minimum assigned-worker share100.000000%, minimum target-symbol share99.898348% and maximum unknown share0.092166%. All grouped PMU events have100% running coverage. There is one panel/profile per selected context/arm, so these do not measure diagnostic uncertainty or isolate a unique cause. Samples are flat user-cycle samples, not removable proof-wall shares.

|Diagnostic context|Retired instructions ON/OFF|Cycles ON/OFF|
|---|---:|---:|
|G1-200-w1|0.985752|0.998528|
|G1-200-w32|0.985773|1.011936|
|G1-352-w32|0.984736|1.000513|
|G2-200-w1|0.988547|1.017001|
|G2-200-w32|0.988542|1.035329|
|G2-352-w32|0.988212|1.024168|

Retired instructions decrease about1.42–1.53% for G1 and1.15–1.18% for G2. G1 cycles range from0.15% lower to1.19% higher; G2 cycles are1.70–3.53% higher. The intended instruction reduction is real, but it does not produce a reliable cycle improvement. Since extra quotient ANDs are absent, removing those masks is insufficient to explain or fix the prior regression. Dependency chains, execution resources and register allocation remain possible costs, not individually proven causes. This does not establish a DRAM or RAM-capacity bottleneck, or a universal impossibility for VPMULLQ.

Timed allocations are zero and all primed retained capacities match across arms. Maximum ON diagnostic process RSS is18.5MiB, excluding proving keys; this is not whole-prover peak/minimumRAM, mobile, memory-pressure or concurrent-proof evidence. Requested worker stacks are not measured stack bounds.

The first review/campaign helper compiler invocations omitted their rustc_demangle dependency. Only the compiler arguments were corrected, before any execution of those helpers; their sources and all scientific build/test/decode/capture commands were unchanged. All scientific commands succeed on first execution. Helper compilation/preparation/publication have tool-transcript evidence rather than full workflow receipts. Prospective review, actual native commands, decodes, capture and result analysis retain exact command/exit/stream receipts. No successful command was rerun.

This fixed variant closes without SDK timing or adoption. No favorable G1 size or worker subset is selected retrospectively. Further work needs a distinct complete arithmetic/useful-work mechanism and its full cost case, rather than another mask/width/layout/threshold sweep or unchanged caller reroll. Production generator integration, target policy, portable/resource/concurrency checks would still be needed for any later adoption.

## Accepted CPU and Metal comparison, inherited

The [latest full SDK campaign](../2026-11-08-tree-policy-sdk176/README.md) supplies these local32-worker accepted-build medians; supplied M5 timings use different hardware. No SDK request or new proof is created here.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10x target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.040649|37|24|4.602x|2.985x|3.7|
|2x2|11.149097|59|30|5.292x|2.691x|5.9|
|4x2|16.069415|109|48|6.783x|2.987x|10.9|
|8x2|24.131178|189|74|7.832x|3.067x|18.9|
|12x2|33.940628|284|101|8.368x|2.976x|28.4|
|16x2|40.164083|335|124|8.341x|3.087x|33.5|
|24x2|66.187704|528|176|7.977x|2.659x|52.8|
|merge24x1|65.361571|507|175|7.757x|2.677x|50.7|

Merge54x1 remains160.890372ms with no supplied M5 reference. Historical same-build7.915095ms remains valid; the later8.040649ms observation does not establish an adopted regression. The full verified single-proof CPUAVX10x objective remains active and unmet0/8. Accepted source176/SDK6d83 and canonical37,023-proof ledger remain unchanged. No new AWS/Aeglos, giant-circuit, phone/ARM/TEE, swap/pressure or concurrent-throughput result is added.

[All contexts](artifacts/analysis.json), [independent review](artifacts/review.json), [actual code](artifacts/codegen.json), [source audit](artifacts/source-audit.json), [identities](artifacts/identities.json), [protocol](artifacts/protocol.md), [full source/receipt/observation evidence](artifacts/evidence.json.gz), [member index](artifacts/archive-index.json), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS). Public analysis omits only duplicate caller/command receipt maps; exact original analysis and all receipts remain archived and hash-bound. Archive includes reused OFF qualification/receipt/decode provenance, labeled as inherited. Binaries/libraries, unchanged full source trees, private keys/images/witnesses/point/scalar/proof arrays and raw perf data are excluded.
