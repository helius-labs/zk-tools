# Two-bank complete K/H caller screen

Question: does avoiding collision-driven projective accumulation reduce real K/H
query latency after charging the second bank and final merge? Qualified native
Window is inherited unchanged. Append a noninstrumented balanced-20-span feeder;
OFF uses the existing balanced fused caller (bits=13), ON changes only bucket
selection/state. No new arithmetic kernel, pool policy or production change.

Preparation allowance: 30 minutes before first executable result; if exceeded,
record why and simplify rather than starting a larger campaign. Measurement
allowance: 5 minutes for this fixed screen, excluding compilation/fixture oracle.
Use existing Rust xtask workflows, no new campaign framework.

Primary real K/H points come from the pinned Euler image through the existing
loader and Key preparation. Scalars come from retained TAGPHI01 input cache:
30529 K and 30922 H. Check every identity marker and canonical scalar; create an
immutable fixture. Independent Arkworks MSM checks the full combined result.
Fixture preparation/Arkworks is separate from all timing processes. No witness
solve, FFT, proof generation or new verified-export ledger entry.

One coherent native executable contains OFF and ON, chosen outside its query
call. Create three independently written byte-identical backings before capture.
Pin hashes/inodes. Fixed workers 1 (CPU2),16 (CPU0..15),32 (CPU0..31), default
512KiB Rayon worker stacks; warm pool setup outside timing. Global zopro allocator
and accepted native/runtime flags. Each process loads identical fixture/oracle,
checks input hash, runs three primes, then 17 timed complete calls. Include pool
entry, digit extraction, allocations, feeding, spills, both bank merges, weighted
reduction, final Horner fold, normalization and result equality inside timer.
All caller-owned temporaries drop inside call. Immutable-input digest and usage
accounting outside timer; record CPU, faults, switches and lifetime peak RSS.
Fixture and oracle loading count toward RSS; it is not full-proof RSS/minimum RAM.

First preflight: one timed call per arm at each worker count on backing0.
Main: three blocks, three backings, three worker counts, two separate-process
arms =54 processes. Rotate backing and worker order by block, alternate arm order
by block+backing+worker index. Preserve every sample; do not extend this screen.
Compare arithmetic process means as ON/OFF matched by block/backing/workers.
Report medians, all per-backing medians, ranges and win counts; no release CI or
adoption claim. If all three backing medians improve at a worker count, select
that context for a short verified SDK screen with resource checks. Mixed backing
signs are inconclusive; coherent regressions close this recipe. Any incorrect
output fails correctness. No minimum improvement floor. No all-shape campaign
before positive caller evidence. No helper/build/hash/analysis runs during live
preflight or recording. Archive raw receipts and protocol, no private fixture.
