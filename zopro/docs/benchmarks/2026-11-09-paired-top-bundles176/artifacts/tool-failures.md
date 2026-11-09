# Preparation failures before native execution

These are transcribed tool outputs, not xtask child receipts.

1. Initial prepare helper compiled, then returned101 at prepare.rs:62:71: `InvalidData: stream did not contain valid UTF-8`. The176-file manifest includes a binary fixture. All staged source edits had already been written; no native command had run. `prepare-initial.rs` preserves that helper. The corrected helper validates existing staged bytes, recovers text only for the four edited parents, and compares every other original as bytes. It completes the missing audit/plan without replacing already prepared source or repeating a scientific test.

2. Initial xtask invocation returned1 before any child: `unknown field guard_only, expected one of name, argv, environment, required_tests, unfiltered, timeout_seconds at line137 column4`. `build-plan-rejected.json` preserves the rejected plan. Removing only those unsupported metadata fields produces `build-plan.json`; source, pins and all actual child commands remain unchanged. The subsequent four workflow steps complete on their first execution.

After completion, an inspection initially used a nonexistent numeric step-directory prefix. The corrected read uses the actual named directory. No test/build/capture was repeated.

The first offline reviewer expected build-only CARGO_MANIFEST_DIR/CARGO_PKG_NAME/CARGO_PKG_VERSION in command.json. The runner deliberately records only config::managed variables there. The corrected reader checks that exact documented projection and separately binds the complete environment through protocol.json. review-initial.rs preserves the failed reader. No scientific source, command, result or pin is changed or replayed.
