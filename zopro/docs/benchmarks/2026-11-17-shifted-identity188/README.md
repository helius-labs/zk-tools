# Cached identity metadata for shifted G2 routing

Caching immutable point-identity bytes improves the shifted-table prototype by
23–29% in paired medians. It remains slower than current G2, so it does not
advance to SDK integration. The experiment confirms a specific avoidable access
stream identified by the [qualified warm profile](../2026-11-17-warm-reducer187/README.md).

| Query | Workers | Current ms | Parent router ms | Cached identities ms | Cached/parent | Cached/current |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 16 | 3.693553 | 5.881665 | 4.438009 | 0.753011 | 1.194237 |
| 1x2 | 32 | 3.216743 | 5.279799 | 4.002157 | 0.765812 | 1.248632 |
| 2x2 | 32 | 4.247981 | 9.376984 | 6.714347 | 0.712875 | 1.578739 |

Cached/parent wins9/9 blocks in each case. Cached/current wins0/9 everywhere.
Primary32 paired cached/parent bounds are[0.742435,0.795098]; cached/current is
[1.172002,1.278804]. These are descriptive zero-based ranks1/7 of nine ratios,
assuming independent continuous blocks,not simultaneous confidence bounds.
Complete samples and contrasts remain in result.json. The same-binary parent
control isolates the metadata intervention; do not compare absolute times across
other diagnostic builds as if source/binary layout were identical.

Each case has three primes per arm and nine rotated/reversed timed blocks.
All108 outputs match retained independent Arkworks full-query oracles. Word copy,
query and final affine normalization are inside timers; oracle hashing is outside.
This component speedup over a rejected parent is not a production proof speedup.
The 2x2 current query also benefits from scalar grouping,which these prototypes
have not matched. No case,sample,threshold or control was dropped or extended.

The builder caches the actual identity state of every shifted point,then routing
reads that byte instead of the point table. It assumes nothing about identities
surviving scalar shifts. Width12,22windows,point order,digits,reducer and pools
stay fixed. Added retained payload is661958bytes for1x2 and1150028bytes for2x2,
on top of84730624/147203584point bytes. Constructor table allowance includes the
extra payload. This remains a private prototype,not complete allocator or concurrent
memory-budget enforcement.

Fresh tests verify cached bytes for identity,inverse and duplicate inputs,compare
cached/parent results,and check one-byte-short denial versus exact-payload
admission. The existing captured-query and shared-table edge tests also pass:
three nonignored private tests. Source and build receipts are retained.

Setup at32workers takes137.69ms for1x2 and240.46ms for2x2,including metadata.
Whole-process peak RSS is197960KiB across all cases; this includes setup and is
not minimum RAM. Aggregate process CPU is24.69user+0.35systemseconds. There is
no per-case CPU or concurrent-memory qualification and no new AWS comparison.

Decision: retain the measured metadata improvement as a component,close this
exact complete-query recipe,and do not adopt it as production. Qualified187IP
samples also show a large point-materialization worker cost. Before another
implementation,review prior borrowed-tree negatives against a possible direct
first-pair gather,which could remove a full point-copy stage instead of changing
chunk constants. No such next implementation is included here.

Private root:`/home/ubuntu/src/zopro-perf-work/shifted-identity188`.
No production change,new proofexport,AWSresource orpublicpush. Canonical56585 and
the all-eight CPU10xgoal remain unchanged.
