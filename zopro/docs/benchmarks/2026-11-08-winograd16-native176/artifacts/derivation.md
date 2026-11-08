# DFT16 identity over any odd-characteristic field containing w of order16

Write S_j=x_j+x_(j+8), T_j=x_j-x_(j+8), j=0..7. Even frequencies are
DFT8(S) with eighth root w². For that even transform set s_j=S_j+S_(j+4),
t_j=S_j-S_(j+4), j=0..3. With i=w^4, c+=(w²+w^6)/2,c-=(w²-w^6)/2:

    p0=i*(s1-s3), p1=i*t2,
    p2=c+*(t1+t3), p3=c-*(t1-t3).
    E0=(s0+s2)+(s1+s3), E4=(s0+s2)-(s1+s3),
    E2=(s0-s2)+p0, E6=(s0-s2)-p0,
    E1=(t0+p1)+(p2+p3), E5=(t0+p1)-(p2+p3),
    E3=(t0-p1)+(p2-p3), E7=(t0-p1)-(p2-p3).

These give F_(2k)=E_k. Odd-frequency calculation uses
A_j=T_j-T_(8-j), B_j=T_j+T_(8-j), j=1..3. Define

    a=(w+w^-1)/2, b=(w³+w^-3)/2, d=(w²+w^-2)/2,
    D=d*A2, G=i*d*B2, H=i*T4.

The symmetric matrix [a b;b -a] times (A1,A3) has outputs U,V and three products:

    m0=(a-b)*A1, m1=-(a+b)*A3, m2=b*(A1+A3),
    U=m0+m2, V=m1+m2.

The matrix [i*b i*a;i*a -i*b] times (B1,B3) gives W,Z:

    n0=i*(b-a)*B1, n1=-i*(b+a)*B3, n2=i*a*(B1+B3),
    W=n0+n2, Z=n1+n2.

Using w^8=-1, w^4=i and i²=-1, odd outputs are

    B1base=T0+D+H+G, B5base=T0-D+H-G,
    B3base=T0-D-H+G, B7base=T0+D-H-G;
    O1=U+W, O5=Z-V, O3=V+Z, O7=W-U;
    F1=B1base+O1, F9=B1base-O1,
    F5=B5base+O5, F13=B5base-O5,
    F3=B3base+O3, F11=B3base-O3,
    F7=B7base+O7, F15=B7base-O7.

Share base sums T0±D and H+G,G-H to realize all four bases in eight add/subs.
There are16 outer S/T add/subs,26 even-half add/subs and32 odd-half add/subs:
74 total, plus13 constant products. Even4+odd3+rotations6=13. Coefficients
are prepared once per root and may contain i; they still count as dense products.
Bit-reversed output is [F0,F8,F4,F12,F2,F10,F6,F14,F1,F9,F5,F13,F3,F11,F7,F15].
For DIT, permute input from bit-reverse to natural and output back to natural.
Those permutations are real native work to qualify, not free model operations.

No novelty claim: this derives the field identity and exact implementation
recipe independently. It does not import a paper matrix, upstream GPL body,
processor timing, a memory claim or a proof-latency estimate.
