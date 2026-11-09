# XYZZ sparse fallback: arithmetic qualification

**The new Rust arithmetic passes independent checks with native and generic field libraries. It is not integrated into the prover or timed.** Production and accepted proof latency remain unchanged.

The proposed use is the sparse projective fallback for collisions and exceptional additions. Ordinary shared-inversion affine batches retain their current implementation. XYZZ stores X,Y,Z²,Z³. The [EFD Sutherland formulas](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-xyzz.html) provide mixed addition and normalization; specializing doubling to the current a=0 curves removes its a-dependent square. The implementation adds no special unit-Z branch, initial-state tag, point table, transformed key, pool or assembly. Prior full-projective-stream and initial-unit experiments stay closed. The earlier triple-affine review mentioned XYZZ versus affine, but did not implement this sparse fallback replacement.

|Base-field work, excluding identity/exception branches|Current Jacobian|XYZZ helper|
|---|---:|---:|
|Mixed addition|7M + 4S|8M + 2S|
|Doubling for a=0|2M + 5S|6M + 3S|
|Normalization after recovering an inverse|3M + 1S|3M + 1S|

These are source operation counts, not latency measurements. Scalar Fq square currently dispatches through the same multiplication entry; Fq2 has different relative costs. Mixed addition may benefit, while doubling can cost more. Batch normalization shares one inversion of nonzero Z³ values; it does not require two inverse passes. Identity, equal/opposite points, cancellation and doubling retain complete handling. This targets a small residual path and is not a standalone route to the required twofold primary improvement.

|Logical point payload|Current Jacobian|XYZZ|Increase per sparse point|
|---|---:|---:|---:|
|G1|96 bytes|128 bytes|32 bytes|
|G2|192 bytes|256 bytes|64 bytes|

The 33.3% increase applies to each sparse accumulator, not the whole proof or key. Future integration must charge every additional capacity and normalization buffer, preserve the original memory-denied/generic fallback, and measure actual peak memory. This stage establishes no process RSS, minimum RAM or pressure result.

Four new focused test functions pass against both the exact original native field1200 and the preexisting generic noASM/noIFMA field library. Each backend executes 38,208 checks across G1/G2: 16,386 pair/raw and 2,718 changing-chain/batch/final-merge checks per curve. The two backends use the same fixtures; eight test executions do not double distinct mathematical coverage.

Independent Arkworks generator-weight results use full-width scalar coefficients. Nonidentity states also check ZZ³=ZZZ² and X=x·ZZ,Y=y·ZZZ in Arkworks, preventing normalization alone from masking an incorrect projective representation. Cases include identity numerators, variable scales, repeated/opposite points, cancellation, doubling, negation, changing state chains, ordinary-affine final merge and immutable batch conversion at lengths 0/1/7/8/9/31/128/257. This is arithmetic qualification, not a complete Window, MSM, Phi-phase or proof test.

All four serialized Rust1.98.1/xtask commands succeed first execution: native test build/run and generic build/run. Both consumers use O3, one CGU, debug assertions and overflow checks. The original field libraries and inherited suites are not rebuilt or rerun. A separate Rust reader checks 189 pins, all command/stream receipts, independent fixture-count arithmetic, payload sizes and all 176 production hashes. The unused test-import warning remains without a cleanup rebuild. Setup/review/publication helper invocations are outside those four receipts.

Next is isolated actual sparse-Window integration with checked accounting, original fallback, both Phi phases, complete queue/finalization and real worker/concurrent-caller tests. A subsequent complete caller comparison must include doubling, allocation, merge, normalization, dispatch and memory. An isolated mixed-add timing would not select production code. No optimization is adopted from this stage.

The following accepted local 32-worker full-proof timings and supplied M5 references are inherited from the latest unchanged SDK campaign. There are no new SDK requests, Gnark exports, performance captures or profiles; the canonical inventory remains 43,730. The full CPU10× goal remains active and unmet, **0/8**.

|Shape|Accepted CPU ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.829286|37.0 /4.726×|24.0 /3.065×|3.7|
|2x2|10.979335|59.0 /5.374×|30.0 /2.732×|5.9|
|4x2|15.803969|109.0 /6.897×|48.0 /3.037×|10.9|
|8x2|23.944017|189.0 /7.893×|74.0 /3.091×|18.9|
|12x2|33.703579|284.0 /8.426×|101.0 /2.997×|28.4|
|16x2|39.858478|335.0 /8.405×|124.0 /3.111×|33.5|
|24x2|66.186709|528.0 /7.977×|176.0 /2.659×|52.8|
|merge-24x1|65.489902|507.0 /7.742×|175.0 /2.672×|50.7|

Accepted primary peak RSS remains 133.50 MiB; merge54 remains 162.541995 ms without a supplied M5 reference. These cross-machine references are not new Apple measurements. No new AWS/Aeglos, mobile/ARM, huge-circuit, pressure/swap, minimum-RAM or throughput evidence is added.

[Helper](artifacts/xyzz.rs), [independent tests](artifacts/tests.rs), [review](artifacts/review-result.json), [design](artifacts/design.md), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Private root: /home/ubuntu/src/zopro-perf-work/xyzz-fallback176. Executables/libraries, unchanged source copies, private key/input/witness/proof data and the full ledger remain local.
