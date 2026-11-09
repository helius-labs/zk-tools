# Full-proof query neutralization diagnostic

This selects diagnostic semantics and implementation work. No native build,
preflight, proof or recording has started; the capture protocol is NOT yet
frozen. Actual tool handles, not this note, determine job liveness.

Question: how much accepted full-proof wall time remains when original A,
G2 or K/H query work is replaced by its exact previously computed result
for the same immutable key and exact witness bytes? Nested Rayon callback
spans and sampled CPU shares do not answer this as additive wall fractions.
This is controlled work removal, not a proving optimization. Its shortened
times MUST NOT count toward the10xgoal, be compared as realprover speed with
M5/Aeglos, or be adopted as universal proof performance.

## Source/history findings

Inspected completed reports matching ablation/bypass/cached query result/
neutralization/counterfactual, including proof-dataflow, whole-query-waves,
coalesce-query, last-finisher, query-rank census, causal-profiler and C7a
architecture review. The matched reports describe other bypass algorithms,
isolated queries, hypothetical subtraction, or the failed Coz qualification.
No completed exact-key/witness result-substitution experiment was found.
Search again before implementation if newer state exists.

Accepted core has two real CPU pipelines:
- Small canonical wires (<65536maxquery) enters solve_canonical_wires.
  Native fused KH dispatches A, fusedKH and exclusiveG2 on the existingpool.
- Larger shapes reach compute_parts then cpu_msms with A,K,H,G2 callbacks,
  folded/public corrections and product finish outside those callbacks.
The canonical fallback and equal-witness branch also exist. Cover the actual
path or decline transparently; preflight must prove requested masks hit.
The small-core-entry hop occurs BEFORE caller TLS would be visible on a
Rayon worker. Capture context once on the caller and explicitly transport
a borrowed request token across that hop and into all callbacks. The prior
coarse-phase-universal176 source is a transport reference, not runtime source
to copy wholesale. Its combined16worker observer gate failed and staysclosed.

## Diagnostic state and binding

Use original176source plus a new private-cfg child and narrowly reversible
core/prover/SDK diagnostic transport. Original field1200library stays exact;
do not link PMNS/fusedscale/signedcarry/quotient experimental libraries.
No field/MSM/solver/keylayout/queue/pool optimization is introduced.

Keep only fixed query results: A G1, fusedKH G1, K G1, H G1 and G2 G2.
Their coordinate payload is576bytes before metadata/mutex/Arc overhead.
No key, scalar, witness, execution-plan or point-array copy. Values never
enter reports/logs/archives. Record only ownership/path/mask/count metadata.
This state belongs to one diagnostic session and cannot be productioncache.

Bind results to an exact diagnostic Key generationID, SHA256 of original
witness bytes, omit_b mode and actual query kind/path. SHA2 already belongs
to core's native dependencies. Do not identify keys solely by reusable raw
addresses; a private-cfg monotone generationID or equivalent typed lifetime
must prevent stale reuse when a Key is moved/dropped/reallocated. Hash and
ownership overhead belong inside diagnostic full-request intervals.

Only CPU rerandomized omit_b=true paths are initial diagnostic targets.
Arbitrary-parts APIs, GPU, unsupported paths, changed key/witness/mode,
incomplete learned state and A0 identity retry with omit_b=false must fall
back to the complete original computation. They must never return stale
points. The source also supports raw APIs, so add explicitNone transport
where no bound request exists. This is not permission to silently omit a
requested measured shape: preflight requires actual callback-hit metadata.

Learn results during the first untimed prime by running ALL original query
work. Capture unblinded query results before corrections/final assembly;
later proofs still solve, construct the quotient, assemble, draw fresh
blinding, serialize and verify. Restore appropriate cached raw results only
at their matching callback boundaries. FusedKH and separateK/H are distinct
slots. Leave public and folded corrections and product finishes in place.

Use masksA=1,G2=2,KH=4, all eight masks0..7. Runtime inactiveOFF and active
mask0 are separate controls. Cache misses execute original callbacks and
are counted. Every observed mask must have the exact requested hit/compute
coverage, no hidden partial miss or skipped unsupported path. Do not zero
scalar arrays to approximate removal: that changes grouping/admission and
does not remove the same callback boundary.

## First implementation and qualification work

Implement bounded owner/session/request/query-slot state, reversible native
privatecfg source integration, Key identity and explicit transport. Before
any clock, qualify cache hit/miss against independent generator points,
changed key and witness, both omit modes, distinct fused/separate query
kinds, unwinding/nested sessions, concurrent distinct owners, shared-owner
rejection/fallback, reset/completion and small-core-entry transport. Use
actual1/4/16/32workers requesting512KiBstacks and nestedjoins.

Run changed complete core/raw/cubic suites unfiltered once and coherent
optimized native/fatLTO/oneCGU SDKlink. Inherit exact unchanged field tests.
Inspect actual linked callbacks/diagnostic branch reachability. Recover all
176original files by reversing declared edits. Generic compile guards are
required; no phone/ARM/runtime/resource claim follows from native tests.

Driver creates one session, learns during prime1, then uses the fixed arm
for subsequent primes/requests. All wrappers and actual request wall timing
must include their hash/lookup overhead. Full SDK verifies fresh randomized
proofs. A valid cached-query proof does not establish fresh-MSM performance.
Exact witness/key binding and sameinput contract must be in the finalreport.

## Prospective capture to freeze AFTER native qualification

Proposed diagnostic contexts: primary1x2 at16/32workers, plus4x2at32 to cover
the ordinary pipeline. Ten arms: acceptedSDK6d83, inactivefreshOFF, active
mask0 and all seven nonzero masks. Retain15rotated/reversedblocks and31timed
requests after3primes, with no exclusions/extensions/reroll/thresholdrescue.
All8+merge54 full-SDK preflights across every arm must establish coverage,
output format and independent Gnark correctness before latency capture.
Bind prior canonical37023ledger so every fresh distinct export is retained
once and previous evidence is not overwritten. Do not rerun old unchanged
verifiers/profiles as bookkeeping; newexports require freshverification.

Declare inactiveOFF/accepted calibration and activeMask0/OFF observer checks
before ANYpreflight, retaining16workers and all prospective failures. Freeze
inputidentities, distinct owner/mask coverage, resources, automated PMU and
fixed-period profiles, independent prospective/results reader, commands and
analysis before ANYrecording. Neutralized-arm speed is descriptive causal
evidence for that changedschedule, without an adoption gain gate. Effects
need not add, and removing work changes helping/locality/overlap. No claim
of an immutable lower bound or exact exclusive phase cost is permitted.

All latency recording must finish before PMU/profiles; allrecording must
finish before offlineanalysis/decoding/Gnark/report/push. DuringANYrecording
includingpreflight ONLYPOLL theconfirmedlivehandle. No otherownedCPUwork.
CommitANDpush the completed diagnostic even if calibration fails; polite
factual fullresults. Its outcome must choose the next real optimization
target, rather than be reported as achieving the prover objective.

Full goal remains verified single-proof CPUAVX>=10x supplied M5CPU across
ALL8with CPU/Metal comparisons. Active0/8, no budget/questions/subagents.
Acceptedlocalprimaryroughly8ms (samebinaryhistorical7.915095/latest8.040649).
No production adoption or reduced goal is authorized by this design.
