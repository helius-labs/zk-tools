# Analysis repairs

The native build, focused tests, decoder tests, SDK captures and Gnark exports
pass first execution. The original analysis attempts are retained privately.

- Preflight PMU reader: a comparison alias `pmu.rs` was missing at the private
  root. Restoring it byte-for-byte from the already pinned runtime source
  fixes the reader. No SDK process repeats.
- Original main profile reader: it iterated the already analyzed smoke jobs
  and rejected their existing scoped output. Version2 requires exact byte
  equality for existing smoke output and continues to reject existing main
  output. No scope, data, arithmetic or quality predicate changes.
- Version2 reaches the declared quality test and fails three profiles with
  slightly more than5% unknown scoped symbols. This failure is preserved in
  `profile-result.json`. Matching libc debug symbols resolve the original IPs.
- The first offline symbolization workflow refuses the altered reader source
  against its original capture pin before any step executes. The original
  source is restored; repaired readers have distinct versioned paths. A fresh
  invocation then rejects the existing incomplete output root, and a resume
  attempt rejects its absent source-pin checkpoint. That incomplete root and
  all error logs are preserved. The six actual offline decode commands then
  execute once in a new output root and all pass.
- Version3 reads the new symbolized streams and writes separate outputs. The
  frozen quality predicates remain exact. An independent reader confirms
  sample identity and reconstructs the clock bounds and G2 interval scope.

There is no scientific capture replay, outcome selection, performance-gate
change or adoption claim. These are local reader and offline workflow repairs.
