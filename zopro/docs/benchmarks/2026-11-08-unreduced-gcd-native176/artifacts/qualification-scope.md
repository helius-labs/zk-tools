# Standalone qualification scope correction

The first compile fails on `b.len()` instead of the `b.len` field; the exact initial source and receipt are retained. The sole source fix precedes the second compile. That compile reaches LTO and fails because one reused analysis/support dependency lacks embedded LLVM bitcode. No arithmetic check has run in either failed workflow.

Continue with unchanged corrected arithmetic source and existing dependencies using O3/native/CGU1, overflow checks enabled, LTO off. This is a correctness-only standalone artifact. It does not satisfy the original design's fat-LTO production-consumer qualification or provide a timing control. A separate coherent fat-LTO consumer/field integration and actual code inspection remain required before any claimed production qualification or clocks. No successful native build, mathematical model, old suite or decoder is replayed.
