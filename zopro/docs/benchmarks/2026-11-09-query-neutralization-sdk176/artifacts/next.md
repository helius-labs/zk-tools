# Continue real CPU prover optimization after the query-removal study

GOAL remains ACTIVE/UNMET: verified single-proof CPU AVX >=10x supplied M5
CPU for ALL8; report both CPU and Metal. None meets it yet. No budget,
questions or subagents. Rust1.98.1/xtask. Commit AND push each completed
experiment. Reports must stay polite and factual. No multi-GB LUT, retained
execution plans, key/point/scalar/witness array copies, new production pool,
C/ASM rewrite or artificial architectural RAM cap.

Current study is CLOSED. Read analysis.json/findings.json/review.json,
reference-review.json and the public report. Do not rebuild/replay native
qualification or rerun any capture/profile/decoder/Gnark verifier. All1370
capture/offline commands succeeded once, all610exports are unique and
independentlyGnarkverified;31840SDKrequests including3primes perprocess.
Canonical37633 ledger: THISROOT/proof-sha256-after-capture.json. All future
exports must seed it exactly, not the nowhistorical37023 seed.

Actual consumers: THISROOT/sdk-OFF andsdk-ON, adding only ru_majflt output
to the qualified harness; native prover libraries/field1200 exact. Their
consumer-audit.json and42old/newactual callbackbody decodes are retained.
No production changes/adoption. Native priorstage34fd5a5d7f remains valid.

Key results (ms; cached query work REMOVED, not realproofperformance):
context       active0    noA      noG2     noKH    noG2/KH  noqueries
1x2-16         9.659894  8.466018 6.514417 7.031360 4.005518 2.237923
1x2-32         8.023722  7.630239 5.446772 6.496744 3.875174 2.452107
4x2-32        16.199434 15.41428312.82949312.606017 8.437663 5.658279
Fullmask1..7means/ratios/CIs and allrawsamples are inanalysis.json.
32worker control calibration+observer gates pass for both contexts.
16worker calibration AND observer CIs fail; retain, no favorable cohort
promotion. Global diagnostic quality FAILS:4x2-32-m7 unknown5.338738%>
5%;17270samples/100%assigned/zeroLOST. Allother29profiles pass. All120PMU
values100%running,720598samples total;no extensions/unknownreclassification.
No exact/additive exclusivefraction, immutablelowerbound or universal
memory/device inference. The32worker query removal sensitivity still
motivates real-work reduction, with these limitations reported.

Atprimary32: noG2 paired/m0 ratio0.6829, noKH0.8080, noA0.9519,
noG2KH0.4819,noqueries0.3048. Removing A looks much more important after
otherqueries disappear: schedule/overlap changes. Microkernel wins cannot
be priced by adding flatprofile shares or these changes independently.
M0 measured ~1.387B instructions/request, noG2~826M, noKH~894M,noqueries
~119M in single descriptive PMU panels. G2/KH point work is material;
genericcachemiss/fault counts do NOT establish RAM as the only bottleneck.
All549processes withnewmajorfaultcounter have timedmajorfault0;accepted
immutable6d83 does not expose that counter. RSS isprocesspeak,notminRAM.

Fresh accepted primary32=8.031248742ms/112.75MiB;4x2=15.961623323ms/
312MiB. No slower or faster build adopted. Historical7.915095 remainsvalid,
private7.859756/7.869400 unadopted. Six other ALL8 timing values inherited
fromtree-policy-sdk176; all8CPU/Metal references have separate review.
Goal0/8. Never compare neutralized timings asfreshprover vsM5/Aeglos.

The first resultsreader failed only on exactfloatarray equality afterJSON
roundtrip (12.907180516129031 vs...033). review-tolerant.rs extends its
alreadydeclared1e-12numeric scalar tolerance to vector/contrast floats;
originalreader/data/gates kept. Newreaderworkflow passes. No measurement,
bootstrap gate,16worker failure orprofilelimitchanged. Analyzer neverrerun.
Three initial helpercompiles failed on json!([0;5]);fixedtoexplicitarray
before those three helpers or any SDK capture executed. Tooltranscriptonly,notworkflowclaims.

NEXT SAFE ACTION / AUTORESEARCH LOOP:
1. Use these results to select a REAL work-saving mechanism across G2/KH
   point arithmetic/MSM. Sourceaudit the actual kernels and prior CLOSED
   reports first; do not claim an alreadyimplemented or failed recipe new.
   Primary G2 currently ALREADY merges phi high/low into the same window
   buckets before weighting. Independent A/B coefficientmatrix contraction
   is alreadyclosed bynear-fullrank (2026-10-25-query-rank-census). GLS4,
   wNAF, shiftedkeytables, ring recoding, shared frontiers, two inversechains,
   borrowedbuckettrees, PMNS compactbridge andmicromultiply variants have
   completedreports. Read them before choosing a supposedly different idea.
2. Freeze a distinct mechanism and reduction in actual complete-query work,
   not an isolated arithmeticspeed estimate or lookup of samewitnessresults.
   Favor reusing existing buffers/layout and original point/scalar ownership.
   Research primary papers/code when needed; user wants clever Rust, not a
   language rewrite. No newpaper has been read in THIS diagnostic study.
3. Implement privatecfg, qualify changed arithmetic/point/path/worker/stack
   boundaries, exactsource recovery and coherent release artifacts. Reuse
   unchanged tested components. Keep granular meaningful tests/benchmarks;
   do not duplicate oldsuccessful suites asbookkeeping.
4. Freeze freshaccepted/OFF/ON calibrated completecaller/fullproof blocks,
   actualgenerator/Gnark correctness, resources, CPU16/32 and automatic PMU/
   profiles BEFORE anyrecording. Goaladoption needs ALL8+merge54 relevant
   cases, not a singlefavorable routine. Keep allfailures; no thresholdrescue.
   During ANY recording onlypoll exactconfirmedlivehandle; nootherCPUwork.
5. Once done, independentlyreview allbindings/means/gates; adopt consistent
   verified fresh-work gains with memory qualification. Preserve modest
   positive changes when supported;0.032ms isn't zero. CommitANDpush every
   completed experiment, update handoff/status and repeat until ALL8goal
   truly verified. Leave goalactive whileunmet; no completion claim based
   on diagnostics or the pipeline being convenient.

Owned live jobs/cloudresources: none after this closed capture. Only actual
newtool handles establish liveness. No lock/readyfilealone proves a wait.
