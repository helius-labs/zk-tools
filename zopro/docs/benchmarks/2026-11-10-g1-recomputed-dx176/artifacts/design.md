# G1 four-vector records with reconstructed X difference

Goal remains verified CPU AVX single-proof >=10x supplied M5 CPU timings for
ALL8, with CPU and Metal comparisons. Accepted primary7.772288ms32workers;
targets0/8. Last block-local-gap native recipe is closed and unchanged.

The original G1 ordinary streaming group retains ax,ay,xsum,denominator,numerator:
five320-byte vectors,1600bytes per eight additions. Candidate retains
ax,ay,bx,numerator: four vectors,1280bytes. Forward still computes denominator
for the original numerator/prefix recurrence; reverse reconstructs denominator
from the stored canonical BX and AX and moves original X-sum construction there.
Thus one extra vector subtraction per group replaces one retained vector. No
coordinate repacking/rereading, new multiplier, inverse, serial preprocessing,
table, key copy, solver plan, pool, heap-vector field or geometry change.

The exact same original X operands reconstruct the exact same original vector
denominator and X sum under both FAST and generic arithmetic. Store old AX/AY
and numerator unchanged. This differs from the closed G1 prefix-only stream
(which rereads/re-packs coordinates), G1 binary-tree records, G2 denominator/X
caches and affine-REDC offsets (which keep five G1 fields/change endpoints).
No prior failed fixed implementation is rerun or reclassified.

Original scalar tails, signs, queues, point routing, all field kernels, G2 and
tree arithmetic remain exact. Change only native AVX512IFMA/VL/DQ ifma8 guarded
G1 record/body. Unsupported targets retain original struct/body. Existing
workspace forecasts derive from size_of<FqAddGroup>, so the same checked
reservations charge the smaller record without a new policy or RAM cap.
Actual capacities/allocations/stack/RSS still need checks; a20% record payload
reduction is not a20% full-proof RAM or latency claim.

First qualify exact raw old/new outputs plus independent Arkworks G1 sums,
every sign mask, zero/tail/vector boundary lengths, write/reserve modes, FAST
and borrowed/owned signs, reuse, all actual1/4/16/32worker indices and four shared
OS callers. Verify capacities in records and record-byte accounting, original
input preservation, complete-core tests and disabled/generic guards. Reuse
original field1200 and accepted176source; all production remains unchanged.

Only then freeze complete G1/G2 caller source/binaries/inputs/order/gates plus
producer and separate reader before preflight. Include source-matched OFF and
retained accepted-source API control, all useful setup and automatic profiles;
no isolated multiply timing is a substitute. Any promotion still needs fresh
fullSDKALL8+54/primary1/16/32/Gnark/resources. No gain floor, subset rescue,
threshold amendment or unchanged capture reroll. Keep all failures.

Canonical45335 unchanged; future exported proofs MUST seed
retained-tail-sdk176/proof-sha256-after-capture.json. Rust1.98.1+serializedxtask,
no helper/build/hash/analysis/report overlap during any preflight/recording;
only poll exact confirmed live handle to terminal. CommitANDpush completed
experiments and update handoff/latest-status. Original full goal stays active.
