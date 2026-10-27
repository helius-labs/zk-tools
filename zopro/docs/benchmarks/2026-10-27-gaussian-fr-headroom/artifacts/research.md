# Scope and prior evidence

The R128 Gaussian implementation in ed00e930a failed: radix-four butterflies were2.247x ordinary radix-two, despite a10.02% internal gain over Gaussian radix-two. Its tests/results/decodedprofiles remain inGit; local-only /tmpbinaries/rawcaptures are now absent. No completed capture is repeated to restore them.

This follows the same norm identity and Gaussian-residue interpretation, changing the radix and proving new coefficient bounds. R130 is fixed because its square matches the existing FFT R260; it is not selected by timing a range of radices. The exact K0 bound permits correction-free normal products, and a power-of-two canonical threshold admits simpler native correction masks. Deferred linear combinations require explicit4H/5H contracts, proved and independently tested here. These are our local algebraic deductions, not performance claims from a paper.

Background: [Gaussian Montgomery reduction](https://www.mdpi.com/2410-387X/5/1/6), the [previous Gaussian component](../2026-10-27-gaussian-fr/README.md), and existing rejected deferredDIF32/lazy-linear/parallel-carry experiments. Those operate on different field invariants and remain negative. No reported hardware speedup is transferred to this CPU. A field/butterfly gain alone cannot qualify conversion, a complete FFT, full proofs, mobile or pressure behavior.
