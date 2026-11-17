# Table-free Boolean G2 sum: serial version

**The complete-query screen fails.** Primary16 is5.26%slower and primary32 is
1.28%slower in paired medians, with0/3wins in both contexts. No SDK integration
or adoption is selected for this fixed serial recipe. Production is unchanged.

The current solver identifies1,753/3,166 HintBits positions in the retained
1×2/2×2 G2 queries, containing511/984 true bits in these witnesses. A Rust helper
checks each original key point against the captured fixture order and guards all
selected scalar words as0/1. Position payloads are7,012/12,664bytes, not point
lookup tables. The old20.5GB Boolean-LUT partition remains closed.

The prototype preserves the original exact-grouping decision. When grouping
declines, it reduces selected Boolean bases using the existing indexed affine
tree, zeros their consumed proof-local words and adds that unweighted sum to the
ordinary query. All guards precede mutation. Grouped calls bypass peeling and
retain their original operations. The serial version completes the Boolean sum
before starting ordinary G2, and drops its temporary indices/tree storage first.
No point/key copy, new pool, field arithmetic, radix or window-width change.

|Query/workers|OFF ms|ON ms|Paired ON/OFF|Wins/3|Peak RSS OFF /ON MiB|
|---|---:|---:|---:|---:|---:|
|1x2/1|32.556919|32.642552|1.002630|1|17.75 /17.75|
|1x2/16|3.341184|3.521294|1.052560|0|27.75 /27.50|
|1x2/32|2.996037|3.018948|1.012829|0|28.75 /28.00|
|2x2/32|3.464004|3.446371|0.986399|2|42.25 /42.25|

The grouped2×2 arm asserts bypass, so its favorable timing is not a measured
Boolean-optimization benefit. Primary32 CPU medians are63.562/61.592ms per query;
the CPU span includes untimed oracle hashing. Lower CPU service does not establish
less arithmetic or a shorter critical path. RSS is process lifetime peak, not
minimum RAM. Three blocks provide a directional screen without confidence bounds.

The frozen rule requires primary32 median<1 with≥2/3wins and all four medians≤1.02.
Primary32 gain and primary16 non-regression checks fail. All observations remain;
no timing repeat, sample extension or favorable subset promotion follows.

Two focused native tests pass, covering generator-coefficient query oracles,
identity/signed inputs, invalid values/indices before mutation, grouped bypass,
1/4/16/32-worker512KiB pools and both retained independent Arkworks query hashes.
The unchanged indexed reducer's general exceptional-point qualification is
inherited; no fresh full core/prover/field or portable suite is claimed.

Eight preflight and24main processes use three byte-identical, distinct-inode
backings. Main processes have3primes,17timed queries and one postcheck. All
**544 queries** match retained oracles and preserve point inputs. Scalar copying,
grouping, guard/index collection, Boolean tree, zeroing, ordinary query and final
normalization are timed. Pool/fixture loading and hashes are outside wall timing.
Route counters assert primary ON peeling, grouped ON bypass and OFF paths. No
owned build or analysis overlaps capture.

The private snapshot has one append-only test parent and one new child:178other
originals match byte-for-byte, and removing the branch/signature edits recovers
the original copied query body. All179 production hashes remain exact. A separate
reader checks source recovery,32receipts/64stream hashes, ratios, gates, backing
identities and unchanged ledger. Its first exact floating-point JSON comparison
differs by2e-16; a1e-12 serialization-comparison tolerance repairs only the reader.
Original reader source is retained; no scientific test or capture is replayed.

Protocol17:04:15UTC; complete result17:08:49, within the15-minute first-result
allowance. All native build/tests/captures pass first execution. No new SDK/Gnark
proof or cloud resource; canonical inventory remains58,915. Accepted SDK194/196,
[all-shape comparisons](../2026-11-17-static-query-fold205/README.md) and
[accepted C7a baseline](../2026-11-17-accepted-c7a201/README.md) remain unchanged.
The verified all-eight CPU10× goal is active and unmet,0/8. No phone, huge-circuit,
physical-pressure, swap, minimum-RAM or concurrent-throughput claim follows.

The serial dependency is a separate next question: after successful guards and
word separation, the Boolean sum and ordinary query can potentially overlap on
the existing pool. That changes execution dependencies and simultaneous scratch
ownership. It is not measured here and cannot rescue207's result; a distinct
source/protocol is required. No table or parameter rescue is selected.

[Protocol](artifacts/protocol.md), [results](artifacts/result.json),
[positions/bindings](artifacts/positions.json), [review](artifacts/review.json),
[source/receipts](evidence.tar.gz), [checksums](SHA256SUMS). Raw key/point/scalar
fixtures, position lists, binaries and full ledgers stay private.
Private root: `/home/ubuntu/src/zopro-perf-work/boolean-g2-207`. No push.
