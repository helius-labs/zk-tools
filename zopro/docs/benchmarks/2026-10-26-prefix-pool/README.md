# Preparation before full-pool entry

The eight-worker preparation-pool candidate does not pass the frozen local nonregression gate. Production remains unchanged. This is a local Ryzen9950X screen; there is no new C7a or Aeglos measurement.

The change performs solver, public correction, canonical conversion, gathers and product FFT in a shared eight-worker pool before entering the global32-worker arithmetic pool. Existing A/fusedKH/G2 kernels are unchanged. Existing wire/scalar owners move without buffer copies; one proof scope covers both stages. Explicit opt-in and geometry/core/custom-pool/concurrency guards retain fallbacks. This avoids early full-pool entry for this proof, but does not guarantee previously active global workers are asleep.

Eight2MiB stacks plus128KiB/worker bookkeeping allowance reserve a17MiB lifetime forecast under the existing resource policy. The allowance is an estimate, not an upper bound. A successful pool remains resident under later pressure. There are no key tables or new universal RAM caps. These costs and untested concurrency/pressure behavior prevent treating this screen as universal qualification.

## Request latency and memory

32workers/CPUs0–31, three rotated/reversed fresh-process blocks,31timed requests after3primes. Each latency below is the median of three process medians. Paired ratios use within-block geometric means. RSS is maximum whole-process RSS including load/setup/primes, not minimum RAM. CPU includes coordination/spinning and excludes priming through driver snapshots.

|Shape|Accepted ms|OFF ms|ON ms|ON/OFF|ON/accepted|Accepted/OFF/ON RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.556801|8.649927|8.790092|1.015128|1.024139|114.0/117.5/74.8|
|2x2|11.722253|11.711222|11.616533|0.993777|0.992804|277.0/278.5/266.8|
|4x2|16.100783|16.121251|15.976338|0.987069|0.992555|312.0/307.5/308.8|
|8x2|24.711576|24.467936|24.597189|1.003631|1.004669|502.0/507.8/507.8|
|12x2|34.732339|34.372129|34.574571|0.991977|0.991448|732.0/719.5/702.0|
|16x2|40.845490|40.754459|40.693394|1.008426|1.003387|853.5/844.0/842.8|
|24x2|68.758215|68.864465|68.645743|1.004876|1.006960|1259.3/1308.2/1310.8|
|merge-24x1|67.509419|68.040402|68.974954|1.013327|1.022582|1194.0/1172.3/1202.2|
|merge-54x1|177.270822|178.099866|175.559284|0.993198|0.996680|1825.0/1937.8/1842.9|

## Supplied M5 references

These are historical supplied reference timings, not same-host comparisons. No54→1 M5 reference was supplied.

|Shape|ON ms|M5 CPU/Metal ms|CPU/Metal speedup|
|---|---:|---:|---:|
|1x2|8.790092|37/24|4.21×/2.73×|
|2x2|11.616533|59/30|5.08×/2.58×|
|4x2|15.976338|109/48|6.82×/3.00×|
|8x2|24.597189|189/74|7.68×/3.01×|
|12x2|34.574571|284/101|8.21×/2.92×|
|16x2|40.693394|335/124|8.23×/3.05×|
|24x2|68.645743|528/176|7.69×/2.56×|
|merge-24x1|68.974954|507/175|7.35×/2.54×|

0/8 meet CPU10×. The original goal remains active and unmet.

## Qualification and verification

Calibration requires OFF/accepted within factor1.05 per block and1.03 paired median. Each ON/OFF and ON/accepted paired geometric mean must be<=1.02; original-eight aggregates<=1.00. No local minimum primary gain is required because the mechanism targets observed C7a prefix scaling. Original-eight ON/OFF=1.002230,ON/accepted=1.004745.

Calibration failures:

- None.

Nonregression failures:

- 1x2-w32 ON/accepted=1.024139030535781
- merge-24x1-w32 ON/accepted=1.0225815971503573

Both original-eight aggregate limits also fail: ON/OFF=1.002230 and ON/accepted=1.004745 exceed1.00. Calibration passes; the candidate still fails the frozen gate.

## What this changes in the investigation

Only1×2 takes the new route in preflight; the other eight shapes retain the existing path. Their small timing differences cannot be credited to the preparation pool. Primary ON/OFF paired latency regresses1.51%, with zero of three block wins, and ON/accepted regresses2.41%.

Primary summed CPU falls from199.61ms/proof OFF to178.39ms ON, about10.6%, while whole-process RSS falls from117.5 to74.75MiB. These observations do not establish lower minimum RAM: extra worker stacks remain resident, and allocator retention is not isolated. Lower total CPU consumption does not imply a shorter critical path. The profiles sample33distinct TIDs for accepted/OFF and41forON, consistent with the additional workers.

This result does not establish that pool handoffs caused the regression or rule out a benefit on96-coreC7a. It provides no qualified reason to promote this candidate or run it unchanged on AWS. The next useful step is to identify which preparation stages actually lengthen the proof, rather than infer latency from aggregate scheduler CPU. A separate experiment must declare its instrumentation, memory cost and gates first.

Profile categories are descriptive symbol matches, not exclusive stages. The inherited `grouping` matcher also includes `prefix_pool` symbols in this experiment; its percentage must not be interpreted as pure scalar-grouping cost. Raw symbol periods are retained.

Native112core tests, raw2/cubic6 and generated IFMA check pass. Field arithmetic is unchanged and its prior qualification is inherited. All2988SDK requests completed;102 distinct exported proofs independently pass Gnark, fixture public hashes,164-byte length and freshness against5826 prior identities (next prior5928). Three profiles contain9219samples with zero loss and oneSDKPID each. Profiles run after all latency processes; all helper binaries were built before capture. No successful capture is repeated or excluded. Route traces are confined to preflight; [route audit](artifacts/route-audit.json) records which shapes used the pool.

No target, phone, TEE, giant-circuit, memory-pressure, swap or concurrent-throughput qualification is added. Source and SDK pins remain unchanged in production. No AWS resources were created. A local pass permits a separately frozen target experiment; it does not adopt this implementation.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [rows](artifacts/performance/rows.json), [config](artifacts/perf-plan.json), [proof inventory](artifacts/proof-inventory.json) and [artifact index](artifacts/artifact-index.json) retain results. Candidate source is archived. Binaries, large keys and raw profiles remain local with hashes.

