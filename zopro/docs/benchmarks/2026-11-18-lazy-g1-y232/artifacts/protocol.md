# G1 compact lazy-Y in current scaled-prefix pipeline232

Goal stays verified CPU AVX <= supplied M5 CPU/10 across all8 shapes, report CPU
and Metal. Previous turn PROGRESS:231 complete pair-cache count closed. Baseline
accepted227/source186/b70SDK, sourceb0a41f30de. Canonical61143 from230, unchanged
until fresh export verification. No production mutation or push.

Hypothesis: keep canonical X and typed numerical Y<2p in ordinary G1 buckets,
skip Y canonical reduction after vector updates, normalize at scalar tails,
equal-X exceptions and final output. Bucket64B and kernel record1600B unchanged;
key points remain canonical and borrowed, no transformed key copy/table/pool.
The failed G2lazyY176 used different fused R256 bounds/correction, and its
complete Window regression remains closed. Old rawR260bucket/key-copy and late
coordinate-reload studies remain closed. This changes only G1 bucket Y liveness
in current227scaled-prefix flow, no radix/storage-width/window/grouping change.

Reuse the qualified old LazyFq/LazyY8 raw import/export prefix, source-bound and
byte-exact except test-only constructor guard. Private new signed numerator
forms2p+by-ay or3p-by-ay in[0,3p], consumes immediately with prefix<2p using
R260REDC. Output<p+6p²/2^260<2p (6p<2^260). Normalized five52-bit limbs retain
the old IFMA row coefficient bound; no input exceeds4wordcapacity since3p<2^256.
General field interfaces/bounds unchanged. New private helper gets independent
schoolbook-product/full-REDC reference and Arkworks congruence/bound tests,
includinginclusive3p, lazy statefeed, allsignmasks and rawroundtrips.

Alternate borrowed G1 Window has separate Bucket{FqX,LazyFqY,marker}, same64B.
First inserts/signs and projective spills use original canonical points.
Equal-X compares canonicalY. Final points canonicalize before projective merge/
weighted tree. Scalar tails canonicalize their current bucket and keep original
scalar formulas. Only vector full batches retain Y lazily. Safe terminal
conversion must be tested for allocation reuse; no portable allocation/memory
guarantee from equal size. Preserve generic/noIFMA routing via original module.
All original controls/operators/geometry/environment remain.

Native field/core unfiltered tests, new fullWindow generator-coefficient tests
with sign/zero/identity/equal/opposite/projective/tail/reuse/bothinverse/worker
cases, sizeof and allocation conversion checks. Fresh coherent OFF/ON library
links with exact accepted driver/support. Generic tests/check cover guards.
Only if correctness/build complete:3fourworker1timed+3primesmokes then3matched
blocks fourcontexts(1x2/16,1x2/32,2x2/32,merge54/32) REF/OFF/ON, exact230screen
251/251/51/7 timed+3primes,threebackingseach,rotate/reverse,defaultstacks/fixed
affinity. Both rawsmokereadersmustPASSBEFOREguardedmain. NoownedCPUcojobduring
ANYSDKcapture;lateGnarkafterallcaptures. No scientificsuccessfulreplay.

Descriptive3blockmedians/noCI, usefulprimaryON/control<1with>=2/3winsagainst
BOTHcontrols at16or32; no gainfloor. OFF/REFcalfactor1.03;RSSON<=eachcontrol
*1.10+8MiB across4contexts. Report everyratio/CPU/memory, noexcludedblocks,
geometry/threshold/cfg/layout/curve subsetrescue/extension/reroll. Linkedcodegen
review beforecapture; staticcounts notspeed. No signal closes exactcomposition;
promising exactbinary gets currentdiagnostics/all8+merge54/portable/resources
andfreshGnarkqualification beforeadoption. This alone cannot explain2x.

Preparation30min,capture15min; report actuals/overruns. Freeze plans/pins and
independentplan review beforecapture. Correctness/build failures are retained
and may be repaired beforecapture, no relabeling or successfulstep repetition.
NoAWSneeded; exactcurrentb70stillnoC7a measurement. Commit completed evidence
locally; no push/subagents/newPython/C/ASM. Researchhistory precedesprotocolclock.
