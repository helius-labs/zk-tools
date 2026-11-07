# Shared-product common-Z reduction screen

Before implementation or capture: test whether a binary sum tree can retain one
Jacobian Z for all live points and replace every inter-level field inversion
with products. This is an algebra/work screen, not a timed candidate or a proof
benchmark. The full all-eight CPU10x goal remains unchanged.

For pairs (X1,Y1,Z),(X2,Y2,Z), define d=X2-X1, e=Y2-Y1.
For a level with m pairs let D=product(d), q_i=product of all d except d_i,
S=D^2, T=D^3, u_i=e_i*q_i. Return

    X'_i=u_i^2-(X1_i+X2_i)*S
    Y'_i=u_i*(X1_i*S-X'_i)-Y1_i*T
    Z'=Z*D.

An odd untouched point is rescaled by (S,T). Prefix/suffix products form q
without any inverse. A final singleton pair uses the ordinary equal-Z formula
without building product buffers. A final affine output still pays one inverse.
This is a common-scale lift across arbitrary independent pairs; the old co-Z
literature review and projective-all-the-way experiment did not implement this
specific shared-product construction. No claim of mathematical novelty.

If an input is identity or any d is zero, the research fast path declines and
complete original projective addition reduces the original inputs. A failed
later level discards all speculative coordinates. Nothing mutates key inputs.
All transformations use the original qualified field library. No new field,
unsafe implementation, point table, solver plan, thread pool or key format.

Qualify both G1 and G2, odd/even lengths through 512, non-unit common scales,
ordinary sums, initial and late equal/opposite/identity cases, and input
immutability. Compare to generator-weighted expected sums and original complete
addition; these are different group algorithms but share the accepted field.
An independent small-prime rational oracle must check the common-scale identity
without importing the native curve implementation. Instrument each recipe's
M/S/I calls and compare with separately written formulas.

The cost comparator is an explicit *abstract* single-chain affine tree:
5m-3 multiplications, m squares and one inverse per level with m pairs. It is
not the SIMD eight-chain G1 kernel or the norm-based G2 implementation. Do not
convert extension M/S into SIMD latency or infer a proof-time bound. Report
break-even I/M requirements with S left symbolic, both returning projective and
returning affine. Include carry-point rescaling and final normalization.

Fixed count lengths: 2,3,4,7,8,15,16,31,32,63,64,127,128,255,256,511,512,
1024,4096. No timing, gain floor, worker/size/layout sweep or resource claim.
Select a native trial only if the declared multiplication/inversion exchange
offers a credible path in the actual caller; otherwise close this recipe with
counts and mathematical evidence. This screen cannot exclude other inverse-free
algorithms. No SDK/Gnark proofs or canonical-ledger changes are authorized by it.
