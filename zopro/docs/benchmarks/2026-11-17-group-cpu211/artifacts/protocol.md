# G2 grouping CPU time versus elapsed time211

Frozen before211source build/capture on2026-11-17. Previous goal turn PROGRESS:
209integrated bounded Boolean metadata but failed short SDK gain selection;
210phase trace showed stable~5msMSM join and an unexplained~0.8ms redistribution
between serial grouping and Phi elapsed spans.209recipe staysclosed/notadopted.
Goal all8CPU10x stillactive0/8; no smaller definition of success.

Assumption under test: the grouping elapsed-time shift may be waiting rather
than a different cost of serial grouping work. Grouping's current path doesnot
fork Rayon tasks; resource admission canlock/queryOS, hashing andallocation
remain. Copy exact210diagnostic source and add CLOCK_THREAD_CPUTIME_ID brackets
inside existing grouping elapsed markers. Record CPUstart/end plus workerindex
atbothends in4atomics. Equalworker tags required. CPUtimeincludesuser+kernel
runningtime and CPU stalls, excludes descheduling/blocking; wall-minus-CPU is
not an exact mutex/scheduler-cause classification. No altered field/MSM/solver,
input, grouping decision, threshold, allocation or prepared-state policy.

Reuse210bounded24eventtrace/driver/xtask workflow. Native existing5focusedtests
plus clock/reset invariant. Fourmodes:BooleanOFF/ON xtraceOFF/ON, same newbinary,
3independentbackings/3rotatedreversedblocks,primary1x2/32only. Four4-worker smoke
processes(1timed+3primes), then12main processes(251timed+3primes). All main capture
before lateGnark; no ownedCPUcojobs.16fresh exports/3064SDKcalls. Seed21058964,
SHA34f60e381bf9259290fa47b26fd5e9478e7624ea1557caa01ef2c068ed7f2d8b.

Reuse all210event/route/proof/source/observer checks. TraceOFF CPUrecords allzero;
traceON currentgroup worker tags equal/in1..workers, CPUend>=start>0, CPUdelta
<=group elapsed+10us forcross-clock overhead/accounting tolerance. Descriptive
perprocess CPU/groupelapsed/unaccountedelapsed medians and ON/OFFpairedchanges,
retaining everyrecord. Wholeproof observer medtraceON/OFF withinfactor1.05 both
Boolean states required before interpretation. These samples are diagnostic,
not latencyadoption qualification or additional209screen observations.

If CPUandwallshifts track, investigate runningwork/PMU behavior ratherthan claim
blockedtime; if CPUstableandwallshifts, isolate wait source next; if neither
reproduces prior split, state that instead of extending samples. No exclusivity
claim forparallel Phi/A/KH spans; only serialgroup CPUbracket has threadCPU.
No unchanged timingretry, threshold/window sweep or automatic causal conclusion.
No large tables/keycopies/newpool, physicalpressure/ARM/C7a/portable/concurrency
claim. Firstresult allowance20min. Preserve failures, commitlocally, do notpush.
