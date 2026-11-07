# Derivation and work accounting

This is a local construction from the standard divisor/sum relation, not an attributed new theorem. Ben Lynn's [Zeroes and Poles notes](https://crypto.stanford.edu/pbc/notes/elliptic/divisor.html), especially the sum map, supply the background: a principal divisor has group sum zero. No performance result is imported from that source.

On y²=x³+b, let five inputs have distinct x coordinates. Put dij=xi-xj for i<j and Delta=product dij. The derivative V'(xi) of V(x)=product(x-xj) is (-1)^i times the product of the four incident edges. The complementary six-edge product Ni therefore obeys (-1)^i Ni V'(xi)=Delta. The degree-four polynomial Y=sum((-1)^i yi Ni)*V/(x-xi) satisfies Y(xi)=Delta yi without a field inverse.

Let V=x5+v4*x4+..., Y=u*x4+Y3*x3+..., and v=Y3-u*v4. Multiplying (ux-v)Y cancels its x5 and x4 terms against u²V. Thus N=(ux-v)Y-u²V has degree at most three. With D=Delta*(ux-v), N(xi)=D(xi)yi, so F=N²-D²(x³+b) vanishes at all five input x coordinates. If n3!=0, its leading coefficient is n3², hence F=n3² V(x)(x-x6). The x5 coefficient gives X=d1²-2n3n2+v4*n3² and x6=X/n3².

The function N(x)-D(x)y has pole order six at infinity when n3!=0. Its six finite zeros therefore sum to the identity with multiplicities. The desired five-point sum is the negative of the sixth intersection. This also follows in the exact model from the independent group oracle and all seven coefficient identities; tests alone are not the general divisor argument.

Set Z2=n3² and H=d1X+d0Z2. Homogenize N as Nh=n3X³+n2X²Z2+n1XZ2²+n0Z2³. A single inverse I=1/(Z2H) yields x6=XHI and -y6=-Nh*H*I². Identity, repeated x (including opposite inputs), zero n3 and zero H decline. If an input has D(xi)=N(xi)=0, F has a double root there; because V has simple roots, x6=xi and H=0. The final pole guard therefore covers that interpolant degeneracy without five additional products. The finite-coordinate model does not implement production fallback or validate arbitrary off-curve inputs.

The full-width tests use fresh SHA256-derived scalars and independent Arkworks complete group addition. A separately implemented integer group law over F43 checks accepted outputs of the same generic field construction; its affine inputs are enumerated on y²=x³+3. The integer group law shares neither interpolation nor point-addition code with the construction. Polynomial identity checks use straightforward separate coefficient multiplication, outside the recipe's counted work.

## Fixed arithmetic recipe

The Ni network shares six two-edge products and one three-edge product. It uses21multiplications for all five Ni, three more for Delta and five for weighted yi:29products before forming the interpolation polynomial. V needs10products; its five synthetic quotients need15; weighted coefficient accumulation needs20. Remaining work forms v,N,D,X,H,Nh and normalized coordinates. The counters total104multiplications, five squares and one inverse. Additions, negations, coordinate validation and correctness-only oracle arithmetic are excluded from these counters.

For g independent eight-lane five-input groups, a favorable native forecast batches all final inverses. G1 ordinary squares use the accepted generic multiply; the g final inverses cost(3g+2)vector multiplies plus one scalar multiplication/inverse through the accepted bridge. Total:(112g+2)VM,1SM,1I. Binary reduction has four point additions per group in three dependent levels:(24g+15)VM,3SM,3I.

For G2 each generic complex product uses2VF, each complex square2VM. Inverting g complex vectors adds gVF for norms,(3g+2)VM for real inverse sharing and2gVM for conjugate scaling, plus1SM/1I. Total:(15g+2)VM+209gVF,1SM,1I. The ordinary binary comparison is(28g+15)VM+20gVF,3SM,3I. Existing prefix-mode uncertainty can add4gVM to that binary comparison; the conclusion does not depend on choosing the lower work bound.

With VM=100 and VF=150 nominal source IFMA sites, the new variable terms are11200g(G1)and32850g(G2), versus2400gand5800g (or6200g with conservative G2 extra work). The dominant terms grow by about4.67×and5.66×respectively. Final inversion depth improves3→1. Scalar inversion costs remain unpriced; these formulas are not cycle models, lower bounds on other algorithms or measured performance. The fixed work gate fails for all declared g; no size/threshold rescue or native benchmark is selected. Other rational/intersection constructions remain an open question, not disproved by this explicit recipe.
