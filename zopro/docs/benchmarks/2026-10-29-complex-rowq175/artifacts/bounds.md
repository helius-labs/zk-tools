# Shared low-word quotient plan for a complex product

Let B=2^52, I=-p^-1 modB. Let a0,a1,b0,b1 be normalized five-limb Montgomery values<=2p. Define u=lo(a0[0]*I), v=lo(a1[0]*I). Original difference2 uses negative operand2p-a, whose prepared low word is -2-lo(a[0]*I) modB. Therefore q contributions must include -2*b1[i]; omitting it changes exact quotients even though it vanishes modulo p in the final field value.

For normal complex product (a0*b0-a1*b1,a0*b1+a1*b0), prepare for each row i:
X=lo(u*b0[i]),Y=lo(v*b1[i]),Z=lo((u+v)*(b0[i]+b1[i])).
w_real=X+4B-Y-2*b1[i]; w_imag=Z+2B-X-Y.
These are congruent to the original input-row contributions times I moduloB, with0<w_real<5B and0<w_imag<3B.

For conjugate product (a0*b0+a1*b1,a1*b0-a0*b1), set
P=lo((u+v)*b0[i]),Q=lo(u*(b0[i]+b1[i])),T=lo(v*(B+b1[i]-b0[i])).
w_real=P+T;w_imag=P+4B-Q-2*b1[i].
Here0<=w_real<2B and0<w_imag<5B. IFMA's implicit52-bit masking of u+v, sums and biased differences is correct because all these computations are moduloB; their full64-bit inputs stay<2B.

Each component uses q_i=(lo(t_old0*I)+w_i)mod2^s,s52/lastmixed48. Row0t_old0=0 removes its q multiply. New sums<6Bfit64bits. Both original FUSED CIOS product/reduction/carry/packing/normalization bodies remain, including the same2p-negative operand in the corresponding difference. Quotients and raw lazy representatives must equal original sum2/difference2 exactly. Full independent REDC=(X+q*p)/R checks R256/R260; new plan does not change field ranges or factors.

Preparation is two low-word products plus3*5row products, versus four plus4*5 for two independent row-quotient calls. Both five-row original kernels retain8old-state q products,200product-half and90reduction-half IFMAs:315total versus322independent rowq and300original. Extra sums/subtractions and ten row vectors can cost more than shorter dependencies save. Nothing predicts a proof speedup. All preparations are operation-local; no key/root/table/cache/heap/point-copy or new field representation.
