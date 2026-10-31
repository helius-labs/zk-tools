# Integer IFMA LogJumps with mixed-radix stages

Previous turn PROGRESS: inverse-hint reuse completed, verified and pushed as
e50085fbfb. It failed its full-SDK screen and remains closed. Named solver
frames were only1.4–2.0% sampled user periods; curve work dominated. The all-eight
verified CPU10x target remains active and unmet, with Metal comparisons retained.

Research checked the primary LogJumps note by Yuval Domb, Koh Wei Jie's author
explanation, and the abstract of Kirbiyik et al., ePrint2026/1324. The latter's
FPGA/BLS12-377 latency result does not predict CPU or BN254 performance. Its PDF
and author-manuscript downloads returned403. Full FPGA paper not claimed read.
The older local exact-FMA-RNE51 trial also used inverse-radix folding; it remains
rejected. This is an integer IFMA52 mixed-R256 schedule, without floating-point
conversions or a different retained field representation.

Candidate: form the full unsigned product convolution (or sum of two), then
parallel LogJumps of104bits,100bits, and one52bit Montgomery cancellation.
First jump: normalized52bit digits at0/52 times rho104/rho52 plus the high part.
Second jump:52bit and48bit digits times rho100/rho48 plus the high part. Total
256bits preserves the host/mixed radix. Only native8lane MIXED products dispatch
to it under a private compile-time cfg. Four-lane and uniform-R260 dispatch stay
original. All point formulas, tails, packing, scratch, scheduling and images
stay original. No per-product environment/atomic lookup.

The two jumps remove serial quotient multiplications. However, accepted CIOS
already omits canceled low products, so no IFMA instruction-count reduction is
claimed. Normal products acquire a final conditional2p correction too. This is
a distinct work/dependency trade, not a predicted2x proof gain.

Constants rho48/rho52/rho100/rho104 are derived by const modular halving, four
32byte residues per prime (256logical bytes for both primes), not per-key tables.
Native code may replicate broadcasts; emitted size is inspected. No new heap
buffer, key/plan copy, lookup cache, pool, memory cap, C or assembly rewrite.

The successful independent BigUint model uses all7^4 corner tuples plus1695
deterministic random tuples, single/fused products, each of two primes:16,384
checks. It compares against C*(2^256)^-1 modp. Exact conservative bounds for raw
single/fused output are2.078p/2.834p inFq and2.522p/3.278p inFr, all<4p, so one
conditional subtraction of2p restores<2p. A plain-product<2p claim is false.
No modeled arithmetic failure. Two preliminary Rust compiles failed on BigUint
shift-type inference before any model execution; sources preserved.

Before timing, qualify actual instructions against independent Arkworks for
both primes, lazy boundaries including2p for the complemented operand, inverse
constants, fused products and chains. Run full native field/core/prover suites
for each compile-time arm and generator consistency. Preserve original control
and coherent libraries, final native/fat-LTO flags. Portable/resource/all8shape
qualification still needed before adoption; no unsupported-device speed claim.

Screen fullSDK1x2/2x2 at16physical/32logical workers with acceptedSDK6d83 and
separate freshOFF/ON builds.15fixed rotated/reversed blocks,101requests/process,
3primes,12verified preflights, then6PMU+6cycle panels251requests each. Eachcaller
exports its last proof for fresh Gnark checks (204exports/21816requests). No
owned concurrent workloads during capture. Original images/requests unchanged.

All contexts retained. EveryOFF/accepted calibration interval must fit factor
1.03; everyON/control upper<=1.03; each32worker shape upper<=1; both four-context
equal-log aggregate upper<1 with>=10/15wins. Ranks4..12of15 nominal96.484375%
marginal under independent exchangeable blocks, no familywise claim. No gain
floor, sweep, exclusion, extension, reroll, favorable-subset or layout rescue.
Publish even a failure, verify source/receipts/proofs independently, commit AND
push. Full goal remains all8verifiedCPU10x, not this preliminary screen.

Primary sources:
- https://hackmd.io/@Ingonyama/Barret-Montgomery
- https://kohweijie.com/articles/25/logjumps.html
- https://eprint.iacr.org/2026/1324
- https://tugraz.elsevierpure.com/en/publications/efficient-parallelization-of-large-scale-modular-multiplication-v/
