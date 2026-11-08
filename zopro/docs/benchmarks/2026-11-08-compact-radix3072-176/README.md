# Compact radix-3072 scalar representation and work types

The codec fits the existing **32-byte scalar row**, with exact reconstruction. It does not establish a prover speedup. On 1×2, modeled mathematical additions fall **0.5606%**, while projective spills rise **1,178 → 4,823** and normalized projective buckets **429 → 2,695**. This fixed batch200/streaming-top proposal closes without a native trial or production change.

This extends the [earlier radix-3072 count screen](../2026-11-07-nonbinary-radix176/README.md) with a compact representation and execution-type accounting. It retains eleven windows, two original partitions and 1536 magnitude buckets per window. There is no parameter sweep, revised threshold or repeated native benchmark.

Each signed Phi component has magnitude below 2^126. Eleven unsigned radix-3072 digits split into ten-bit lows and ternary highs. The lows occupy 110 bits; the highs occupy 17 because the last trit is at most one. A sign bit completes 128 bits per component. Overlapping local carries give signed digits of magnitude at most 1536. Eleven fixed small reciprocals suffice to extract trits; there is no large or per-key lookup table. The diagnostic reader computes the reciprocal expression on demand; division-free compiled extraction and conversion cost are not qualified. This is a representation result, not production memory or latency evidence.

The checks cover 117,882 saved components, 16,887 additional signed boundary/deterministic values, 1,299,078 exhaustive reciprocal-quotient comparisons and 9,437,184 adjacent-digit combinations. A separately written reviewer checks the top/error bounds, all 1,024 carry patterns, 278 small weighted-tree identity masks and nine dense sizes. Both implementations are by the same agent; this is not an external audit.

The model follows the accepted LIFO busy queue, batch200/queue199 limits, end-of-component flushes, indexed leading sign bins/1024-point tiles, merge batches and weighted256-point trees. Baseline ordinary counters exactly match the [saved native census](../2026-11-05-g2-operation-census176/README.md) on both shapes. Primary aggregate indexed/reduction/weighted counters also match exactly, including all 3,935 layers, 95,094 generic pairs and 9,838 scalar-tail pairs. No native census was rerun.

| Shape | Work category | Accepted model | Compact radix3072 |
|---|---|---:|---:|
| 1x2 | Mathematical additions | 633620 | 630068 |
| 1x2 | Projective queue arrivals | 1178 | 4823 |
| 1x2 | Adds to existing projective accumulators | 749 | 2128 |
| 1x2 | Normalized projective buckets | 429 | 2695 |
| 1x2 | Queue overflow flushes | 2 | 13 |
| 1x2 | Streaming affine batches | 2605 | 2907 |
| 1x2 | Streaming affine pairs | 516476 | 576839 |
| 1x2 | Affine reduction layers | 3935 | 528 |
| 1x2 | Reduction scalar-tail pairs | 9838 | 402 |
| 1x2 | High affine-transform Fq products | 82044 | 65380 |
| 1x2 | High projective-transform Fq products | 442 | 2924 |
| 1x2 | Weighted bucket slots | 20544 | 16896 |
| 2x2 | Mathematical additions | 653424 | 649869 |
| 2x2 | Projective queue arrivals | 376 | 2633 |
| 2x2 | Adds to existing projective accumulators | 16 | 748 |
| 2x2 | Normalized projective buckets | 360 | 1885 |
| 2x2 | Queue overflow flushes | 0 | 7 |
| 2x2 | Streaming affine batches | 2688 | 3015 |
| 2x2 | Streaming affine pairs | 535224 | 598830 |
| 2x2 | Affine reduction layers | 3991 | 528 |
| 2x2 | Reduction scalar-tail pairs | 9224 | 402 |
| 2x2 | High affine-transform Fq products | 82072 | 65384 |
| 2x2 | High projective-transform Fq products | 406 | 1856 |
| 2x2 | Weighted bucket slots | 20544 | 16896 |

Both shapes save about 0.55% of mathematical additions. Smaller buckets also reduce transform products, weighted projective-combination slots and reduction scalar tails. Those positive counts remain visible. The cost mix changes: primary vector affine pairs across streaming, merges and reductions increase 622,456 → 624,680; scalar affine tails decrease 9,986 → 565; queue arrivals handled projectively increase 1,178 → 4,823. These operations have different costs and are not combined into a predicted runtime.

Primary ordinary-window spills alone rise 1,178 → 2,523 and normalizations 429 → 1,458. The new top adds 2,300 spills and 1,237 normalized buckets, replacing the original indexed top. On grouped2×2, total spills rise 376 → 2,633 and normalizations 360 → 1,885. Thus the unfavorable projective change is present beyond the changed top path. No alternate top, queue, batch size or shape is selected as a rescue.

Ten Horner transitions use 110 doublings and ten extra projective additions rather than 120 doublings. Encoding needs eleven radix divisions per component; on-demand extraction adds integer arithmetic. No cost is assigned to these, actual inversions, scheduling, cache/traffic, common upstream scalar grouping or exceptional curve cases. The compact reader uses a different valid carry-tie convention from the earlier centered model; its primary addition count is one lower.

Grouped2×2 retains 52,274 → 28,852 first-occurrence scalar classes. Grouped point cancellation is unknown; both models assume generic noncancelling sums. The raw primary baseline matches native branch counts, but that does not qualify exceptional behavior under the new recoding. Diagnostic arrays are model scratch, not a production architecture. Smaller bucket capacity and unchanged scalar-row size do not prove lower RSS, minimum RAM or mobile/pressure behavior.

This closes the fixed proposal at the model stage: the codec removes the earlier scalar expansion issue, but the complete counts do not provide a sufficiently supported performance case for selecting this implementation. This is an engineering decision, not a measured slowdown or an impossibility result. No minimum positive percentage gain was required. The original 176 production-source hashes and both fixture hashes remain unchanged.

The screen and separate review each pass on their first executions through Rust1.98.1/xtask. Reviewer compilation first failed on an ambiguous iterator-sum type; only an explicit u64 annotation and unused import were repaired before execution. Initial source is retained. Helper compiles/preparation/publication have tool-transcript evidence rather than full workflow receipts; screen and review command/exit/stream receipts are included. No successful screen or review was replayed. No new solver, native curve suite, proof, profile or timing measurement was run.

## Accepted proof comparison, inherited

The [latest full SDK campaign](../2026-11-08-tree-policy-sdk176/README.md) remains authoritative for the latest local32-worker medians. Supplied M5 CPU/Metal references were not remeasured; ratios compare different machines. The same accepted build previously measured **7.915095 ms** on primary1×2; the latest **8.040649 ms** does not establish an adopted regression.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | CPU speedup | Metal speedup | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 | 8.040649 | 37 | 24 | 4.602× | 2.985× | 3.7 |
| 2×2 | 11.149097 | 59 | 30 | 5.292× | 2.691× | 5.9 |
| 4×2 | 16.069415 | 109 | 48 | 6.783× | 2.987× | 10.9 |
| 8×2 | 24.131178 | 189 | 74 | 7.832× | 3.067× | 18.9 |
| 12×2 | 33.940628 | 284 | 101 | 8.368× | 2.976× | 28.4 |
| 16×2 | 40.164083 | 335 | 124 | 8.341× | 3.087× | 33.5 |
| 24×2 | 66.187704 | 528 | 176 | 7.977× | 2.659× | 52.8 |
| merge24×1 | 65.361571 | 507 | 175 | 7.757× | 2.677× | 50.7 |

Merge54×1 remains 160.890372 ms, without a supplied M5 reference. All eight CPU10× targets remain unmet (0/8). Accepted SDK6d83 and canonical37,023-proof ledger are unchanged. No AWS/Aeglos/Gnark,250/4999,device/concurrent-proof or memory-pressure result is added.

[Design](artifacts/design.md), [screen source](artifacts/screen.rs), [split/Booth reference](artifacts/split.rs), [all counts](artifacts/analysis.json), [review source](artifacts/review.rs), [review result](artifacts/review.json), [source/input pins](artifacts/pins.json) and [checksums](SHA256SUMS) preserve the evidence. Public files contain source, aggregate counts and receipts; no point/scalar arrays, witnesses, keys, proofs, executables or libraries are included.
