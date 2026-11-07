# Exact on-demand FFT factor preparation

The new quotient-preparation algorithm passes **87,060 exact integer cases**, including both BN254 primes, both correction branches, and an exhaustive small-radix population. This qualifies the integer calculation; **native SIMD, complete-column latency and proof speed are not yet measured**. Production remains unchanged.

The accepted 32-point FFT column performs 80 butterfly multiplications. Its gap-one stage reuses one root 16 times. The proposed implementation prepares that root on demand, uses the existing fixed-factor product for those 16 butterflies, then discards the preparation. It adds no per-key table or retained cache. The proposed prepared payload is 640 bytes plus fixed arithmetic temporaries; the reciprocal constant is 48 bytes per prime. These are size calculations, not measured stack or RAM peaks.

For radix `B=2^52`, `R=B^5` and canonical factor `b<p`, precompute the prime-only constant `mu=floor(R²/p)`. A truncated high product estimates `floor(bR/p)` from below by at most one. A low product computes `D=(-q0*p) mod R`. Since `0<=D<2p<R`, one conditional subtraction and quotient increment recover the **exact quotient and remainder**. The [design](artifacts/design.md) gives the omitted-product and reciprocal-error bounds. The candidate uses machine-word digits and products; independent arbitrary-precision division and remainder provide its oracle.

|Population|Distinct operands|No correction / one correction|
|---|---:|---:|
|Base 16, modulus 65537, exhaustive|65,537|61,937 / 3,600|
|BN254 Fq|10,752|10,740 / 12|
|BN254 Fr|10,771|10,770 / 1|

Full-width cases include prime, limb and power boundaries, quotient-transition inputs and deterministic random operands. All outputs have normalized digits and exact quotients/remainders. The initial compile failed on an unnecessary borrow in an oracle assertion; removing that borrow leaves the candidate algorithm and population unchanged. Original source and failure metadata are retained. The corrected build and model run succeed once.

This differs from the rejected [prepared-leaf cache](../2026-10-24-fixed-leaf-fft/README.md) and [partial-Montgomery column](../2026-10-24-half-root-fft/README.md) implementations. The former stored factors in the FFT plan; the latter used a different preparation/product identity at several reused stages. Neither is rerun. The old full-width fixed-factor constructor required 260 scalar division steps per factor. This new constructor has a literal source proxy of 69 IFMAs, excluding canonical root decoding.

With conservative decoding charged as one ordinary product, the proposed complete-column proxy is **8,238 versus 8,400 IFMAs**, about 1.93% fewer. Added carry/correction work, decoding, register pressure, spills and scheduling can erase that saving. This is a source-operation count, not emitted instructions or a latency prediction. No positive-gain floor is imposed.

Review verifies 189 pins, two successful workflow receipts, all 176 unchanged production sources, exact reciprocal constants, six retained examples, source reuse counts and the work ledger. It does not replay the successful model or old native field/FFT suites. The next stage is native preparation and a matched complete-column caller with preparation, decoding and output handling inside the clock, both directions/layouts, small stacks, 1/16/32 workers and automatic profiles. Full SDK, all-nine correctness, fresh Gnark and resource/fallback qualification remain required before adoption.

Accepted proof timings and the 33,449-proof ledger are unchanged. [Current all-shape CPU/Metal comparisons](../2026-11-07-sign-mask-allshape176/README.md) retain the original verified CPU 10× goal: active and unmet, **0/8**. No new proof, AWS, mobile, memory-pressure or minimum-RAM result is claimed.

[Model source](artifacts/model.rs), [results](artifacts/model-result.json), [review](artifacts/review.json), [design](artifacts/design.md), [source and receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
