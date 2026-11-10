# Continuation and autoresearch loop

Goal ACTIVE/UNMET: verified CPU AVX single-proof >=10x supplied M5 CPU for all
eight shapes; report CPU+Metal. No narrower success condition. Accepted local32
ms7.783593581/10.932730677/15.757088065/23.973294548/33.827268968/39.900916645/
66.041082710/65.717993516. Targets3.7/5.9/10.9/18.9/28.4/33.5/52.8/50.7;0/8.
Accepted16primary9.016864226ms;merge54 162.170157871ms with no M5 reference.

Previous goal turn PROGRESS: incoming-Y fusion qualified and measured FAIL7/48,
pushed17c33c001d. Current turn PROGRESS: independent ELF+MMAP2 mapping of existing
G2 profiles cross-checks every instruction sample against perf annotate. No new
native candidate, recording, timing or proof. All10offlineworkflowcommands PASS.
No owned inherited live job or cloud. Analysis49609 terminal0; tools terminal0.

Read README/protocol/analysis/analyze.rs. Both retained binaries match published
identities. Correct ELF virtual/file page offset matters: naive file-offset
relocation wrong by4096B. All target IPs exact instruction boundaries and all
perf annotate counts agree. OFF/ON function bytes51466/51415, instructions8672/
8625, prologue sub rsp7872/7872B. Explicit-rsp static accesses1619/1593;
samples4367/4031; vector-rsp samples4245/3879. IFMA samples12969/13640;
other-vector10572/10486; vector moves5003/4787. vpmullq only14/25 samples out
of30112/30575 function samples (wholeprofile41414/42113). Do not infer causal
latency or spill counts from sample IP: skid possible, stack arrays intentional,
shared host/separate panels. Integer multiply and increased aggregate stack
spilling not established as causes. Do not claim a solved bottleneck.

Underlying candidate still CLOSED FAIL7/48, no adoption/SDK trial: all18nonreg/
6RSS/12diagnostics PASS;1G132cal +3G232 +3aggregate gains FAIL. Full evidence in
/home/ubuntu/src/zopro-perf-work/fused-incoming-dy176. Its snapshot is181members,
179byteexact to scalar-quarter-codec176; changed field helper+core quarter kernels.
Native196608exactlanes/4Windowtests/24allocationcases PASS. Native15/build5/
capture426/post2 all448commandsPASS. Current profile analysis is not a rerun.

Prior exact negative evidence read this turn: whole-loop G2 prefix/reverse helper
separation in2026-11-08-g2-phase-callers176 was5.27%slower aggregate vsfreshOFF,
0/15wins; per-fused-product outlining in2026-10-29-g2-fused-outline175 slowed
primarySDK2.34%vsaccepted,0/15wins. Differentbuilds/fixtures, no speed transfer.
Do not repeat those unchanged or assume helper boundaries improve codegen.

NEXT source/dataflow review before any implementation: shorten lifetime of wide G2 reverse-loop
values with a Rust helper boundary between slope and coordinate finishing.
Current G2Scratch::apply computes ax/bx/ay/dy/d/factor/square/x/y in a large
inlined function. Retaining values can force stack traffic, but current profile
does not prove this is why fusion regressed. Try no new heap/record expansion:
compute factor/update inverse, call a slope helper using borrowed existing8bucket/
incoming-point arrays; return two Wide2 slope components, then load coordinate
inputs needed for x/y finishing. Reloads and helper call costs must be charged.
Do not assume source scope changes guarantee emitted liveness; inspect actual
assembly and frame/stack/static sample evidence. Compare all G1/G2 contexts,
not G2-only rescue or worker/parameter selection. G1 code may remain exact.
The within-eight-point slope/finish boundary differs from prior whole-loop or
per-product outlines, but requires an explicit liveness/reload cost model before
trying it. Inspect actual source/assembly; do not proceed only because it is new.
If the helper merely moves work, changes memory badly or does not improve
complete caller, close without rescues. Pick a stronger mechanism if inspection
supports one. Tiny constant-mask replacement of vpmullq is deferred, not disproven.

No field arithmetic changes needed for a lifetime-only attempt: reuse exact
qualified field library; new coherent core tests must cover all old complete
prefix/Window/sign/tail/Phi/zero/worker/sharedpool/input-preservation cases and
allocation/layout invariants. Reuse independent fixture bytes/old controls.
New actual callers and diagnostics before SDK/fullquery. Any accepted production
change requires fresh ALL8+54 SDK/Gnark proofs/resource checks, not microprobe wins.

Autoresearch: revalidate goal/source/live handles; choose a distinct falsifiable
mechanism from evidence; prove type/numeric/lifetime invariants; prospective
protocol before clocks; isolated coherent Rust source/build; meaningful independent
tests, fullcaller correctness and allocations. Freeze controls/inputs/binaries/
order/thresholds/producer/rawreader before recording. During ANY preflight or
recording ONLY poll exact confirmed live handle until terminal; timeout/truncation
not terminal; no owned helper/build/hash/analysis/report overlap. After terminal
independent raw stats/resources/decisions, preserve every failure, no unchanged
rerun/favorable-group/worker/parameter rescue. Publish polite factual report,
CPU+Metal ALL8 inherited explicitly; commitANDpush/exactremoteverify/checkpoint.
Continue until actual full-goal proof, never redefine success around a component.

Canonical44265 unchanged. Seed ALL future proof exports:
/home/ubuntu/src/zopro-perf-work/frontend-overlap176/proof-sha256-after-capture.json.
AcceptedSDK /home/ubuntu/src/zopro-perf-work/busy-reset-sdk176/sdk-ON
SHA7aff5f602a1e250a6b9b28889bf6278b51ec400af8399c7481c36248a8718d01.
Rust1.98.1+serializedxtask; no questions/subagents/Python/CASM/giantLUT/expanded
keys/solverplans/newproductionpool/arbitraryRAMcap. No new AWS/ARM/mobile/huge
2504999/pressure/swap/minRAM/proof measurement in this offline stage.
