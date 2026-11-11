# Joint query window screen

Question: can a shared set of ready window/reduction jobs across A, K/H and G2
avoid query-local waiting and reduce verified proof latency? Earlier K/H-only
frames improved the16worker caller but not reliably the full proof. Historical
neutralization already establishes that K/H and G2 work matter; no new query
removal or function-return census is needed. Earlier scoped continuations still
submitted individual Rayon reduction jobs. This prototype claims leaves directly.

Keep original point arithmetic, A grouping/window12/onepart, balanced K/H windows,
G2 grouping/phi12/twoparts/top handling, and final folds. Prepare A/G2 before the
joint stage, introducing a preparation barrier that is charged, not hidden.
Run fixed drainers on the existing pool. A drainer returns when no ready work
is claimable; any in-flight producer continues draining after publication.
No spinning for incomplete frames, new pool, persistent table or unsafe code.
Top-window and bucket-merge internal Rayon operations remain unchanged.

Added workspace: retain all A/KH/G2 bucket arrays and partials until joint return,
per-drainer G1/G2 scratch, and a G2 scalar copy when grouping does not own one.
No admission claim; this private prototype requires memory/fallback qualification
before production. Preserve original path outside primary geometry/settings.

Before timing: independent generator-weighted oracle for all three outputs,
zero/one/dense/signed/repeated/identity cases,1/2/4/16/32workers, nested and concurrent
shared-pool callers. Verify fresh real SDK exports independently with Gnark.
Preparation allowance30minutes to qualified SDK, capture allowance5minutes.

Reuse short SDK screen:12preflight processes(REF/OFF/ON primary1/16/32 and2x2/32),
81main(3blocks×3backings×3contexts(primary16/32,2x2/32)×3arms),3primes+17timedcalls.
REF exactacceptedc88a3528;OFF/ONsamefreshSDK,threebyteequalbackings. FixedCPU2/0–15/
0–31,defaultSDKstacks,acceptednative/allocatorflags. Freshassignment/solve/randomness/
proof/SDKverification/serialization included;load/primes/export excluded.
No owned work overlapscapture; onlypollconfirmedhandle. All timings precede
verifiers in each phase. Seedcanonical54100/SHA
88fbb44d60da915f835fd7f3a5b09eaff14df230f140029bee24414a30d0c6bd.

Primary all3backingmedianON/OFF andON/REFratios<1 selects mechanismprofiling and
resourceadmission. Mixedsignsinconclusive;coherentregressionrejectsrecipe.
No sampleextension,gainfloor,adoption,C7aorallshapeclaim. Keep this fixed question;
do not sweep chunk sizes if it fails. Report CPU/RSS alongside latency.
