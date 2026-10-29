# Imaginary Toom-3 Fq6 multiplication

**Rejected at the component gate; no prover integration or adoption.** Five coefficient products do not establish a CPU gain over the retained six-product algorithm on this machine. All calibration and nonregression checks pass, but the all-four aggregate and complete final-exponentiation gain checks fail against both controls. Production remains unchanged.

Complete final exponentiation is **0.625% slower** than fresh OFF: paired ratio 1.006254290 [1.003486468, 1.011849971], 7/31 wins. The positive independent-stream estimate remains in the table, with uncertainty spanning parity. No subset is promoted.

## Arithmetic and applicability

Inspired by the imaginary Toom–Cook discussion in Aranha and Barreto's [paper 2026/2667](https://eprint.iacr.org/2026/2667), this independently written cubic multiplication evaluates degree-two polynomials at 0, +1, −1, u and infinity, where u²=−1. Their product has coefficients d0…d4. With evaluation products V0, V1, Vm1, Vi and Vinf:

```text
d0 = V0; d4 = Vinf
s  = (V1 − Vm1)/2                     = d1+d3
d2 = (V1 + Vm1)/2 − d0 − d4
t  = −u·(Vi − d0 + d2 − d4)           = d1−d3
d1 = (s+t)/2; d3 = s−d1
result = (d0 + (9+u)d3, d1 + (9+u)d4, d2)
```

This replaces six Fq2 products with five while adding interpolation and three Fq2 halves. Each base-field half conditionally adds p to an odd canonical Montgomery residue, then right-shifts four words. Since x<p and 2p<2²⁵⁵, there is no discarded top carry; the output is below p and keeps the same Montgomery scaling. No multiplication by inverse2 or field inversion is added. Fq2 multiplication, Fq6 square and sparse products, Miller loop and final-exponentiation schedule remain exact. No new heap storage, key table, persistent cache, point copy, dependency, assembly or unsafe arithmetic is introduced. Compiler stack usage is not measured.

This targets scalar native extension arithmetic in verification, not the dominant vector MSM kernels. It was never a predicted 2× proof optimization. Earlier Gauss/streamed-complex and G2-coordinate trials remain separate negative evidence. The [SQIsign AVX-512 paper](https://arxiv.org/html/2608.13948v1) was also reviewed: its field section relies on special primes c·2^f−1, unlike BN254, and its SIMD organizations overlap existing trials. Its reported gains are not transferred here.

## Fixed measurements

Ryzen 9950X, one worker pinned to CPU 2. Thirty-one fixed blocks rotate/reverse four patterns and three executables: frozen accepted field library, fresh OFF and new ON. The scalar native field libraries retain accepted compiler flags, plus the private cfg only for ON. All identical harnesses link with LTO disabled because the Arkworks dev-dependency objects lack embedded bitcode. This is a real field-library call benchmark, not qualification of the SDK's fat-LTO artifact. No source/layout sweep, excluded observation, sample extension, changed threshold or successful rerun occurs. No owned build/helper/hash/archive/second workload overlaps capture.

Chain uses 100,000 updates; streams4 uses four independent updates per iteration for 100,000 iterations; Fq12 uses 30,000 updates. Three 128-update checked primes precede reset and timing. Final exponentiation performs 16 complete calls after three single-call checked primes; black-box inputs prevent hoisting. Final output includes their sum. Fresh and ON outputs match independent Arkworks closed-form affine recurrences or complete final exponentiation outside timing, with matching cross-variant digests. All 388 processes pass. Every timed loop reports zero global allocator calls.

Units are nanoseconds per field update or complete final-exponentiation call; streams4 divides by four. Marginal medians and paired ratios need not agree algebraically.

|Workload|Frozen ns|OFF ns|ON ns|Paired ON/OFF change|Bounds|Wins/31|
|---|---:|---:|---:|---:|---|---:|
|chain|306.168600|306.044660|306.995250|+0.356%|[1.000521877, 1.005799644]|6|
|streams4|303.405390|303.515975|303.930105|-0.040%|[0.993554391, 1.007133429]|17|
|fq12|962.291500|975.417900|980.738600|+0.807%|[1.003061935, 1.012979122]|4|
|final|164570.187500|165499.437500|166501.375000|+0.625%|[1.003486468, 1.011849971]|7|

The fixed gate requires strict upper ratio<1 and at least 21/31 wins against both controls for the equal-log aggregate of all four patterns and for complete final exponentiation. Each ON/control upper ratio must be≤1.03, and each OFF/frozen calibration interval must lie inside [1/1.03,1.03]. There is no gain floor. Bounds use ranks 10–22 of 31, nominal 97.055% per comparison under independent, exchangeable blocks, not familywise guarantees.

|Comparison|Ratio|Bounds|Wins/31|
|---|---:|---|---:|
|chain/off/frozen|0.999821085|[0.998479575, 1.002282511]|16|
|chain/on/frozen|1.003193174|[1.000602060, 1.003979267]|9|
|chain/on/off|1.003564513|[1.000521877, 1.005799644]|6|
|final/off/frozen|1.003374606|[0.998111471, 1.007861926]|11|
|final/on/frozen|1.010951383|[1.007021459, 1.014779075]|6|
|final/on/off|1.006254290|[1.003486468, 1.011849971]|7|
|fq12/off/frozen|1.012922457|[1.007864524, 1.018531861]|2|
|fq12/on/frozen|1.020945721|[1.015711217, 1.025422941]|1|
|fq12/on/off|1.008071725|[1.003061935, 1.012979122]|4|
|streams4/off/frozen|1.002310238|[0.999539486, 1.006629784]|11|
|streams4/on/frozen|1.002372940|[0.997262321, 1.004379191]|14|
|streams4/on/off|0.999604283|[0.993554391, 1.007133429]|17|
|all4/on/frozen|1.009747363|[1.006203661, 1.012621629]|2|
|all4/on/off|1.005919397|[1.002268733, 1.007497773]|6|

Failed checks: `["all4/on/frozen/gain","all4/on/off/gain","final/on/frozen/gain","final/on/off/gain"]`. All positive estimates and uncertainty remain.

## Correctness, profiles and resources

Fresh OFF passes all 33 field tests; ON passes all 35. Two new cases cover 4,102 canonical-Montgomery half values, including raw parity/modulus corners, and 8,633 cubic products: 441 pairs of basis/zero/sign boundary values and 8,192 changing-chain states. Both compare against independent Arkworks arithmetic. Existing tower, pairing and complete final-exponentiation/reference tests execute unfiltered in both builds. Repeating the 33 original cases under ON is not additional distinct-case coverage. These field tests do not qualify the core/prover, portable paths, phones or giant circuits.

The private snapshot has 176 files: 174 original files are exact, one original Fq6 file contains cfg/module dispatch edits, and one child is new. Reversing those edits and removing the child recovers all 175 accepted files. All production hashes remain exact. Original Fq2 arithmetic is unchanged. OFF/ON select at compile time; there is no hot-loop environment lookup.

The first workflow passes OFF's tests, library build and freeze, then harness linking stops for missing Arkworks LTO bitcode before any benchmark runs. Those successful steps are reused. The repaired workflow links OFF, then ON's tests fail to compile for a missing AdditiveGroup import. That test-only import is corrected before ON's first successful execution. The final five-step build workflow tests/builds/freezes ON and links ON/frozen. All sources, original protocols and failure receipts are retained. No successful field suite, library build, harness link or capture is repeated. The old controller executable was replaced during the linker repair; its source/hash receipts remain, without a claim that its original bytes are retained.

After all 372 latency processes, four separate FIFO-gated 199 Hz cycles:u profiles cover chain (1,000,000 updates) and final exponentiation (4,096 calls), OFF and ON. Loading, primes and independent checks are excluded by enable/ack and disable/ack. All four profiles precede owner transfer and offline decoding. Each has one sampled PID and zero lost records. Symbol periods are retained; they are not exclusive wall-time fractions or proof that interpolation, carries, instruction dependencies or cache behavior alone caused a result.

The short chain recordings contain only 59 samples each; final-exponentiation recordings contain 134 each. They support coarse inspection only and are not resampled or treated as fine-grained cost attribution.

|Profile|Samples|
|---|---:|
|cycles-chain-off|59|
|cycles-chain-on|59|
|cycles-final-off|134|
|cycles-final-on|134|

Per-process resource snapshots and allocation counts are in every raw result. CPU differences cover the timed work plus accounting boundaries. The rusage RSS snapshot is the process high-water mark at loop completion, before the final oracle/digest/output; it is not a whole-lifetime peak, incremental kernel memory or minimum RAM requirement.

All 404 capture/analysis steps and all closing pins pass. Encoded source and receipt archives round-trip byte-for-byte with SHA256 checks. Raw perf and native executable/library files remain local with recorded identities. No SDK requests, proofs, Gnark verifications or AWS resources are created; the independent proof ledger remains **20,755**. This exact recipe is closed without SDK integration or a layout/subset rescue.

## Inherited supplied M5 CPU and Metal comparison

These are unchanged accepted-production timings and supplied historical Apple references, not measurements from this component. Primary accepted peak RSS remains 134 MiB. The prior full-performance-gate failure and physical16 engineering exception remain retained.

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

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The original all-eight CPU 10× goal remains active and unmet, 0/8. No new Aeglos, ARM/mobile/TEE, huge-circuit, pressure/swap/minRAM, concurrency or throughput claim follows.

[Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [all blocks and profiles](artifacts/analysis.json), [kernel](artifacts/kernel.rs), [source audit](artifacts/source-audit.json), [review](artifacts/review.json).
