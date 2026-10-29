# Direct signed CIOS product difference screen

Research/history review: DoT2604.21566 carry normalization already tested24Oct;
do not repeat it. PSSM concerns SIMD single-scalar multiplication,2025/933
changes proving circuits, and the old shared/complemented products remain
negative. This new candidate directly subtracts raw products inside a signed
single-output CIOS accumulator, then adds p iff negative. Derivation inbounds.md.
No paper speedup is transferred, no upstream code is copied.

Use the exact accepted175 field originals. Freeze an ordinary executable before
private includes; then add the helper/test includes and compile a fresh ordinary
and candidate in the same executable. Only2new native Arkworks tests run;33old
field tests inherited after byte recovery checks. BigInt model covers30084
exact signed cases. Actual native outputs must be normalized and below2p.

Four workloads: complex chain, four independent streams, product+real scaling,
and complete affine arithmetic expression with a supplied factor. The latter
omits inversion/bucket handling and is not a valid-curve operation or SDK proof.
31fixedrotated/reversed matched blocks x4patterns x3labels,100000iterations,
3primes2000iterations. All4preflights thenall372latencies then4FIFO-gated199Hz
cycles:u profiles(chain/affine fresh/candidate),1000000iterations. RecordCPU
IDs/timestamps. Allcaptures beforeowner-userdecode/loss. Allhelperscompiled
beforecapture. NoownedCPUworkoverlap, noAWS. Fourdistinctstreamseeds preventCSE.

Gate beforecapture: candidate upperpaired affine and equal-log-weight4pattern
aggregate<1 AND>=21/31wins vsBOTHfrozen/fresh. Every candidate/context upper
<=1.03; every fresh/frozen interval within[1/1.03,1.03]. Median rank10..22 gives
nominal97.055% individual coverage underindependent/exchangeableblocks, no
familywiseguarantee. No minimumfloor/exclusions/extensions/unchangedreroll/
subsetselection/thresholdrescue. Passingcomponent selectsactualcaller/fullSDK
qualification only. Production175 remainsunchanged, all8CPU10xgoalactive.
No newproofs; ledger21259. KeepallCPU+Metal acceptedreference rows inreport.
CommitANDpush completepositiveornegativeexperiment beforeanothercandidate.
