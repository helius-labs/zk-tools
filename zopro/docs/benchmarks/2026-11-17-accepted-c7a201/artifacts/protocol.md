# Exact adopted SDK194: full-shape C7a measurement

Question: what complete fresh-proof latency does the exact locally adopted SDK194
achieve on C7a across all eight supplied shapes and separate merge54? No prover
code or compiler change; reuse SHA bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1.
The current all179-source manifest and58033-proof inventory remain authoritative.
Measure adopted selectors CERTIFIED_OUTPUT=1/FUSED_PREPARATION=1/INDEXED_TILE_BUNDLES=1.
For primary only, add same-SDK CERTIFIED_OUTPUT=0 control and exact retained Aeglos
Sorted eab9fbfd4b94aeef9ced3ce3e3ede21a37ab70ad38b217720f1afd06fceba99b
(reference SHA must be checked against actual retained binary; do not accept a
mistyped hash). No new field tests or claimed ARM/phone/huge-circuit performance.

One c7a.24xlarge,96 physical cores/noSMT;32/96 workers, fixed process masks and
existing Zopro individual placement. Native host execution for all arms inside
one30GiB/no-swap service. No Docker CPU quota. Identical allocator policy, fresh
process per job, same request/image/public bindings as qualification196. Prepare
three byte-identical independent executable backings for each engine, rotate
backings byblock%3. Key load/preparation outside latency but inside lifetimeRSS.
Time solving, quotient, fresh blinding, proof construction, serialization and
self-verification. Aeglos uses verified_ms. Every process primes three proofs,
then times17 fresh proofs (one timed in preflight), exporting its last only.

Four local functional smokes: primaryON/OFF/Aeglos and merge54ON at4workers.
Independent Gnark verification and freshness checks before packaging; merge their
exports into58033 seed before remote runs, retaining old branch. Remote22
preflights (all9 shapes*2workers ON, primaryOFF*2, Aeglos*2) are independently
verified before main. Nine rotated/reversed blocks each contain22 jobs:18 ON
contexts plus primaryOFF/Aeglos32/96. Thus198main+22preflight=220exports/4048SDKcalls,
plus4localexports/16SDKcalls. No successful process replay, extension, favorable
subset or threshold change. All main captures precede late Gnark verification.
Failure stops and preserves partial work; resume only demonstrably missing steps.

Report per-context median process mean,9 paired blocks for primaryON/OFF/Aeglos,
sorted ranks1..7 nominal96.09375% marginal median interval under independent
continuous block assumptions (no familywise guarantee), every backing, CPU/RSS,
and96/32 scaling. Compare supplied M5CPU and Metal on all8 ON shapes, without
inventing an M5 reference for54. Count a measured shape as meeting CPU10x only if
the upper rank7 process-mean bound is at/below its CPUref/10 at a declared worker
count; retain both counts and avoid calling this universally attainable hardware
performance. This is a measurement/control campaign, not a new candidate adoption.
PrimaryON/OFF gain interval upper<1 is evidence of target gain; otherwise report
inconclusive or slower. Aeglos comparisons descriptive. Local194adoption stands
unless correctness/resource evidence contradicts it. No retroactive new veto.

Pin exact consumers/tools/config; input files hashed before and after capture,
individual image/public/proof hashes checked by independent verifier per export.
xtask per-step pins cover binaries/config/runner, not repeatedly all2GiB inputs.
Verify worker counts/placement, freshproof uniqueness versus canonical seed and
acrossall224newexports; retain actual cgroup enforcement/events and resource scope.
Retrieve and independently verify remote exports on host before canonical union.
One owned VM only, existing restricted network/SSM role,90min terminateTTL,
credentials lifetime validated. Delete owned VM/volume/private transferbucket;
no ECR needed. No public push without new user authorization. Initial preparation
and first result allowance30minutes; if exceeded record cause before continuing.
