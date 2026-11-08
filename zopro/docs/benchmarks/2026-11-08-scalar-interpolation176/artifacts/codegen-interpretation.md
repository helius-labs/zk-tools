# Static count clarification, after capture

The pinned manual route audit correctly identifies the actual candidate calls,
native guards and9point iteration. Its statement that1400G1IFMA sites comprise
600new plus800fallback is an inference from subtracting whole-function counts,
not a separately decoded control-flow partition or dynamic per-path count. It
must not be used as such. The defensible observation is1400sites in the ON
function containing both paths, versus800in OFF. Compiler sharing/inlining and
the additional fallback code prevent an executed-work claim from that difference.
No decoder, benchmark, analyzer, threshold or result is rerun/changed. This
clarification does not affect any timing, resource or profile-quality gate.

The dynamic PMU observations are the actual complete-caller evidence: measured
instructions increase about61-63%forG1 and65-67%forG2 versusOFF in the selected
diagnostic contexts. These panels do not independently establish an execution-
port, register-pressure or memory-capacity cause.
