# Limit simultaneous K/H windows on the shared Rayon pool

On the ordinary 1×2 reference, while the 22 G2 fills started, the balanced K/H feed occupied an average of 15.52 scheduled threads. This experiment gives the other queries more opportunity to run by grouping some of the twenty K/H windows in pairs. Each job runs one or two complete windows in sequence. It writes the original result positions, keeps the same buckets, and leaves the final group arithmetic in the original order.

Compare release, the new binary with ordinary scheduling, and 20, 16 or 12 jobs. Twenty jobs tests the cost of the new iterator and result allocation with one window per job. Sixteen jobs have four pairs, and twelve have eight. Collect ten counterbalanced process blocks plus one separate CPU profile per arm on the original 1×2 at 16 and 32 threads, two further witnesses at 32 and the original 2×2 at 32 as a path control. Run fifteen warm timed proofs per process. Check dispatch separately and verify exports with the pinned key. Run the native core suite and the K/H generator oracle in every mode. Qualify any gain against both the 20-job and ordinary paths.

[Competing phases](../2026-11-19-competing-feed644/protocol.md) · [Candidate source](artifacts/source.patch)
