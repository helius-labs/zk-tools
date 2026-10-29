# G2 fused product components minus canonical anchors

**Rejected at the arithmetic-component gate; no bucket-caller or SDK integration.** The affine expression is 6.653% slower against fresh ordinary arithmetic. The four-workload aggregate is 17.496% slower, with 0/31 winning blocks. Every workload regresses; control calibration passes. Production remains unchanged.

## Arithmetic and scope

The current G2 reverse pass computes a complex product, then subtracts a canonical Y anchor. The private native eight-lane helper preserves the fused CIOS product/cancellation/mixed-radix prefix. It folds `p − anchor` into the final signed carry pass of each real/imaginary component, then uses the original conditional 2p correction. With raw fused result T<3p and anchor<p, the new total satisfies 0<T+p−anchor<4p, so one correction restores normalized output below 2p. Canonical values match; lazy representatives may differ. [Bounds](artifacts/bounds.md) specify the native mixed-R256 preconditions.

The earlier [G1 signed-final-subtraction study](../2026-10-28-signed-final-sub174/README.md) and [failed SDK trial](../2026-10-28-signed-final-sub-sdk174/README.md) remain separate. This tests the two-product G2 range, bias and correction rather than repeating their ordinary-product helper. No new representation, key table, buffer, persistent cache, point copy or heap allocation is added. No source-level operation saving is treated as a speed prediction.

## Fixed component results

Ryzen 9950X, one thread pinned to CPU 2. Thirty-one fixed rotated/reversed blocks compare frozen ordinary code built before the includes, fresh ordinary and candidate methods sharing one new executable. Each process runs three 2,000-iteration primes and 100,000 measured iterations. Input/output conversion and independent scalar checking are outside the timer. No owned build/helper/hash/archive/second workload overlaps capture.

The affine workload spells out the current native G2 denominator, numerator, real scaling, square and coordinate expressions, changing state after each iteration. It uses arbitrary fields and a supplied factor: it omits inversion, batch construction, bucket collisions and scheduling, and is **not a valid-curve loop or actual borrowed-G2 caller**. Product-sub chains and streams use canonical constant anchors.

Values are ns per complex field element, additionally divided by four for independent streams. Percentage change is the median paired per-block candidate/fresh ratio; positive means slower.

|Workload|Frozen|Fresh|Candidate|Paired change|Wins/fresh|
|---|---:|---:|---:|---:|---:|
|chain|6.435586|6.444980|8.233149|+27.794%|0/31|
|streams4|6.303635|6.304859|8.021384|+27.241%|0/31|
|scaled|9.953551|9.954616|10.941651|+9.942%|0/31|
|affine|24.365979|24.341020|25.957076|+6.653%|0/31|

Candidate/fresh aggregate is 1.174963 [1.174181,1.175648], 0/31 wins; candidate/frozen is 1.175141 [1.174821,1.175606],0/31. Affine/fresh is 1.066529 [1.065332,1.068204]. All eight candidate/control nonregression checks and all four gain checks fail. The declared requirement was improving affine and equal-weight aggregate upper bounds with at least 21/31 wins against both controls, every context upper≤1.03, and each fresh/frozen calibration within factor 1.03. The declaration's `all12caseupper` count label was mistaken; the pre-capture design and executable distinguish eight nonregression and four calibration checks. No gate changed.

Intervals use ranks 10–22 of 31, nominal 97.055% conditional on exchangeable independent blocks, without a familywise guarantee. All 372 latency observations and four preflights remain. No exclusions, sample extensions, threshold changes or successful reruns occur. This exact recipe is closed without a subset or layout rescue.

## Correctness and profiles

Two new field tests pass **172,544** changing-chain lane comparisons against Arkworks across Fq/Fr, fused sums/differences, lazy and inclusive 2p multiplicands, canonical anchors and normalized output bounds. The 33 unchanged native field tests are filtered out and inherited. The independent BigUint model passes **21,568** exact mixed-R256 fused REDC cases and **8,324** wide unnormalized signed carry cases. An initial ambiguous shift type was repaired before its first successful execution; the initial source/log remain.

Every process checks 32 canonical Fq2 outputs outside timing. The first three workloads use a scalar closed-form affine-power oracle; affine uses repeated independent scalar Fq2 math. Chain/scaled/affine duplicate 8 final states into four slots, so 32 outputs do not mean 32 independent states. All corresponding final digests agree.

Four whole-process cycles:u profiles follow all latency: chain/affine fresh/candidate, 1,000,000 iterations at 199 Hz. They include setup/primes/scalar verification; every profile result passes its oracle.

|Profile|Samples|Vector-loop period fraction|Lost|
|---|---:|---:|---:|
|affine-candidate|890|4.600%|0|
|affine-fresh|892|4.368%|0|
|chain-candidate|21|99.960%|0|
|chain-fresh|16|99.919%|0|

Affine profiles are dominated by the intentionally untimed scalar oracle in `run()`. Their total samples cannot be attributed to the vector arithmetic. Short chain recordings are descriptive and not resampled. All four profiles have one PID and zero lost records. [Hot code](artifacts/hot-code.json) and selected linked disassembly preserve both paths; whole-function instruction/stack sites include alternatives and cold corrections, not dynamic costs or measured spills. No exclusive dependency, branch, cache or spill cause is established.

## Provenance and retained evidence

All 392 workflow steps complete; 288 source/tool/library/binary pins close. The 175 production source hashes remain exact. Removing two include lines from the private eight-lane field module recovers all 17 original field files; the other 16 stay exact. The private methods are qualified only for the mixed-R256 fused call and canonical anchor preconditions, not as a public generic dispatch. No original prover/core suite or verifier is repeated. Archive review reconstructs all 1,564 encoded streams byte-for-byte. Three whitespace findings are trailing blank lines in two original disassembly excerpts and one native test log; these bytes remain unchanged. Authored Rust source has no whitespace findings.

[All observations](artifacts/observations.json) retain every result. [Run receipts](artifacts/run-receipts.json) encode original command/exit/stdout/stderr bytes as UTF-8 text with SHA256, including zero-length streams; they can be reconstructed exactly. Four expanded perf loss dumps and original perf recordings remain local with hashes, keeping raw padding-heavy dumps out of Git. Raw source/build failures are retained.

Zero SDK requests, proof exports or new Gnark verifications occur; ledger remains **17,811**. There is no new accepted proving-time/RSS result, AWS/Aeglos comparison, mobile/ARM/TEE, huge-circuit, pressure/minRAM, concurrency or throughput qualification.

## Inherited CPU and Metal comparison

Accepted local timings against supplied historical M5 references, not measured by this component. Accepted primary peak RSS remains 134 MiB. The prior full-performance-gate failure and 16-worker calibration engineering exception remain.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

All-eight CPU 10× goal remains active/unmet (0/8); accepted 54→1 remains 162.250464 ms without a supplied M5 reference.

[Design](artifacts/design.md), [declaration](artifacts/declaration.json), [model](artifacts/model.json), [kernel](artifacts/kernel.rs), [analysis](artifacts/analysis.json), [review](artifacts/postcapture-review.json), [pins](artifacts/pins.json), [artifact hashes](artifact-index.json).
