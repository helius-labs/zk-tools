# Trace-one quadratic-extension basis arithmetic screen

29 October 2026. The separately declared component gate **fails**. Production remains unchanged. This tests a different Fq2 polynomial basis, including multiplication, squaring, norm and a complete affine-expression recurrence. It is an arithmetic-only screen: conversion is outside the clock, so results are an optimistic bound for integration, not proof latency. No key-format or curve implementation is changed.

Multiplication improves **6.29% in a dependent chain** and **9.56% across four streams** versus fresh ordinary arithmetic. The square recurrence is **103.70% slower**, norm recurrence 13.48% slower and norm-affine 24.10% slower. The six-context aggregate is **15.02% slower**, with 0/31 winning blocks. All control calibration checks pass. The extra operations elsewhere outweigh the multiplication gain in this fixed recipe.

## Arithmetic and memory

For BN254 Fq, choose c²=−5 and j=(1+c·i)/2, where i²=−1. Then j²=j+1. Encoding x+y·i produces (x−y/c, 2y/c); decoding (a,b) produces (a+b/2, cb/2). Exact BigInt models verify roundtrip, products, squares and norms. This extension-basis change differs from the earlier positive-Gauss multiplication: it removes two operand preparations by changing the polynomial relation. The earlier Gauss caller regression remains in its own record.

Multiplication forms S=a+b, D=c+2p−d and shared P=S·d, then X=P+a·D and Y=P+b·c. For inclusive normalized inputs [0,2p], raw S,D≤4p and X,Y≤12p². Three raw products and two unsigned CIOS reductions require 250 IFMA sites instead of ordinary complex multiplication's 300. Raw temporaries are not exposed as normalized public fields.

Squaring includes its additional cost: P=a(a−2b), Q=(a+b)(a+3b), v=(Q−P)/3, result (P+v,v). Division operates directly on a Montgomery representative z by choosing k∈{0,1,2} with z+kp divisible by three and performing exact radix-52 digit division. Norm is a(a+b)−b²; conjugation is (a+b,−b). No alternative square recipe is selected after measurement.

Coefficient storage remains two field values; the private vector kernels add no heap buffer or per-key lookup table. Basis conversion, curve constants, serialization, portable fallbacks and key loading would need separate qualification before integration. There is no process-RSS, minimum-RAM or memory-pressure measurement here.

## Method and results

Ryzen 9 9950X, CPU 2. A common fat-LTO harness compares a baseline built from all 17 exact accepted field files, a fresh ordinary path, and the candidate. The private copy changes only an include suffix in ifma8.rs and adds kernel/test sources; removing the suffix recovers all 17 files exactly. Flags use the accepted native build configuration, with a private golden_basis cfg in the fresh executable. The same deterministic full-width inputs are converted before timing. Every output agrees with scalar recurrence or closed-form oracles after decoding.

Thirty-one fixed rotated/reversed blocks cover six contexts. Each process performs three 2000-iteration primes and 100000 timed iterations. Four-stream results divide by four streams and eight lanes; other results divide by eight lanes. Norm recurrence includes scaling and a norm accumulator. The affine recurrence includes norm, conjugation, two complex multiplications, scaling, square and coordinate expressions; it omits inversion, actual curve/bucket construction, packing and scheduling. It is not a complete G2 batch.

Nanoseconds per lane operation; ratios pair blocks and need not equal ratios of displayed medians. Lower is faster.

|Context|Frozen ns|Fresh ns|Candidate ns|Candidate/fresh|Bounds|Wins / 31|
|---|---:|---:|---:|---:|---|---:|
|chain|6.381617|6.428330|6.027599|0.937141613|[0.934206979, 0.938293065]|31|
|streams4|6.277699|6.285264|5.681634|0.904357974|[0.902788741, 0.906007340]|30|
|scaled|9.929011|9.977026|9.545839|0.956290054|[0.955043936, 0.957971697]|31|
|square|5.012121|5.000010|10.176077|2.037000272|[2.029242757, 2.038219167]|0|
|norm|6.746833|6.740231|7.646491|1.134826491|[1.133578652, 1.135965162]|0|
|norm-affine|28.895101|28.870229|35.820194|1.241030015|[1.239044964, 1.241982669]|0|

|Six-context equal-log aggregate versus|Ratio|Bounds|Wins / 31|
|---|---:|---|---:|
|frozen|1.151567348|[1.151036593, 1.152014619]|0|
|fresh|1.150230752|[1.149539815, 1.151634071]|0|

Every candidate/control upper paired ratio must be ≤1.03. Both aggregate and norm-affine gains require upper<1 and ≥21/31 wins against both controls. Fresh/frozen calibration intervals must lie inside [1/1.03,1.03]. Ranks 10–22 of 31 provide nominal 97.055062652% individual median coverage under independent/exchangeable blocks, not a familywise guarantee. No sample extension, exclusion, threshold change, favorable-layout search or unchanged rerun is applied.

Failed calibration checks: `[]`.

Failed nonregression checks: `["square/frozen","square/fresh","norm/frozen","norm/fresh","norm-affine/frozen","norm-affine/fresh"]`.

Failed gain checks: `["norm-affine/frozen","norm-affine/fresh","aggregate/0","aggregate/1"]`.

## Verification and automatic profiles

The model passes 15042 exact CIOS cases, 6497 basis/product/square/norm cases and 32485 divide-by-three cases, including inclusive 2p edges and dependent outputs. The unfiltered native field suite passes 35 tests: 33 existing tests plus two new Arkworks comparisons covering both R256 and R260. Each new test checks 258816 field outputs, including normalization and dependent chains. These are component checks, not proof verifications.

All 558 latency processes precede six separate FIFO-gated 199 Hz cycles:u profiles: chain, square and norm-affine, fresh/candidate, with 1000000 iterations after primes. CPU IDs and timestamps are explicitly recorded. All captures finish before ownership transfer, decoding, lost-record audits and analysis. All helpers are compiled before capture; no owned compiler, helper/hash/archive or second CPU workload overlaps capture.

|Profile|Samples|Lost records|
|---|---:|---:|
|chain-candidate|7|0|
|chain-fresh|7|0|
|norm-affine-candidate|49|0|
|norm-affine-fresh|36|0|
|square-candidate|13|0|
|square-fresh|6|0|

Every profile has one sampled process and zero lost records. Flat sampled symbol residence is not exclusive wall time or an isolated cache/port cost. All 588 workflow steps complete. Raw perf data and ELF binaries remain local with hashes; source and workflow receipt archives round-trip byte-exactly.

The short recordings contain only 6–49 samples each, too few for detailed attribution or precise symbol-share comparisons. They are retained without a second capture. Read-only static inspection finds 200 IFMA sites and 645 total instructions in ordinary square, versus 206 and 921 in the new square. The affine functions contain 1,250/1,156 IFMA sites and 3,178/3,990 total instructions respectively. These are complete static function-body counts, not dynamically retired instruction counts or an isolated explanation of the slowdown. [Offline interpretation and counting scope](artifacts/interpretation.md).

## Decision and prover comparison

This fixed arithmetic recipe does not advance under its declared gate. Every positive subcase remains in the results; the failed aggregate or operation checks are not rescued by selecting multiplication alone. No SDK integration or production adoption follows this screen.

Accepted local 1×2 remains 8.022165 ms and 134 MiB observed peak RSS. Latest AWS c7a.24xlarge remains Zopro 11.915591 ms at 32 workers versus Aeglos Sorted 7.959184 ms at 96. These are inherited observations, not measurements of this candidate. The original eight supplied M5 CPU/Metal comparisons remain:

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

Accepted 54→1 is 162.250464 ms without a supplied M5 reference. The all-eight verified CPU 10× objective remains active and unmet, 0/8. No SDK requests or new Gnark exports are added; the distinct-proof ledger stays 22144. No AWS, mobile/ARM/TEE, giant-circuit, pressure/swap or concurrent-throughput result is claimed. The accepted physical-16 calibration engineering exception and separate final-artifact qualification remain unchanged. [Current local/AWS summary](../2026-10-29-current-cpu-summary/README.md).

[Protocol](artifacts/design.md), [all results](artifacts/analysis.json), [model](artifacts/model.json), [kernel](artifacts/kernel.rs), [native tests](artifacts/native-tests.log), [closing pins](artifacts/pins.json), [source](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz).
