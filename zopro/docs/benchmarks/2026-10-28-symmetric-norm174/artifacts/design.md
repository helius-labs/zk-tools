# Fused symmetric G2 denominator norms

Prospective private experiment on accepted174/source e623/SDK81033.
The previous whole-query G1-then-G2 ordering is closed negative (af74c0027).
History audit also closes radix52 bucket storage, prefix-only G1 records,
packed G1 records, SRE searches, and the earlier G1 symmetric-square pilot.
The latter preserved Fq2 sum2 norms verbatim; this candidate changes only that
fused norm. Existing complex square/multiply and G1 remain unchanged.

For a,b normalized below2p in R=2^256, compute REDC(a*a+b*b) with symmetric
product diagonals and one common Montgomery reduction. Product IFMAs fall
100 -> 60 per eight-lane norm; reduction still uses accepted cancellation carry.
The six-coefficient schedule derives from the archived integrated square, but
adds both squares before each cancellation. Double only cross products, never
the contributions already added by reduction. Final reduce_fused preserves
the lazy bound. Since 4p<R, T<8p^2<2pR, ordinary REDC<3p; conditional subtraction
of2p yields a nonnegative representative below2p. Carries must be normalized
and all coefficient accumulators remain below2^57 (conservative sum bound).
No assumption that both inputs or their sum are canonical is allowed.

Only eight-lane native Fq/R256 norm dispatch changes under private compile cfg
zopro_symmetric_norm. Original field methods and OFF norm expression remain
verbatim. Add a bounded field child module with new independent Arkworks norm
tests, lazy/edge/range tests and dependent chains. No unsafe instructions beyond
the required AVX512 intrinsic implementation; no tables, heap scratch, key copy,
extra plan, pool, allocator, scheduling, scalar conversion or forecast change.
Unsupported paths retain original norm arithmetic. Three original files differ
(ifma8.rs module declaration, simd.rs wrapper, msm.rs norm dispatch),171 exact;
remove these and the one new module to recover174. Candidate175 files.

Full OFF/ON native field suites,122 core and8 prover tests each before capture.
ON field suite includes all new tests; names and actual counts audited. Freeze
coherent native transitive rlibs/proc macros and actual field/core flags/features.
OFF/ON are separate binaries (compile-time selection), with retained accepted
SDK as independent control. No source/layout/flag sweep or control reroll.

Fixed7rotated/reversed blocks ×4cells (primary16/32,16x2/32,merge54/32) ×3variants.
Twelve one-round SDK preflights independently Gnark verify before all84latency
processes;151 primary/31 others after3primes, then84Gnark verification.
Primary32 advancement requires ON/OFF and ON/accepted upper<1, >=5/7wins, and
OFF/accepted extrema within factor1.03. Larger ON comparisons upper<=1.03;
calibration separately reported for other cells. Physical16 descriptive.
Seven-ratio extrema median bounds98.4375% nominal under independent exchangeable
blocks, not familywise. No gain floor/exclusion/extension/successful capture
repeat. A pass permits further all8/clean/portable/resource qualification only.

After all latency/verification:6FIFO grouped PMU processes(primary16/32×3v),
31requests+3primes, events{cycles:u,instructions:u,cache-misses:u,branch-misses:u}.
Require actual running>=99.9%. Then6separate199Hzcycles:u profiles,251requests+
3primes, one sampledPID and zeroLost each. All captures before diagnostic
verification/decoding/hashes/archive. No owned build/helper/hash/archive/
verifier/second benchmark during any proof capture. Root for all perf variants,
normal user for latency. No global kernel setting or installed probes.

Total9672internallyverifiedSDKrequests,108fresh164Bexports/216members,
ledger15325->15433. Profiles/counters describe CPU work but their times do not
replace latency. Inherited accepted all8CPU+Metal table retained; original10x
goal ACTIVE/UNMET0/8. No AWS/Aeglos/mobile/ARM/TEE/giant/minRAM/swap/pressure
claim. No prediction of a twofold proof gain from this limited arithmetic scope.
Commit AND push complete experiments, negative too. Production unchanged.

Research checked primary SRE arxiv2610.27452v1, already represented in older
bounded/global-window studies; no SRE code or speed claim is imported. Sources:
https://arxiv.org/html/2610.27452v1
https://orbilu.uni.lu/bitstream/10993/57189/1/thesis.pdf
The latter radix51/Karatsuba discussion is not a matched BN254 result or an
implemented candidate. This norm design follows symmetry in the current source.
