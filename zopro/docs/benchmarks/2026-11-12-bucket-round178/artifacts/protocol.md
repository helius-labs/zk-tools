# Destination-ordered G2 rounds

Challenge the streaming filler's input-order dependency: route borrowed signed
point indices by bucket, then advance one accumulator per bucket in rounds.
Each round has distinct destinations and uses the existing signed affine adder.
No busy bitmap, collision queue, projective overflow or intermediate tree point
arrays are needed inside this filler. Identities, doubling and opposites retain
complete scalar handling. High component still precedes low, with the same phi.

This differs from balanced borrowed trees (multiple intermediate point arrays),
independent component planes (extra bucket sets and merges), and bucket-directed
collision queues (preserved arrival order). It retains their existing index-route
builder and 256-bucket groups. It may lose from extra routing, scattered key reads,
long skewed-bucket chains and small late inversion batches. No speedup is assumed.

Reuse the prior coherent current-baseline caller and its three modes, replacing
only mode2's tree reducer. Mode0 original streaming; mode1 staged balanced tree;
mode2 destination rounds. Reuse the conservative tree reservation, which exceeds
the new band accumulator/descriptor bound; source and actual-capacity checks cover
that claim. No persistent cache, copied key, new pool or production edits.

First screen allowance:15 minutes for preparation/build/measurement after this
protocol; simplify or record an explicit reason if exceeded. No new harness.
Reuse focused native cases and unchanged independent Arkworks full-query fixtures.
Five tests include signed/padded batches, skew/cancellation/reuse, full queries,
admission bounds and 1/4/16/32-worker 512KiB-stack pools. Native oracles share
accepted field arithmetic; complete-query fixtures provide independent oracles.

Freeze three executable backings, fixed ordering, six contexts (1x2/grouped2x2 at
1/16/32), three primes,17 timed calls and a postcheck.19 preflights include denied
admission, then54 main processes in three rotated/reversed blocks. Retain all
samples. Only poll live capture handles; no owned analysis/build overlaps timing.
Progress if mode2 beats streaming primary32 in every block and both shapes at
16/32 stay within1.03 in every block. This is directional selection, not adoption.
Profile only if it survives or results leave an actionable mechanism question.
Report CPU/RSS and all caller costs; no full-proof/AWS/phone performance claim.
Canonical ledger55028 unchanged; all-eight CPU10x objective remains active.
