# Input-known denominator for two affine additions

This is a local derivation from the ordinary short-Weierstrass chord formulas. It is not a novelty claim and has not been implemented in the prover.

For finite P=(a,b),Q=(c,e),R=(r,s), define d=c-a and n=e-b. Assume d!=0. The first slope is l=n/d and S=P+Q has xS=l²-a-c and yS=l(a-xS)-b.

The denominator of the second addition is r-xS. It can be represented before computing l:

    H=d²(a+c+r)-n²=d²(r-xS).

Assume H!=0. Let U=d(b+s)+n(r-a). Then the second slope k satisfies

    k=(s-yS)/(r-xS)=dU/H-l.

Set D=dH,A=nH,B=d²U. All three depend only on the original coordinates. One inverse F=1/D therefore yields l=AF and w=BF=l+k. The output coordinates are

    xS=l²-a-c
    x=k²-xS-r
    y=w(xS-x)+l(x-a)+b.

For the last identity, expand w=l+k and substitute yS=l(a-xS)-b. It gives y=k(xS-x)-yS, exactly the ordinary second chord formula. No curve-specific value of a4 or a6 is used in these identities.

Infinity, d=0 and H=0 are guarded BEFORE inversion. If d=0 the first addition needs the existing complete rules. If H=0, x(P+Q)=xR and the second addition needs doubling or inverse-point handling. The model returns need-fallback, not coordinates, for those cases. It does not implement native complete fallback or batch repair. A native layer must preserve all original inputs until guards are resolved and must defer compaction until all reverse writes finish.

## Batched inverse and the extension field

For G1, use D itself in the prefix chain. For G2 over Fq[i], i²=-1, use norm(D)=D0²+D1² in an Fq prefix chain. Recover D^-1=conj(D)/norm(D) and share it between A and B. One inversion of the product of all norms handles a whole ordinary triple batch. Binary comparison performs two successive affine batches and hence two central inversion stages.

The exact field model uses the current lowering: Fq2 multiplication takes one fused difference2 and one fused sum2; Fq2 square takes two ordinary Fq products; norm is a fused sum2; scaling by an Fq inverse takes two ordinary products. The special Fq H expression uses difference2(d²,sum_x,n,n). G2 H uses a complex multiply minus the cheaper complex square. These are explicitly specified spellings, not all-algorithm lower bounds.

Per triple, the G1 direct body uses8 ordinary Fq products,3 ordinary squarings and3 fused2 calls:17 elementary products. Two binary additions use10 ordinary products and2 squarings:12. G2 direct uses13 ordinary Fq products and21 fused2 calls:55 elementary products. The two binary additions use14 ordinary products and10 fused2 calls:34. Thus this design increases per-triple arithmetic; its opportunity is inversion-stage and fixed-cost removal.

## Native-template ledger and exclusions

Accepted ordinary/square vector products have100 IFMA sites and fused2 has150. Eight independent inverse chains use the existing five-vector-product inversion bundle, one additional scalar Fq product and one scalar Fq inverse. Its packing, scalar arithmetic, shuffles and other instructions are excluded from IFMA counts and kept explicit. The exact scalar model itself carries one chain; the native ledger adds this separately inspected eight-chain bundle.

For g=m/8 full groups of triples:

    G1 direct:1550g+500 IFMA; binary:1200g+1000 IFMA
    G2 direct:4450g+500 IFMA; binary:2900g+1000 IFMA.

Direct uses one scalar product/inverse bundle versus two. At m=8, G1's2050 sites are below binary2200 even before charging scalar inversion. At larger batches its variable work grows faster. No cycle comparison follows from these counts, especially for G2.

The minimal proposed records retain prefix,D,A,B for G1 (four Fq vector words per lane), and prefix,norm(D),D,A,B for G2 (eight Fq vector words per lane). They require rereading original coordinates in reverse. For G1, retaining four additional coordinate words can instead avoid that reread/repacking. No record layout is selected or qualified here. Index lists, exceptional masks, compaction, data movement, binary-to-lane conversion, register allocation, pressure admission and scheduling are all unimplemented or excluded. Record word counts do not establish native peak RAM or minimum required memory.

## Other coordinate systems

The inspected co-Z formulas require a shared projective coordinate. Arbitrary incoming affine MSM points do not automatically share an accumulator's coordinate, so the reduced formula count cannot be imported without rescaling and output conversion. The EFD XYZZ mixed-add recipe is8M+2S before normalization, with four stored coordinates; the current batched affine baseline must be compared with its actual shared inverse rather than an independent inverse per point. No native co-Z/XYZZ replacement follows from this review. The old complete homogeneous-projective weighted-reduction negative remains separate evidence.
