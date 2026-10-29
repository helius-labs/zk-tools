# Eight-point identity (independent derivation)

Let w be a primitive eighth root, i=w², c+=(w+w³)/2, c-=(w-w³)/2.
For j=0..3 define S_j=x_j+x_{j+4}, T_j=x_j-x_{j+4}.
Compute four products in parallel:

p0=i*(S1-S3), p1=i*T2, p2=c+*(T1+T3), p3=c-*(T1-T3).

Then bit-reversed outputs are:

F0=(S0+S2)+(S1+S3)
F4=(S0+S2)-(S1+S3)
F2=(S0-S2)+p0
F6=(S0-S2)-p0
F1=(T0+p1)+(p2+p3)
F5=(T0+p1)-(p2+p3)
F3=(T0-p1)+(p2-p3)
F7=(T0-p1)-(p2-p3)

This follows by separating even/odd powers in sum_j x_j*w^(jk).
In particular p2+p3=w*T1+w³*T3 and p2-p3=w³*T1+w*T3.
It preserves root choice and both independent half-vector transforms.
Testing every basis vector proves this linear identity over Fr assuming the
inherited field operations; random/boundary and repeated-state tests also
exercise the actual lazy native representation. Four constant multiplications
per transform pack into ONE eight-lane multiplication for two transforms.
The existing DIF8 performs two full vector products, including trivial lanes.
Extra adds and permutations can erase the saved vector product. No gain is
inferred from this algebra, and no large-FFT decomposition is implemented here.
