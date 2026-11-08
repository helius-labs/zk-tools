# Signed small-quotient binary GCD matrix

Selected8November2026 BEFORE model/native builds/clocks. Original verifiedCPUAVX
singleproof>=10x suppliedM5CPU ALL8 +CPU/Metal reporting ACTIVE/UNMET0of8.
Production176/SDK6d83 unchanged; canonical35171. Latestprimary8.002769629139072ms
(older SAMEbinary7.915095/7.928660 unexplainedvariation),other7fusedepochinherited.
No questions/subagents/budget. LowRAMuniversal/nohugekeytables/newpool/CASMrewrite.
CommitANDpush each finishedexperiment/report. PBQP0b8131bd74 closednegative;
carryprefix/rank5/G2staging/packedrow/Euclidquotient/broker histories retainoutcomes.

The read-only history check found previous vectorized full31bit integerrows,
packed31bit scalar matrix rows, fused coefficient carry, active-word updates,
Euclid one-word quotient helpers and LogJumps field REDC. This candidate changes
the small GCD matrix step itself to cancel multiple low bits. It is not a new
claim about any of those closed implementations.

Primarycontext: Pornin2020/972 is the accepted batchedapproximateGCD basis.
bitcoin-core/secp256k1 doc/safegcd_implementation.md (primary upstream document
read8November2026) describes low-bit modular-inverse cancellation in variabletime
divsteps. We borrow the cancellation idea, not its delta-state algorithm or
performance. No external source or speed figure is copied. This implementation
hypothesis combines signed small quotients with the current approximate positive
binaryGCD and encodedtwo-row matrices; no novelty claim.

For odd b, b^-1=b mod8. After choosing the larger approximate operand as a,
take q=ab mod8 represented in{-3,-1,1,3}, so a-qb is divisible8. Normalize its
sign and divide out trailingzeros up to remainingbits. Update row0-=q*row1;
if the numerator was negative, negate row0. Shift row1 by the consumedzeros.
For final2remainingbits use signedmod4 q in{-1,1}; for1bit use originalq=+1.
First strip a's evenrun as before. Matrix rowencodingE=f+2^32*g mod2^64 remains.

qabs<=3 gives |a-qb|<=4max(a,b), needing TWO headroombits. Therefore select
BITS30, leadingBITS+2=32, lowBITS30: total62bits, and4*(2^62-1)<2^64.
This is the largest k satisfying2k+2<=62, not a tuning sweep. Accepted k31
uses33leading+31lowbits with no such headroom. Approximate mapping/error and
exactfullrow sign normalization need independent checks; do not assume the
approximate sign equals the actual fullinteger sign.

RowL1norm starts1. For a step consuming s>=3, |q|+1<=4<=2^s; for the final
2/1bits it is<=2/2, so each rownorm remains<=2^consumed. Thus blockrowbound
2^30, two-block composition2^60, bothfit signed64. Native coefficient updates
would divideby2^30/2^60 with unchanged generic Montgomery logic, not31/62;
the original64bitEuclidfinish andfinalscale stay. No table/heap/key/plan/pool
or newpublicruntimeflag. Morefullinteger/coefficientupdates may erase fewer
small-matrixiterations. Inversion is only part of the proof; no2xforecast.

FIRSTSTAGE ONLY: exactRustBigInt model comparing accepted31bit zero-run scheme
and signedjump30bit scheme on identical canonicalrawnonzero inputs. Preserve
per-input smallmatrixjumps, outerrounds, fullintegerrows, modularcoefficientrows,
Eucliddivisions, quotient/sign histograms and maxrowL1/maxrounds. Check each
encodedrow against independent signedintegerrow tracking, determinant magnitude,
fullrowdivisibility, sign-normalizedintegerupdates, modularcoefficientinvariants,
finalrawinverse againstBigUintFermat and exactacceptedfieldlibrary. No timings.

Use2primes, canonical1,p-1, half/mod4boundaries, allrawpowersoftwo andneighbors,
2048fixedfullwidthrandominputs/prime and512correlatednearbyinputs/prime.
Also exhaust a<=1023,bodd<=1023 for eachmatrix algorithm, plus lowbits/top62bit
boundary patterns. Allcases require bounded arithmetic/matrixnorm and exact
relations. Wholeinverse mustfinishwithin original32outerroundcap on this fixed
population; a failureblocks native selection and remains visible. No parameter
or quotientset/headroom rescue after observingthis recipe's results.

Model onlyqualifies a possible algorithm. No fewer-iteration speedclaim. If
the new scheme weakly increases ALL four whole-population expensive-work counts
(smallmatrixjumps,fullintegerrows,modularcoefficientrows,Eucliddivisions) forBOTH
primefullwidthrandomcohorts, with some strictincrease, close it as dominated
before native work. Otherwise inspect the complete tradeoff before selecting
a separatelydeclared native correctness/caller trial. No performance gate,
adoption orSDKselection is implied by this mathematical census. Successfulold
test/profile/decoder/capture is inherited, not replayed. Currentfullgoalpersists.
