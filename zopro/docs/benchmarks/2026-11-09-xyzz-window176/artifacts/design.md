# Sparse XYZZ Window integration

Continue the qualified XYZZ helper in actual borrowed G1 and G2 Window owners.
Keep ordinary affine batches, input order, queue rules, window geometry, field
library and weighted reduction unchanged. Replace only sparse residual points
with cached Z²/Z³ coordinates, selected statically once per operation after
checked resource admission. No dynamic branch per point and no second populated
point array. Generic, disabled and resource-denied operations use Jacobian.

Admission uses the existing process-wide resource policy, not the legacy table
ceiling or an invented RAM cap. A token lives across all parallel windows,
high/Phi/low feeds, final merge and normalization. Charge every potential window
and part, the extra field in growing sparse and combined vectors, an entire old
XYZZ allocation for transient growth, and bookkeeping. The original required
forecast stays unchanged. No per-window OS query is introduced.

Qualify actual Window feeds, signed collisions and queue pressure, exceptional
points, balanced and uniform digit tails, Phi x mapping, final merge and batch
normalization against independent Arkworks points. Exercise 512 KiB workers,
1/4/16/32-worker pools, four OS callers sharing eight workers, and resource
denial/overflow with the original fallback. Native and generic checks precede
any timing. Correctness runtimes are not performance evidence. Full caller
controls, memory measurements and automatic profiles must be declared and
frozen before recording; only a complete caller gain can justify SDK replay.

Production remains unchanged. This residual path alone is not a twofold route.
The full goal remains CPU10× for all eight shapes, with CPU and Metal reported.
