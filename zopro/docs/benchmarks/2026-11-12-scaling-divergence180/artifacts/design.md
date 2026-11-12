# Aeglos stage scaling from retained C7a requests

Question: which reported Aeglos stage supplies the 32-to-96-worker latency
reduction in the November 10 matched campaign? Reuse all 15 blocks for 1x2 and
2x2 at 32/96 workers. No fresh capture, binary rebuild, proof or cloud resource.
This retrospective decomposition is distinct from the completed function/IP and
worker-residence audits. Those audits are not rerun.

Read every SAMPLE from the exact stored Aeglos driver streams. Check round IDs,
expected count (101 primary,17 secondary), finite nonnegative durations, cache hits,
and exact agreement with the already verified campaign's SDK sample arrays and
medians. Pin streams, old analysis/rows and source definitions. Keep all blocks.
Report median process means and median process medians for stages separately.
Use block-paired differences of process means for additive scaling accounting;
compute arithmetic averages of those differences for an exact decomposition.
Do not sum medians and call the result a total. Keep engine residual and outer
request residual explicitly. Stages are the driver's named accumulated timers;
check source nesting before interpreting the residual. Preserve a failure if
stage sums exceed their enclosing measured total beyond rounding tolerance.

Stage duration is elapsed time under this existing observer, not exclusive CPU,
ready work or idle time. Both worker counts already use the same observer; no
new claim of its overhead qualification is made. Source and timings are historical,
not current Zopro phase measurements. Aeglos pinned0290613 Sorted includes its
retained preparation policy. No conclusion that shifted tables, language, or a
particular scheduler caused the gain. No optimization/adoption decision.

The result should narrow the missing current-source observation. Prefer a small
coarse phase boundary diagnostic over another per-task trace (the prior96-worker
trace overhead failed). Before new AWS capture require matched external verification,
source/driver identity, fixed32/96 placement, phase-definition review and a retained
unobserved control. No new framework, universal pool, huge LUT or knob sweep.
Canonical verified-export ledger remains55209; fullall8CPU10x0/8; no push authorized.
