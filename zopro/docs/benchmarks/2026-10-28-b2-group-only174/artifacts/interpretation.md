# Isolated B2 bypass: negative result

The private B2-only bypass fails both primary gain checks. At32workers the
pairedON/OFFmedian is1.0128484354602343, bounds
[1.0023316081800504,1.0258291719885329],0/7wins; paired savingNEGATIVE0.104376ms.
ON/accepted is1.0193354376892214,[1.0098022192914078,1.022932275956623],0wins,
NEGATIVEsaving0.156495ms. WholeSDKmedians: accepted8.105792ms,OFF8.123635,
ON8.228011. PrimaryOFF/accepted calibration passes:
1.0034757055027492,[0.9962293655821169,1.0168332596754848]. This is an observed
regression in the declared protocol, not merely an uncertain small gain.

All seven prospective checks except the two primary gain conditions pass.
Larger default32 ON comparisons satisfy their3%upper bounds. Their point
estimates do not improve:16x2 ON/OFF1.0056435706,ON/accepted1.0093319172;
54ON/OFF1.0114276584,ON/accepted1.0174948506. Larger paths mostly retain their
original dispatch; their variation is not an isolated B2 arithmetic attribution.
Physical16 ON/OFF has a positive1.882%median estimate and5/7wins, but wide
bounds[0.8719287583,1.2366858013]. ON/accepted is1.0199327184 with2/7wins.
Retain both the positive OFF comparison and the uncertainty; do not infer a
confirmed physical16 win or erase the primary32 regression.

Memory is lower for the primary:32-workerONmedian/maxRSS85.25/88.25MiB versus
OFF130.25/136.00,accepted132.25/136.25.16-workerON64.25/67.25 versus
OFF88.25/89.25. These are lifetime peaks including keyload/primes, not minimum
RAM or universal savings.16x2/54peaks are mixed. Primary32 summedCPU rises
188.197->192.407ms/request(OFF->ON), despite removing the grouping attempt.
The original workspace forecast and globalDEDUP=1 are exactly retained, so the
negative is not explained solely by the previous global-switch forecast change.

What is established: removing this exact caller preparation does not shorten
the tested proof. What is not established: a unique allocator, cache, wakeup,
preemption or kernel mechanism. Allocation history, memory residency, timing of
shared-pool work and generated code can matter. ON/OFF share one executable
and differ only at the private branch, so do not attribute their comparison to
a different executable backing. OFF/accepted is a different build and remains
separately calibrated. The traced1.4ms elapsed grouping span is not a removable
critical-path budget. Global and now isolated bypasses are closed without an
unchanged rerun, profile-based rescue, wave-size sweep or threshold relaxation.

Next direction should reduce actual CPU work or instruction-working-set
interference in the shared MSM phase. A plausible distinct hypothesis is
homogeneous G1 and G2 execution phases using the SAME full Rayon pool, rather
than fixed worker partitions or arbitrary delays. Check older source/results
and actual execution code (including available Aeglos scheduling) before
selecting it. A whole-G1-then-G2 barrier is not the rejected B-first work-steal
ordering or product-overlap change; it changes which kernels can coexist and
can reduce live query scratch. It might also lose parallelism, especially on
C7a, so it needs a prospective matched latency/CPU/profile experiment. No gain,
exclusive cache cause, universal scheduling rule or AWS prediction follows
from this negative. Do not add a synthetic warmup allocation to imitate the
discarded hash map, or adopt a fixed memory cap/table to chase this result.

Validation: both private modes pass unfiltered122core+raw2/cubic6 tests,
including8G2geometries×4scalarfamilies and borrowedG2edge cases. One original
source file changes;173other originals exact; reversing the guard recovers174.
GlobalDEDUP,workspace/G1/fusedKH/key/solver/field SOURCE and accepted nativeflags
are exact. All native libraries were rebuilt coherently and frozen, with actual
core/field fingerprints and proc-macro shared objects. No new field-suite
qualification or binary-identical-arithmetic claim. OFF/ON use the exact same
candidateSDKpath/inode; accepted uses retainedSDK81033. The candidateSDK is
57472a9f3001cef0dc4ae913a4f789415e0c80308afbfb9c948184725d5ca8f3.

All12preflight exports independently verify before84latencycaptures;84latency
exports independently verify afterward.7944complete SDK requests include
primes,96fresh164Bproofs/192proofpublicmembers match bindings and are unique
beyond15115prior identities; ledger15211. All192workflowsteps complete once;
session20631terminal/reaped0. Closing source/library/SDK/helper/inputpins pass.
Native66415terminal/reaped0; no builds/helpers/hashes/archives/profiles/other
benchmarks overlap SDK captures. No failures, exclusions, extensions or repeated
successful captures/tests/links. No production source/default change; no further
portable/concurrency/pressure/clean-source qualification after the failed gate.

Accepted174source and8.366931ms remain authoritative. Fullall8M5CPU+Metal table
is inherited explicitly; newpilot timings coveronly1x2/16x2 suppliedreferences,
plus54/physical16. Both referencedONtimings remain above10xCPU thresholds.
GoalACTIVEUNMET,accepted0/8. No newAWS/Aeglos/mobile/ARM/TEE/giant/swap/minRAM/
pressureclaim, no ownedcloudresource. Commit/push the complete negative politely.
