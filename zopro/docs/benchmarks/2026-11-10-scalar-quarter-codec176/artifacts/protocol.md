# Fixed-mask scalar quarter codec

Prospective10November2026. PreviousgoalturnPROGRESS: persistentquarter actual
Window FAIL6/36 with allcalibrations/diagnosticsPASS, pushed77b4c939b7. Full
verifiedsingleproofCPUAVX>=10x suppliedM5CPU ALL8 remainsACTIVE/UNMET0/8,
reportCPU+Metal. Noquestions/subagents/Python/newAWS. CommitANDpush completed
experiment. No live inherited recording/build/cloud handles.

Distinct mechanism: replace ONLY scalarStored::encode two conditional modular
halvings. ForcanonicalnumericY<p,p≡3mod4,Y=4a+r,0<=r<=3:
quarter(Y)=a+r*T=(Y+r*p)/4<p, T=(p+1)/4. Fixedfourword carry sum ofshiftedY,
T maskedby rbit0 and2T maskedby rbit1. Eachintermediate carry<3, no u128
overflow, outputcanonical<p,zero->zero. No tables/input-sizedstate/newpool,
key orsolverplan/repr/layout/schedule change. Currentcodecinstructions/branches
mustbechecked; sourceif statements alone do notprove emittedbranches. Require
newprobe no conditionaljumps and old>=1 beforetiming; instructioncounts/calls/
mul/div retained, no predictedgain or minimumpercentagefloor.

Qualification fixedbeforeclocks:1276scalarboundary inputs(0..256,p-1-i,
2^i+[-1,0,1] i0..253),32768deterministicrandomcanonicalwords,1024changing
chains64steps=65536fed cases. Actualold/nativeNEW Stored.encode output exact
against independentBigUint (Y+r*p)/4, canonicalbounds, decode returnsY.
Fourexisting prefix/completeWindow/worker/layout tests MUSTrun against NEW
coherentfield/core because scalarboundary arithmeticchanged. This is newcode
qualification, not an unchanged bookkeeping replay. Inherit oldlowlevelvector
arithmetic and originalgeneric fallback sourceinactiveunder noIFMA; same-source
genericfield/core compile ifneeded toclose newsourceguards. Reuseoldfield/core,
REFdriver, originaldriver, fixture/oraclebytes withoutregeneration orrebuild.

Ifnative/structuralpass, one completeWindow comparison with FOURarms:
REF retainedaccepted-sourceREF;BASE newcore originalWindow path;OFF retained
oldquartercore candidateWindow;ON newcodec+quarterWindow. BASE/REFcalibration
checks equivalent algorithms. OFF is intentionally differentfromaccepted and
is NOTsubjectto equalitycalibration. This retains bothmechanism andaccepted
controls withoutrelabellingoldquarter differences ashostnoise.

Fixedsame6contextsG1/G2 15045points,width12,column0,batch200,workers1/16/32,
sameimmutablefixtures andperworkerdescriptors,512KiBstacks,CPUi/solo2.
3checkedprimes+17timedwaves;15fixedrotated/reversedcontext/fourarm blocks.
WholeWindow construction/feeds/queues/scalartails/normalization/fulloutput
equality/fixedchecksum/drop timed; loading/pool setup/hash excluded. Wave
isoneWindow/worker, NOTfullMSM/proof/serverthroughput. No subset rescue,
extension/unchangedreroll. Allobservations retained.

Frozen48checks:6BASE/REFcalibrationfactor1.03;18ON/nonreg upper<=1.02vs
BASE/REF/OFF;3G232gain+3equal-logG1/G232gain upper<1 >=10/15winsvsall3;
6RSS guards ON<=maxcontrols*1.05+1MiB (notarchitecturalcap);12diagnosticquality
checks. Fixedorderstat ranks4..12 of15 nominal96.484375%marginalcoverage under
independentexchangeableblocks,no familywiseguarantee. No gainfloor.

24preflights+360latencies complete BEFORE6PMUpanels+6cyclesprofiles, each
BASE/OFF/ON forG1/G2worker32,2048waves, FIFO warmgate. Countersgroup
cycles/instructions/cache-misses/branch-misses:u coverage>=99.9%. Profiles
period16M/512pages>=1000samples,unknown<=5%,zeroLOST/PIDTIDCPU/MMAP.
30offlineowner/events/loss/decode/metadata AFTERallrecording.396callers,
426capturecommands. Freezeall sources/compiler/deps/binaries/input/order/gates/
producer/rawreader BEFOREpreflight. DuringANYrecording ONLYpoll exactlivehandle
untilterminal;nohelper/build/hash/analysis/archive/report/push overlap.

Onlyqualifiedcompletecallers justify fullquery/ALL8+54SDK andfreshGnark export
verification, resource/smallstack/fallbackchecks beforeproductionadoption.
No declaredtargetreduction: CPU10xALL8+CPU/Metalcomparisons stillrequired.
Canonicalproofcount44265 unchanged; seedfutureexportsfromfrontend-overlap176.
