# Research and implementation scope

The current accepted primary profile is dominated by G1/G2 streaming and reduction kernels; dictionary hashing alone did not improve primary SDK latency. The proposed bounded packed-X record is derived from the existing G2 forward/reverse implementation, not copied from an external implementation. No paper speedup is transferred to this prover.

Earlier local reports already cover full-radix truncated Montgomery (24 October), PMNS (23 October), RNS/MRS (28 October), indexed G1 trees (22 October) and direct G1 reference packing (28 October). Their failed checks remain unchanged. No repeated timing of those algorithms is selected.

The author-published [PSSM paper](https://www.mdpi.com/2079-9292/15/16/3660) concerns aligning independent single-scalar operation sequences, rather than directly reducing this existing eight-lane MSM kernel. Its SIMD comparison does not establish a gain here. The [RNS vector arithmetic paper](https://eprint.iacr.org/2025/1068.pdf) reports its performance advantage for larger moduli; it supplies no measured BN254 prover result. These were screening references only, not new arithmetic implementations or causal evidence for the temporary-X change.

The new record retains existing packed X values only for the current input batch, so no proving-key expansion or global prepared-key plan is introduced. Extra scratch traffic can erase the saved packing; the complete caller experiment decides the result.
