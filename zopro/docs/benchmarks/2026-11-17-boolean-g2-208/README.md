# Overlapping Boolean and ordinary G2 reduction

**The complete-query screen selects full-SDK follow-up.** Primary32 paired ON/OFF
is0.980841,3/3wins. All four declared median guards pass. This is a three-block
directional result, not a qualified full-proof speedup or adoption.

The [serial207](../2026-11-17-boolean-g2-207/README.md) version completed the
Boolean sum before ordinary G2 and failed its screen. This version first guards
all selected values/indices, collects true positions and zeros consumed words.
It then runs Boolean reduction and ordinary Phi query in `rayon::join` on the
existing pool. Both read immutable key points; only the ordinary branch owns
the remaining scalar words. The original exact-grouping decision remains first;
grouped calls bypass peeling. No point table, key copy, new pool, arithmetic,
radix, grouping cutoff, tree size or window-width change.

|Query/workers|OFF ms|ON ms|Paired ON/OFF|Wins/3|Peak RSS OFF /ON MiB|
|---|---:|---:|---:|---:|---:|
|1x2/1|32.522537|32.548936|1.000575|1|18.00 /18.00|
|1x2/16|3.335992|3.369092|1.016829|0|27.75 /27.50|
|1x2/32|3.020135|2.935835|0.980841|3|28.25 /28.75|
|2x2/32|3.443914|3.392997|0.985215|3|43.00 /43.00|

Primary32 block ratios are0.964274/0.980841/0.990005. Primary16 is slower in all
three pairs, within the predeclared2%median guard. Grouped2×2 asserts bypass, so
its favorable observation cannot be attributed to Boolean processing. This
experiment has no simultaneous serial207 arm; absolute cross-build differences
do not isolate a causal overlap speedup. No samples or favorable subsets are
promoted, added or excluded.

Primary32 OFF/ON CPU spans are62.782/61.271ms per query, including untimed oracle
hashing. RSS is process lifetime peak, not minimum RAM or a pressure result.
Selected-index storage and the Boolean tree can now overlap ordinary workspace;
any production planner must account for their **sum**, not the serial maximum.
Full allocation admission and metadata lifetime remain unimplemented.

Two focused native tests pass. The stronger synthetic fixture places repeated
points, exact opposites and identities at Boolean-selected positions, checks
generator-coefficient query sums and guard failures before mutation, exercises
grouped bypass and uses1/4/16/32-worker512KiB pools. Both retained real queries
match independent Arkworks hashes in OFF/ON modes and preserve point inputs.
Existing field/indexed-reducer qualification is inherited, not rerun.

Eight preflight processes precede24main processes across three byte-identical
independent executable backings. Three rotated/reversed blocks,3primes+17timed
queries+onepostcheck give **544 checked queries**. Scalars copying, original
grouping, guard/index preparation, zeroing, both reductions, joining, addition and
final normalization are timed; fixture/pool loading and hashes are outside wall
timing. Route counters assert primary ON peeling, grouped ON bypass and OFF
behavior. No owned compiler/helper/analysis overlaps capture.

The static1,753/3,166 position lists and their solver/key/fixture bindings are
inherited byte-for-byte from207; witness values are still read fresh each query.
No point or witness lookup table is created. The private snapshot retains178exact
originals, one append-only test parent and one new child. Removing the added
branch/signature recovers the original query body. All179 production hashes
remain exact. A separate reader checks32receipts/64stream hashes, backing
identities, source recovery, all ratios and the positive selection decision.
All builds/tests/captures/readers pass first execution; unused-import warnings
are retained. No successful stage is replayed.

Protocol starts17:13UTC; complete screen result17:15:56, within the15-minute
allowance. There are zero new SDK/Gnark exports or cloud resources. The canonical
inventory stays58,915. The all-eight verified CPU10× goal remains active/unmet,
0/8. Accepted runtime194/196 and its local/C7a CPU/Metal comparisons are unchanged;
this candidate has no full-proof or AWS measurement.

Next: integrate this exact dependency structure with bounded solver-derived
metadata and overlapping-workspace admission, then run a short verified full-SDK
screen against accepted production and a fresh OFF control. Positive full-proof
evidence selects remaining qualification. Do not replace the candidate with a
table/width/threshold sweep or treat this component result as adoption.

[Protocol](artifacts/protocol.md), [result](artifacts/result.json),
[review](artifacts/review.json), [source/receipts](evidence.tar.gz),
[checksums](SHA256SUMS). Raw points/scalars/keys, position lists, binaries and
full ledgers remain private. Private root:
`/home/ubuntu/src/zopro-perf-work/boolean-g2-208`. Committed locally; not pushed.
