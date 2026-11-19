# Separate trace overhead from executable backing

Studies 643, 644, 646 and 647 protected the ordinary run from probe registration by using a separate byte-identical inode for tracing. Their plain-to-traced difference can include an executable backing effect. Earlier Ryzen experiments found a material difference between byte-identical files. Reuse the exact retained inodes.

For each case compare three modes in all six process orders. The first two run plain from the source executable and the retained probe copy. The third traces that same retained copy. Use the original 1×2 witness for each of 643, 644, 647 and the 12-job candidate in 646. Also check the original 2×2 in 644. Use the original probes, masks, key and request. Each process primes three proofs and times seventeen. Check the trace counts, independent proof and public-input digest. Report copy-versus-source and trace-versus-copy paired differences separately.

[Earlier same-byte observation](../2026-11-11-ibs-backing177/README.md)
