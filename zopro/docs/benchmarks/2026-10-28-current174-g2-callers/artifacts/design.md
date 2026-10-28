# G2 caller boundary diagnostic

Follow the completed exact174 stage study's1.55–1.71ms join-to-raw delay.
Use the SAME acceptedSDK81033 inode/source174/flags/environment and parser.
No prover/dependency/source build, arithmetic or scheduler optimization.
Trace SDK, join callsite, G2 query, scalar_groups, group_affineG2, Reservation
analysis, headroom and G2 raw entry/return:16owned zopro174stage events. Derive
file offsets from executable PT_LOADs, retain caller disassembly, validate
actual event attributes. Direct restricted tracefs writer avoids tee-a seek.
Only remove this owned group; no globalenable/sysctl/ring/kernel-policy change.

Three fixed complete blocks each trace primary32(CPUs0–31) and primary16(CPUs
0–15distinctcores), reversing order in the middle block.3primes+31verified SDK
requests each.6captures,204SDKcalls,6fresh independentGnarkexports, prospective
ledger15053->15059. Warm FIFO gates exclude loading/primes; every traceperiod1,
usercycles199Hz. No owned build/helper/hash/archive/secondbench overlaps captures.
No OFF rerun: this is a distinct caller-interval question, not latency or speed
qualification; prior coarse-tracing overhead and all observations remain.

Require one sampledPID/no lost records and31SDK/join/query/raw spans per process.
Query must nest insidejoin, rawinsidequery, grouping before rawonqueryTID.
Optional group-affine/admission/headroom spans stay explicit; at mostone own
call in their expected scopes. Partition query pre-raw into grouping, optional
grouped-point reduction and remainingcaller work; retain dispatch gap fromjoin
toquery. Admission/headroom are SUBSETS ofgrouping, not additive phases.
Thread-matched sampled user-cycle period may locate active caller work but is
not a calibrated elapsed CPU timer or proof of one wait cause. Keep every
request, block and complete raw event stream; no extension/exclusion/recapture.

SDKverification remains inside each call; all6final164Bexports independently
Gnarkverify, match fixturepublichash and differ from15053prior identities.
Report diagnostic times/limits plus inherited all8CPU+Metal table/goal0of8.
Accepted8.366931ms remainsbenchmark; no newAWS/Aeglos, mobile/ARM/TEE, giant,
swap, minRAM/pressurequalification. HistoryB-first/emptyprobe/guardbatch/
batchtransfer/groupingprefix/parallelgroups/bitmap candidates remainclosed.
Use this evidence to choose a DISTINCT CPU change; do not infer a fix from
lateinclusivecallback time. Commit and push the completed diagnostic.
