# Fixed worker placement as a neutral measurement control

Goal remains verified single-proof CPU AVX >=10x supplied M5CPU for all8,
report CPU+Metal. Active/unmet0/8; no budget/questions/subagents. Rust1.98.1/
xtask, commit AND push completed experiments. Universal lowRAM; no key/point/
scalar/witness copies, execution plans/LUT, extra productionpool, C/ASM rewrite.

Previous goal turn PROGRESS: safe initialized-prefix trees pass native/Gnark,
0.416% supported32-worker gain, but16-worker controls/nonregression unqualified.
Production176 and acceptedSDK6d83 unchanged. Preserve that result, no rescue.

Existing weighted-live-prefix176 rows show16-worker wall variance associated
with involuntary context switches, with little CPU/minor-fault movement.
This is exploratory, not a causal proof. History already has Oct24 physical
CPU-mask/context-switch traces, Oct27 paired neutral controls within one SDK,
Oct22 rejected idle-yield changes and Oct21 failed32-worker per-worker pinning
optimization. Do not repeat those recipes. This distinct intervention asks if
fixed one-to-one worker placement makes the current16-worker neutral control
more stable than inherited broad affinity. It is a benchmark-control question,
not an arithmetic optimization or adoption of the weighted candidate.

One benchmark-only driver links exact8 accepted libraries; no library rebuild.
Both runtime modes explicitly initialize the existing global Rayon pool once
before loading, with standard default stack policy and equal worker counts.
Unbound mode retains the inherited mask for every worker. Fixed mode calls
Linux sched_setaffinity once in each worker start_handler, binding worker i to
CPU i of the inherited ordered mask. Main thread keeps its mask. No extra pool
or worker, global OS/CPU-policy setting, secure randomness or proof code change.
A broadcast records index/TID/affinity before primes and after the timed loop.
Those receipts, actual task count and parent mask must match the mode; fixed
mode must have bijective masks. No callbacks/accounting inside sdk.prove.

Three arms: exact accepted standardSDK; explicit-unbound neutral driver; same
neutraldriver fixed placement. Contexts primary1x2 at16workers/physical0-15 and
32workers/logical0-31. Fixed32 is a control condition, not a rerun of Oct21 PGO
optimization. No claim of novel pinning algorithm or general performance gain.
Freeze31 matched rotated/reversed blocks×151 requests/context/arm,3primes.
Correctness preflights primary+merge54 atbothcohorts/all3arms. Alllatencies then
separate31requestgroupedPMU+501requestcyclesprofiles (16Mperiod,512pages), all
recordings before independentGnark/decode. Duringrecording ONLYpoll livehandle.
Seedcanonical37864 fromweighted-live-prefix176. Everyexportunique/proof/image/
public/VKbindingchecked. No repeats/exclusions/extensions/relaxedpostdatagates.

Descriptive stability per arm: process-mean coefficientofvariation (sample SD),
relativeMAD, max/min and all pairedblockratios. Prospective support requires
fixed CV<unbound CV/2 AND fixed relativeMAD<unbound relativeMAD/2 at16workers,
fixed/eachcontrol95%pairedbootstrapupper<=1.02 and>=21/31wins for latency,
and all masks/TID/taskcount/proofs/profilequality checked. No minimum mean
gain floor. Unbound/standard calibration interval[1/1.03,1.03] is reported as
an observer-context check; failure makes absolute timing interpretation
unqualified and cannot be waived. 32-worker nonregressionupper<=1.02 both
controls. Resource screen RSS<=maxcontrols*1.05+1024KiB. Qualifying numerical
control criteria alone do not adopt runtime pinning or any arithmetic change.
Bootstrap20kseed0x551c36a48ee2d971 ranks499/19499, nofamilywiseguarantee.
Profile1000samplesmin,95%assignedmin,5%unknownmax,zeroLOST,99.9%PMUrunning.

Preserve original32-worker weighted gain and original16-worker failure. If the
intervention supports a useful control, a separate prospective follow-up can
compare exactweightedOFF/ON under commonplacement; all8+54/admission/portable/
concurrency remain required before adoption. If not, close this intervention
and pursue distinct G2/KH arithmetic/work reductions without unchanged rerolls.
NoM5/Aeglos/AWS/phone/TEE/minRAM/pressure/swap/throughput result inferred.
