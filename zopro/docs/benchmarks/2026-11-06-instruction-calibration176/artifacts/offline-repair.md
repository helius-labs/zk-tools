# Offline analyzer repair

All 240 direct-PMU commands completed successfully once (1,680 measured repetitions). The final analyzer failed at an exact floating-point equality after JSON parsing: reconstructed mean 2.0008929116385326 versus parsed serialized mean 2.000892911638533. The raw integer counters, affinity, running-time and retirement gates did not fail at that point.

Preserve the original source, binary, plan, pins, all capture receipts and failed analyze directory unchanged. A separate offline-analysis.rs copies the original analyzer and changes only that exact floating equality to a relative tolerance of 1e-12 for serialization roundoff. It still reconstructs every mean from raw integer counts and enforces the original hardware gates. This is not a changed performance threshold or new measurement. Execute only its analyze entry point; do not resume the original workflow or rerun any successful native command.

Before preparation, the LLVM-based analyzer also received a source-read correction: its object-dump notice is written to stdout, not stderr. That repair occurred before its first assembly workflow. The two LLVM capability failures and all successful assembly/decode/check receipts remain preserved. An initial offline patch failed to match multiline formatting before any compile or analyzer execution; the corrected patch changes the intended equality only.
