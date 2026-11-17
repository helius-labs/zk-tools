# Certified finite bases in the current G2 query

The component screen advances to a complete-SDK test. A checked borrowed token
certifies the immutable raw bases have no identities,allowing ordinary streaming
windows to omit repeated identity checks. It adds no point table or per-point
metadata. Scalar grouping and arithmetic remain;grouped sums retain all checks
because cancellation can produce identities. Narrow top windows retain checks.

| Query | Workers | Current ms | Certified ms | Paired ratio | Wins |
|---|---:|---:|---:|---:|---:|
| 1x2 | 16 | 3.540161 | 3.424322 | 0.980869 | 9/9 |
| 1x2 | 32 | 3.119276 | 2.960828 | 0.933039 | 9/9 |
| 2x2 | 32 | 3.614210 | 3.345293 | 0.930677 | 9/9 |

Primary32 descriptive bounds are[0.895753,0.971592],zero-based ranks1/7 ofnine
ratios under independent continuous block assumptions,not simultaneous confidence.
All54 timed queries and18primes match retained independent Arkworks full-query
hashes. Word copy,grouping,query and affine normalization are inside;oracle hashing
and certification scan are outside. The scan takes84/56/137microseconds for the
three contexts after fixture loading,not a cold-storage key-load measurement.
Whole-process peak RSS is51968KiB across cases;notminimumRAM.

Attribution remains uncertain:the2x2fixture groups its points,so its changed
wrapper still calls the original checked grouped kernel. Its improvement suggests
layout/inlining or other consumer effects contribute. Do not assign every saving
to identity-check elimination. Retain the exact candidate and test complete fresh
proofs with accepted REF,shared OFF/ON binary,and explicit raw/grouped route counts.
This is a positive component screen,not a production adoption or proof speedup.

The first native suite passes131tests but its workflow rejects missing required
private tests:the new test module was not registered. The repair adds registration
only in a separate source-fixed snapshot. Four focused tests then pass,including
identity rejection and captured certified-query oracle checks;fourtiming tests
are ignored and131existing tests filtered. The earlier131suite is retained under
unchanged runtime-body bindings,not relabeled as a final unfiltered suite. No
successful timing capture was repeated. No new field primitive is introduced.

The next SDK implementation must certify the actual loaded key,retain immutable
association,avoid scanning unsupported large keys,charge setup to loading,and
observe raw/grouped/fallback entries. Generic/portable/default activation and
concurrency remain unqualified. The current snapshot is a private test consumer.

Private root:`/home/ubuntu/src/zopro-perf-work/finite-query190`.
No production change,AWSresource,newproofexport orpublicpush. Canonical56585 and
all-eight CPU10xgoal remain unchanged. [Raw results](artifacts/result.json).
