# Small-factor recoding work screen

Frozen before model execution. No native SDK modification or latency capture.

Review Oleksandr Hrabar's Waterloo thesis, chapter 4, especially section 4.1. Implement an independent BN254 scalar screen for original canonical scalars, signed centering, divisible-by-two via the subgroup order, priority 3/2, and priority 5/3/2. The latter two require separate subgroup MSMs and explicit output corrections. Do not count them as one unchanged bucket table.

Use the accepted native overlapping Booth function verbatim and independently reconstruct its signed integer digits. Check canonical field identities against Arkworks and independent G1 MSM outputs, including zero, one, modulus-minus-one, powers of two and full-width random inputs. Pin the accepted 172 production source files and SDK. Tests are exact correctness checks; the screen is not a timing benchmark.

Fixed workloads: 32768 uniform rejection-sampled canonical scalars; all integers 0..16384. Fixed deterministic SplitMix64 generator and seed. For every width 4..15 and each strategy, count nonzero signed digit terms, negative terms after the recoding sign, occupied absolute-digit destinations, and ideal insertion additions (terms minus first assignments) separately for each factor group. Record actual maximum bucket index per group/window and dense bucket capacities. These counts omit exceptional point arithmetic, batching, grouping, scheduler, transform, allocation, negation and correction costs. They are not CPU cycles or a full-proof speed estimate. No representative actual-witness claim.

All strategies have exact modular identity gates. A work-count improvement preserves a hypothesis for a later real-input/native test, with no arbitrary minimum gain. Lack of a complete-window reduction rules out a whole-window-removal explanation, not every possible speedup. A full-width improvement coupled to small-scalar inflation requires a guarded policy and new qualification; do not adopt universally. No new point lookup tables or key plans.

No benchmark timing, proof verification, or cloud resource is needed for this screen. Existing accepted performance and proof ledger remain unchanged. Commit and push the completed study, including negatives and limitations.
