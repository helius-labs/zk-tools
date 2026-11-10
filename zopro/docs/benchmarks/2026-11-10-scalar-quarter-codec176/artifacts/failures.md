# Preserved setup failure and negative result

The initial prepare-perf binary returns101 before writing build-plan.json:
missing18preflight270latency4PMU4profiles20offline. An earlier replacement of
270 with360 had already changed this progress-message text. The repair expects
18preflight360latency4PMU4profiles20offline and emits the intended24/360/6/6/30
message. This changes generation bookkeeping only; protocol had already fixed
fourarms/48checks/426commands. No numerical formula, threshold or population
changes, no clock yet. Original prepare-perf-before-freeze-text-repair.rs and
prepare-perf binary retained; corrected helper is prepare-perf-repaired.
The failure has transcript evidence, not an xtask command receipt.

All446workflowcommands pass:13native,5consumer/structural/tool,426capture,
2postanalysis. Bothfrozenanalysis and separate rawreader pass first execution.
No successful native/probe/capture/oracle run is replayed for bookkeeping.

The complete candidate FAIL8/48 is retained, including one G132calibration.
No SDK/adoption or favorable worker/group subset rescue. Positive G216 and
G232 observations and branch-miss reductions remain reported in full.
