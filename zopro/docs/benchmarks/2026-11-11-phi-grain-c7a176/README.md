# G2 scalar-grain candidate: native and functional preparation

The current-source control and candidate both pass native qualification and fresh functional proof checks. **AWS performance has not been measured.** Publication stops before creating cloud resources because the dev SSO token has expired and refresh failed. No optimization is adopted.

The [captured C7a job-family analysis](../2026-11-10-task-job-families176/README.md) attributes 99.848% of additional instrumented MSM job publications at 96 versus 32 workers to G2 scalar decomposition. The observer failed its target overhead screen, so those counts do not establish uninstrumented costs or a speedup. The exact minimum-grain idea [already showed no confirmed gain locally](../2026-10-27-g2-scalar-grain/README.md); this preparation targets the previously unmeasured C7a condition with 96 physical cores. No old local timing comparison is repeated.

One reversible edit selects `with_min_len(128)` on the existing in-place `phi_split` iterator through an experimental compile-time cfg. OFF retains the original iterator verbatim. The other 175 original files and all 176 production file hashes are unchanged. Scalar arithmetic, ordering, eligibility, query dependencies, points, buckets, allocation policy and resource forecasts are retained. No runtime experimental selector, extra buffer, lookup table or worker pool is added.

|Build|SDK SHA-256|Native tests|
|:---|:---|:---|
|Fresh OFF|`4ba7aad6d3e71104b971a0c44b9d7d8c5207e627e3e51331702e85adf069c2ef`|127 core, 2 prover, 6 cubic|
|Candidate ON|`c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`|127 core, 2 prover, 6 cubic|

Both builds coherently rebuild eight linked libraries with Rust 1.98.1, znver4 and the accepted manual flags including dense busy reset. The original accepted driver and allocator are retained. Libraries and proc-macro objects are frozen before explicit Rust 1.98.1 SDK linking. Native tests are unfiltered. Field source is exact and its accepted qualification is inherited; this stage claims no new full-field, generic, phone or concurrent-proof qualification.

Three reader models pass before preflight: equal ratios and ties, nonproportional paired ordering, and exact binomial coverage for the declared interval. The independent reader then checks all command argv/environment and exit/stream hashes, worker placement, sample counts, proof/public/image bindings and freshness.

Local container preflight runs the candidate on all nine shapes and the accepted/fresh controls on 1×2, at four workers. Each process makes one measured request after three primes: **44 SDK calls and 11 distinct fresh 164-byte Gnark-verified exports**. The raw samples are functional evidence, not a performance comparison. The verified-export ledger advances **50,208 → 50,219**, SHA-256 `287a840843fcc114fac764bce8f56d74a6c4184288f5a44ee57dab8e16998d11`.

The [target protocol](artifacts/target-protocol.md), consumers and both readers are frozen before the first local SDK request. The prepared final image contains accepted REF, fresh OFF and candidate ON binaries plus the sealed preflight inventory. Planned cloud work is 21 matched blocks at 32/96 workers, 151 requests after three primes per process, with six functional preflights: 19,428 SDK calls and 132 new exports if completed. The proposed final ledger is 50,351; it has not been reached. Candidate96 must improve latency against both controls; calibration, 32-worker nonregression, CPU and RSS checks must also pass. All-shape qualification is still required before adoption.

The final image is built locally and pinned, but is **not published**, and no instance, volume or transfer bucket is created for this experiment. The owned-resource lifecycle helpers are prepared with a unique run ID, automatic termination and credential checks. A new valid dev SSO session is required before publication and launch. Completed builds, tests, preflight and final-image construction must be reused.

An initial outer workflow attempts to nest retained xtask workflows and fails its host lock before any Cargo command. The retained correction runs the two native plans directly; all successful science executes once. Thirty-seven successful command receipts and the one host-lock failure are hash checked. Direct preparation/compiler invocations and the authentication failure are outside that workflow count. No owned build/helper/hash/analysis overlaps any SDK preflight request.

No AWS latency, new Aeglos comparison or minimum-RAM/pressure/ARM/TEE result is supplied by this preparation. The original all-eight verified 10× M5 CPU objective remains active and unmet. Existing [accepted CPU and Metal comparisons](../2026-11-10-accepted-current-c7a176/README.md) remain separate historical context.

[Native review](artifacts/native-review.json), [functional review](artifacts/local-review.json), [source edit](artifacts/source-audit.json), [authentication state](artifacts/auth-block.json), [publication checks](artifacts/publication.json), [sources, receipts and public proofs](artifacts/preparation-sources-receipts-public-proofs.tar.gz), [checksums](SHA256SUMS). The archive round-trips exact member bytes and hashes; SDK executables, compiled libraries, production proving inputs and credentials are excluded. Reproduction requires the pinned toolchain, verifier and private inputs.

Local experiment directory: `/home/ubuntu/src/zopro-perf-work/phi-grain-c7a176`; executables: `sdk-control-znver4` and `sdk-candidate-znver4`.
