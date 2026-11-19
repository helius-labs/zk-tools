# Range fills and competing G1 work on the ordinary release

Use the exact ordinary SDK binary on the original 1×2 and 2×2 witnesses, at 32 pinned Ryzen hardware threads. Trace a byte-identical private copy with Linux uprobes on the SDK proof, G2 query, inner range closure, K/H feed and G1 streaming symbols. Record thread context switches on the same monotonic clock. Run four blocks per shape, alternating which process goes first. Each process primes three proofs and times seventeen. Plain controls run from the ordinary executable on its original inode. Verify every exported proof against the pinned key and public input.

Report the spread in start times of the 22 bucket fills, the time from final fill to G2 return, and work by other prover threads while the fills start. Intersect sampled thread scheduling intervals with function intervals before reporting time on CPU. Distinguish repeated within-process proofs from independent processes. Compare traced latency against the paired uninstrumented control to quantify probe effects.

[Earlier experimental flat schedule](../2026-11-19-ready-ranges641/README.md) · [Earlier K/H observer on a different binary](../2026-11-18-sibling-observer270/README.md)
