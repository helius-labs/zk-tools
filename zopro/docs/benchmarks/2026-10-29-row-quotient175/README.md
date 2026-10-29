# Row-local Montgomery quotient preparation

The intended Fq/R256 three-context aggregate improves **0.912% against fresh control**, ratio 0.990875659, interval [0.989586727, 0.994122613], **31/31 wins**. The full component gate is **false**: both fields' R260 four-stream nonregression checks fail against both controls. No blanket replacement, SDK integration or production adoption is selected.

## Arithmetic

At each CIOS row, distributivity rewrites the cancellation digit using the old low coefficient and precomputed current-operation input contributions. With B=2^52 and I=−p⁻¹ modB, prepare a0I and c0I, then five values w_i=lo(a0I·b_i)+lo(c0I·d_i) in fused mode. The row digit is lo(I·t_old0)+w_i modulo B (or 2^48 in the final mixed row). It no longer waits for the current raw product row. It still depends on the preceding row's state.

The original input products, cancelled-low carry, modulus rows, final packing, normalization and fused correction are unchanged. This uses only the existing one-word inverse, unlike the [paired](../2026-10-28-paired-cancel/README.md) and [three-row](../2026-10-29-triple-cancel175/README.md) quotient-prefix methods. Ordinary/fused templates use **105/161 IFMAs versus 100/150**, including per-operation preparation and the zero-state first-row simplification. Five additional row vectors and prepared low operands can increase register pressure. There is no key-dependent table, retained cache, heap buffer, point copy or representation change. [Derivation and bounds](artifacts/bounds.md).

The primary indexed excerpt of [the NTRU Prime AVX2 technical report](https://eprint.iacr.org/2023/604.pdf), section 2.1, supplies background for placing an inverse-mod-word factor on a known operand. The direct PDF fetch returned 403 and the full paper was not read. This is an independently derived multirow 52/48-bit adaptation; no upstream code or paper speedup is adopted. The [history audit](artifacts/history-audit.md) records overlapping MSM ideas already tested elsewhere.

## Fixed component results

Ryzen 9950X, one thread on CPU 2. Thirty-one fixed rotated/reversed blocks compare ordinary frozen code built before the includes, fresh ordinary and the candidate sharing one executable. Each process primes three times for 2,000 iterations and measures 100,000 iterations. Input/output conversions and independent scalar checks are outside the timer. Per-operation preparation is in the source call; invariant work in these fixed-operand patterns may be hoisted by the compiler. This is not an actual bucket caller or full SDK benchmark. No owned build/helper/hash/archive/second workload overlaps capture.

Values are nanoseconds per field-lane update, divided by four again for independent streams. Paired changes use within-block ratios and need not equal ratios of displayed medians. Positive means slower.

|Case|Frozen|Fresh|Candidate|Paired change vs fresh|Wins/fresh|
|---|---:|---:|---:|---:|---:|
|Fq-R256-chain|2.966092|2.969210|2.964163|-0.123%|16/31|
|Fq-R256-streams4|2.262346|2.265226|2.297713|+1.385%|0/31|
|Fq-R256-fused|3.181373|3.177265|3.058966|-3.773%|31/31|
|Fq-R260-chain|2.837235|2.841256|2.848006|+0.212%|10/31|
|Fq-R260-streams4|1.942678|1.946425|2.000744|+2.880%|0/31|
|Fq-R260-fused|2.953105|2.946754|2.885452|-2.008%|31/31|
|Fr-R256-chain|2.970111|2.972654|2.971827|-0.235%|16/31|
|Fr-R256-streams4|2.260452|2.264403|2.298549|+1.411%|0/31|
|Fr-R256-fused|3.191766|3.181847|3.054946|-3.977%|31/31|
|Fr-R260-chain|2.841681|2.842696|2.847041|+0.133%|11/31|
|Fr-R260-streams4|1.941626|1.946140|2.000903|+2.922%|0/31|
|Fr-R260-fused|2.951715|2.946767|2.882571|-2.063%|31/31|

All four fused contexts improve against both controls with 31/31 wins. Fq/R256 fused improves 3.773% against fresh, while Fq/R256 four-stream ordinary arithmetic regresses 1.385%, also visible rather than discarded. Both R260 four-stream cases lose all blocks; their upper bounds exceed 1.03. The aggregate gain does not override those declared failures.

|Fq/R256 aggregate against|Median|Interval|Wins|
|---|---:|---|---:|
|frozen|0.991299379|[0.988858041, 0.993999851]|31/31|
|fresh|0.990875659|[0.989586727, 0.994122613]|31/31|

Calibration failures: `[]`. Nonregression failures: `["Fq-R260-streams4/frozen","Fq-R260-streams4/fresh","Fr-R260-streams4/frozen","Fr-R260-streams4/fresh"]`. Both aggregate gain checks pass. The full gate requires all of these checks; it remains false.

The fixed gate requires each fresh/frozen interval within factor 1.03, candidate upper≤1.03 against both controls in all twelve cases, and the intended Fq/R256 three-case geometric aggregate upper<1 with at least 21/31 wins against both. Individual ranks 10–22 of 31 have nominal 97.055% conditional coverage under exchangeable independent blocks, without familywise guarantees. No gain floor, exclusions, extensions, successful reruns, threshold changes, flag/geometry sweeps or subset promotion occur.

## Validation and profiles

The exact BigUint model passes **60,168 products and 300,840 cancellation-digit comparisons** across both primes and radices, ordinary/fused products, inclusive2p edges, full-width random tuples and changing chains. Each raw input-row integer and cancellation digit agrees; exact final REDC and lazy correction agree. A separate **16,412-case redundant-coefficient audit** validates cancelled-low carry and postrow52/mixed48 packing identities. It uses different cases and does not repeat the successful full model. All new quotient-only partials stay below3B; the copied coefficient envelope stays below2^61.

Two new native tests pass **345,088 Arkworks lane comparisons plus exact lazy-digit equality** to ordinary CIOS, including eight-step chains, both primes/radices and inclusive2p values. These reuse the earlier oracle case families on the new kernel; they are not new distinct mathematical vectors. The 33 unchanged native field tests are filtered and inherited. All 17 original field files recover exactly after removing the two includes; 16 are byte-exact throughout. All 175 production sources and accepted SDK20fde remain exact. No native build, model, test or successful capture fails or is repeated in this experiment.

Every one of the 1,116 latency processes,12 preflights and four profiles independently checks 32 canonical outputs outside its timer. Chain/fused duplicate eight final states into four slots; only streams4 has 32 independent final states. All corresponding digests agree. There is no timed heap allocation in these fixed-array loops.

Four whole-process 199 Hz cycles:u profiles follow all latency, 30 million iterations each (Fq/R256 chain/fused, fresh/candidate). Setup, primes and scalar checking are included in those recordings. All profiles finish before ownership changes and decoding; perf decodes as the new file owner. No privileged-input ownership failure or recapture occurs.

|Profile|Samples|Vector-loop period fraction|Lost|
|---|---:|---:|---:|
|chain-fresh|150|100.00%|0|
|chain-rowq|150|100.00%|0|
|fused-fresh|160|100.00%|0|
|fused-rowq|159|100.00%|0|

Total 619 samples, one PID each, zero lost. Rust v0 symbols are demangled before loop attribution. Small counts are descriptive, not an exclusive stall cause or removable proof-time estimate. Whole-function IFMA/RSP sites include setup and cold paths; stack operands are not measured spills or peak RSS. The analysis field static_code.instructions counts raw disassembly lines, including wrapped byte-only lines, and is not an instruction count. [Linked code](artifacts/hot-code.json) and selected disassembly are retained.

All 1,144 workflow steps complete; 291 closing pins pass. The compact archive encodes 1,144 command receipts and 4,572 original command/exit/stdout/stderr streams (four expanded loss stdout dumps remain local with hashes), 1,132 RESULT observations and 17 private field-source members. Every encoded member and compressed stream is checked byte-for-byte. ELF/library artifacts and raw perf data remain local with SHA identities. Zero SDK requests or new proof exports occur; the independent proof ledger remains **18,076**. No accepted latency/RSS, AWS/Aeglos, mobile/ARM/TEE, huge-circuit, pressure/minRAM, concurrency or throughput result follows.

Publication review trims trailing whitespace in derived disassembly displays; originals remain local with recorded hashes. One trailing blank line in the raw native test log is retained. Authored Rust has no whitespace findings.

## Inherited CPU and Metal comparison

Accepted local measurements against supplied historical M5 references; not newly measured here. Primary peak remains 134 MiB. The original full-performance-gate failure and physical16 calibration engineering exception remain.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

The all-eight CPU 10× goal remains active and unmet (0/8). Accepted 54→1 remains 162.250464 ms without a supplied M5 reference.

[Design](artifacts/design.md), [bounds](artifacts/bounds.md), [full results](artifacts/analysis.json), [observations](artifacts/observations.json), [source/tool pins](artifacts/pins.json), [review](artifacts/postcapture-review.json).
