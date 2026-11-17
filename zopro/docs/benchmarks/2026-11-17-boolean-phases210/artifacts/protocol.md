# Whole-proof branch phase diagnostic210

Frozen before build/capture,2026-11-17.209 fullproof screen is closed/notadopted;
these are not extra209screen samples. Positive208isolated query timing and lower
209aggregateCPU do not locate the proof's final dependency. Test actual branch
start/finish boundaries rather than claiming a scheduler/cache cause.

Copy exact209runtime source into isolated instrumentation build. Preserve every
arithmetic/solver/query operation; add24-slot bounded atomic timestamp record and
markers only. Trace solver,canonical gather,product,A,K/H(or fusedKH),G2parent,
groupdecision,guard,Phi,Boolean tree,outerjoin/output. Driver resets before each
SDK call and snapshots after its timer stops, prints records after all calls.
No per-event allocation/printing/newpool. Single sequential driver only; shared
trace buffer is diagnostic and not a concurrent production feature. Elapsed
stage durations include cooperative steals: not exclusive CPU or exact idle time.

Native focused209 guard/oracle/metadata tests plus trace invariants before capture.
New diagnostic SDK has four modes: BooleanOFF/ON x traceOFF/ON, same exact binary,
primary1x2/32 only,3independent executablebackings/3rotatedreversedblocks.
Four4worker1timed+3prime smoke processes/Gnark checks, then12main processes with
3primes+251timed each; externalGnark after all main capture.16fresh exports,
3064SDK calls, seeded from20958948proof ledger SHAe63bea37a40fb006269ea4b5bf9209a558ec7a712a40827c349c0dfa0321365b.
Existing certificate/fused/indexed flags stay on. No owned CPU cojobs during
capture. Keep all wall/CPU/RSS/route records and every timestamp. TraceOFF is an
observer control, not a new adoption comparator. Median traceON/OFF wall ratio
must lie[1/1.05,1.05] for both Boolean states before interpreting timings.

For each trace assert ordered solver/gather/product, correct known route, all
required branch endpointswithinSDK wallinterval, childphasecontainment,join at
or after branch ends, no nonzero unexpectedslots. Report perprocess median
branch durations, finishorder/tail gaps and mechanism branch counts; aggregate
summaries are descriptive, no confidence or speedup/adoption claim. If observer
or eventintegrity fails, report diagnostic inconclusive, preserve source/captures.
Never use a fast trace subset to rescue209 or extend samplecount for latency.

Decision: if G2 ceases to finish last but peer end moves later, focus on shared
resources/criticalpeer rather than standalone G2 timing. If G2 stilllast and
its measured span does not shrink, query208doesnot reproduce inwholeproof.
If G2 span shrinks but join doesnot, quantify dependency overlap withoutassuming
causality. If observations disagree, retain uncertainty and change question.
No new optimization threshold/window/scheduler variant follows without evidence.
Firstresult allowance20min; record elapsed. Nativeonly,notportable/pressure/ARM/
C7a or concurrency qualification. Existing goal active0/8. Localcommit/no push.
