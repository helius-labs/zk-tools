# Direct signed CIOS product difference

The arithmetic component gate **fails**. Production remains unchanged. The private helper subtracts raw product halves inside a signed Montgomery accumulator, then adds p only if the result is negative. It avoids the original carry-dependent operand negation and positive fused-result correction, while adding per-half vector subtraction and signed carries. No key table, heap buffer, persistent cache or point copy is added.

The affine expression improves **7.38% versus fresh ordinary arithmetic**, winning all 31 blocks. Its aggregate also improves, but dependent-chain and scaled-product regressions fail three nonregression checks. This mixed result does not establish a general field replacement or a full-proof speedup.

## Component measurements

Ryzen 9 9950X, one thread pinned to CPU 2. Thirty-one fixed rotated/reversed blocks compare an ordinary executable frozen before the private includes, fresh ordinary arithmetic and the candidate in one executable. Each process measures 100,000 iterations after three 2,000-iteration primes. Input packing and independent scalar output checks are outside the clock; four distinct initial vectors prevent independent-stream CSE.

Units are ns per complex field element, additionally divided by four for streams4. These are arithmetic expressions, not full curve/MSM/SDK proofs. The affine expression includes denominator/numerator, real scaling, square and coordinate operations with arbitrary fields and a supplied factor; inversion, bucket construction and scheduling are outside its scope.

|Expression|Frozen ns|Fresh ns|Candidate ns|Paired change versus fresh|Wins out of 31|
|---|---:|---:|---:|---:|---:|
|chain|6.426843|6.437300|6.622786|+2.887%|0|
|streams4|6.291021|6.294186|6.255185|-0.634%|31|
|scaled|9.941920|9.932253|10.314085|+3.807%|0|
|affine|24.317663|24.281746|22.508827|-7.378%|31|

|Aggregate comparison|Median ratio|Bounds|Wins out of 31|
|---|---:|---|---:|
|candidate/frozen|0.996063041|[0.995173163, 0.996418380]|31|
|candidate/fresh|0.995466094|[0.995083779, 0.996310001]|31|

The prospective gate requires affine and equal-log-weight aggregate upper ratios below one with at least 21 wins against both controls. Every candidate/context upper ratio must be ≤1.03; all fresh/frozen intervals must fit [1/1.03, 1.03]. Ranks 10–22 give nominal 97.055% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. There is no minimum gain floor, exclusion, extension, successful rerun, threshold change or subset promotion.

Failed checks:

- calibration_failures: `[]`
- nonregression_failures: `["chain/frozen","scaled/frozen","scaled/fresh"]`
- gain_failures: `[]`

## Arithmetic and correctness

For stored a,b,c,d≤2p and 4p<R, Z=ab−cd satisfies −4p²≤Z≤4p². With q=(−Z/p) mod R, T=(Z+qp)/R lies strictly between −p and 2p. Adding p only for negative T gives [0,2p). Low quotient digits depend on low bits even for negative coefficients; arithmetic shifts preserve signed carries and mixed 52/48-bit packing. A conservative signed-coefficient envelope stays below 2^61. [Bounds](artifacts/bounds.md).

This is distinct from the earlier [complement/seed/correction](../2026-10-29-complement-difference175/README.md) and [shared Gauss](../2026-10-25-shared-complex-products/README.md) designs. It has no added Rd term or shared three-product inter-output state. It retains four raw products and two reductions per complex multiplication. Fewer corrections do not establish lower total instruction cost or faster proofs.

The independent BigInt model passes **30,084 exact cases** across Fq/Fr and R256/R260: 7^4 boundary tuples, 4,096 full-width samples and 1,024 changing-chain cases per configuration. It checks quotient equality, signed CIOS state, repacking, final correction and exact representative, including negative results and inclusive 2p inputs. The model records absolute coefficient maxima both before and after row shifting/repacking.

Two native tests pass **172,544 changing-chain lane comparisons against Arkworks**, checking canonical values and normalized output below 2p for both fields/radices. They reuse the earlier test geometry for this new implementation; the 33 original field tests are filtered out and inherited after source checks. Only private helper/test includes alter one of the 17 field files; removing the suffix recovers every original byte. All 175 production files remain exact.

The initial model build failed because a standalone tool named build.rs was auto-discovered as a Cargo build script. Renaming it prepare-build.rs fixes the harness without changing arithmetic. The initial inputs and failure are archived; no successful model, test or timing is repeated. Phase 1 freezes ordinary arithmetic before helper installation; phase 2 tests and builds the private method.

Publication review preserves the original trailing blank line in native-tests.log. The source archive prefixes field files with `bn254/`, keeping its Cargo.toml distinct from the harness Cargo.toml. This publication repair changes no runtime source, test, timing or gate. [Review](artifacts/publication-review.json).

## Automatic CPU profiles

All four preflights and 372 latency processes finish before four FIFO-gated 199 Hz cycles:u profiles, chain/affine fresh/candidate, one million iterations each. Setup/primes and the scalar oracle are outside the enabled region. CPU IDs and timestamps are recorded explicitly. All captures finish before owner-user decoding and lost-record checks. No owned build/helper/hash/archive/other CPU benchmark overlaps capture.

|Profile|Samples|Lost records|
|---|---:|---:|
|affine-candidate|35|0|
|affine-fresh|37|0|
|chain-candidate|8|0|
|chain-fresh|9|0|

Each profile has one sampled process and zero lost records. Raw recordings and full disassembly remain local with hashes. Flat sampled periods and static instruction sizes do not establish exclusive stalls or a causal cost breakdown. All 380 arithmetic-capture processes check 32 canonical complex outputs. Chain/scaled/affine repeat eight final states across slots; streams4 has four distinct states. Corresponding output digests agree.

## Research and scope

[DigitsOnTurbo](https://arxiv.org/html/2604.21566v1) separates independent arithmetic from carry management. Its normalization adaptation was already tested in the [24 October component](artifacts/history-audit.json) and is not repeated. [Eagen et al.](https://eprint.iacr.org/2025/933) optimize scalar multiplication inside proving circuits; that abstract does not establish a native-kernel speedup for these fixed keys. The [PSSM](https://www.mdpi.com/2079-9292/15/16/3660) single-scalar scheduling abstract was also reviewed, while fetching the full publisher page returned HTTP 429. No upstream implementation or reported speedup is copied into this candidate.

All 392 capture/offline steps complete. The archive preserves source, protocols, original command/exit/stdout/stderr bytes, field checks and all arithmetic observations. Four expanded loss streams remain local with hashes. Zero SDK proofs, independent exports or Gnark verifications are added; the canonical proof ledger stays **21,259**. No peak-prover RSS, minimum RAM, mobile/ARM/TEE, forester 250/4999, memory-pressure/swap, concurrent-throughput or AWS/Aeglos result follows from this component.

## Accepted CPU and Metal comparisons

These are inherited accepted local timings against supplied historical M5 references, not candidate proof measurements. Accepted primary peak RSS remains 134 MiB. The accepted full performance gate remains false because of its documented physical-16 calibration exception; its separate final-artifact gate is true.

|Shape|Accepted local ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The original all-eight verified CPU 10× objective remains active and unmet, 0/8. [Current local/AWS summary](../2026-10-29-current-cpu-summary/README.md).

## Decision

The generic four-workload qualification fails and is not relabeled. No general field replacement is adopted. The affine-expression gain selects a separately declared actual-G2-batch qualification, including inversion, packing, tails and threaded execution. That is a new caller experiment; it must retain the chain/scaled regressions and this failed gate. No unchanged component reroll or subset of these timings is promoted as passing.

[Protocol](artifacts/declaration.json), [all results](artifacts/analysis.json), [model](artifacts/model.json), [kernel](artifacts/kernel.rs), [native checks](artifacts/native-tests.log), [closing pins](artifacts/pins.json), [source](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz).
