# Continue after shared sum/difference weighted folding

Goal ACTIVE/UNMET: verified single-proof CPU AVX >=10x supplied M5 CPU for all8
shapes, reporting both CPU and Metal. Targets0/8. This turn is PROGRESS: new
bounded Rust topology/curve/work model, checked optimized execution, independent
symbolic/count/receipt/source reader, report and commit/push. No native or proof
speed improvement is established. No live jobs/cloud remain after completion.

Root /home/ubuntu/src/zopro-perf-work/weighted-sumdiff176. Read design.md,
result.json,review-result.json,source-binding.json,publication.json. Workflow36000
terminal0/two commands, model and reader succeed first execution. No old native
curve tests, proofs, recordings or decoders replayed. Preparation/review/publication
helper invocations are manual; they do not all have xtask receipts.

The new local question was a weighted sum/difference butterfly with shared
denominators, rather than another scalar shortcut or C/ASM change. For n=2h:
S=A+B,D=A-B;W(A,B)=W(S)+(h/2)*(T(S)-T(D)),T(A,B)=T(S),for h even. Base1/2 keep
the accepted reconstruction. The accepted control reproduces weighted_sum's
side copies/simultaneous layer reductions/double-subtract reconstruction from
the actual accepted msm.rs. Shared ordinary P+Q,P-Q uses one denominator
inverse and4M+2S, with complete identities/equal/opposite guards.

108 cases cover G1/G2,n1..256 and six dense/identity/equal/opposite/half-matching
patterns. Independent generator-coefficient W/T oracles check432 outputs;
paired complete group oracles check4096, total4528. Inputs/populations reused;
these are not distinct proofs. Model candidate reuses one n-point slice, with
no input-sized scratch in its recursion. This does not qualify native layout,
stack/heap/RSS/admission/ARM behavior. Scalar Arkworks model is not native SIMD.

The work gate FAILS all4 contexts n32/64/128/256. DenseG1 variable proxy charges
ordinary5M+S and paired7M+2S (optimistic3M amortized denominator sharing). At256:
accepted502ordinary affine adds,2510M+502S=3012;candidate254shared pairs+248other
adds,3018M+756S=3774,+25.2988%. Final projective-add calls8->15,doubles8->22.
Separate reader's closed counts: accepted12n-6log2(n)-12 units;
candidate15n-6log2(n)-18;excess3(n-2)>0. It also checks508 independent linear
coefficients and176 unchanged accepted-source bindings. This is a declared
research-admission proxy, not a latency theorem or universal algebraic bound.
It excludes packing/tails/fixed inversion bridges and doesn't relabel itsG1
work as the native G2 norm-prefix cost. The exact topology/formula is CLOSED
without a native trial or a threshold/curve/shape/worker/layout rescue.

Targeted history/literature checks rediscovered existing square/endpoint/prefix
tree,SRE and Waterloo scalar-reuse work; no old recipe or external speedup is
reopened. Relevant nearby negatives: integrated-square-proof, signed-complex-
square, joint-coordinate-delta, g2-prefix-tree and prior weighted experiments.
The global unit-Z method and initial-unit state recipe remain closed too.

NEXT: select another distinct bulk-work/dependency/resource mechanism; do not
rerun this model or launch an unchanged candidate. A narrowly checked relevant
history plus a real bounded Rust model/implementation must follow, not another
broad read-only inventory or status-only turn. Current dominant accepted flat
profile remains packedG1stream21.47%,borrowedG2apply15.81%,Fq inverse6.87%,G2
reduce5.56%,G1reduce4.37%,balanced feed4.75%,crossbeam pin8.55%. These are not
wall-critical fractions or a unique memory/frequency cause. No next mechanism
is selected by this negative screen; don't invent a speed forecast.

Production, originalfield1200 and accepted bitmap-reset SDK remain unchanged.
Accepted manifest:
/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-09-busy-reset-adoption176/artifacts/production-source-manifest.json.
Accepted SDK /home/ubuntu/src/zopro-perf-work/busy-reset-sdk176/sdk-ON;
core /home/ubuntu/src/zopro-perf-work/busy-reset176/libs/ON/libzopro_core.rlib.
Latest matched accepted primary7.803403940ms/132.50MiB. Prior state candidate
7.817298887ms was rejected4/79checks;retain its0.492%primaryON/OFF signal but
no acceptedREF/all8 gain. No new SDKrequest/proof/performance/profile thisstage.

CANONICALNOW43073 remains:
/home/ubuntu/src/zopro-perf-work/initial-unit-sdk176/proof-sha256-after-capture.json.
Seed all future exports here,not42464/42438/41829/41803/41792. Latest complete
comparison/report/diagnostics at initial-unit-sdk176;don't rerun successful
native qualifications/builds/recordings/decoders for bookkeeping.

Accepted ALL8 CPUms7.803403940/10.940318516/15.809849355/24.043742968/
33.720354516/39.753009613/65.983136871/65.378103258. M5CPU37/59/109/189/284/
335/528/507;Metal24/30/48/74/101/124/176/175;CPU10x targets3.7/5.9/10.9/18.9/
28.4/33.5/52.8/50.7. All0/8 remain unmet. Merge54 162.189200065ms/no supplied
M5 reference. Already sub8;historical7.699867differentcampaign/statistic. No
newAWS/Aeglos,ARM/mobile,huge250/4999,pressure/swap/minRAM/throughput evidence.

Autoresearch loop persists: actual source/profile/relevant history -> distinct
work/dependency/resource hypothesis -> Rust model/implementation -> targeted
independent math/curve/resource/generic/concurrency qualification -> FROZEN
coherent complete SDK all8+54 controls/readers/protocol/automatic profiles
BEFOREANYpreflight -> serializeRust1.98.1xtask -> duringANYpreflight/recording
ONLYpoll exact confirmed live handle until terminal (timeout/truncation isn't
terminal) -> verify every observation -> polite factual report,commitANDpush,
verify exact remoteHEAD/checkpoint -> continue full goal. No questions,Python,
subagents,large persistent LUT/key expansion,new production pool,artificial
RAMcap,C/ASM rewrite or favorable-subset rescue. No goal completion from a
negative screen, a small positive subset or a nearly exhausted context.
