# Direct first-pair gather in shifted G2 reduction

Borrowing both inputs for the first tree level removes the initial full point
materialization pass, but does not yield a useful complete-query gain. The exact
candidate does not advance to SDK integration. No production change is adopted.

| Query | Workers | Current ms | Cached-identity parent ms | First-pair ms | Paired first/parent |
|---|---:|---:|---:|---:|---:|
| 1x2 | 16 | 3.858122 | 4.459978 | 4.652972 | 1.043902 |
| 1x2 | 32 | 3.414545 | 4.045075 | 4.036800 | 0.997503 |
| 2x2 | 32 | 4.535901 | 6.688264 | 6.815994 | 1.018436 |

Primary32 first/parent bounds are[0.963704,1.031985],with5/9wins: improvement is
inconclusive. At16workers it loses all9pairs;2x2 wins only1/9. Against current,
primary32 paired median is1.161659[1.143729,1.275029],with1/9wins. Thus it remains
slower than current at the actual caller. No minimum gain floor is applied.
Bounds are descriptive zero-based ranks1/7 ofnine ratios,under independent
continuous block assumptions,not a simultaneous confidence guarantee.

The new first pass gathers both immutable operands into the existing canonical
G2 borrowed-add formulas and writes only their sum. Ragged tails,identities,
doubling and cancellation use complete operations. Later indexed layers,tile
size1024,bundle packing,partial merging and weighting stay unchanged. The extra
first-operand reference and pair-descriptor vectors replace initial full point
copies;there is no new persistent metadata beyond the188identity cache.
This differs from the closed176borrowed-tree trial,which replaced ordinary
streaming with routed trees. Here the input was already a routed shifted table.

Output slots use spare capacity. Each odd/exceptional slot is written directly;
each remaining unique slot is written by the joined first-pair kernel. Length
is set only after all writes return. No input/output alias cast is used. Complete
resource admission for added descriptors is unqualified;this is a private native
prototype. The direct path requires the existing enabled canonical borrowed
G2 configuration;capture receipts bind nativeIFMA and prefix-records=1,with no
recorded opt-outs. There is no separate per-call first-pair route counter.

All135 nonignored native core tests pass,with4explicit timing tests ignored and
none filtered. The new generator-weighted test compares every ragged bucket with
both the original indexed reducer and an independent scalar-sum oracle,covering
0/1/7/8/9/31/32/1023/1024/1025/4097leaves,empty/skewed buckets,identity,inverse and
double cases,1/4/16/32workers and512KiBstacks. Existing captured-query oracles also
pass. Field arithmetic and G2 arithmetic expressions remain unchanged.

One fresh binary supplies all three arms and three contexts. Three primes per
arm precede nine rotated/reversed timed blocks. All108 component outputs match
retained independent Arkworks hashes. Word copy,query and affine normalization
are timed;oracle hashing and table setup are outside. Full-proof latency is not
measured. Whole-process peak RSS is213512KiB acrossallcontexts;CPU23.31user+
0.38systemseconds includes setup and every arm. Neither is per-case resource
qualification or minimum RAM.

The first source generator stopped before build because its function extraction
matched two functions. Balanced function extraction repaired the generator;
its failed version is retained. No successful test or timing capture was rerun.
The result does not show that hot store addresses were themselves removable
exclusive cost:borrowing also changes later input reads and descriptor work.

Close this exact first-pair recipe. Preserve the188identity-cache mechanism gain
and187qualified profile. Change the algorithmic question before another pointer,
chunk-size or helper-layout variant. No new AWS run,proofexport orpublicpush.
Canonical56585 and all-eight CPU10xgoal remain unchanged. Private root:
`/home/ubuntu/src/zopro-perf-work/first-pair189`.
