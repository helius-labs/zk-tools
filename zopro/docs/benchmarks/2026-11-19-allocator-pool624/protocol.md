# Does the smaller FFT pool save memory with stock glibc?

The ordinary SDK showed a repeatable reduction in peak RSS on four 24×2 requests under the benchmark’s tuned glibc settings. Use the exact adopted SDK executable and original request on the same Ryzen host. Cross the two placements with either stock glibc or the three previously measured allocator overrides. The production-relevant contrasts are pool versus main within each allocator. Both pool variants set the same fixed 30-worker mask. All arms receive 32 pinned main workers, three warmups and 17 timed proofs per process across seven rotated blocks.

Run the existing Rust xtask perf workflow. Separately profile 21 warm requests in two processes per variant with the SDK-controlled CPU-cycle profiler. Report lifetime peak RSS and paired wall-time changes, and keep profile samples separate from the latency comparison. Each process exports one proof for independent verification with the original key and established public input. These are single-proof measurements on one request.

[Exact xtask configuration](artifacts/config.json) · [Release-source comparison](../2026-11-19-release-pool621/README.md)
