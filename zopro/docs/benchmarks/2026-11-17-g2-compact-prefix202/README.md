# Compact G2 prefix records

**Not adopted.** Packing the saved prefix/norm records reduces their payload by
20%, but the primary local G2 query is 2.34% slower, losing all three pairs.
The SDK progression check and the separate local rule for selecting a C7a test
both fail. Production and full-proof timings remain unchanged.

Two existing normalized Montgomery values are stored through the unchanged
`PackedIfma8` API: four vectors per field instead of five, then expansion before
reverse multiplication. Field arithmetic, radix, point layout, scalar tails,
exception handling, grouping and scheduling remain original. This differs from
retaining extra denominators/X values or compressing tree point arrays. Added
packing instructions are charged to the actual caller; no new field algorithm,
retained table, copied key, pool or RAM cap is introduced.

| Record payload | Original bytes | Compact bytes |
|---|---:|---:|
| Eight pairs | 640 | 512 |
| 200-pair batch | 16,000 | 12,800 |
| 352-pair batch | 28,160 | 22,528 |

This is one buffer's requested payload, not total scratch, working-set residency
or process memory. A new 24-byte Vec header is charged; the conservative forecast
sums its capacity bound with the original allowance to cover retained reuse.
No whole-prover memory reduction or minimum-RAM claim follows.

## Complete-query screen

Ryzen9950X; fixed 1/16/32 workers, 512 KiB stacks, three independent byte-identical
executable backings and three rotated/reversed blocks. Each process primes three
queries, times17 and checks one final query. Scalar reset, grouping, split,
allocation, all fills/merges/weighting and normalization are inside timing.
Values are medians of process means; paired ratios need not divide those medians.

| Shape / workers | Original OFF ms | Compact ON ms | Paired ON/OFF | Wins /3 |
|---|---:|---:|---:|---:|
| 1×2 /1 | 32.762570 | 32.952138 | 1.007604 | 0 |
| 1×2 /16 | 3.381190 | 3.391460 | 1.005934 | 1 |
| 1×2 /32 | 2.969098 | 3.038516 | 1.023380 | 0 |
| 2×2 /1 | 35.361168 | 35.440311 | 1.003169 | 0 |
| 2×2 /16 | 3.826766 | 3.892729 | 1.013308 | 1 |
| 2×2 /32 | 3.437332 | 3.423060 | 0.979193 | 2 |

Primary32 block ratios are1.023380/1.055125/1.008386. Its OFF/ON CPU medians are
63.665/63.138ms per query; peak process RSS28.25/28.50MiB. Grouped32 CPU is
68.796/68.070ms and RSS43.75/43.25MiB. CPU includes untimed output hashing; RSS
includes loading and primes. These observations do not isolate cache or packing
costs. The grouped positive estimate does not override the primary failure.

The predeclared C7a spending rule requires all six local median ratios<=1.02;
primary32 exceeds it. No cloud test is selected. This does not establish how the
candidate would perform with C7a's smaller L1 data cache. No profile, full-SDK
campaign, parameter sweep or packing-layout rescue is selected for this version.

## Checks

Two focused native tests and two existing complete-window tests pass. Coverage
includes all256 sign masks, tails, three-pass reuse, grow/shrink capacity, complete
generator oracles, retained independent Arkworks MSM results, exceptional inputs
and1/4/16/32-worker stacks. The unchanged field/packing qualification is inherited.
Native build/tests take60.24s; the local result is available within the declared
15-minute allowance.

Twelve preflights precede36 main processes. All816 queries, including primes and
postchecks, match the retained oracles; immutable point inputs and mode selection
are checked. A separate same-agent reader verifies48 receipts/96 stream hashes,
reconstructs ratios and both decisions, recovers all three original parents and
the original arithmetic body, and checks all179 production hashes. The field
crate is unchanged. This is not an external audit or full release qualification.
All tests/captures/readers succeed once; no sample or gate is changed or extended.

There are no new SDK/Gnark exports or AWS resources. Canonical inventory58257 and
its SHA remain unchanged. The adopted SDK's current AWS/local CPU and Metal
comparisons are in [201](../2026-11-17-accepted-c7a201/README.md) and
[196](../2026-11-17-certified-all196/README.md). All-eight CPU10× remains active
and unmet,0/8; merge54 retains its separate result. No runtime change is adopted,
and nothing is pushed.

[Protocol](artifacts/protocol.md), [results](artifacts/result.json),
[source/receipt review](artifacts/review.json), [evidence](evidence.tar.gz),
[checksums](SHA256SUMS). Private root:
`/home/ubuntu/src/zopro-perf-work/g2-compact-prefix202`.
