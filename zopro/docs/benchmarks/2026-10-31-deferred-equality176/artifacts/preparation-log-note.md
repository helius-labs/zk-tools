# Pre-capture log collision

The manual `campaign prepare` invocation redirected stdout to prepare.log,
which had previously held the short native-plan preparer's success message.
The shell truncated that file before preparation. The capture pin collector
then hashed it while it was empty, before the campaign wrote its success line.
No capture command or measurement had started.

The old pin map is retained as pins-before-log-repair.json. A separate helper
requires this to be the ONLY mismatched pin and its old hash to be SHA256 of
empty bytes, then pins the completed, static log and this repair's provenance.
All other pins and the complete command plan must remain exact. No gates,
runtime source, tests, inputs, executables or capture steps change or rerun.
The earlier native-plan preparer's stdout line was overwritten and is not
claimed preserved. Its generated plan and all actual qualification command
receipts remain unchanged. The failed native parse and corrected qualification
logs are preserved. This correction occurs before the first capture.
