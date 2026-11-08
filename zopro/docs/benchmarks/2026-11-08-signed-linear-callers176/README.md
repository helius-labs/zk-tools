# Signed coordinate carry normalization: complete point callers

**Rejected; no adoption or SDK trial.** Across all 24 contexts the paired ratio is
1.017713 [1.009259,1.028984] versus original OFF and 1.023487
[1.014096,1.031467] versus frozen source 176, with 0/15 wins against either.
All 24 ON/OFF context medians exceed one. Correct outputs and native arithmetic
qualification do not establish faster complete point operations.

The fixed gate fails 43/78 checks: 6 control calibrations, 31 nonregressions and
all 6 aggregate gains. Calibration failures affect G1 at all four sizes and
G2 sizes 200/201 at 32 workers. These comparisons remain inconclusive under
the declared calibration rule. Every single-worker and 16-worker context
calibrates; all eight G2 contexts there lose every ON/OFF block, with median
slowdowns of 2.81–4.45%. Their regressions do not depend on the noisy 32-worker
comparisons. G1 solo/16 medians are 0.72–0.96% slower; one 16-worker interval crosses
parity. No subset is promoted, observation excluded or capture rerun.

## Implementation and qualification

The [qualified signed-linear methods](../2026-11-08-signed-linear-carry176/README.md)
replace serial carries in canonical coordinate differences with independent
transfers, an all-lane high-bit predicate and exact signed repair. Signed dy
forms negative limbs directly and retains the original strict2p correction.
All outputs are bit-identical to the original methods before IFMA consumption.
The parent mathematical/native/Arkworks/standalone-code evidence is inherited,
not replayed or counted as new proof verification.

Only two native-eight-lane FqBatch wrappers select those exact methods.
Canonical input subtraction and signed canonical dy change; generated ifma8,
all multiply/REDC, generic add/sub, G1signed-scaled delta, inverse, FFT, keys,
point formulas and scheduling remain original. The178-file private snapshot
recovers all176originals after removing two routing edits, the field include
and two existing diagnostic parent suffixes. Nineteen qualified field members
are reused exactly. No old quotient, CIOS normalization, phase, descriptor,
tree or offset runtime is composed.

Five new optimized commands build ONcore, build/run127unfiltered core tests,
link the complete caller and compile coherent generic no_asm/no_ifma guards
with the selector present. New ONchecks include39,960sequential and162,816
threaded scalar-generator point comparisons, all53actual workers in1/4/16/32
pools with512KiB requested stacks and nested joins, signs/tails/immutability/
reset/reuse. The scalar oracle uses unchanged scalar arithmetic; this is not
a new independent cryptographic proof verifier. Control OFFartifacts, tests
and decodes are inherited unchanged and measured freshly. The native field
artifacts are reused without rebuilding.

One new symbol listing and four new actual ONrun/kernel decodes reuse saved
OFFrows. Both actual caller runs transfer to their production kernels.

|Kernel|Instruction sites OFF to ON|IFMA sites OFF to ON|VPMULLQ sites OFF to ON|RSP operand sites OFF to ON|Unsigned high-compare sites OFF to ON|
|---|---:|---:|---:|---:|---:|
|G1 streaming|4425 to 4497|800 to 800|0 to 0|591 to 601|0 to 2|
|G2 borrowed|8590 to 8697|1450 to 1450|0 to 0|1469 to 1429|5 to 11|

The added unsigned high comparisons correspond to two G1 and six G2 repair
checks in these actual kernels. The new source's shorter local carry recurrence
comes with more predicates/branches and changed compiler liveness. These are
static sites, not executed counts, spill time or complete-stack bounds. No
unique cost is isolated from these counts.

## Fixed comparison

Local shared Ryzen9950X,16physical/32logical CPUs; CPU2solo and CPUs0–15/0–31
otherwise. Both curves x200/201/352/353 x1/16/32 give24contexts. Fifteen fixed
rotated/reversed blocks compare original frozen callerf07229ce, reused original
OFF45e583b0 and new ON, with exact identities retained. The frozen diagnostic
is distinct from accepted proof SDK6d83. Three checked128round primes/reset
precede8,192changing measured batches per worker. Complete arithmetic,
inversion, dispatch and reference preparation are timed; setup/checks are not.

Independent prospective review passes before ANYpreflight. All72preflights,
checker,1,080latencies,18PMUpanels and18profiles finish before90offline steps:
1,279commands,1,188checked caller processes and199,292,928worker batch calls
including primes. Counts are not proofs. During any recording only confirmed
live handle5664 was polled; no owned helper/build/hash/decoder/analyzer/verifier/
report/archive/push overlapped. Both result analysis commands pass first run.

|Aggregate|ON/OFF [fixed rank interval]|Wins/15|ON/frozen [fixed rank interval]|Wins/15|
|---|---:|---:|---:|---:|
|G1|1.011019512 [0.950485006, 1.076277090]|7|1.032235153 [0.974608072, 1.079077732]|5|
|G2|1.015440313 [1.011806617, 1.025303799]|1|1.025068900 [1.008347710, 1.031427509]|0|
|ALL|1.017712852 [1.009258710, 1.028984404]|0|1.023486669 [1.014096350, 1.031466761]|0|

G1/G2 aggregates combine four sizes at32workers; ALL combines24contexts with
equal log weights within each block. Pairing uses block ratios; their median
need not equal the quotient of marginal medians. Ranks3/7/11of15give marginal,
not simultaneous familywise intervals. All24control calibrations require
factor1.03,48nonregression uppers<=1.02, and each of six gain checks needs
upper<1 and>=10wins against its control. There is no positive gain floor.

|Context|OFF us/wave|ON us/wave|ON/OFF [interval]|Wins/15|Control calibrated|
|---|---:|---:|---:|---:|---|
|G1-200-w1|4.232|4.264|1.007641 [1.006264, 1.008678]|0|true|
|G1-200-w16|4.801|4.852|1.008769 [1.005946, 1.012905]|0|true|
|G1-200-w32|6.903|7.049|1.012498 [0.975915, 1.170531]|4|false|
|G1-201-w1|4.353|4.387|1.008529 [1.006020, 1.009652]|0|true|
|G1-201-w16|4.939|4.972|1.007247 [0.993611, 1.012925]|5|true|
|G1-201-w32|8.341|8.387|1.010234 [0.860404, 1.213619]|4|false|
|G1-352-w1|6.962|7.026|1.008498 [1.007620, 1.009191]|0|true|
|G1-352-w16|7.958|8.042|1.009453 [1.008365, 1.011492]|0|true|
|G1-352-w32|11.656|11.843|1.012877 [0.997650, 1.267814]|4|false|
|G1-353-w1|7.071|7.137|1.009648 [1.008003, 1.009906]|0|true|
|G1-353-w16|8.089|8.161|1.008662 [1.006078, 1.013088]|0|true|
|G1-353-w32|11.841|11.967|1.007186 [0.811585, 1.018476]|5|false|
|G2-200-w1|7.695|7.958|1.035057 [1.033588, 1.035428]|0|true|
|G2-200-w16|8.933|9.237|1.033977 [1.032359, 1.036207]|0|true|
|G2-200-w32|14.543|14.942|1.025978 [0.985831, 1.031661]|5|false|
|G2-201-w1|8.085|8.404|1.040028 [1.038599, 1.040696]|0|true|
|G2-201-w16|9.380|9.739|1.037645 [1.036306, 1.040165]|0|true|
|G2-201-w32|15.086|15.444|1.029286 [0.987058, 1.033622]|6|false|
|G2-352-w1|13.020|13.387|1.028126 [1.027883, 1.028760]|0|true|
|G2-352-w16|15.232|15.911|1.044501 [1.041287, 1.045848]|0|true|
|G2-352-w32|24.933|25.376|1.019291 [1.015487, 1.024764]|0|true|
|G2-353-w1|13.382|13.822|1.031782 [1.030982, 1.032811]|0|true|
|G2-353-w16|15.637|16.266|1.040455 [1.036090, 1.042465]|0|true|
|G2-353-w32|25.438|25.915|1.017992 [1.016440, 1.024871]|0|true|

## Automatic diagnostics and resource scope

All 18 profiles qualify: 120,750 samples, zero LOST, minimum 3018 samples, minimum assigned-worker share 100.000000%, minimum target-symbol share 99.897855%, maximum unknown 0.102145%. All grouped PMU events run 100%. Each selected context/arm has one panel/profile; diagnostic uncertainty and causal stall attribution are unmeasured.

|Diagnostic context|Retired instructions ON/OFF|Cycles ON/OFF|
|---|---:|---:|
|G1-200-w1|1.004730|1.007379|
|G1-200-w32|1.004707|1.020603|
|G1-352-w32|1.005139|1.012432|
|G2-200-w1|1.010407|1.035115|
|G2-200-w32|1.010407|1.061466|
|G2-352-w32|1.010768|1.051300|

G1 retires 0.47–0.51% more instructions and uses 0.74–2.06% more cycles in these
panels. G2 retires 1.04–1.08% more instructions and uses 3.51–6.15% more cycles.
The intended dependency change did not produce a complete-caller benefit.
Extra branches, execution resources and register allocation remain possible
costs, not individually proven causes. These data do not establish a DRAM or
RAM-capacity bottleneck. Flat CPU samples are not removable proof-wall shares.

All timed allocations are zero and all primed retained capacities match across
arms. Maximum ONdiagnostic process RSS is18.5MiB, excluding proving keys. This
is not proverpeak/minimumRAM, mobile, pressure, concurrent-proof or stack-bound
evidence. No heap buffer, table, key/point copy, retained plan or production
pool is added. All new scientific commands succeed first execution; no failed
build/test/capture or successful replay occurs. Helper compiler/preparation/
publication invocations have tool-transcript evidence rather than full workflow
compiler receipts; scientific argv/exit/streams remain archived.

This fixed consumer application closes without further native/SDK selection.
No signed-mask/repair/layout/width/threshold sweep or favorable G1 subset is
selected retrospectively. Arithmetic correctness remains valid; universal
impossibility of other normalization or arithmetic mechanisms is not claimed.
Further work needs a distinct complete-work/dependency case that accounts for
the earlier negative carry and quotient results.

## Accepted CPU and Metal comparison, inherited

These local32-worker accepted-build medians come from the
[full SDK campaign](../2026-11-08-tree-policy-sdk176/README.md), not this point
caller experiment. Supplied M5 references use different hardware.

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

Merge54x1 remains160.890372ms without an M5reference. Historical same accepted
binary7.915095ms remains valid; later8.04ms is not an adopted regression.
Production176/SDK6d83 and the canonical37,023proof ledger remain unchanged.
The full verified single-proof CPUAVX10x objective remains active/unmet0of8.
No new SDKrequest/proof/Gnark export, AWS/Aeglos, huge-circuit, phone/ARM/TEE,
swap/pressure or concurrent-throughput result is added.

[All contexts](artifacts/analysis.json),[independent review](artifacts/review.json),
[actual code](artifacts/codegen.json),[source audit](artifacts/source-audit.json),
[identities](artifacts/identities.json),[protocol](artifacts/protocol.md),
[full source/receipt/observation evidence](artifacts/evidence.json.gz),
[member hashes](artifacts/archive-index.json),[publisher](artifacts/publish.rs),
[checksums](SHA256SUMS). Public analysis drops only duplicate caller/command
maps; exact full original analysis and all receipts remain archived/hash-bound.
Inherited OFFprovenance is explicitly labeled. Executables/libraries, unchanged
full source copies, proving material/point/scalar/proof arrays and raw perf data
are excluded. Every archive-member hash and gzip roundtrip is verified.
