# Retained-capacity SIMD tails in complete SDK proofs

Prospective fixed protocol, 2026-11-10. Native qualification is committed and pushed at ecc0623b3b08469c193808cf0f0c7369ca806fa0. Its 130 native tests, 6,960 new cases/797,472 Arkworks pair outputs, 114 threaded bucket trees, capacity/source checks, core no_ifma compile guard and coherent optimized SDK links are inherited by identity. No successful native build/test is replayed.

The candidate pads an incomplete vector in the ORIGINAL reduce_layer only when its existing pair-index capacity and selected original group/prefix-record capacity already fit the rounded count. It uses a valid ordinary dummy pair, original vector field formulas/inversion, real-only scatter and descriptor truncation before original compaction. Otherwise original scalar tails remain. It adds no requested capacity, table, key copy, plan, pool or RAM cap. No window/queue/FFT/solver/grouping/field/geometry change. Dummy arithmetic, branches and compiler effects can erase savings; no gain is predicted.

Actual binaries are reused at original paths, without copying/rebuilding:

- REF: busy-reset-sdk176/sdk-ON, SHA256 7aff5f602a1e250a6b9b28889bf6278b51ec400af8399c7481c36248a8718d01.
- OFF: retained-tail-lanes176/sdk-OFF, SHA256 c6f0e32fcccb17d0bbb9bb8f8f5e65a11ed8dda653ac5a07c8297edb956b7262.
- ON: retained-tail-lanes176/sdk-ON, SHA256 203086b54aed8184ca5dccfd2abb2d8f458bde2d16b755c64a6fe7c423ea51b8.

All use original field1200, dense busy reset, identical fixed-placement driver/support, native/O3/fat-LTO/one-CGU SDK consumers and original environment/allocator policy. Driver creates its Rayon pool with DEFAULT stacks, with no explicit stack_size or RUST_MIN_STACK in the fixed environment. The earlier simd-bucket-sdk176 report's claim of requested512KiB SDK stacks was incorrect: 512KiB applied to native qualification pools. Its actual SDK timings used this unchanged default-stack driver. This is a scope/prose correction, not a runtime change or reason to rerun previous recording. No minimum-stack claim follows.

Before timing, all nine shapes at32workers and primary1/16 must pass OFF/ON SDK preflight and fresh independent Gnark checks:22SDK processes with3primes+1observed/exported request each, then22Gnark commands.88internallychecked requests/22unique fresh exports are requirements, not assumed outcomes. Preflight clocks select no performance result. Pin compiler/dependencies/source/binaries/driver/support/images/requests/VK/public/proof identities and unique export hashes.

Seed EVERYnew export from simd-bucket-sdk176/proof-sha256-after-capture.json, canonical44800. Only actual successful independent verification and fresh inventory may advance44800->44822preflight->45335final. Exports differ from internally generated proof requests. Full canonical-ledger and private key/request/witness/proof payloads stay local.

Measure complete SDK requests: solving, randomized proving, internal verification and serialization. Key load, pool creation, three primes and proof export are outside latency. Ryzen9950X16physical/32logical/sharedhost. SoloCPU2, otherwise workers i->CPUi and main inherited full context mask. Cases: ALL8+merge54at32workers plus primary1/16,11contexts, all3arms.15fixed blocks rotate context indices by block%11, arms by block%3, reversing both on odd blocks.31observed queries/process after3primes,495latency processes. No exclusion, extensions, reordered rerolls, favorable subset, parameter/worker rescue or changed threshold.

Unit is process arithmetic mean of integer-rounded SDK nanoseconds. Pair matching is by context/block. Bounds/median are sorted zero-based ranks3/7/11of15, nominal96.484375%marginal median coverage conditional on independent/exchangeable blocks, without familywise or stationarity guarantee. Ratios need not equal quotients of displayed medians. No arbitrary minimum gain floor.

Freeze77checks:11OFF/REF calibration intervals within[1/1.03,1.03];22ON/control nonregression upper<=1.02; primary32 and equal-logALL8 gain upper<1 and>=10/15wins against BOTHcontrols;22RSS guards ONmax<=1.05*controlmax+1MiB;9PMU+9profile quality checks. Merge54 lacks a suppliedM5reference and remains explicit nonregression/resource scope. Every failure and positive observation remains visible.

After ALLlatency, run one grouped PMU panel(31queries) and one cycle profile(251queries), after3primes, for primary16/primary32/2x232 and all3arms. FIFO gates cover observed calls. Group cycles:u/instructions:u/cache-misses:u/branch-misses:u, >=99.9%running. Profiles16Mcycle period/512pages, >=1000samples, <=5%unknown, zeroLOST, correct PID/TID/CPU/executableMMAP/event ownership. One-shot diagnostics are descriptive, not exclusive cost or causal attribution. Absolute period coverage is unvalidated.

All recording precedes offline decoding/verification. Each new profile decodes once; verify all513capture exports independently.1071capture commands:495latencies+18diagnostics+45offline+513Gnark.19422capture SDK requests+88preflight=19510;535new unique exports including preflight. Source/compiler/deps/binaries/input/seed/order/gates/producer AND separate raw reader are frozen BEFOREfirstpreflight. Deterministic capture-plan generation after functional preflight changes no selection or gates.

During ANYpreflight or recording ONLYpoll exact confirmed live handle until terminal. Timeout/truncationNOTterminal. No overlapping owned helper/build/hash/analysis/report/push/workload. Analyze after capture terminal. Preserve setup failures; do not replay successful old native tests/builds/oracles/decoders/proofs/captures for bookkeeping.

RSS includes image load/primes/allocator history and is not minimumRAM, pressure/swap qualification or a universal memory saving. Driver reports minor faults, not major-fault service; counts do not establish disk paging. no_ifma core compile with native field is not physical ARM/mobile/TEE execution. This trial adds no AWS/Aeglos, huge250/4999 or concurrent-throughput qualification.

Report CPU AND Metal comparisons for every supplied shape. Supplied M5CPU37/59/109/189/284/335/528/507ms;Metal24/30/48/74/101/124/176/175ms. CPU10xtargets3.7/5.9/10.9/18.9/28.4/33.5/52.8/50.7ms. Original verified CPUAVX>=10xM5CPU goal stays active/unmet untilALL8 genuinely achieved. Current accepted primary7.770238516ms,0/8. Commit ANDpush completed experiment with polite factual report and exactremote verification/checkpoint. Native correctness is not adoption.
