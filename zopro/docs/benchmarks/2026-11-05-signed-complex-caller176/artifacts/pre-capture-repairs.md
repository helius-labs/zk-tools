# Qualification preparation repairs

Initial native core-OFF suite:127 pass, two caller adapter tests fail because the setup omitted explicit DIRECT/CANONICAL_BATCH/VECTOR_INVERSE/G2_PREFIX_RECORDS runtime options. The exact original tests assert that configuration. No caller source or flags changed. Preserve original native directory, plan/pins and all127 successful tests.

`build-fixed-plan.json` reruns only the two failed exact tests using the existing OFF executable, then continues libraries/freezes and the not-yet-executed ON full core suite. Runtime options are explicit. It does not repeat the127 OFF successes. The freezer helper changes only its workflow log directory from native to native-fixed, with original native.rs preserved. Successful qualified field tests/model are not rerun.

Before the repaired workflow started, compilation of `pin-fixed-helper.rs` failed with E0382: `path` was moved into fs::read(path) before use in the JSON index. Original source is preserved as `pin-fixed-helper-initial.rs`; the repair passes `&path` to fs::read. The corrected helper alone is compiled/executed; native-fixed-helper's successful compilation is reused. This helper failure occurred before any repaired test/build workflow execution and changes no runtime source, flags or test gate.
