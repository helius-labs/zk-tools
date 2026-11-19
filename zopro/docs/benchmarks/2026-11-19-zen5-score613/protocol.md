# Release, prepared Aeglos and two Rust CPU targets on Ryzen

The Ryzen 9 9950X implements Zen 5. The reference experimental executable was built for Zen 4, the architecture also used on C7a. Build the identical prover source for znver4 and znver5 with the same pinned Rust toolchain, features and release flags. Native core tests and independent full-proof verification qualify both targets. The comparison measures this Ryzen alone.

Five fresh randomized blocks compare the accepted release, the frozen candidate executable, a fresh Zen 4 build, the Zen 5 build and prepared Aeglos. All five use identical main CPU affinity per shape and block. Small reference shapes receive 35 timed requests and the rest eleven, after three warmups. Measure end-to-end verified proof latency, highest process RSS, both supplied M5 ratios, and paired wins over the fresh Zen 4 control. Include all eight M5 shapes and merge-54×1. The only experimental per-shape configuration change from the previous nine-shape trial is the sampled centered H profile on 2×2, verified across independent requests. The three experimental executables use that same updated profile. Larger transfers retain prior group and row profiles. Each Aeglos timed call must report a prepared key hit.

[Schedule](artifacts/schedule.json)


The first Zen 4 core test run used Rust's parallel test harness. Four tests failed while sharing process-wide allocator guard counts or FFT settings. The identical test executable passed all 159 tests with RUST_TEST_THREADS=1, matching the original repository benchmark build procedure. Both target builds use serialized core tests for the comparison. These tests precede timed proofs.
