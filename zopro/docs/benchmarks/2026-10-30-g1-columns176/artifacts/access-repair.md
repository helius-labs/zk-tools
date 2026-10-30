# Read-only perf decode repair

All200caller processes completed before decoding began. The first evlist decoder failed because privileged perf created four root-owned0600files. SHA256 values were saved through sudo, ownership changed only on those four generated recordings to ubuntu:ubuntu, and every byte hash was rechecked unchanged.

The first --resume reused all200successful processes, then refused to restart the already failed decoder directory. That failure and stdout/stderr remain in capture-resume.log. Move only that failed read-only decode directory to decode-failed-access, preserving its command/exit/stdout/stderr, and resume again. All200caller processes are reused again; only the16read-only decoder/event/loss/metadata steps execute. The final216-step result is complete. No test, proof, latency process or perf capture repeats; source/protocol/input pins do not change.
