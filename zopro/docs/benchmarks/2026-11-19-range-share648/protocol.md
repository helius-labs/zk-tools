# Give the earlier G2 partition a larger share of points

Partition one supplied the final range return in 87 of 102 ordinary 1×2 proofs across three different inputs. Keep both range jobs, all K/H windows, the signed windows and the original nested scheduler. Change only the first contiguous range length. Compare ceil(7n/16), ceil(9n/16) and ceil(10n/16) against the equal split in the same binary and the ordinary release. The shorter first range is a directional control. Each point belongs to exactly one of the two slices. No per-key table or extra bucket storage is allocated.

Use ten balanced process blocks and one separate CPU profile per arm. Measure the original 1×2 at 16 and 32 hardware threads, two additional 1×2 inputs at 32, and original 2×2 at 32. Trace the actual split and check independent proofs. Run the 157-test native suite and the generator oracle on ragged inputs with all three unequal boundaries. Record control-relative latency and peak RSS by witness. Qualify each share against both the equal split and release.

[Window and partition identities](../2026-11-19-fill-identity647/README.md) · [Candidate source](artifacts/source.patch)
