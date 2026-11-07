# Uniform scaled prefix arithmetic: mathematical qualification

The new G1/G2 prefix construction passes scalar mathematical qualification. **Native execution and performance remain unmeasured; nothing is adopted.** Accepted production, the accepted SDK and the proof ledger remain unchanged.

## Construction

Let q=16. Applying the existing uniform R260 Montgomery reduction directly to normalized raw R256 digits implements `a ★ b = ab/q` under the ordinary R256 interpretation. This uses no coordinate conversion. Use this operation throughout the full-vector prefix, numerator, factor and reverse recurrences, then correct the inverse bridge once by q. Final coordinate arithmetic and scalar tails retain their ordinary operations; the correction follows scalar-tail reversal.

For G1, `p_i = D_i/q^i` and the stored numerator is `n_i D_(i-1)/q^i`. With inverse state `q/p_i`, the scaled reverse multiplication returns `n_i/d_i`. For G2, the scaled norm and numerator each carry 1/q; the prefix carries `q^(-2i)` and the reverse factor carries `q²/Norm(dx)`, again returning the ordinary slope. The correction is independent of prefix length. No table of powers is needed.

For a complete eight-lane batch, the ordinary inverse product tree can incorporate the correction at its single scalar root. Choose `k = -z p^-1 mod 16`, compute `(z+kp)/16`, canonicalize, and call the existing inverse. This returns `16/z`; the ordinary reverse product tree distributes the factor. The tree's own multiplies stay ordinary. Partial batches use the original inverse and scalar-tail reversal before applying the correction.

This differs from the closed whole-coordinate R260 experiments and from the closed experiment that scaled only three G2 fused expressions. Point/key/input/output encodings and prefix record layouts stay unchanged. No additional lookup table, key copy, solver plan or production pool is proposed. The multiplication count is unchanged; any benefit must come from reduced packing/correction work or dependencies in actual emitted consumers.

## Qualification

The fresh deterministic model checks:

- 10,404 exact raw ordinary-product reductions across both BN254 primes, including inclusive range boundaries and changing chains.
- 8,206 four-bit scalar-root cancellation cases and 16,384 corrected inverse lanes across both primes.
- 43,434 G1/G2 point outputs over 324 cases, covering 18 lengths from 0 to 511, three sign patterns and three reuse passes. Candidate and ordinary control agree with complete Arkworks addition; untouched slots are checked.
- Twelve guard cases. Deliberately omitting the bridge correction or applying it before scalar tails fails as expected.

The general range argument gives `T ≤ 8p²` and `REDC_R(T) < 2p` for normalized inputs because `8p < R`. For scalar-root cancellation, `z ≤ 2p` gives `(z+kp)/16 ≤ 17p/16 < 2p`, so one subtraction canonicalizes. The algebra and finite checks do not qualify the future SIMD implementation.

The first model compile failed because G1/G2 configuration aliases were imported from the crate root. An import-only repair compiled and ran successfully. Both sources and the failed receipt are retained. The earlier exact REDC helper fragment is reused byte-for-byte with a new test population; successful old models and proofs were not replayed.

A separate reader passes input hashes, 176 accepted production-source hashes, four successful command receipts, the import-only repair, counts, fresh raw CPU statistics and 1,024 symbolic scale-exponent cases. It is not a second BigUint REDC/curve implementation or a formal native proof.

## CPU accounting audit

The audit reuses all 45 processes and 6,795 timed SDK samples from the closed borrowed-tree pilot. Driver boundaries place process CPU accounting after warmup and before proof export/output. Process user/system time includes all threads, including search/spin/runtime work. It is not pure arithmetic work or a useful-work utilization measure.

|Arm|Median CPU ms/request|Median summed logical-CPU occupancy|Conditional same-CPU-time / 32, ms|
|---|---:|---:|---:|
|Accepted|178.458609|22.390908|5.576832|
|Fresh OFF|178.707185|22.192094|5.584600|
|Tree ON|193.121119|23.265329|6.035035|

The last column is conditional accounting, **not a physical latency lower bound or a speed prediction**. CPU time changes with IPC, clocks, SMT, scheduling and runtime work. The slower tree candidate had higher occupancy and more CPU time; busier workers alone did not improve latency. No new capture or causal idle/DRAM estimate is added.

## Accepted comparisons, unchanged

These are inherited local 32-worker complete-SDK measurements and supplied M5 references. They are not timings of the new construction. All eight CPU 10× targets remain unmet.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.915095|37 / 4.675×|24 / 3.032×|3.7|
|2x2|11.169812|59 / 5.282×|30 / 2.686×|5.9|
|4x2|16.085882|109 / 6.776×|48 / 2.984×|10.9|
|8x2|24.517520|189 / 7.709×|74 / 3.018×|18.9|
|12x2|33.838626|284 / 8.393×|101 / 2.985×|28.4|
|16x2|40.083672|335 / 8.358×|124 / 3.094×|33.5|
|24x2|65.506198|528 / 8.060×|176 / 2.687×|52.8|
|merge-24x1|65.200982|507 / 7.776×|175 / 2.684×|50.7|

Merge54 remains 161.718956 ms without a supplied M5 reference. No new AWS, Aeglos, RAM-pressure, mobile or large-circuit result is claimed. Canonical independently verified proof count remains 31,699.

Next: qualify the native primitives and inspect actual optimized G1/G2 consumers before selecting complete-proof timing. There is no 2× speed estimate from the mathematical result.

[Artifacts](artifacts/) preserve sources, designs, bounds, plans, pins, analyses and raw text/JSON receipts. The archive is compared byte-for-byte with originals. No keys, requests, witnesses, proofs, public inputs, executables, libraries or raw perf data are included.
