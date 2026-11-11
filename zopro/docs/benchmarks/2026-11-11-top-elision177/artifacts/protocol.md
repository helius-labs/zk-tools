# Fixed-input indexed-top elision diagnostic

Question: does eliminating signed indexed-top reduction service materially lower
complete G2 query latency at 16/32 workers, or mainly reduce single-worker work?
Prior top packing/bundling reduced CPU work without qualified threaded gains.
This counterfactual removes more top work and is not an optimization candidate.

Clone current sources into a private snapshot. Retain the original top reducer
byte-for-byte. Add a wrapper around each of four part/component calls that saves
its first result. Both arms initialize identical four output vectors on their
first excluded prime. OFF always executes the original reducer; ELIDE returns a
clone of the corresponding saved result thereafter. Window merging, Phi mapping,
weighting, ordinary windows, grouping and normalization remain in both arms.
Cache identity is positional and valid only for one immutable fixture/process.
The harness rejects changed output via the retained independent Arkworks oracle;
this fixed-input cache must never be exposed as a production proving path.

Use the existing query driver with only the API name, mode handling and cache
coverage assertions changed. Every prime, sample and postcheck is oracle-checked;
point inputs remain unchanged. Reuse original independent fixtures/oracles.
Three excluded primes, seventeen timed queries, one postcheck per process.
Reset, grouping, allocation, MSM, normalization and pool dispatch remain timed.
No SDK/proof export, real-witness reuse claim or production memory policy.

Build once with current accepted native cfgs, Rust1.98.1, znver4, O3, fat LTO,
one CGU. Make three distinct ordinary executable copies before capture. Six
contexts: primary and grouped2x2 at1/16/32workers. Pin solo toCPU2 and others to
CPUs0..workers-1;512KiB worker stacks. All12 one-round preflights must pass output
and exact four-call/query cache coverage before timings. Then three fixed blocks
use each executable copy once, rotating context order and reversing alternate
blocks; mode order alternates by context+block.36 timed processes total. No
extensions, exclusions, fastest-copy selection or post-result variant change.
No owned build/helper/hash/analysis runs during captures. Poll only live workflow.

Report all paired ratios, absolute differences, CPU service and RSS, with no
confidence/adoption claim from three blocks. This is a directional research screen,
not a measured upper bound on full-proof benefit: graph/cache/allocator interactions
and dispatch/clone/counter overhead remain. Same-binary OFF is diagnostic control,
not exact accepted SDK. A disagreement across copies makes direction inconclusive.
Do not transfer the result to AWS or other shapes. Use it to prioritize ordinary
window work versus another top-specific algorithm. The all-eight CPU10x goal and
latest accepted CPU/Metal comparisons remain unchanged.
