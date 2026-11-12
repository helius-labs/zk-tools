# Destination-ordered G2 accumulation

**The complete-query screen fails; the candidate is not adopted.** Ordering
updates by destination beats the staged borrowed tree in threaded contexts but
does not beat original streaming consistently. All 1,229 complete queries match
the retained independent Arkworks results. No full-proof campaign is warranted.

The prototype routes signed point indices by bucket, then advances one accumulator
per bucket per round. Each round has distinct destinations and uses the existing
signed affine adder. It removes the filler collision queue and intermediate tree
point arrays. Routing, 256-bucket groups, two point partitions, high-before-low phi
order, indexed top, merges and weighting remain. Complete scalar handling retains
identities, doubling and opposites. There is no persistent table, copied key or
new pool. The tree's conservative reservation remains above the new storage bound;
this is not an optimized memory-admission policy or minimum-RAM claim.

Three modes share one newly built caller: original streaming, staged tree, and
destination rounds. This changes the staged tree's reduction topology; it does
not repeat the previous borrowed-pair staging edit or independent-component planes.
All 177 production bindings remain unchanged; four private parent edits recover
exactly and 173 parents are byte-identical. Existing field arithmetic is reused.

Ryzen9950X; fixed individual worker affinity, 512KiB worker stacks, Rust1.98.1,
znver4/O3/fatLTO/oneCGU. Six contexts use three rotated/reversed blocks and three
distinct byte-identical executable backings. Nineteen functional preflights include
denied admission. Each of 54 main processes has three primes,17 timed queries and
one postcheck. All samples remain. Timing includes routing, allocation, grouping,
scalar reset/split, fills, merges, weighting and normalization. Output checking is
outside timing. This is a directional screen, not a confidence-qualified gain.

|Query / workers|Streaming ms|Staged tree ms|Rounds ms|Paired rounds/streaming|Wins /3|
|---|---:|---:|---:|---:|---:|
|1x2 /1|32.527|37.599|37.984|1.1716|0|
|1x2 /16|3.328|3.574|3.466|1.0327|0|
|1x2 /32|2.994|3.127|3.014|1.0048|1|
|2x2 /1|35.013|41.144|39.338|1.1235|0|
|2x2 /16|3.858|4.095|4.012|1.0401|0|
|2x2 /32|3.489|3.828|3.567|1.0202|0|

Times are medians of process means; paired ratios need not divide displayed
medians. Primary32 rounds/tree is0.9645, but rounds/streaming fails the required
three-block gain. Threaded nonregression checks also fail. No favorable block,
worker count or parameter is promoted.

Primary32 median CPU service is62.06/74.88/73.06ms per query for streaming/tree/
rounds; peak RSS is30.75/49.25/33.50MiB. At2x2/32 the corresponding values are
67.50/85.70/82.05ms and60.25/64.50/58.25MiB. RSS includes process history and
loading. Removing intermediate point storage helps relative to the tree, but
does not remove enough complete-caller cost to outperform streaming. Routing,
long bucket chains and shrinking late batches remain plausible costs; this screen
does not isolate their individual contributions. No new profile is needed to
decide against this fixed recipe, and no parameter sweep is queued.

Five focused tests pass, including signs/padding, exceptional points, skew/reuse,
complete scalar-query oracles, admission bounds and1/4/16/32-worker pools. Native
test build/run took59.29s, caller build43.88s, preflight1.76s and main capture17.98s.
Protocol creation to reviewed result was about2m45s, excluding preceding history
review and this report. The separate raw reader initially rejected an extra blank
line before the test module. A retained corrected reader compares the exact runtime
body and new stage separately; it passes without any source, build or capture replay.

Production and canonical55,028-proof ledger are unchanged. No new SDK/Gnark,
AWS/Aeglos, mobile, pressure or huge-circuit result is added. The full CPU10x goal
remains active, unmet0/8; all-eight CPU/Metal comparisons remain in the
[accepted report](../2026-11-12-bundle-adoption178/README.md).
The prepared [C7a test](../2026-11-12-phase-search-c7a178/README.md) remains separate.

[Protocol](artifacts/protocol.md), [stage](artifacts/round_stage.rs),
[raw review](artifacts/review.json), [results](artifacts/result.json),
[resources](artifacts/resources.json), [source/receipt archive](artifacts/evidence.tar.gz).
Private root: `/home/ubuntu/src/zopro-perf-work/bucket-round178`. Local and unpushed.
