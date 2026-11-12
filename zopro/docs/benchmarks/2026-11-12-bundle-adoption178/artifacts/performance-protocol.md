# Exact bounded G2 candidate: local all-shape qualification

Question: does retained SDK b646c7b8... preserve the primary16 improvement and
acceptable all-shape latency/CPU/memory versus accepted c88a3528...? The prior
short screen and mechanism profiles remain closed. No new build or copies.

Reuse the original three REF and three FRESH executable backings from bundle-sdk178.
ON explicitly sets ZOPRO_CPU_INDEXED_TILE_BUNDLES=1. OFF is not a veto/comparator:
its mechanism role was answered in the previous screen/profile. Accepted REF is
the sole adoption comparator. Rust1.98.1 znver4, accepted allocator/environment,
fixed CPU0..15 or0..31, default SDK stacks and complete fresh-request boundaries
are inherited. No reused query results or witness/proof caching.

Contexts: all eight supplied shapes plus merge54 at32 workers, primary1x2 also16.
Preflight: both consumers in all10 contexts, plus ON with optional preparation
ceiling zero in primary16,2x2/32 andmerge54/32:23 processes,1timed+3primes each.
This tests configured denial fallback, not physical RAM exhaustion or minimumRAM.
Main:30 paired processes per context,17timed+3primes each. Rotate context order,
reverse odd blocks, alternate consumer order by block/3. Backing block%3 gives10
pairs/copy/context.600processes,10200timed/12000calls inclprimes. Allcapture before
verifiers in each phase; only poll confirmed livehandle during capture.
Preflight ends before main. Do not extend samples, drop contexts or swap copies.

Analysis unit: paired process mean. Report median ON/REF and sorted ranks10,21
of30 (zero-based9,20), nominal95.7226% marginal median interval under independent
exchangeable blocks; no familywise guarantee. Also report all three backing
medians, CPU, faults and peak lifetimeRSS. Primary16 latency upper must be<1;
other9 latency uppers<=1.02. All10 CPU uppers<=1.02. All10 maximumRSS ON must be
<=1.05*REF+1MiB. These are same prior local qualification resource/nonregression
margins, not an architectural RAM cap. No minimum gain floor. 30 checks total.
Per-backing primary16 medians must also all be<1:3 additional checks. Primary32
backing spread>1.02 is diagnostic, not a new numerical performance veto; unresolved
artifact interactions must be reported before any production decision.

Thirty independent process pairs per context improve on the nine-pair screen;
17calls averages request noise without repeating long warm loops. Rank-based
uncertainty may remain inconclusive; no promise of resolving an arbitrarily small
gain. The full objective remains all8 CPU10x, not this qualification's gain gate.
Report supplied M5 CPU/Metal separately; merge54 has no supplied reference.

Correctness: SDK verifies every request; each process exports last proof for
independent Gnark and image/public binding checks. Extend54304 ledger, expected
54327 after preflight and54927 after main. Source/body qualification inherited as
in bundle-sdk178, not retested. Check ONprimary/2x2 have4indexedcalls/request;
other shapes record complete route counts (can fall back), without claiming active
candidate coverage when zero. Denied cases require0entries and0maxcharge. Report
all route observations. Portable/concurrent/production selection remain separate
requirements if performance survives; no release/adoption from timing alone.

Preparation allowance30minutes; capture allowance20minutes excluding independent
verification. Reuse existing Rust readers/xtask; retain all helper failures. No
owned parallel builds/helpers/analysis during preflight or main recordings.
No new AWS resources or publication. Seed ledger SHA256
 a53b7f952c40444ab5dc5f88e38d1689716b2deb2e0cf465021784b8c7eadcf5.
