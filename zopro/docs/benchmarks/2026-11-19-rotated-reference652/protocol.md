# Remeasure the 12×2 reference across Ryzen CPU placements

The mapped-representative executable measured 28.375 ms on this reference in study 605, just below the 28.4 ms target. Its experiment used a different 30-thread mask per block. Study 634 measured 28.628 ms on the indexed main-pool executable using one fixed mask. Compare both binaries at the same placements with the same image, public inputs, witness and number of rounds.

Run the dense and mapped choices of the retained 605 binary, the retained 628 indexed binary, and the faster of study 651's two new leaf implementations on its original 12×2 reference. The latter is selected before this independent comparison, with fused winning a tie. Use all eight placements in the earlier rotation, including the eighth mask outside the original seven-block run. Each mask covers all 16 physical cores and omits one SMT sibling in each CCD. The four arms at each placement run in four balanced process blocks and one separate profile per arm, with three warmups and fifteen timed proofs each. Verify every exported proof. Report pooled medians, per-mask medians and threshold counts for all arms. Differences between mask medians may include run-to-run variation.

[First mapped-representative measurement](../2026-11-19-mapped-representatives605/README.md) · [Fixed-mask indexed comparison](../2026-11-19-indexed-pool634/README.md)
