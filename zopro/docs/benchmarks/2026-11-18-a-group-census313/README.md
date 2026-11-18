# Scalar grouping on the 1×2 A query

The integrated prover passed 21,006 scalars to the canonical G1 grouping routine on the original 1×2 fixture. The exact distinct nonzero count was 17,454. A 32 KiB bitmap established a lower bound of 16,837, above the existing admission cutoff of 16,804. This proves that the full grouping map would be discarded for that input.

Across eight additional 1×2 requests with varied output amounts and blindings, the bitmap established rejection on 8 of eight. Its observed bound across all nine inputs ranged from 16,837 to 16,894. Each 1×2 request used the same circuit, input notes and tree. The 2×2 fixture exercised three canonical G1 input sizes. Results and public-input bindings are in the observations file.

GDB attached to the retained integrated SDK at the general grouping function entry. It counted four calls per recorded size and caller, inspecting the first scalar array for each. The independent calculation used the same low-limb mixing, bitmap width and zero rule as the Rust implementation. Debugger execution times are excluded. Every process exported a proof that passed the independent verifier. Experiment 314 tests the full-proof latency consequence separately.
