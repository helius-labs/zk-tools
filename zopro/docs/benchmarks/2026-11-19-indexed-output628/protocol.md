# Indexed grouping on the fastest measured 12×2 and 24×2 output paths

Apply only the clean ordinary-SDK index dictionary patch to the frozen experimental SDK that produced the direct-output and auxiliary-pool comparison. Cross the original and indexed executables with two FFT configurations on each shape. On 12×2, compare ordinary versus tiled output with the same 30 main workers. On 24×2, compare ordinary main versus tiled output on the smaller 30-worker pool with caller-allocated storage, keeping 32 main workers. Both enhanced arms use the same mask for a given shape.

Use the reference and three previously measured transfers per shape, including the scattered 12×2 input. Run eight counterbalanced latency blocks on each reference and four on each other input. Collect a separate warm CPU profile per arm and input. Each process warms three requests and times eleven. Check generated image and sidecar identities, full SDK route markers, original-key verification and proof freshness. Compare the joint path to the frozen enhanced path, the new dictionary alone, and the frozen ordinary path. The M5 CPU cutoffs are 28.4 and 52.8 ms respectively.

[Indexed source](../2026-11-19-release-index625/artifacts/source.patch) · [24×2 combined placement](../2026-11-19-combined-output623/README.md) · [Indexed grouping confirmation](../2026-11-19-indexed-transfer622/README.md)
