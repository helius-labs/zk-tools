# Pre-capture helper compile failure

Both missing consumer links, the capture/analyzer and raw reader compile
successfully. The fifth build step fails because build-review.rs has one extra
closing delimiter. No preflight, timing or profile has started.

The repair removes that delimiter and teaches build review to validate the
original failed workflow plus the repair workflow. Capture preparation changes
only its build-result/pin-file metadata paths to the repair workflow. Its
statistics, gates, scheduling, recording commands and analysis remain exact.
The original capture helper source/binary, failed reader source, original plan,
pins and failure stdout/stderr/exit are retained. The two successful consumer
links and raw reader are not rebuilt. The two changed helpers compile once.

Seven build attempts therefore comprise six successes and this one failure.
All 378 recording/offline-capture commands and both post-analysis commands
then succeed on their first executions. Total387commands/386success/1failure.
No successful native suite, field build, model, proof or capture is replayed.
