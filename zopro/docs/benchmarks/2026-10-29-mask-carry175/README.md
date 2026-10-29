# Branchless final carry normalization

**Rejected at the component gate; no SDK integration or adoption.** The intended Fq/R256 aggregate is 1.091876 against fresh ordinary arithmetic, bounds [1.089922, 1.094193], with 0/31 winning blocks. All 12 contexts regress; every fresh/frozen calibration passes. Accepted production remains unchanged.

## Change

A private eight-lane CIOS method replaces only the final serial carry pass with independent quotient transfers followed by exact AVX-512 mask carry propagation. Generate/propagate masks handle every cascade without a conditional ripple branch. Product rows, Montgomery cancellation, mixed-radix packing, representation and fused correction retain the accepted code. The [bounds](artifacts/bounds.md) show exact integer equivalence for coefficients below 2^61. No heap allocation, lookup table, key copy or retained setup is introduced.

The [24 October parallel normalization](../2026-10-24-parallel-normalize-component/README.md) and [prefix-only integration](../2026-10-24-prefix-controlled/README.md) used a conditional correction path and remain rejected. This experiment changes the correction algorithm rather than repeating either implementation. The current [IBS diagnostic](../2026-10-29-stream-ibs175/README.md) motivated examining arithmetic dependencies; it did not predict a speedup.

## Fixed measurements

Ryzen 9950X, one thread pinned to CPU 2. Thirty-one fixed rotated/reversed blocks compare a frozen ordinary executable built before the include with fresh ordinary and masked methods sharing one new executable. Each process measures 100,000 iterations after three 2,000-iteration primes. Conversions and output checking are outside the timer. No owned build, helper, hash, archive or second workload overlaps capture.

Units are ns per field-lane update; four-stream cases divide by four. Percentage changes are medians of paired per-block ratios against fresh ordinary, not ratios of displayed medians. Positive means slower.

|Case|Frozen|Fresh|Masked|Masked change|
|---|---:|---:|---:|---:|
|Fq-R256-chain|2.964789|2.966029|3.213983|+8.015%|
|Fq-R256-streams4|2.260029|2.260333|2.497633|+10.464%|
|Fq-R256-fused|3.184741|3.173495|3.461589|+8.981%|
|Fq-R260-chain|2.836084|2.842946|2.986992|+5.015%|
|Fq-R260-streams4|1.937890|1.944622|2.207060|+13.504%|
|Fq-R260-fused|2.946893|2.945053|3.230076|+9.787%|
|Fr-R256-chain|2.970988|2.970124|3.215236|+8.152%|
|Fr-R256-streams4|2.259678|2.261369|2.497949|+10.460%|
|Fr-R256-fused|3.177153|3.177715|3.454099|+8.693%|
|Fr-R260-chain|2.838400|2.838325|2.977651|+5.047%|
|Fr-R260-streams4|1.938294|1.944462|2.207461|+13.448%|
|Fr-R260-fused|2.949585|2.945302|3.231743|+9.711%|

Fq/R256 aggregate versus frozen is 1.091435 [1.089644, 1.093233], also 0/31 wins. Calibration bounds for every case stay within factor 1.03; all 24 candidate/control nonregression checks fail. The predeclared intended aggregate requires upper ratio below 1 and at least 21/31 wins against both controls; every context must remain within factor 1.03. There is no minimum gain floor. Individual order ranks 10–22 of 31 give nominal 97.055% coverage conditional on exchangeable independent blocks, without a familywise guarantee. No exclusions, extensions, successful reruns or threshold changes occur. The exact recipe is closed.

## Correctness and profiles

The exact model checks **122,880 BigUint integer/normalization equalities**, including boundary and fixed random arrays at radices 16, 256 and 2^52. It also exhausts all **1,048,576** radix-4 arrays with every coefficient below B². Native tests check **40,960** coefficient arrays and **172,544** changing-chain field-lane results against Arkworks across both primes/radices, ordinary/fused products, inclusive 2p inputs, lazy outputs and output bounds. Two new tests pass; the 33 unchanged field tests are filtered out and inherited. These counts are comparisons, not proofs.

All 1,116 latency processes, 12 preflights and four separate profiles check their returned canonical fields against a scalar closed-form affine-power oracle outside the timer. Each returns 32 fields, but chain/fused duplicate eight independent final states into four slots; those are not 32 independent chains. Original arithmetic and the probe are byte-retained. A temporary post-build probe formatting change was restored to the exact original build bytes before pins/capture; neither successful build was repeated.

Four whole-process cycles:u profiles run after latency, 30,000,000 iterations at 199 Hz. They include setup/primes/oracle and do not enter the gate.

|Profile|Samples|Lost records|
|---|---:|---:|
|chain-fresh|150|0|
|chain-masked|166|0|
|fused-fresh|160|0|
|fused-masked|179|0|

Nearly all sampled periods are attributed to the expected matching loop; one PID per profile, zero lost records. Profiles confirm the intended code runs, without establishing an exclusive stall cause. Full raw perf files remain local with hashes; decoded samples and loss dumps are archived.

Actual Fq/R256 dependent-loop static instruction sites increase **307→327**, with the same **100 IFMA sites**, **42 RSP operand lines** and **384-byte literal stack reservation**. These are static code observations, not dynamic instructions, measured spills or an RSS bound. The new [hot-code audit](artifacts/hot-code.json) distinguishes literal stack reservation from the original observer's broad subtract-with-RSP field, which also includes vector subtracts. This normalization removes a serial source carry chain but adds work; the measured regression rejects this implementation rather than proving all dependency-reduction approaches unhelpful.

## Scope and provenance

All 175 production source hashes remain exact. The private field copy contains the original 17 files; removing one include recovers the original eight-lane module, while other 16 originals remain exact. The appended helper copies the accepted CIOS body with only its name and final normalization block changed. No four-lane dispatch or production consumer changes. Frozen/fresh probes use the same source, seeds and oracle. Sources, libraries, executables, observer and xtask have 281 pins closed before/after. All workflow steps complete.

No SDK requests or new exported proofs occur; independent proof ledger remains **17,811**. No new proving-time, RSS, minimum RAM, AWS/Aeglos, mobile/ARM/TEE, large-circuit, memory-pressure, concurrency or throughput qualification follows.

Whitespace review found 763 entries in original perf dumps/headers, two tool logs and two disassembly endings. These artifacts remain byte-exact; authored Rust source has no whitespace findings.

## Inherited CPU and Metal comparison

Accepted local timings against supplied historical M5 references; these are not component measurements. Primary peak RSS remains 134 MiB. The accepted full-performance-gate failure and 16-worker baseline-calibration engineering exception remain unchanged.

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

All-eight CPU 10× goal remains active and unmet (0/8). Accepted 54→1 remains 162.250464 ms without a supplied M5 reference.

[Design and gate](artifacts/design.md), [declaration](artifacts/declaration.json), [exact model](artifacts/model.rs), [native kernel](artifacts/kernel.rs), [all results](artifacts/analysis.json), [hot-code audit](artifacts/hot-code.json), [source/tool pins](artifacts/pins.json), [artifact hashes](artifact-index.json).
