# Initial-affine accumulator state

This private candidate leaves the accepted field crate unchanged. Borrowed G1
and G2 Window exception accumulators use the existing sparse index high bit to
record initialization from a nonidentity affine point. A first update, or its
finish join, uses the standard unit-Z mixed-add formula with complete identity,
equal-point and opposite-point guards. Every update clears the state, regardless
of the resulting Z. G2 endomorphism changes X only and preserves the state.

The state is enabled only for bucket counts below 2^31. Larger bucket domains
retain the original u32 indices; this adds no limit within the original index
representation. MAX remains the empty sentinel. No Window fields, vectors,
dimensions, capacities, tables, point copies, plans, pools or forecasts change.
The synthetic boundary test exercises the existing representable domain without
allocating large arrays. It is not physical 32-bit/ARM validation.

The work opportunity is at most one specialized addition per nonidentity
exception accumulator, saving three field multiplications and two squares when
the operands are neither identity, equal nor opposite. Costs include a domain
check and index masking on access, tag construction at initialization, a flag
test and clearing store on an existing-accumulator update, and a newly required
index lookup/domain/flag test at finish. Logical storage is unchanged. These
counts are not a latency prediction or proof-wide opportunity measurement.

This differs from the rejected global unit-Z method: it does not test Z or alter
the control flow of all mixed additions. It also differs from the historical
bucket-occupancy flag, which tracked affine bucket identity. Both earlier
negative results remain closed; this candidate receives its own qualification
and must pass the complete prospective SDK comparison before any adoption.

Qualification builds native OFF/ON libraries, native OFF/ON targeted core tests,
and generic ON targeted core tests. Tests cover independent Arkworks output,
exact original Jacobian output for the helper, index boundaries, actual Window
state/finish/queue paths, identity/equal/opposite cases, G2 endomorphism, 53 Rayon
workers with requested 512 KiB stacks, and four callers sharing an eight-worker
pool. Old bitmap-reset and field qualification tests are not replayed. There
are no timed proofs in this stage. Production remains unchanged.
