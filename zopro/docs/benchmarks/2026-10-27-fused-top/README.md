# Combined K/H top-digit reduction

The combined top-digit candidate does not improve primary32 consistently: paired candidate/accepted is **1.003943** and candidate/fresh is **1.008057**, with one of three block wins against each. The short screen fails. No full stage or cycle profiles follow, and no change is adopted. There is no minimum positive-gain cutoff.

Primary16 wins all three blocks against both controls, with a 472.702 µs paired median saving against fresh. That positive observation is retained, alongside its failed control calibration. At32, subtracting the displayed medians gives a 32.141 µs saving against accepted, but the comparisons paired within each block do not reproduce that improvement. The two summaries answer different questions; the within-block comparisons determine the declared decision.

The final short digit in fused K/H previously reduced two independent sets of digit buckets and combined their weighted sums. This candidate treats the two borrowed point slices as one indexed input, combining equal-digit buckets before the existing affine reductions. It can share inversion batches and duplicate bucket weighting. It changes arithmetic topology rather than launch ordering. The [previous trace](../2026-10-27-arithmetic-leaves/README.md) showed prompt K/H window starts under instrumentation; it did not establish removable service time or predict this candidate's speed.

Only `msm.rs` and `msm_affine/fused_kh.rs` change. The indexed reducer is factored through a typed point getter; the original single-slice caller retains its input and tile geometry. The combined top path allocates one existing kind of four-byte index order over the borrowed inputs. It preserves ordinary fused windows, eligibility, positive final digits, complete curve arithmetic, tile size, field kernels, other query paths and runtime policy. There is no new point table, copied input point array, second execution plan, pool or field primitive. Reversing the recorded edits recovers the original files; the other168 originals are exact.

## Verified SDK latency

Ryzen9950X. Fresh blinded proving, witness construction, self-verification and serialization are timed; loading and primes are excluded. The exact accepted SDK and the pinned baseline from the preceding fusion study are controls. That baseline is reused unchanged with its source/library/consumer pins rechecked; no new baseline build is claimed. The candidate uses the same native flags, fat LTO, one code-generation unit and standard SDK driver.

Times are medians of process medians; ratios and microsecond savings pair within rotated/reversed fresh-process blocks. CPU includes coordination; lifetime RSS includes load and primes and is not minimum RAM.

|Case|Accepted ms|Fresh ms|Combined ms|Combined/accepted|Combined/fresh|Paired saving vs fresh µs|Wins vs fresh|Combined RSS MiB|Combined CPU ms/request|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|1x2-w16|10.376143|10.699604|10.226902|0.964011|0.957588|472.702|3|77.25|128.948|
|1x2-w32|8.641642|8.494304|8.609501|1.003943|1.008057|-111.441|1|94.75|195.063|

The screen uses three blocks, primary16 on CPUs0–15 and primary32 on CPUs0–31, with17 timed requests after3 primes per process. Fresh/accepted must stay within factor1.05 in every block and factor1.03 for paired medians. Primary32 must improve against both controls with at least2/3 block wins, and both cases must stay within2% against both controls. All observations and failures remain included. There are no threshold changes, posthoc selectors or repeated successful captures.

calibration_failures: `["1x2-w16 block0 fresh/accepted=1.0561836587687605","1x2-w16 block2 fresh/accepted=0.936784476415825","1x2-w16 median fresh/accepted=1.03117352950899"]`.

primary_failures: `["1x2-w32 combined/accepted=1.0039433336150987,wins1","1x2-w32 combined/fresh=1.0080568023257952,wins1"]`.

regressions: `[]`.

The full stage and cycle profiles did not run. The [last complete all-shape SDK matrix](../2026-10-26-pairing-x-chain-sdk/README.md) retains every supplied M5 CPU/Metal comparison and remains authoritative outside this screen.

## Correctness, memory and limits

All110 native core tests, two raw tests, six cubic tests, generator and portable/no-IFMA consumer checks pass. The new generator oracle checks independent expected sums and the original separate top reductions for odd K/H lengths, signed and identity bases, zero/one/minus-one/random/mixed/boundary scalars, widths12/13/14 and pools1/4. The existing full fused-query oracle also covers30529+32768 inputs, complete-curve cases and every supported width. Field source and its accepted qualification are inherited unchanged; no new field-suite execution is claimed.

Combining index orders preserves their aggregate four-byte input-entry storage. With the same point tile size, the combined number of tiles cannot exceed the sum of the separate counts. Temporary tree storage and allocator lifetimes still depend on concurrent work; no intrinsic whole-process saving or minimum-RAM guarantee follows from that bound. A passing latency result would still require pressure, resource-policy and compatibility review before adoption.

Candidate preflight covers all nine shapes at four workers: one timed request plus three primes per process,36 SDK requests and nine fresh exports. All measured exports pass independent Gnark verification,164-byte proof length, pinned public binding and freshness against7,233 prior identities plus newly exported proofs. No helper, build, hash, archive or verifier overlaps timed captures.

Total **396 SDK requests**, **27 fresh independently verified exports**, **54 proof/public members**; proof ledger **7,233 → 7260**. All170 production files and the accepted SDK remain unchanged. The original all-eight CPU10× objective remains active and unproven.

No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, memory-pressure, concurrency or throughput qualification is claimed.

reused fresh executable: `/tmp/zopro-poseidon-fused-dot/native/fresh-sdk`; SHA-256 `d847ae9fd2ae52c6c74172a95004780eaa5481ca2a5f8889d7c9d0063dcd1eaa`.

combined executable: `/tmp/zopro-fused-top/native/combined-sdk`; SHA-256 `fbcca12d88a2e606e99866111ef070921edb38087f06fb1c1988179e19a465fd`.

[Design](artifacts/design.md), [source audit](artifacts/implementation-audit.json), [edits](artifacts/edits.json), [screen analysis](artifacts/screen-analysis.json), [native tests](artifacts/native/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
