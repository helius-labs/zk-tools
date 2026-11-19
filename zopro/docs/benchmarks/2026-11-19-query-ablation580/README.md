# Which curve queries set the 1×2 latency floor?

Each diagnostic process handles repeated proofs of one fixed, generated transaction. It evaluates the three deterministic curve sums normally on its first proof and can substitute selected results on later proofs. The eight combinations cover the A commitment, combined K and H commitment (including H scalar conversion), and B2 commitment. All combinations still construct the witness, run the solver and FFT, assemble a fresh randomized proof, and run SDK verification. The probe binds the entire input assignment to a single loaded key. A negative test presents two distinct requests to a single process and confirms the second is rejected before any saved sum is used.

Five shuffled blocks on 32 Ryzen workers test the original input and two other public fixtures. Each arm primes three requests and times 21. The prior executable and a same-binary ordinary control measure the effect of the observer. Bench-only substitution is an intervention on repeated identical input. Its timing estimates the effect of removing selected work under this schedule. Latency on unseen transactions must be established separately by a prover that computes every required sum.

| Request | Substituted work | Prior / ordinary control / diagnostic ms | Paired ms saved vs control (wins) | Control overhead vs prior |
|---|---|---:|---:|---:|
| Original | None | 7.030 / 7.771 / 7.771 | +0.000 (0/5) | +10.67% |
| Original | A | 7.030 / 7.771 / 7.245 | +0.502 (5/5) | +10.67% |
| Original | K+H | 7.030 / 7.771 / 5.519 | +2.261 (5/5) | +10.67% |
| Original | A + K+H | 7.030 / 7.771 / 4.988 | +2.761 (5/5) | +10.67% |
| Original | B2 | 7.030 / 7.771 / 4.889 | +2.905 (5/5) | +10.67% |
| Original | A + B2 | 7.030 / 7.771 / 4.397 | +3.358 (5/5) | +10.67% |
| Original | K+H + B2 | 7.030 / 7.771 / 3.106 | +4.656 (5/5) | +10.67% |
| Original | A + K+H + B2 | 7.030 / 7.771 / 1.727 | +6.035 (5/5) | +10.67% |
| 29 | None | 6.981 / 7.666 / 7.666 | +0.000 (0/5) | +9.70% |
| 29 | A | 6.981 / 7.666 / 7.124 | +0.511 (5/5) | +9.70% |
| 29 | K+H | 6.981 / 7.666 / 5.384 | +2.300 (5/5) | +9.70% |
| 29 | A + K+H | 6.981 / 7.666 / 4.954 | +2.704 (5/5) | +9.70% |
| 29 | B2 | 6.981 / 7.666 / 4.787 | +2.833 (5/5) | +9.70% |
| 29 | A + B2 | 6.981 / 7.666 / 4.278 | +3.361 (5/5) | +9.70% |
| 29 | K+H + B2 | 6.981 / 7.666 / 3.009 | +4.624 (5/5) | +9.70% |
| 29 | A + K+H + B2 | 6.981 / 7.666 / 1.758 | +5.908 (5/5) | +9.70% |
| 32 | None | 7.027 / 7.710 / 7.710 | +0.000 (0/5) | +9.12% |
| 32 | A | 7.027 / 7.710 / 7.130 | +0.582 (5/5) | +9.12% |
| 32 | K+H | 7.027 / 7.710 / 5.393 | +2.266 (5/5) | +9.12% |
| 32 | A + K+H | 7.027 / 7.710 / 4.977 | +2.721 (5/5) | +9.12% |
| 32 | B2 | 7.027 / 7.710 / 4.744 | +2.946 (5/5) | +9.12% |
| 32 | A + B2 | 7.027 / 7.710 / 4.306 | +3.415 (5/5) | +9.12% |
| 32 | K+H + B2 | 7.027 / 7.710 / 3.012 | +4.685 (5/5) | +9.12% |
| 32 | A + K+H + B2 | 7.027 / 7.710 / 1.732 | +5.978 (5/5) | +9.12% |

The 1×2 target is 3.7 ms for fresh 1×2 proofs. The diagnostic rows do not count toward that target. All 136 exported proofs verified against the original key. [Samples](artifacts/samples.json.gz) · [Branch counters and witness guard](artifacts/ablation.rs) · [Integration change](artifacts/integration.patch) · [Verification](artifacts/verification.json)

## Which reductions survive overlap?

The first table gives median complete diagnostic proof latency. Substituting a previously computed point preserves the other concurrent branches and can change the number of workers available to them. Only the original input has a supplied M5 reference, with a 3.7 ms fresh-proof target.

| Request | Ordinary / only A substituted / only K+H / only B2 / all three, ms |
|---|---:|
| Original | 7.771 / 7.245 / 5.519 / 4.889 / 1.727 |
| 29 | 7.666 / 7.124 / 5.384 / 4.787 / 1.758 |
| 32 | 7.710 / 7.130 / 5.393 / 4.744 / 1.732 |

For each randomized block, the next table computes the reduction when substituting a branch first, when substituting it after the other two, and its average marginal reduction across all six orders (Shapley allocation). The three allocations sum to the full intervention's time saving within each block. Table entries are the medians across blocks and do not have to sum exactly. These measured conditional effects include schedule changes; they do not predict the exact benefit of faster arithmetic on new requests.

| Request | Query | First / last / average-order ms saved |
|---|---|---:|
| Original | A | +0.502 / +1.383 / +0.789 |
| Original | K+H | +2.261 / +2.671 / +2.306 |
| Original | B2 | +2.905 / +3.246 / +2.917 |
| 29 | A | +0.511 / +1.242 / +0.748 |
| 29 | K+H | +2.300 / +2.512 / +2.274 |
| 29 | B2 | +2.833 / +3.207 / +2.952 |
| 32 | A | +0.582 / +1.284 / +0.765 |
| 32 | K+H | +2.266 / +2.563 / +2.275 |
| 32 | B2 | +2.946 / +3.244 / +2.933 |

[All paired intervention times and allocations](artifacts/conditional-effects.json)
