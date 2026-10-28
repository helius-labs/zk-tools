# Interpretation

The new counts isolate actual unweighted first-layer inputs. All three arrays
are identical for each of1x2,2x2,16x2 and merge54. Every observed triple is
ordinary in these fixtures: no identity, equal-first-X or H-zero guard fires.
This does not justify deleting guards for arbitrary keys, witnesses or later
ternary layers. The12 fresh proofs remain full SDK requests with fresh blinding.

Primary1x2 has only2 ordinary G1 triples in affine_group and zero G2 triples.
That context contains204 G1 inputs and448 G2 inputs; G2 groups are all empty or
singletons (maximum group1, zero binary slots). It includes final reductions of
indexed tile sums, not just scalar deduplication. A blanket scalar-group triple
replacement therefore misses almost all primary opportunities.

Instead, primary indexed_tile/G1 has73773 input points,24524 ordinary triples
in204 tiles,203 tiles with at least8 ordinary triples. Most group lengths are
256–511:187 tiles, with7 full2048-point tiles. The maximum is682 triples per
tile. Indexed_tile/G2 has55277 points and18269 ordinary triples in448 tiles,
443 with at least8 triples, but maximum group164/54triples. These are initial
layer counts, not timings or the later ternary workload.

The earlier native caller timed8/16/32/64 triples. G1 direct improved all four
sizes, but its larger-case gain was smaller; G2 regressed at32/64. The primary
G1 indexed tiles can exceed that timed range substantially. The G2 opportunities
mostly fall near its unfavorable larger range. No transfer of a native small-
caller gain to all these jobs or a blanket G2 replacement is justified.

At2x2, affine_group has1970 G1 and1023 G2 ordinary triples, versus28148 G1
and18889 G2 in indexed tiles. The singleton/doubleton mixture still dominates
many affine groups. At16x2, group reduction becomes substantial:160461 G1 and
94926 G2 ordinary triples, plus105168 G1 and5845 G2 indexed-tile triples.
At merge54, the corresponding group totals are547376 G1/324043 G2, and indexed
totals301007 G1/12939 G2. G1 group jobs can hold11991 initial triples; these
large jobs cannot inherit a64-triple result without measuring their full data
movement, record footprint, reduction topology and exception handling.

The possible next caller is a complete unweighted G1 reduction at observed
group sizes, preserving the accepted weighted binary recurrence. A native
candidate would need to include gather/guard work, record construction,
ordinary and complete fallback, tails, compaction, later layers and scratch
admission. It must compare complete reductions against the current production
algorithm, not just two synthetic per-destination additions. The old mixed
streaming implementation remains closed; no collision queue is needed for
these already grouped inputs. Neither these counts nor the prior component
results establish that such an implementation is faster.

Do not silently cap a large job into many small native calls and count one
saved inversion per tile: tiling adds inversions and can erase the structural
saving. Do not charge smaller output count as fewer mathematical group adds;
one direct triple still sums three points and adds field work. Do not infer a
twofold proof speedup from the first-layer fraction. The primary grouped path
is tiny, while the indexed path is only one part of its complete MSM workload.
The full all-eight target remains larger than this diagnostic's scope.

The source also exposes a separate copy question: an unweighted chunk with no
binary work still packs points into limb storage and converts them back. The
primary affine_group/G2 observation has zero binary slots, so it proves such
work occurs. No bypass or speed claim is implemented here. Any follow-up must
first check the complete history for singleton/identity fast paths, preserve
zero groups and index ordering, and measure whether dispatch/scans cost more
than the saved copies. This observation must not be relabeled a new latency
improvement or substitute for reducing the dominant streaming work.

The Aeglos source audit confirms that its shifted Sorted path stores all GLV
window bases and uses one bucket set. The older ablations already measure its
combined table/algorithm tradeoff on Ryzen and C7a. They do not isolate only
RAM capacity, and their figures are historical. The small shifted-table and
ring candidates are also already in the history. No table, new Aeglos timing
or repeated closed candidate is selected by this review. Full third-party
source remains local; pinned primary URLs and hashes support the audit.

Observation uses two source hooks before first-layer reduction. Weighted
recurrences and streams are not hooked; later binary reductions are excluded.
No point or scalar is changed, and no coordinate is exported. The fixed counter
array has1024 bytes plus a gate; the local accumulation array is256bytes per
hook. Extra field guard arithmetic, atomics, stack temporaries and layout
changes perturb execution. There is no diagnostic latency, CPU, RSS, minimum
RAM, pressure or speed qualification. Priming is unobserved; collection occurs
after joined SDK work. Counters reset cleanly and are concurrency-tested.

All124 native core and8 prover tests pass. New tests cover real G1/G2 ordinary,
identity, equal-X and H-zero triples, reset/disabling, concurrent accumulation
and conservation. Field source and its qualification are inherited; optional
absent-fixture scenarios are not claimed. The private source has175 files:
172 originals exact, two reversibly edited originals and one diagnostic module.
Reversing the edits recovers all174 accepted original bytes. No production
code/default, solver/key representation, table, plan or resource policy changes.

Native75170 and capture33910 are terminal/reaped0. All24 workflow steps finish:
12 captures followed by12 independent Gnark checks.48 SDK calls verify
internally,12 fresh164-byte exports pass public binding and proof freshness,
24 proof/public members; ledger15541→15553. All source/library/executable/
driver/helper/input pins pass after capture. No owned compilation, helper,
hash/archive/profile, independent verifier or second benchmark overlaps capture.
The consistency condition is true, all raw arrays and count histograms remain.

Accepted174/SDK81033, primary8.366931ms and the inherited all-eight CPU/Metal
comparison remain unchanged. The original all-eight10x CPU objective remains
active/unmet0/8. No AWS/Aeglos, mobile/ARM/TEE, giant, minimum-RAM, swap,
pressure or concurrent-proof measurement is added. The evidence narrows the
next algorithm experiment; it does not complete or redefine the goal.
