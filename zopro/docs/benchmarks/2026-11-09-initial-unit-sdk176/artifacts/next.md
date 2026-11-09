# Continue after the initial-affine state comparison

Goal ACTIVE/UNMET: verified single-proof CPU AVX at least10× supplied M5 CPU
for ALL8 shapes, reporting CPU and Metal comparisons. Targets met0/8. The last
goal turn made progress: four frozen helpers/provenance,26 verified preflight
exports, full1263-command comparison, automatic PMU/profiles, independent raw
reader, report/commit/push. This is a measured negative, not a blocker.

Current root /home/ubuntu/src/zopro-perf-work/initial-unit-sdk176. Read protocol.md,
analysis.json,review-result.json,decision.json,diagnostic-summary.json,
fresh-comparison.json,publication.json. Helpers39963terminal0/four commands;
preflight8390terminal0/52;main65631terminal0/1263;analyzer94042terminal0;
reader66066terminal0;prospective81300terminal0. Preparation35390/39119 terminal0.
Every recording ended before analysis/report. No runtime/helper/reader/recording
repair occurred in this measurement stage. Native qualification's test-only
oracle correction is inherited from initial-unit176, not rerun. No live jobs
or owned cloud resources remain after the completed workflow.

Verdict4/79fail, all11 calibration checks pass. Failed: primary32 ON/REF gain,
primary32 ON/REF wins, ALL8 ON/OFF gain, ALL8 ON/REF gain. All22 nonregression,
22 RSS and18 diagnostic checks pass. Primary REF/OFF/ON7.803403940/7.848710596/
7.817298887ms. ON/OFF ratio0.995080949,CI[0.993129092,0.996922488],24/31 wins:
a retained0.492% paired component gain. ON/REF1.001355632,
CI[0.999430482,1.003232990],11/31 wins: no verified gain over accepted.
OFF/REF1.006305701,CI[1.004515344,1.008115824] passes declared calibration.
Don't call it calibration failure or claim a unique code-layout cause.

Primary16 REF/OFF/ON9.195841013/9.300008086/9.221444894ms. ON/OFF0.992868721,
CI[0.990810830,0.994882128],28/31;ON/REF1.001305889,
CI[0.999666028,1.002853686],8/31. Solo REF/OFF/ON94.508759636/95.869561727/
93.968370545ms,ON/OFF0.981229871 CI[0.977055471,0.984932362],15/15;
ON/REF0.994605056 CI[0.991961557,0.997474108],12/15. Preserve these positives;
they do not authorize a posthoc worker/curve/shape subset adoption.
ALL8 ON/OFF1.000364539 CI[0.998464134,1.002328016], no overall gain.
ALL8 ON/REF1.006494089 CI[1.004209227,1.008729147],0.649% slower.
The exact initial-unit state recipe is CLOSED without adoption or unchanged
recapture/compiler/inlining/layout/threshold/curve/shape/worker rescue. Global
shared unit-Z and historical affine occupancy tagging also remain closed.

Primary32 whole-process maxRSS REF/OFF/ON132.50/133.50/134.50MiB, all declared
5% guards pass. Logical storage unchanged; these are neither minimum RAM nor
pressure/mobile measurements. All9 PMU panels qualify at>=99.9% coverage.
Nine profiles119775 samples,min8880,maxunknown2.088816%,zeroLOST,actual executable/
PID/TID/CPU and fixed16M cycles:u period bound. Single primary32 ON/OFF panel:
instructions0.993492827,cycles0.989284099,cachemiss0.987149194,
branchmiss0.982859969. ON/REF instructions0.994296984,cycles0.996061173.
These single panels have no replicatedCI and do not establish a unique memory,
dependency or frequency cause. Flat samples are not wall-critical fractions.

In the current accepted primary32 flat profile, stream_g1_impl21.4654%,
g2_borrowed::apply15.8116%, Fq inverse6.8670%, G2 reduce5.5586%,G1 reduce4.3692%,
balanced feed4.7498%,crossbeam pin8.5481%. New initial-unit helper ON shares
G1~0.531%,G2~0.063%; these are attribution, not removed wall-time potential.
The limited scalar-path signal is not a twofold algorithmic improvement.

NEXT needs a distinct bulk work/dependency mechanism, not further unit-Z tags
or a broad read-only audit. Current source was inspected after recording:
zopro-core/src/msm.rs stream_g1_impl stores five FqBatch fields per group and
uses a forward prefix and reverse inverse recurrence; msm/g2_borrowed.rs stores
prefix+norm and reconstructs coordinates in reverse. Reuse the accepted source
and existing operation/dependency evidence, especially docs/benchmarks/
2026-11-06-stream-work-census176/README.md. It already qualifies G1(6g+5)V and
G2(7g+5)V+5gF with bridge+tails, record1600g/640g bytes. It is not a universal
lower bound. Prefix peeling, independent chains, norm replay, uniform scaled
prefix/R260, raw fused products and G2 prefix-tree have prior histories; do not
restart a closed exact recipe. The specific next mechanism is NOT yet selected;
do not pretend a new profile percentage proves a breakthrough. Make the next
turn concrete with a distinct bounded Rust work/dependency/resource model or
implementation after narrowly checking the applicable prior negative. Avoid
another status-only turn or unchanged proofs/native suites/decoder replay.

CANONICALNOW43073:
/home/ubuntu/src/zopro-perf-work/initial-unit-sdk176/proof-sha256-after-capture.json.
Always seed future exports from THIS, not42464/42438/41829/41803/41792.
This stage44210 SDK requests/635 fresh unique independent Gnark exports:
preflight104requests26exports42438->42464,main44106requests609exports42464->43073.
Main609 SDK processes=591latency+18instrumented;39741 timed latency samples.
Calls aren't independently exported proofs. All exported proof164-byte/public/
image/freshness/placement/source/library/receipt bindings verified. Separate
same-agent reader reconstructs raw means/all79 checks; shared bootstrap and
diagnostic parsers unchanged, not an external audit.

Accepted production176 and exact SDK remain bitmap-reset adoption. Manifest:
/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-09-busy-reset-adoption176/artifacts/production-source-manifest.json.
Accepted SDK /home/ubuntu/src/zopro-perf-work/busy-reset-sdk176/sdk-ON;
accepted core /home/ubuntu/src/zopro-perf-work/busy-reset176/libs/ON/libzopro_core.rlib;
original field1200 in g2-vector-sign-default175/frozen/candidate unchanged.
Rejected initial-unit SDKs at initial-unit176/sdk-{OFF,ON}; don't treat ON as REF.
Qualified native state source/builds remain in initial-unit176; no replay needed.

Fresh REF observations of the SAME accepted artifact: ALL8 CPUms7.803403940/
10.940318516/15.809849355/24.043742968/33.720354516/39.753009613/65.983136871/
65.378103258. Primary RSS132.50MiB; merge54 162.189200065ms/no supplied M5 ref.
M5 CPU37/59/109/189/284/335/528/507;Metal24/30/48/74/101/124/176/175;
CPU10× targets3.7/5.9/10.9/18.9/28.4/33.5/52.8/50.7;all unmet0/8. Already
sub8;prior7.803148748 same artifact in previous campaign,not a milestone.
Historical7.699867 had a different campaign/statistic. No new AWS/Aeglos,
ARM/mobile,huge250/4999,pressure/swap/minRAM or concurrent-throughput result.

Autoresearch loop and full goal persist: actual code/profile/relevant history;
distinct work/dependency/resource hypothesis; bounded Rust implementation;
targeted independent curve/resource/generic/concurrency qualification; freeze
coherent complete SDK all8+54 controls/protocol/readers and automatic profiles
BEFOREANYpreflight; serialized Rust1.98.1 xtask; duringANYpreflight/recording ONLY
poll its exact confirmed live handle until terminal (timeout/truncation isn't
terminal); then verify every observation, write a polite factual report, commit
ANDpush, verify exact remote HEAD, checkpoint; continue full goal. No questions,
Python,subagents,large persistent LUT/key expansion,new production pool,
artificial RAMcap,C/ASM rewrite or favorable-subset rescue. Small positive
signals are retained; success still requires verified latency targets forALL8.
