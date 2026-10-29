# Eight independent DFT8 columns in native DIF leaves

Declared before source/build/capture. Accepted175/a1585b86/SDK20fde remains
8.022165ms primary/134MiB peak; all8CPU10x active/unmet0/8, target3.7ms.
Ledger17239. Prior packed DFT8 caller3587a185 remains rejected (12% slower).
This changes SIMD dataflow and reduces wasted lane arithmetic; it does not
rerun that layout or change flags/annotations to seek a favorable reroll.

Actual native DIF leaf's final three stages operate on pairs of Pair vectors.
Four such pairs contain eight independent8pointDFTs. Transpose4x4 separately
in each half to expose eight independent sequences across lanes. Evaluate
same independently derived DFT8 identity with4vectorM+26linear operations,
then transpose back into exact original bit-reversed output layout. Input
and output transposes need32Pair shuffles in total, the same static shuffle
count as four old packed kernels; old arithmetic uses8M+24linear operations.
These source-operation counts are a hypothesis, not machine instructions,
latency, cache evidence or a proof of a bottleneck. New compiler live ranges,
stack traffic, constant setup and data movement can lose.

Apply only to native IFMA8 untwisted DIF serial leaves with at least8Pair
vectors. Keep all higher DIF stages byte-equivalent; replace finalgap1
butterflies plusdif_within. DIT/forward, twisted roots, shorter leaves, query/
MSM/field/solver/key/scheduler/FFT geometry and unsupported paths unchanged.
Root constants built once per affected leaf from its actual last layer;
canonical half baked as Fr constant, no runtime inverse. Existing allocation,
source storage and buffer capacity unchanged. Fixed tile scratch and root
constants add bounded compiler stack/live temporaries, not a key plan/table,
input-sized allocation or new unsafe/assembly. Frame/pressure bounds require
separate qualification if this passes. Production source unchanged in pilot.

Source176: one parent cfg-only hook/module addition and one child;174 of175
originals exact; inverse edits recover175. All17field files exact/inherit33
field tests. FreshOFF124/ON126core and8prover each; officialgeneratorcheck.
Two new tests verify tile transpose/basis/random/full-width boundaries versus
scalar directDFT atall4primitive8roots, and complete selected native DIF leaves
versus independent scalar transforms at short/full lengths. Existing complete
FFT polynomial and512KiB-stack multiworker tests still included. Explicit
separate inverse-roundtrip supplement qualifies the prior packed caller's
promised check without repeating its3passedtests or any performance capture.
Actual linked candidate must reach eight::dif_serial, absentOFF, before timing.

Direct actualSDK pilot; do not substitute an isolated field gain for proof
latency. All9shapes×3accepted/OFF/ON preflights+Gnark before45primary32latency
processes;15fixedrotated/reversedblocks151requests after3primes. All45latencies
before45Gnark;3FIFO4eventstats31requests and3separate199Hzcyclesprofiles251
requests after3primes precede6Gnark/9decode.165steps7902SDK78freshGnark156
members, prospectiveledger17239->17317 onlyafterclosingpins/binding/freshness.
Allhelperscompile+PMUfeasibility+pinsbeforecapture. Noownedbuild/helper/hash/
archive/secondworkload duringcaptures. Noexclusivehostclaim.

ONupperpairedratio<1 and>=10/15wins vsBOTHcontrols;OFF/acceptedcalbounds within
[1/1.03,1.03].Ranks4..12of15nominal96.484375% percomparison under independent
exchangeableblocks, nofamilywise guarantee. No gainfloor, exclusions,
extensions, changedthreshold, successfulcapturerepeat orfastestlayoutselection.
Failurecloses exacttile/layoutintegration;passonlypermitsseparatelydeclared
all8+54/physical16/portable/resource/cleanartifact qualification, not adoption
or goalcompletion. Reportfreshprimaryand inheritedacceptedall8CPU+Metal with
scopesexplicit;retainphysical16baseline-calibrationengineeringexception.
Archiveall176source/non.rs/tests/commands/proofs/decodedprofiles;rawperfELF
localhashed,ownedchownonlyaftercapture. Commitpushcompletedexperimentincluding
negative,politereports.NoAWS/Aeglos/phone/TEE/giant/concurrent/pressure/minRAM
result inferred from this screen. No cloud/resources/subagents/questions.
