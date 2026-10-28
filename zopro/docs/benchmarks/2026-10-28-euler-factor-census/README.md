# Factoring the Euler output at equal or zero rows

The actual primary 1×2 Euler output has **21,155 nonzero one-product rows, 391 zero rows, and 9,376 rows needing two products**. An ideal lane-compaction model reduces its IFMA arithmetic proxy by **24.073%**. Handling only pure existing eight-row blocks reduces it by **3.225%**. These are operation-count screens, not native kernel or proving-time improvements. Production is unchanged.

At a row where the original values satisfy `A=B`, the expression factors exactly:

```text
E(A)*B + A*E(B) = A*(E(A)+E(B)).
```

This preserves both derivatives. It does not assume `E(A)=E(B)`, and differs from the invalid `2*A*E(A)` shortcut discussed in the [constraint-structure census](../2026-10-24-constraint-structure-census/README.md). If one original value is zero, one product remains; if both are zero, the output is zero. The [author's differentiation notes](https://alinush.github.io/differentiation-tricks) give background for derivative quotient evaluation; this pointwise factorization and the local screen are our own application. No claim from another implementation is used to predict runtime.

A history search found the earlier formal row census and adopted eight-row fused sum2 output, but no matching factor/compaction implementation. This is not an exhaustive source-history novelty claim. Reviews of hexagonal/binary ring MSMs, truncated/PMNS arithmetic, preconverted point coordinates and the current G2 lattice split found prior implementations or negative evidence, so they were not rerun here.

Two new native observer tests pass once. They cover all 256 eight-row equality masks, deliberately unequal derivatives, zeros, opposite values, partial A/B inputs, capture gating and category conservation. The derivative counterexample uses `A(X)=1+X`, `B(X)=2X` at `X=1`: values are equal but their Euler derivatives differ; the factored result is correct and the twice-one-derivative shortcut is not. A separate full-width Arkworks Fr model checks 131,072 cases: 98,304 factor/zero cases and 32,768 unchanged general controls.

The private diagnostic adds two reversible hooks: a public module and one observer call immediately before the existing Euler output overwrites A. All 172 accepted production files remain exact; 170 private originals remain byte-identical. No field value, derivative, output, routing decision, key representation or arithmetic operation is changed by the observer. Only finite category masks and counts are exported, not witness field values. Counter scratch is diagnostic and is not a proposed retained key plan.

Three fixed processes each use 32 workers on CPUs 0–31, load the accepted primary Euler image, run three unobserved primes, and observe one proof. All three maps are identical. All 12 SDK requests verify internally; three fresh 164-byte exports pass independent Gnark verification and public binding against the prior 12,682 identities. The ledger is now 12,685. All observations preceded all independent verifications and offline analysis; no owned build, helper, hashing or archive work overlapped SDK capture. All 370 capture pins still match. No test, build, capture or verifier failed or was repeated.

| Actual output property | Count |
|---|---:|
| Rows | 30,922 |
| Equal original values, including zeros | 19,033 |
| Both original values zero | 391 |
| Nonzero one-product rows | 21,155 |
| Two-product rows | 9,376 |
| Existing eight-row blocks, including final tail | 3,866 |
| Pure equal-value blocks | 43 |
| Pure single-product blocks, including zero blocks | 314 |
| Pure zero blocks | 30 |
| Longest equal-value run | 253 |

Actual equality includes witness-specific equalities; it is not the formal all-input equality classification from the earlier report. One-product categories include equal values or either input zero. Opposite-value masks were also counted as diagnostic evidence but are not included in the one-product policy modeled here.

| Arithmetic schedule proxy | IFMAs | Change from current |
|---|---:|---:|
| Current eight-row sum2 | 579,900 | — |
| Pure single/zero blocks only | 561,200 | -3.225% |
| Ideal global compaction by category | 440,300 | -24.073% |

The proxy charges 150 IFMAs for an eight-lane two-product sum and 100 for an eight-lane multiplication, including separate final groups for each compacted category. It excludes classification, derivative summation, zero handling, row indices, shuffles, packing/import/export, extra group tails, stores and scheduling. Ideal global compaction is optimistic and is not a bounded-memory implementation. A finite tile may require additional tail groups and erase the gain. These counts are not CPU cycles, a critical-path bound or a full-proof speed estimate.

| Most frequent full-block one-product mask | Blocks |
|---|---:|
| value=db | 1143 |
| value=6d | 1138 |
| value=b6 | 1137 |
| value=ff | 314 |
| value=b7 | 21 |
| value=ed | 20 |

Mixed masks dominate. The cheap pure-block shortcut alone saves relatively little arithmetic and still pays classification. The next useful experiment is a bounded SIMD compaction caller that handles all masks and tails, preserves both derivatives, reuses A for output, and charges all scratch and scatter work inside its timer. Its independent tests must prove each row is consumed and written exactly once before any full-proof trial. No mask or key-specific outcome table should select a favorable measured case.

No new latency, CPU profile, RSS, minimum-RAM, mobile, TEE, concurrency, pressure or AWS result is claimed. This does not establish a route to a twofold proof improvement; the affected final-output phase is only part of quotient computation. The global all-eight CPU 10× objective remains active and unmet for every row. The accepted [FFT adoption](../2026-10-27-clean-fft-adoption/README.md) retains its documented engineering exception and false full-performance/qualified-scope gates. The following local timings and supplied M5 references are inherited.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU ratio | Metal ratio |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |

Accepted historical primary peak RSS remains 134.50 MiB. Merge 54→1 remains 161.341835 ms with historical peak RSS 1,910.1875 MiB and no supplied M5 reference. These are not minimum RAM requirements.
