# C7a coarse MSM scaling diagnostic

Question: which MSM phase accounts for Aeglos Sorted's improvement at32->96
physical cores while current Zopro full-request time increases? Reuse the exact
full-request SDKs qualified in msm-full180 and measured in msm-local180. No
optimization, production change or adoption decision. Do not replay local timing.

Primary1x2 only, identical key/request/public bindings. Zopro accepted REF and
same-binary diagnostic OFF/ON. Aeglos retained REF, unmodified native-source FRESH
with matched harness, and diagnostic OFF/ON, separately Sorted and Windowed.
Retain source versions, native preparation limits and all original policies.
Warm-up/setup outside timing remains visible through whole-process peak RSS.
No large lookup tables, artificial small workspace cap or new production pool.

Create three independent byte-identical executable backings for each of the five
distinct binaries. All15 files have distinct device/inode pairs; hash and retain
before/after identities. Pair corresponding controls and worker counts using
backing index block%3. Nine blocks give three blocks per backing. Rotate engine
and arm order, reverse odd blocks, alternate worker counts, exactly as frozen in
the local panel. Never select a fastest copy or repeat to rescue a failed gate.

Local container compatibility smoke: Zopro ON, Aeglos Sorted FRESH and ON, and
Windowed ON,4workers,3primes+1timed each. Four new exports16SDKcalls; no timing
claim. This checks the new container boundary, not a replay of host qualification.
Canonical55440 becomes55444 after independent Gnark verification. Before target
timing, freeze the resulting smoke receipt in the final image; do not rebuild
SDKs or repeat the smoke after successful packaging.

Target:22 functional preflight processes (11arms at32/96,one timed+3prime), then
198timed processes (9blocks*11arms*2counts),51timed+3prime each.10780SDKcalls,
220fresh exports; final expected ledger55664 if no intervening local exports.
The host finalizer must union the disjoint verified delta into the actual current
canonical ledger, not overwrite later history. Only final proofs are exported;
every request includes SDK verification. All exported proofs independently Gnark
verify on target and again after transfer. Preserve failed/partial artifacts;
resume only missing work, never replay successful SDK calls for bookkeeping.

Calibration: Z OFF/REF, A FRESH/REF and OFF/FRESH, all ON/OFF. For each of16
comparisons require the interval sorted ratios[1]..[7] of nine process-median
ratios inside[0.95,1.05]. Nominal96.09375% median coverage under independent
continuous block ratios, no simultaneous-coverage claim. Additionally require
each of three per-backing median ratios inside[0.95,1.05]:48 backing guards.
Report all64 results, not only aggregate pass. A trace is representative only
within its passed controls/worker/mode and environmental checks. Failed gates
do not imply an implementation bug or license a tuned rerun.

Require96 online logical CPUs with96 distinct physical cores. Record model,
topology, flags, container limits, CPU pressure, faults/switches and RSS. Target
container uses96CPU quota,30GiB memory ceiling and no swap, following retained
campaign policy; this is an isolation guard, not a workspace recommendation or
minimum-memory test. Require no CPU throttling or cgroup memory high/max/OOM/swap
events during capture. Missing environmental data rejects interpretation. Aeglos
major faults are unreported by its retained harness; don't infer zero. Zopro
worker affinity remains fixed; Aeglos retains native policy within the same mask.

Validate every emitted phase including primes: expected count, ordered bounded
intervals, completion flags, worker count, per-job item conservation. Exclude
primes from phase means. Preserve full-request residuals and all merge rounds.
Zopro A/KH/G2 callbacks overlap and include helping/waiting; don't add them as
exclusive CPU or wall time. Scheduled items are not equal-cost work or observed
queue readiness. Capture is sequential with no owned analysis/build/hash overlap.
Only poll the confirmed capture handle until authoritative terminal completion.

Coarse phase records are the profiling instrument here; no concurrent PMU capture.
Report paired phase changes across all nine blocks, not only fastest cases. No
causal optimization claim from phase clocks or scheduler symbol shares alone.
Use the result to choose an intervention that changes the measured limiting work
or dependency while preserving bounded workspace and universal circuit support.

Scope excludes other shapes, mobile/ARM/TEE, huge circuits, swap/pressure behavior
and parallel-proof throughput. Full CPU10x goal and CPU/Metal comparisons for all
eight shapes remain unchanged and unmet0/8. No GitHub push authorized. AWS SSO
currently expired; prepare locally, no launch until authentication and owned
lifecycle prerequisites pass. One c7a.24xlarge maximum,90min terminateTTL, cleanup
instance/volume/temporary registry tag/transfer bucket after capture and retrieval.
