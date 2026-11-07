# Single-vector common-Z G1 finish

Selected after the algebra/work screen a5e9d1908a, before native timing.
Implement the qualified shared-product construction on at most sixteen G1
points: one eight-lane pair vector at the first level. This limit follows from
existing LANES=8, with no threshold/width/worker/layout sweep selected.

Keep the accepted field1200 library unchanged. Use public Ifma8 operations,
halves and shuffle_pair; introduce no unsafe code or field export. Butterfly
pair/quad/oct products form D and every cofactor in five vector products,
without a scalar inverse or a scalar horizontal product. Inactive denominators
are one. Keep Z packed between levels; serialize coordinates between levels
through existing get/new APIs. Their complete cost must be measured.

The singleton-pair level uses the already-qualified equal-Z formula. Odd
carried coordinates rescale by D²/D³. A final affine result pays one ordinary
accepted scalar inverse. Initial identities, zero denominators at any level,
lengths above sixteen and degenerate cases use original complete projective
addition of the original immutable inputs. This research fallback is complete
but is not yet the accepted group_affine caller; real integration must preserve
that caller and include all cost. No performance adoption from this prototype.

Qualify every size0..17, empty/singleton, ordinary full-width generator walks,
input immutability, initial equal/opposite/identity, late equal/opposite and
actual1/16/32 Rayon workers on512KiB stacks. Verify butterfly products and all
other products for varied nonzero fields and inactive lanes. Scalar oracles
share the unchanged accepted field but not the SIMD product tree. Reuse the
completed algebra screen and old field/core suites; do not replay them.

No point table, retained key copy, duplicate plan, production pool, heap
scratch or huge/mobile/pressure claim. Research qualification pools test stack
and dispatch only. No timing yet: after correctness, prepare a separately pinned
prospective complete caller comparison with accepted original, fresh control,
all declared contexts, profiles and output checks before any claim of benefit.
