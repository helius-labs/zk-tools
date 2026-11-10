# SIMD admission of eight ordinary bucket updates

Goal remains verified CPU single-proof >=10x supplied M5CPU ALL8, with CPU and
Metal comparisons. Accepted primary32 is7.783593581ms, all8targets unmet.

Hypothesis: reject a packet collectively on repeated bucket indices, pending
destinations, zero first X limbs or equal first X limbs, then append all eight
ordinary borrowed updates without eight scalar dispatch sequences. Existing
arithmetic batches, flush boundaries, signed operands, input order, capacities,
queues, field library, window geometry, allocation policy and pool stay fixed.
Any rejection uses the exact original per-input state transitions and reuses
the digits already extracted. SIMD conflict detection checks bucket indices,
not bitmap-word indices. Different destinations in the same bitmap word remain
eligible; bitmap OR writes are scalar and preserve every bit.

Require eight real inputs, eight nonzero digits, room for eight ready updates,
every destination in range, nonzero first X limbs on both points, unequal first
X limbs, no prior pending destinations and pairwise distinct destinations.
Nonzero first limb implies the original identity test cannot hold; unequal
first limbs imply full X inequality. Therefore every admitted input would take
the ordinary original add branch. No affine mutation occurs during admission.
If the eighth append fills the batch, execute and consume the original queue
at that same boundary. Mixed/partial/zero/identity/equal/opposite/pending/full
packets retain the original scalar path, including mid-packet batch flushes.

Feature selection is compile-time AVX512F+AVX512CD in addition to the existing
native path. Other CPUs and generic builds retain original dispatch. A fixed
handful of stack arrays replaces no key data and adds no heap/table/plan/pool.
This is a dispatch-work hypothesis, not a predicted twofold proof improvement.

Before clocks: qualify the actual conflict/mask predicate against an independent
scalar predicate; qualify native G1/G2 packet state transitions, complete point
outputs, exceptions, tails, queue/batch boundaries, immutable inputs, scratch
reuse, allocation/pointer stability and1/16/32/shared-pool calls. Inspect emitted
actual code for intended vector admission. Use current176source hashes, original
field1200 and zopro_dense_busy_reset; API-only original REF plus fresh OFF/ON.
Any prospective timing must include complete queries and all dispatch costs,
both controls, real fixtures,1/16/32workers, automatic PMU/profiles, and separate
raw reader frozen before preflight. No unchanged reroll or subset promotion.
Passing components require current SDK ALL8+54/freshGnark/resource qualification.

Targeted history: fused-epoch-sdk176 establishes no gain; g2-ternary-span176
fails threaded observer calibration; weighted-double-bench176 loses parallelism;
g2-late-y/reverse-leaf/staged and recent early-factor/shared-inverse recipes
remain closed. This changes collective input admission, not their arithmetic,
observer, scalar recoding or queue policy. No SIMD-conflict admission result was
found in the targeted existing report search; this is not an exhaustive novelty
claim. No benchmark has started at this design checkpoint.
