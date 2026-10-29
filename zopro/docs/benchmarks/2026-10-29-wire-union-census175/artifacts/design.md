# Shared proof-local A and B classification opportunity

This screen tests one work-removal idea after the offline control audit:
classify the union of A/B witness wires once per proof, then reconstruct each
query's original stable groups from shared class labels. This differs from
changing dictionary probing, static EqualLayout preparation or query order.
No execution plan, retained key table, point copy or witness reuse is proposed.

Read the accepted solver-image header and its strictly increasing A/B indices,
check every query scalar against its original witness wire, and compare raw-key
and serialized-plan scalar solutions for all nine existing fixtures. Use the
production representation geometry: canonical first two shapes, Montgomery
remaining shapes. Private values stay in memory and are never persisted.

Count nonzero query scalar hash visits for separate maps and for the wire union.
Count scalar classes as a second metric; do not confuse fewer visits/classes
with a runtime prediction. Sampling remains per query. Queries that sampling
rejects must not require global classification. Reconstructed local metadata
must match the source-extracted original builder exactly, including values,
stable class order, counts, starts, input lists and decline outcomes.

Model new dense class/remap arrays with checked sizes and compare their payload
to the existing conservative map-allocation bound. Do not claim a complete
prover forecast or minimum RAM from this screen. A native implementation would
need to charge all lifetimes, use already-owned wire values, and account for
front-loading global classification before independently scheduled queries.

Necessary gate for a later actual caller prototype: exact metadata agreement
in every case, fewer nonzero hash visits for primary A/B together, and the new
primary dense metadata payload no larger than the old conservative map bound.
There is no minimum gain floor. Keep all other shapes, including any negative
model results. A passing count model alone cannot adopt source or establish a
speedup; constructor scheduling and full SDK evidence would still be required.

Independent synthetic cases cover overlapping/disjoint/empty query sets,
zeros, repeated values at different wires and full-width words. Reuse the
existing accepted source/frozen libraries; no production build/change, proof,
AWS capture or performance timing occurs. Commit/push the completed screen.
The all-eight verified CPU10x objective remains active and unmet0/8.
