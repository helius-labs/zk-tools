# Complemented fused product difference

**Rejected at the component gate.** The affine expression is 6.56% slower than fresh ordinary arithmetic; the four-pattern aggregate is 7.03% slower, with 0/31 wins. Calibration passes, but all eight nonregression checks and all four gain checks fail. Production remains unchanged.

The candidate replaces carry-dependent operand negation with independent digit complements, an accumulator seed and an exact correction after Montgomery reduction. It keeps the same fused IFMA count and adds no table, heap buffer or key state.

For normalized stored integers a,b,c,d≤2p and R>4p, set n=R−1−c and X=ab+nd+d=ab−cd+Rd. Ordinary unsigned CIOS gives T=(X+qp)/R. Correcting before final normalization gives U=T+p−d=(ab−cd+qp)/R+p, with 0<U<3p. One original conditional subtraction of 2p restores the established output bound. Lazy representatives may differ. The private complement operand is not exposed as a public below-2p field value. [Derivation and bounds](artifacts/bounds.md).

This differs from Gauss: product count is unchanged. It also differs from the earlier canonical-anchor product-subtraction method: correction removes the added Rd term, and d may be as large as 2p. Existing field operations and core callers remain unchanged. No source-level saving is treated as a proof-speed prediction.

## Fixed component measurements

Ryzen 9950X, one thread on CPU 2. Thirty-one fixed rotated/reversed blocks compare frozen ordinary code built before the private includes, fresh ordinary and candidate. Each process measures 100,000 iterations after three 2,000-iteration primes. Packing and scalar output checks are outside timing. Four distinct initial vectors prevent common-subexpression elimination in the independent-stream case. No owned build/helper/hash/archive/other benchmark overlaps capture.

The affine expression includes the current complex products, real scaling, square and coordinate operations with arbitrary fields and a supplied factor. It omits batch inversion, bucket handling and scheduling; it is not a valid-curve loop or an actual G2 caller.

Units are ns per complex field element, divided by four for independent streams. Changes are paired median ratios; positive means slower and need not equal a ratio of displayed medians.

|Expression|Frozen|Fresh|Complement|Paired change/fresh|Wins/fresh|
|---|---:|---:|---:|---:|---:|
|chain|6.430679|6.434211|7.071001|+9.877%|0/31|
|streams4|6.294332|6.292482|6.830043|+8.583%|0/31|
|scaled|9.948861|9.937777|10.285032|+3.415%|0/31|
|affine|24.327223|24.295198|25.904150|+6.560%|0/31|

Aggregate candidate/control ratios:

- Versus frozen: 1.070625676 [1.069852137, 1.071108743], 0/31 wins.
- Versus fresh: 1.070303790 [1.069612194, 1.070972553], 0/31 wins.

The fixed gate requires improving affine and equal-log-weight four-case aggregate upper ratios, with at least 21/31 wins against both controls, every candidate/context upper ratio ≤1.03, and each fresh/frozen interval within factor 1.03. There is no minimum gain floor. Sorted ranks 10–22 of 31 have nominal 97.055% individual coverage under independent exchangeable blocks, without a familywise guarantee. No exclusions, extensions, successful reruns, changed thresholds or subset promotion occur. A passing component admits a separate actual-caller study; it does not qualify adoption. [Design](artifacts/design.md), [analysis](artifacts/analysis.json).

## Correctness and profiles

The independent BigInt model passes 30,084 exact cases across Fq/Fr and R256/R260: 7^4 boundary tuples, 4,096 full-width samples and 1,024 changing-chain cases per configuration. It checks the signed and unsigned quotient identity, complete CIOS, repacked integer, correction and exact representative. The model output field `all_coefficient_bound` records pre-shift row maxima; [review](artifacts/postcapture-review.json) clarifies this label. The conservative signed-64 bound in the derivation covers all stages.

Two new native tests pass 172,544 changing-chain field-lane comparisons against Arkworks, across both primes and radices, inclusive 2p inputs and normalized below-2p output bounds. The 33 unchanged field tests are filtered out and inherited after source/reversal hash checks. All 380 capture processes check 32 returned canonical Fq2 values independently. Chain/scaled/affine duplicate eight final states into four slots; they do not create 32 independent chains. Corresponding digests agree.

Four separate FIFO-gated cycles:u profiles follow all latency at 199 Hz, one million iterations, chain/affine and fresh/candidate. Enable follows setup and priming; disable precedes conversion and scalar oracles. All four recordings finish before decoding.

|Profile|Samples|Lost|
|---|---:|---:|
|affine-candidate|39|0|
|affine-fresh|36|0|
|chain-candidate|9|0|
|chain-fresh|7|0|

Each recording contains one sampled process. Counts are descriptive, not exclusive stall attribution. Linked arithmetic loops and static instruction/stack observations are retained in [hot code](artifacts/hot-code.json); these are not executed costs, measured spill events or memory bounds. Raw perf data, full disassembly and expanded loss streams stay locally with hashes.

|Linked wrapper|Fresh / candidate instruction sites|Fresh / candidate IFMA sites|Fresh / candidate literal stack reservation bytes|
|---|---:|---:|---:|
|Chain, including scaled alternative|1120 / 1103|500 / 500|2368 / 2496|
|Four streams, shared inner loop|723 / 711|300 / 300|1456 / 1456|
|Affine expression|2689 / 2699|1000 / 1000|4352 / 4416|

The chain and stream wrappers have fewer static instructions yet run slower. Static code size alone does not explain the result. The extra seed, correction, changed lazy representatives and surrounding compiler choices are not isolated by this experiment.

## Provenance and accepted proof comparison

All 392 workflow steps finish: four preflights, 372 latency processes, four profiles and 12 postcapture steps. Original command/exit/stdout/stderr streams are preserved as text plus SHA256; archive review reconstructs all 1,564 encoded streams byte-for-byte. Four full loss stdout streams remain local with hashes. No successful native test or model execution is repeated. All 175 production files remain unchanged; removing two include lines recovers all 17 private field originals. No core/prover rebuild or verifier is claimed.

Whitespace review finds trailing blank lines in two original disassembly excerpts and the native test log. Those three artifacts remain byte-exact; authored Rust source has no whitespace findings.

Zero SDK proof requests, exports or new Gnark verifications occur. The proof ledger remains 17,811. There is no new AWS/Aeglos, phone/ARM/TEE, huge-circuit, pressure/minimum-RAM or concurrent-proof result. Accepted source 175 and SDK `20fde…` retain the prior full-performance-gate failure and physical16 calibration engineering exception.

The following accepted local figures are inherited, against supplied historical M5 references. Primary peak RSS remains 134 MiB.

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

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The all-eight CPU 10× goal remains active and unmet (0/8).

This exact complement/seed/correction recipe is closed before caller integration. The next candidate needs a distinct arithmetic or operation-removal argument; an unchanged rerun is not justified.
