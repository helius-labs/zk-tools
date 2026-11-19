# Confirm the Euler half-leaf on four 24×2 requests

In study 651 the half-leaf mode took 56.148 ms and peaked at 908.5 MiB on the 24×2 reference. The frozen indexed, tiled, pooled profile took 56.764 ms at 941.9 MiB. Repeat that exact comparison against the frozen binary and the ordinary leaf in the new binary. Use twelve balanced blocks on the reference and six on each independent witness 97, 99 and 101. Each process primes three calls and times fifteen. Collect a separate CPU profile per arm and input. Check that sampled leaf arithmetic is present in the new mode and verify all exported proofs. Report latency, whole-process RSS and the reference 52.8 ms threshold. Compare both controls before selecting a profile.

[Initial experiment](../2026-11-19-leaf-boundary651/README.md) · [Previous 24×2 configuration](../2026-11-19-indexed-output628/README.md)
