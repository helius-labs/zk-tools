# Exact packed input subtraction

For canonical raw Montgomery representatives 0<=a,b<p, T=p+a-b lies in[1,2p-1], including a=b (T=p). BN254 primes satisfy2p<2^255, so T fits four unsigned64-bit words. Subtracting b from a modulo2^256, followed by unconditional addition of p modulo2^256, is exactly T. The subtraction's top borrow equals the addition's top carry. No Montgomery multiplication or change of scale occurs.

For each subtraction word d=(a-b)mod2^64, its outgoing borrow for incoming c in{0,1} is [a<b] OR([a=b]AND c). The corrected word is(d-c)mod2^64. For addition s=(t+p)mod2^64, its outgoing carry is[s<t] OR([s=2^64-1]AND c); corrected word(s+c)mod2^64. Generate/propagate formulas are exact for all inputs. Four-word scalar oracle separately computes a+p with u128 carries, then b with overflowing_sub, checking both terminal carries zero. It checks exact lazy words before five-digit radix split; the result must match accepted sub_canonical exactly, not just modulo p.

All four input limb references are valid initialized Fp fields. Copied transpose uses only unaligned reads over complete initialized arrays and deterministic lane permutations, under the accepted AVX512 IFMA/VL/DQ module guard. 52-bit output digits split the exact<2^255 result; no vector shifts reach64 and no signed overflow is used. Private API exists only for mixed R256. Unsupported representation/CPU entry and integration are not implemented or claimed.

No heap operation in constructor. Inputs are borrowed; small fixed register/stack arrays replace conversion temporaries. Stack size, RSS, minimum RAM, allocation failure and pressure behavior are unmeasured. Standalone component fixture arrays are diagnostic input storage, not a new key cache or deployed plan.
