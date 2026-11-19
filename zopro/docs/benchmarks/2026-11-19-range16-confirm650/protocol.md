# Recheck the 9/16 first G2 range on sixteen physical cores

The original 1×2 on CPUs 0–15 improved in eight of ten pairs against the equal split in study 648. On 32 hardware threads the same comparison split five and five. Repeat the 16-core measurement with the same release and candidate binaries, using four arms: ordinary release, equal, 7/16 and 9/16. The shorter first range tests the direction of the effect.

Use twelve balanced process blocks on the original input and eight each on witnesses 29 and 32. Each timing process primes three proofs and measures 21. Run one separate CPU profile per arm and witness, then verify every export against the pinned key and public inputs. Evaluate replication on the original and direction on the two additional inputs against both ordinary controls. This is a local physical-core configuration.

[First experiment](../2026-11-19-range-share648/protocol.md)
