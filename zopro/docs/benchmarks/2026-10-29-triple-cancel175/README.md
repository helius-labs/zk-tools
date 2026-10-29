# Three-row Montgomery quotient prefix

**Rejected at the component gate; no SDK integration or adoption.** The intended Fq/R256 aggregate is 0.795% slower than fresh ordinary CIOS, with 0/31 winning blocks. Control calibration and all nonregression checks pass, but neither aggregate gain check passes. Production remains unchanged.

## Change

The private eight-lane kernel forms three input product rows before cancellation, then computes three low quotient digits using a 156-bit negative inverse. Their calculation includes the carry from the second inverse-product coefficient. Once those digits are known, the original cancelled-low identity applies them at three offsets. The original last two CIOS rows, mixed 48-bit packing, final normalization and fused correction remain unchanged. [Algebra and bounds](artifacts/bounds.md).

This differs from the [two-digit prefix](../2026-10-28-paired-cancel/README.md), the earlier [full-width truncated quotient](../2026-10-24-truncated-redc-component/README.md), and [FIPS scanning](../2026-10-29-fq-fips175/README.md). Those results remain separate; this is not a rerun of their implementations. The template uses 106/156 ordinary/fused IFMAs, versus accepted 100/150 and paired 104/154. It has eight temporary coefficients in the prefix and returns to six for the remaining rows. Two extra inverse words cost 16 logical constant bytes per field; there is no heap, key table, persistent cache, new representation or copied point array.

## Fixed component measurements

Ryzen 9950X, one thread pinned to CPU 2. Thirty-one fixed rotated/reversed blocks compare frozen ordinary code built before the private includes, fresh ordinary and the new prefix in one fresh executable. Each process runs three 2,000-iteration primes and 100,000 measured iterations. Conversions and scalar output checks are outside the timer. No owned build/helper/hash/archive/second workload overlaps captures.

Units are ns per field-lane update, additionally divided by four for independent streams. Paired change is the median within-block new/fresh ratio; positive means slower, and need not equal a ratio of displayed medians.

|Case|Frozen|Fresh|Three-row prefix|Paired change|Wins/fresh|
|---|---:|---:|---:|---:|---:|
|Fq-R256-chain|2.964875|2.967118|2.964024|+0.025%|15/31|
|Fq-R256-streams4|2.257715|2.261328|2.309262|+2.107%|0/31|
|Fq-R256-fused|3.183525|3.178843|3.189987|+0.392%|6/31|
|Fq-R260-chain|2.833879|2.841606|2.826928|-0.422%|25/31|
|Fq-R260-streams4|1.940573|1.944800|1.931281|-0.575%|29/31|
|Fq-R260-fused|2.955559|2.948082|2.989748|+1.409%|0/31|
|Fr-R256-chain|2.965702|2.960029|2.966893|+0.217%|13/31|
|Fr-R256-streams4|2.257499|2.260454|2.306507|+2.136%|0/31|
|Fr-R256-fused|3.175549|3.176725|3.189474|+0.424%|8/31|
|Fr-R260-chain|2.839164|2.837936|2.826991|-0.406%|25/31|
|Fr-R260-streams4|1.939687|1.944083|1.934305|-0.544%|25/31|
|Fr-R260-fused|2.954242|2.946090|2.983674|+1.430%|0/31|

The Fq/R256 aggregate is 1.007946 [1.006634, 1.009716] versus fresh and 1.007801 [1.006309, 1.009604] versus frozen, both 0/31 wins. Ordinary mixed chains span parity, independent streams regress about 2.1%, and fused chains regress about 0.39% versus fresh.

Positive R260 estimates are retained. Fq/R260 streams improve 0.575% versus fresh [ratio0.990039–0.996290,29/31 wins] and 0.535% versus frozen [0.993310–0.998938,25/31]. Fr/R260 chains improve 0.406% versus fresh [0.994781–0.997780,25/31] and 0.326% versus frozen [0.993225–0.999621,22/31]. These are context-specific component observations, not a gain in the intended mixed-R256 arithmetic or complete prover. No subset is promoted after seeing results.

The predeclared gate requires the three Fq/R256 patterns' geometric aggregate upper ratio below 1 and at least 21/31 wins against both controls, all 12 cases' upper nonregression bounds≤1.03 against both controls, and each fresh/frozen calibration within factor 1.03. There is no minimum gain floor. Individual order ranks 10–22 of 31 have nominal 97.055% conditional coverage under exchangeable independent blocks, without a familywise guarantee. No observations are excluded, extended or rerun, and no threshold/flag/layout rescue is selected.

## Correctness and linked code

The exact interpreter passes **60,168** full reductions across Fq/Fr, R256/R260, ordinary/fused, inclusive 2p corners, fixed random tuples and changing chains. Every three-digit quotient and post-prefix integer matches an independent BigUint modular-inverse calculation; mixed final packing and full exact REDC representatives also match. All modeled coefficients stay below2^61. The unexecuted initial prefix-only draft remains labeled separately.

Two new native tests pass **345,088** field-lane comparisons against Arkworks, plus exact lazy-digit equality against ordinary CIOS. Compile-time bit-lifted inverse constants match the independently generated three-word fixtures. Native tests cover both primes/radices, ordinary/fused products, lazy boundaries and eight-step chains. The 33 unchanged field tests are filtered out and inherited. An initial missing macro brace prevented native test and fresh-probe compilation; the failed sources/logs remain. The one-character wrapper repair preceded all successful native execution and timing.

All 1,116 latency processes, 12 preflights and four profiles check 32 returned canonical fields against the independent scalar affine-power formula after timing. Chain/fused duplicate eight final states into four slots, not 32 independent chains. All corresponding digests agree. There is no timed heap allocation in these fixed-array arithmetic loops.

Actual linked Fq/R256 loops confirm the expected IFMAs:

|Loop|Fresh/new IFMA sites|Fresh/new instruction sites|Fresh/new RSP operand lines|Fresh/new literal stack reservation bytes|
|---|---:|---:|---:|---:|
|Dependent chain|100/106|307/331|42/58|384/704|
|Fused chain|150/156|344/407|44/84|448/1536|

These are static whole-function observations, not dynamic instructions, measured spills or process RAM. The longer live prefix comes with larger generated frames and more stack-reference sites. It does not establish an exclusive cause for the measured differences. [Hot-code audit](artifacts/hot-code.json) filters literal scalar stack reservations; the original broad `stack_subtractions` observer field also included vector subtracts.

## Profiles and provenance

Four separate whole-process cycles:u recordings follow all latency, at 199 Hz for 30,000,000 iterations (Fq/R256 chain/fused, fresh/new). They include setup, primes and output checks. All recordings finish before decoding.

|Profile|Samples|Lost|
|---|---:|---:|
|chain-fresh|149|0|
|chain-triple|150|0|
|fused-fresh|159|0|
|fused-triple|161|0|

All 619 samples belong to one PID per recording with zero lost records; matching arithmetic loops lead sampled period. Small profile counts are descriptive, not an exclusive stall diagnosis or removable proof-time estimate. Original perf recordings and expanded loss dumps remain local with hashes.

All 1,144 workflow steps complete and 289 pins close. Removing two includes from the private eight-lane module recovers all 17 original field files; the other 16 are exact. All 175 production sources remain unchanged. Original CIOS bodies and probe/oracle source remain exact; the appended helper replaces only the prefix of its separately named method. No core/prover rebuild or new verifier run occurs. Archive review reconstructs all 4,572 encoded streams byte-for-byte. Three whitespace findings are trailing blank lines in two original disassembly excerpts and one native test log; these bytes remain unchanged. Authored Rust source has no whitespace findings.

[All observed results](artifacts/observations.json) and [encoded run receipts](artifacts/run-receipts.json) retain original command/exit/stdout/stderr text with SHA256. Initial failed-build sources and logs are retained. Zero SDK requests or proof exports occur; independent proof ledger remains **17,811**. No accepted latency/RSS, AWS/Aeglos, mobile/ARM/TEE, huge-circuit, pressure/minRAM, concurrency or throughput result follows.

## Inherited CPU and Metal comparison

Accepted local timings against supplied historical M5 references; not newly measured here. Primary peak RSS remains 134 MiB. The prior full-performance-gate failure and 16-worker baseline-calibration engineering exception remain.

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

All-eight CPU 10× goal remains active/unmet (0/8). Accepted 54→1 remains 162.250464 ms without a supplied M5 reference.

[Design](artifacts/design.md), [bounds](artifacts/bounds.md), [history](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [source/tool pins](artifacts/pins.json), [artifact hashes](artifact-index.json).
