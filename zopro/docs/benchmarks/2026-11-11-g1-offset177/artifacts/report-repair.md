The final report's resource projection was shortened by removing repeated
affinity rows already retained in the raw archive. The editing helper initially
put a Rust block directly inside json! object syntax in finalize.rs. A compile
check caught the macro parse error before that changed generator was executed.
The projection now computes the value outside json!, and the corrected helper
compiles. The initial source remains at finalize-format-error.rs. No benchmark,
proof, verifier, test suite, archive creation or analysis was repeated. Existing
resource output was produced correctly by the separate editing helper.
