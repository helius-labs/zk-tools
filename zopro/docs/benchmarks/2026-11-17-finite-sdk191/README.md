# Complete-proof finite-base screen

The exact candidate passes its short-screen advancement criteria. Primary latency
medians improve slightly;uncertainty versus accepted production includes parity.
Retain this build and finish qualification. No minimum gain floor or adoption is
implied by the screen.

| Local Ryzen9950X case | Accepted REF ms | Candidate OFF ms | Candidate ON ms | REF / ON peak MiB |
|---|---:|---:|---:|---:|
| 1x2,16workers | 8.787472 | 8.785048 | 8.768246 | 79.25 / 77.25 |
| 1x2,32workers | 7.576237 | 7.558764 | 7.538125 | 110.75 / 103.00 |
| 2x2,32workers | 10.555379 | 10.562011 | 10.639327 | 267.50 / 266.00 |

Values are medians of nine process medians;paired ratios need not equal ratios
of these medians. Primary16 ON/REF paired median is0.995346[0.989374,1.006783],
6/9wins;ON/OFF0.996362[0.990141,0.998087],8/9. Primary32 ON/REF is
0.998515[0.993195,1.003017],6/9;ON/OFF0.997791[0.993797,1.006417],6/9.
2x2 ON/REF1.000399[0.996128,1.013328],4/9. Bounds are descriptive zero-based
ranks1/7,under independent continuous blocks,not simultaneous confidence.
All declared median latency,CPU and RSS screen guards pass. The2x2 result is not
an established improvement. RSS is whole-process peak,notminimumRAM.

At key loading,the explicit native selector certifies finite B2points only for
1024<=n<65536 and stores one private Boolean. Ineligible large keys skip the scan.
The immutable key's existing exclusive G2 calls use the certificate. Grouped
point sums retain checks because cancellation can produce identities. Narrow top
windows also retain checks. There is no point lookup table or per-point metadata.
`ZOPRO_BENCH_FINITE_G2=1` enables the private candidate;default isOFF.

Raw/grouped/known counters are retained in the measured build. Every timed primary
ONrequest records one known and one raw entry;every2x2ONrequest records one known
and one grouped entry. AllOFFcounts arezero. Thus the grouped control does not
exercise unchecked finite feeding. Component gains must not all be attributed
to identity-check elimination. Accepted fusion and G2batching are enabled in all
arms. Added counter reads occur outside request timings;per-call atomic increments
remain inside and are charged to the candidate.

Three independent executable backings per binary are balanced over nine rotated/
reversed blocks. Each main process makes31timed fresh SDK requests after3primes.
Two functional4workerOFF/ONpreflights each make1timed+3prime. All83fresh exports
independently verify with Gnark and fixedimage/publicbindings;2762SDKcalls include
fresh witness solving,proof randomness,proving,SDK verification and serialization.
Loading,primes and export are outside latency. All135nonignorednativecoretests
pass,4explicit timing tests ignored,nonefiltered. No successful capture was rerun.

Candidate SDK SHA256:
`b655c4c68483d54e2343a0593818b49be05e140c949fe83cd6216d332ecb1897`.
Accepted REF:`c285cbc88f783abac203c66db43ea86bda8207338b9fb209df7eb5d846126202`.
Private root:`/home/ubuntu/src/zopro-perf-work/finite-sdk191`.
Canonical exports advance56585->56668,SHA256
`2b56de9ef8b481ab2bbf714748ece2232146b06799bd34b9cc68bd5915c7e085`.
Mechanism profiling,all-shape performance,portable/concurrent/resource release
checks remain pending. Production is unchanged. NoAWSorpublicpush. The full
all-eight CPU10xgoal remains unmet;retain accepted CPU/Metal comparisons until
this candidate completes qualification. Small gain size alone will not block it.
