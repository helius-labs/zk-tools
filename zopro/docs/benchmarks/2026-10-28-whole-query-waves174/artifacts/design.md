# Whole-query G1 then G2 phases

Prospective experiment on accepted174source/SDK81033; no performance claim.
The global grouping bypass50c046eb5 and isolatedB2bypassfbe87fec6 both regressed
primary32, despite lowerRSS. The isolatedcase kept forecasts/otherqueries fixed
and increased summedCPU188.197->192.407ms. This motivates testing interactions
between simultaneous kernels, not deleting the same grouping work again.

One fixed candidate: in the existing small fused-K/H canonical-wire branch,
complete A and fusedK/H through the SAME rayon::join and existing globalpool,
then run the existing full B2 query. OFF retains the original three-query join.
PrivateZOPRO_CPU_G1_THEN_G2_PROBE0/1 chooses the mode in one executable. No
worker partition/pool, affinity policy, artificial sleep/spin delay, warmup
allocation, arithmetic/digit/grouping/key change or new table/plan/unsafe code.
Original workspace forecast stays conservatively unchanged; query scratch may
stop overlapping. Concurrentproofs can still mix kernels globally, so claim
only ordering within ONE proof. Unsupported paths retain original dispatch.

History/source audit: 2026-10-24-joint-msm-phases lost~23% by flattening fills,
partition merges and weighted reductions across allqueries; it kept G1/G2 mixed
and extended bucket lifetimes. 2026-10-25-worker-groups-local lost~21% with
11/11/10worker partitions. B-first ordering and product-overlap are separate
closed negatives. This whole-query completion barrier is different from each.
Aeglos pinnedcommit0290613e8eeca17a4ce88f6f2d726e8e06a5385d still equals the
CPUbranch head. Its MSM run sorts jobs bydegree and shares eachpass acrossjobs;
it does NOT implement the proposed whole-G1-then-G2 algorithm. Primarysources:
https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm_impl.hpp#L388
https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/pool.cpp#L123
No code or claimed Aeglos speedup is copied. Sharedinstruction-working-set or
allocation/interleaving effects are hypotheses, not established causes. It may
lose parallelism, especially at96cores; no AWS transfer claim.

Add one private helper module with ordering/panic tests; preserve original
OFFjoin body and exact G1/B2 closures. One original lib.rs differs,173exact;
remove helper/edit to recover174originals. Candidate175files. NativeOFF/ON
unfiltered124core+raw2/cubic6 pass before timing; fieldsource/flags unchanged,
coherentfullnative dependencies frozen and actualflags/features audited. Do
not publish the private selector as a production option or adopt a shapewhitelist.

Fixed7rotated/reversed blocks×4cells(primary16/32,16x2/32,54/32)×accepted/OFF/ON;
151primary/31others after3primes.12one-round preflights independentlyGnarkverify
beforelatency;84latency exports verify afterward. Primary32 advancement needs
ON/OFFandON/accepted upper<1 with>=5/7wins plus OFF/acceptedboundsfactor1.03.
LargerONcomparisons upper<=1.03;calibrationreported separately. Physical16 is
descriptive; extrema7medianbounds98.4375%nominal/no familywiseguarantee. No
gainfloor/exclusion/extension/successfulcapture repeat/source-or-layout reroll.
Passing onlyselects furtherfullall8/clean/portable/resourcequalification.

AfterALLlatency and its verifier phase:12FIFO-gated groupedcounter processes
(primary16/32×3variants×fetch/stallpanels),31requests+3primes; fourevents/panel,
actualrunning>=99.9%. Fetch:{cycles:u,instructions:u,L1-icache-load-misses:u,
iTLB-load-misses:u};stall:{cycles:u,instructions:u,stalled-cycles-frontend:u,
L1-dcache-load-misses:u}. Then6separate199Hzcycles:u profiles,251requests+3primes,
onePID/noLost required. No callchains/globalsettings. Run allcaptures before
decoding/hashing/archive; no ownedbuild/helper/otherbench duringcaptures.
Diagnostic event shares are not removablewalltime or an exclusivecache cause.

Totals ifcomplete:9876internallyverifiedSDKrequests,114fresh164Bexports,
228proof/publicmembers, ledger15211->15325.96exports belongtopreflights/latency,
18tocounters/profiles. All independentlyGnarkverify/publicbind/freshness; no
profiling times replaceunprofiled latencies. SameOFF/ONpath/inode/equallength
mode values;acceptedretainedSDK81033. BothGLOBALDEDUP1/forecasts unchanged.
Keep inherited all8M5CPU+Metal table andoriginal10xgoalACTIVE0/8. No AWS/Aeglos
timing, mobile/ARM/TEE, giant, minRAM/swap/pressureclaim. Commit/push complete
experiment includingnegative. No production source/default change frompilot.
