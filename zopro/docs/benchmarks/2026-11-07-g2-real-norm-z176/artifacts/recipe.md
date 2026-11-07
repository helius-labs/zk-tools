# Concrete model recipe, fixed before counts

Use eight logical lanes and the accepted operations: VM (one eight-lane Fq
product), VF (one eight-lane fused sum/difference of two products), SM/SS/SF
(scalar ordinary multiply/square/fused-two-product), and I (one scalar inverse).
Keep them separate. Nominal accepted CIOS site counts100VM+150VF are an IFMA
proxy only; scalar work, carries, loads, conversions and instruction scheduling
are additional. The accepted x86 Fq2 scalar multiply is TWO SF, not three SM.

The real-norm recipe keeps one compact canonical cofactor buffer. Iterate output
slots in group order: a slot is either a point pair or an odd carried point.
Build full eight-slot blocks, padding inactive slots with denominator1; carry
slots also have denominator1. This uniform recipe charges padded lanes and
untouched singleton groups. It does not filter a favorable subset.

1. For every block, compute norms, store the eight prefix chains, multiply chains.
2. Butterfly the final eight chains to obtain broadcast D and chain cofactors
   in five VM calls. Reverse blocks, recomputing norms from original points,
   update suffix chains and overwrite prefix records with final cofactors.
3. Compute D²/D³ and shared real Z. Forward blocks compute pair outputs; blocks
   containing carries additionally scale all four coordinate components and
   select the appropriate result. Read all inputs of each block before writing
   its compact outputs, avoiding a separate carry buffer. Group boundaries stay
   intact. Compact coordinate conversion/selection is unpriced bookkeeping.
4. Repeat all levels with the same recipe, including singleton pair levels.
   Normalize all final groups with one real scalar inverse. Keep a single prefix
   allocation sized to the first frontier, retaining capacity across levels.

The generic complex-D reference uses the same output geometry/cofactor lifetime,
with Fq2 prefix products, complex D²/D³/scalings and ordinary complex final
normalization. Its purpose is a mathematically comparable construction; it is
not the accepted source comparator and never had native timings.

The accepted-source counter uses floor(m/8) complete pair blocks and m%8 scalar
tails, prefix records when m>=32, captured Group records below32, and the exact
five-VM-plus-one-SM-plus-one-I product-tree inverse bridge at each level. Charge
the tails as7SM+5SF each; complete blocks as7VM+5VF in prefix mode or8VM+5VF in
captured-record mode. These counts are source-derived only for ordinary,
nonexceptional pairs with vector_inverse enabled. Algebra tests do not execute
the accepted private reducer or qualify branch counts on production witnesses.

Track real and complex model point/cofactor capacities with size_of, and compare
the required retained source field payload:128bytes per point,640bytes per
prefix record,2880bytes per captured Group. Source prefix and Group buffers
coexist after different levels have used them. Report pair-index and group
metadata separately; ignore no live arrays. These are typed payloads, not native
RSS, full allocator capacity bounds or minimum RAM. In particular, do not claim
a point-layout saving: accepted native FqLimbs stores one32-byte Fq.

All-zero/singleton-only group sets return unchanged points without entering the
recipe. Identity inputs or a zero pair norm at any level decline transactionally;
complete original addition of the original inputs supplies the mathematical
fallback. Exception-path cost is unpriced and not assumed absent universally.

Qualify both constructions against complete generator-weighted outputs and an
independent small-prime Fp2 affine oracle. Independently reconstruct operation
counts from level/output geometry; no field/oracle/native benchmark replay for
bookkeeping. No timing or native integration is selected by this recipe.
