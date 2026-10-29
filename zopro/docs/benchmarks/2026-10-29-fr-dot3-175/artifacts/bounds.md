# Sum3 bounds

B=2^64, R=B^4. Inputs are canonical0<=a,b,c,d,e,f<p; both BN254 primes
have4p<R. Let each CIOS row add three products a*b_i+c*d_i+e*f_i, then
q_i*p with q_i=-t0/p0 modB, and divide byB.

Inductively the state before a row is<4p. Before division the state is
<4p+3p(B-1)+p(B-1)=4pB<RB. Thus five u64 words suffice throughout every
positive partial sum, top-word additions never overflow, and the divided
state is<4p<R. Each elementary multiply-add computes x*y+word+carry with
word,carry<=B-1, so it is<=B^2-1 and fits u128 exactly. Cancellation makes
the discarded low word zero. After every row the fifth output word is zero.

After all four rows U=(T+Qp)/R, T=ab+cd+ef<3p^2,0<=Q<R. Hence U<p+3p^2/R
<1.75p<2p; the existing single subtraction yields the unique canonical result.
There is no changed lazy IFMA contract or broadened unchecked-input contract.

The explicit arithmetic template uses48 full raw word products,16 full modulus
products, and4 low quotient products. The existing sum2+mul template uses48 raw,
32 modulus and8 low quotient products. These are source-level arithmetic sites,
not emitted instructions, cycles or proof-speed estimates. Register/stack,
scalar carry dependencies and compiler output still need measurement.
