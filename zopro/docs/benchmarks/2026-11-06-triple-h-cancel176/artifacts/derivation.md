# Cancel the first slope from three-point affine addition

This is a local algebraic candidate, not a novelty or speed claim. The prior triple-affine-screen, triple-callers, mixed-sdk and unweighted-triple-caller174 outcomes remain closed. The change below removes actual products/denominator factors from their input-known dH formula; it is not a rerun of their flags, thresholds or record layout.

For finite P=(a,b), Q=(c,e), R=(r,s), define

```
d = c-a
n = e-b
S = a+c+r
H = d² S-n²
U = d(b+s)+n(r-a)
q = U/H
w = d q
v = n q
x = w²-2v+a+c-r
y = w(r-x-2S+v)+b+e+s
```

For d≠0,H≠0 these coordinates equal P+Q+R. Earlier formula used D=dH, A=nH, B=d²U, lambda=A/D=n/d, w=B/D=dU/H, k=w-lambda, x0=lambda²-a-c, x=k²-x0-r and y=w(x0-x)+lambda(x-a)+b. In x, both lambda² cancel and lambda*w=nq=v, giving the proposed x. In y, substitute n²q=d²Sq-U from qH=U; the remaining 1/d terms cancel, giving the proposed y. Thus D,A,B and recovery of lambda are unnecessary when only the final point is wanted.

If d=0 with finite valid P,Q and H≠0, n≠0, so P=-Q. Then q=-(r-a)/n, w=0, v=-(r-a), and the proposed output is R. Equal P=Q gives d=n=H=0 and requests complete fallback. Infinity or H=0 requests fallback before inversion; no unsupported exceptional result is returned. Verify this domain by exhaustive toy-curve addition and independent BN254 G1/G2 complete references, not just symbolic manipulations. Characteristic must exceed3; both supported BN254 fields do.

With scalar-field two-product lowering, H uses difference2(d²,S,n,n), U uses sum2(d,b+s,n,r-a). Before shared inversion-prefix accounting, the proposed generic Fq work is4M+2S+2fused2=10elementary products and1division. A scalar prefix inverse adds3M/element, giving7M+2S+2fused2=13elementary products; earlier dH formula gives8M+3S+3fused2=17. Two ordinary batched updates give10M+2S=12 elementary products, but need two central inversion stages/bundles rather than one. Chosen100/150IFMA templates predict equal asymptotic G1 vector-product instruction sites to binary, plus one saved fixed inversion bundle; this is not a native performance prediction. Fq2 lowering must be counted independently with exact existing square/product/norm routines.

Native memory idea, NOT implemented: retain only H and U*prefix, reconstruct linear d,n,S,a+c-r,b+e+s from immutable original points during reverse. Distinct destinations/no input overwrite until their reverse store must hold. This replaces the earlier eight-vector triple record with two vectors at the cost of rereading points; actual scratch/stack/codec/admission costs must be charged and measured. Do not silently adopt it from a scalar cost model. Weighted binary trees require their original weighted identity; a ternary reduction is not a drop-in replacement there.

The 2003 Eisenträger–Lauter–Montgomery [paper](https://arxiv.org/html/math/0208038v2), sections3–3.1, is background for eliminating unused intermediate coordinates in2P+Q. It retains two field divisions and does not prove this general three-input formula or its batched-prover benefit. Its published speed estimates are not transferred. The abstract of [2026 differential Edwards addition](https://arxiv.org/abs/2606.20831) requires a known difference and a different curve model; those assumptions are not supplied by arbitrary current MSM bucket inputs. No full-paper review of that work is claimed.
