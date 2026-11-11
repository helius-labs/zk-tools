# Two temporary accumulators per G1 bucket

Use the retained primary K/H scalar/identity cache and original twenty balanced
255-bit spans, widths12/13 and capacities200/352. Compare a generic noncancelling
one-bank state machine against a two-bank extension. Preserve operand order,
LIFO collision queue, size-minus-one flush threshold and final execute/spill.

Prefer bank0 whenever not busy, otherwise bank1, otherwise queue. Queued entries
retain logical destinations; draining selects a currently free bank by the same
rule. First writes and exceptional projective routing are accounted per original
semantics; actual equal-X/cancellation dispatch is outside this scalar model.
Both banks share one ready batch. Spill accumulators remain per logical bucket.
Finish the original sparse projective/affine merge and normalization in bank0,
then merge bank1 into bank0 in one logical affine batch, skipping empty sides.
Charge one generic affine update for each doubly occupied destination; its own
inverse stage, vector groups and scalar tail count are separate.

Count arrivals/first writes/updates/queues/spills/normalizations, all stream and
final-bank-merge inversion stages, nominal six-product G1 affine updates, and
projective additions separately. No invented conversion between these counts and
cycles. Extra mandatory affine storage is64bytes per bucket plus busy-bit storage;
no extra key table, point-input copy or production pool. Admission, actual vector
capacities and peak RSS remain unimplemented.

Reproduce the earlier one-bank baseline counts. Check all resulting logical bucket
sums with signed integer tokens and all input digits with an independent bitwise
reader. Test bursty/uniform/signed/empty patterns, capacity boundaries, two feeds,
collisions in both banks, final merging and ordering. These are state-model
checks, not curve or native qualification.

Select further native feasibility work only if the two-bank model reduces spill
adds and normalization buckets, does not increase total nonassignment joins
(affine updates including bank merges plus projective arrivals), and does not
increase total inversion stages. Replacing expensive projective routing with more
affine updates is allowed and must remain visible. This is a fixed
necessary work screen, not an adoption threshold or a latency lower bound.
One fixture, no bank-count/size/order sweep, no favorable subset promotion.
Use Rust xtask; preserve failures. Accepted source and53917proofs unchanged.
No push or publication authorized.
