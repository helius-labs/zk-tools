# Boolean G2 overlap: full SDK209

The positive [208 query screen](../2026-11-17-boolean-g2-208/README.md)
did not improve this short full-proof screen. The primary 32-worker ON/accepted
paired median is **1.002879**, with **1/3 wins**. The fixed209 implementation is
not adopted. This is a directional screening decision, not evidence of a
statistically established slowdown or a rejection of small improvements.

|Shape/workers|Accepted194 ms|New OFF ms|New ON ms|Paired ON/accepted|Wins|ON/OFF|
|:---|---:|---:|---:|---:|---:|---:|
|1×2 /16|8.669986|8.658652|8.720058|1.005775|1/3|1.007092|
|1×2 /32|7.594016|7.547676|7.595830|1.002879|1/3|1.007915|
|2×2 /32|10.690120|10.631164|10.819883|1.006145|0/3|1.017752|

Displayed milliseconds are medians of three process means. Paired ratios are
medians of corresponding process ratios and need not equal ratios of displayed
medians. All three predeclared latency, CPU and RSS nonregression guards pass;
the primary directional gain condition fails. Primary32 CPU paired ON/accepted
is0.991993, a favorable directional result that does not establish latency gain.
Grouped2×2 bypasses the Boolean route: its timings are not Boolean arithmetic
benefit evidence. No new C7a measurement or Aeglos comparison was commissioned.

|Shape/workers|Accepted CPU ms/proof|ON CPU ms/proof|Accepted peak MiB|ON peak MiB|
|:---|---:|---:|---:|---:|
|1×2 /16|109.337|109.589|72.00|74.00|
|1×2 /32|168.847|167.495|92.75|89.50|
|2×2 /32|251.359|249.915|224.00|240.25|

CPU columns are medians of process CPU/proof; peaks are maxima across the three
main processes. They are not minimum-RAM requirements or isolated workspace
measurements. A lower primary peak does not imply negative added workspace.

## Implementation and resources

The actual loaded solver supplies HintBits output ranges. A small sorted/merged
range vector is merged with ordered B extraction indices; only1753/3166u32
positions are retained for1×2/2×2. Runtime never reads the private benchmark
position lists. Each proof guards every position and current canonical0/1 value
before changing any exclusively consumed scalar word. The original grouping
decision runs first; grouped calls keep their existing route. Ungrouped calls
join the original Phi MSM and an unweighted indexed point sum on the existing
Rayon pool, then add results. There is no point table, key copy or new pool.

Construction checks allocation capacities and uses fallible metadata allocation.
A Key-bound reservation forecasts the original workspace **plus** all concurrent
Boolean selected indices, first/partial trees and metadata. It remains live if
the normal solver schedule declines preparation. The normal schedule retains
its independent original forecast: baseline workspace is conservatively forecast
twice. This can deny the optional preparation earlier under memory pressure;
it does not omit simultaneous ownership. Preparation replacement and solver
invalidation drop the metadata and reservation. Explicit opt-in
`ZOPRO_CPU_BOOLEAN_QUERY=1` is required; unsupported geometry or denied admission
keeps the original path. No new artificial RAM ceiling was introduced.

The native SDK digest is
`e661251632f21c391c5ea1ffa55207e0d70be6990a2386d6fb6b02db5366d6ed`.
There are four isolated parent edits and one new child;175original private files
and all179production files remain unchanged. All BN254 package files match the
accepted manifest. The SDK uses the194 driver with Boolean-route observations
added, with the same native flags and link policy. Accepted194 remains digest
`bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1`.

## Validation and fixed screen

All138native core tests pass, including complete-query Arkworks-hash oracles,
identity/opposite/repeated-point cases, invalid guards,1/4/16/32worker small-stack
pools, real Plan metadata equality, sorted/overlapping range handling, capacity
overflow, explicit zero-budget denial, and ordinary/basis solver invalidation.
Unchanged field qualification is inherited. The first native compile stopped on
an unqualified G2Affine type in the new workspace estimator, before tests or
captures. The corrected snapshot qualifies that type only; both versions and
the original failure receipt are retained. No oracle, formula or gate changed.

Six4-worker smoke processes precede27main processes: three rotated/reversed
blocks of three contexts and accepted/OFF/ON arms. Each artifact has three
independently copied executable backings. Main processes use three primes and
17timed verified SDK calls; all main captures finish before late Gnark checks.
No owned compiler, analyzer or verifier ran during main capture. Route counters
confirm actual primaryON peeling, groupedON bypass and no OFF metadata/routes.
All33fresh exports pass Gnark;564SDK calls complete. The independent reader
checks66execution/verifier receipts,132stream hashes, source/backing pins,
statistics, route records and ledger disjointness. Its first execution passes;
all scientific steps after the documented compile repair pass first execution.

The protocol was frozen17:24:55UTC; result confirmed17:33:04UTC,8m09s within the
45-minute allowance. The short screen is not release qualification. Portable,
full-SDK functional concurrency, physical pressure/ARM and huge-circuit resource
coverage were not newly run after the latency selection failed. No minimum gain
floor is used; the condition requires a favorable primary paired median and at
least2/3wins. No unchanged timing retry or extra samples are planned.

Canonical distinct verified exports are now**58,948**:
`/home/ubuntu/src/zopro-perf-work/boolean-proof209/proof-sha256-after-main.json`,
SHA256`e63bea37a40fb006269ea4b5bf9209a558ec7a712a40827c349c0dfa0321365b`.
Raw inputs, points/scalars, proof bytes, full ledgers and binaries are kept out
of this report archive. Complete private evidence is at
`/home/ubuntu/src/zopro-perf-work/boolean-proof209`.

The accepted[local all-shape baseline](../2026-11-17-certified-all196/README.md)
and[C7a baseline](../2026-11-17-accepted-c7a201/README.md) remain unchanged;
0/8suppliedCPU10×targets are met. Aggregate CPU and isolated query timings do
not identify the whole-proof critical path. The next diagnostic should measure
actual branch start/finish boundaries with observer controls before another
scheduling intervention. This closes209's fixed recipe while preserving208's
component evidence and the bounded metadata implementation for future work.
