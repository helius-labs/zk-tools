# Preparation correction before any SDK measurement

The first field suite passed all39 tests. The first core compilation failed
because the copied test-only linear_carry_caller.rs includes a seventh child,
linear_carry_caller_tests.rs, omitted from the initial file list. No core tests
or SDK calls ran. The arithmetic and production parent files are unchanged.

Add that exact previously qualified test source, with its original build-pin
identity checked. The source count is186 (179 accepted +7 children), not185.
Keep original setup/protocol/source-audit/build-plan/build-pins and failed core
receipt immutable. Record source-audit-v2/build-pins-v2/build-plan-v2 and use a
new build-run-v2 folder. Inherit the successful39-test field run by unchanged
field source/configuration identity; do not rerun it. Core must compile and its
unfiltered suite must pass before the coherent libraries/link and any capture.
Runtime protocol, sample counts, comparisons and criteria are unchanged.
