# G1 denominator tree using existing anchor slots

The accepted G1 stream scales each signed numerator by a sequential prefix,
then multiplies that numerator by a sequential reverse inverse. This candidate
stores raw signed dy, builds a denominator product tree and propagates inverse
products to leaves before finishing points. For m full eight-lane groups the
denominator/numerator schedule uses 3(m-1)+m products instead of 4m. Forward
and inverse-tree multiplication dependency depth is logarithmic instead of
linear. This is not a prediction of a twofold or any full-proof improvement.

There is no new scratch vector. Internal tree nodes use existing FqAddGroup.ax
slots; leaves use .denominator, overwritten by inverse denominators only after
their product has been consumed. The original .ay/.xsum remain and .numerator
stores raw dy. Finishing rereads and packs bucket X, since the saved AX slots
now hold tree nodes. Original five-vector record capacity and scalar tails
remain. This adds real loads/packing and loses the original fused signed-dy /
prefix product. Tree indexing and Option-free original inversion costs remain
inside timing. Less dependency depth is not less executed CPU time by itself.

This explicitly follows the rejected G2 denominator-tree experiment's heap
indexing, without relabeling its result. That G2 norm tree lost calibrated solo
and16-worker tests and failed its full gate. G1's numerator-prefix dependence,
five-vector record and fused signed numerator differ. The new trade is a G1
raw-dy tree in reused anchor slots with anchor rereads; the prior G2 norm tree
has not been rerun or promoted. Earlier G1 two/four-chain, G1 prefix-only record,
unit-prefix, packed point-tree, native bucket-column, triple and field trials
also remain closed. No heap/layout/cutoff sweep is proposed.

Only the diagnostic ordinary G1 filler dispatch changes. Original Window
admission, identity/equal-X/doubling/cancellation handling, busy indices, queues,
batch size, signed Booth recoding, field arithmetic and final normalization
remain exact. The kernel keeps the original finite/nonzero-denominator and
distinct-destination contract. Short/noncanonical/nonvector calls use the
original stream. The old functions remain verbatim, no production hook.

No input-sized buffer, key table/copy, transformed key, duplicate solver plan,
pool, arbitrary memory cap or C/assembly rewrite. Existing unsafe record writes
are copied unchanged; no new unsafe operation. Equal vector capacities are not
equal process RAM, exact stack bounds or minimum RAM.

Qualification precedes captures: all original core tests plus independent heap
root/leaf inverses through65groups, changing signed generator-batch oracles with
tails/reuse, and complete queue/exception windows under1/4/16/32-worker512KiB
pools. All field-member source bytes must match accepted176; field35 qualification
is inherited with zero fresh field tests. No standalone prover/portable claim.

The complete-filler screen uses existing independent raw-A inputs/oracles for
1x2 and2x2, all21 ordinary width12 windows, including allocation, feed, finish
and drop. Raw pre-group populations need not match the final routed proof.
Fixed15 rotated/reversed blocks at1/16/32 workers, original/OFF/ON,20 passes,
three checked primes and one checked post. All270 latency callers precede six
PMU/six FIFO fixed16Mcycles:u profiles at primary16/32,200passes.18one-pass
preflights and30read-only ownership/decode/events/loss commands give330commands,
300callers,9018completepasses and51609600checkedprime/postbucketoutputs.

Gate: all six OFF/original intervals inside[1/1.03,1.03], all ON/control upper
ratios<=1.03, and both equal-log two-shape32-worker aggregates upper<1 with>=10
winning blocks. Ranks4..12of15 nominal96.484375%marginal coverage under independent
exchangeable blocks, no familywise guarantee. No gain floor, exclusions,
extensions, successful reruns or favorable subset promotion. CPU2 solo,0–15
physical,0–31logical whole-process affinity,512KiB stacks and worker migration
within mask. No owned CPU work overlaps latency/profiles. Profile floor400,
unknown<=5%, zeroLOST/ownership/CPU/mmap/task/period checks; below1000 is thin.
PMU running>=99.9%. Inherited RSS/fault counters are diagnostic, not RAM minima
or stall attribution. A passed caller would still need full SDK/proof/resource
qualification and all-eight CPU/Metal reporting before adoption.

Previous goal turn is progress: deferred-equality176 qualified, measured,
rejected, archived and pushed5e6a1bd47d. Production176/SDK6d83 and proof ledger
28847 remain unchanged. Goal remains verified single-proof10x suppliedM5CPU
on all eight shapes, reporting both CPU/Metal; accepted currently0/8.
