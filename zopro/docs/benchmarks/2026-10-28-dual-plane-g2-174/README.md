# Joint G2 bucket-plane SDK screen

The candidate failed its declared primary latency gate and is not adopted. On this Ryzen 9950X host, 1x2 with 32 workers takes 8.453546 ms versus 8.094728 ms for fresh OFF and 8.076584 ms for the accepted executable. The median paired ON/OFF ratio is 1.046468, with bounds [1.042416, 1.050465] and zero wins in 15 blocks. Production source174 and its accepted executable remain unchanged.

## Change and scope

The private candidate fills separate low/high G2 phi bucket planes in one traversal of immutable bases. Both planes share the existing borrowed pending/collision state, with twice the buckets and batch capacity. It transforms only the high bucket sums, then merges them into the ordinary low output in serial 512-point chunks. It retains complete exceptional-point handling, the narrow indexed top window, scalar decomposition, field kernels, query widths/parts, G1, solver and scheduler. It creates no transformed input-point cache or per-key lookup table.

One checked reservation per query covers all potentially overlapping two-plane leaves and scratch, including normalization, output and merge buffers. Existing proof forecasts remain conservatively charged. Denial selects the original implementation. The returned vector has ordinary half-plane capacity; temporary doubled capacity drops while its reservation is live. This is an accounting mechanism, not a hard RSS or minimum-RAM guarantee. Ordinary allocations after admission retain baseline allocation behavior.

The first draft reserved per leaf. Source review found that this would repeat OS-headroom reads under the admission lock. Its tests/builds completed, but no performance capture ran. The once-per-query revision was independently rebuilt and tested before this fixed capture. Both versions and their receipts are preserved. No outcome-selected source or protocol change occurred during capture.

The 175-file current snapshot has 172 byte-exact original files, two reversible parent changes and one new module. Original field arithmetic is unchanged. OFF is separately rebuilt and has a different hash from accepted; calibration is measured rather than assuming binary identity.

## Measurements

Fifteen fixed rotated/reversed blocks compare accepted, OFF and ON. Each process executes three primes followed by 151 complete SDK requests, including solving, fresh blinding, proving, internal verification and serialization. Loading is excluded. CPU affinity is 0–31 with 32 Rayon workers on 16 physical cores. The host is not claimed exclusive.

| Variant | Median block latency ms | CPU ms/request | Median process peak MiB | Maximum process peak MiB |
|---|---:|---:|---:|---:|
| Accepted | 8.076584 | 186.575 | 132.25 | 136.00 |
| Fresh OFF | 8.094728 | 186.444 | 130.50 | 135.00 |
| ON | 8.453546 | 194.144 | 131.00 | 133.75 |

| Paired comparison | Median ratio | Bounds | Wins/15 | Decision |
|---|---:|---|---:|---|
| OFF/accepted | 1.003499 | [0.992697, 1.008761] | 7 | Calibration passes |
| ON/OFF | 1.046468 | [1.042416, 1.050465] | 0 | Fails |
| ON/accepted | 1.047085 | [1.044094, 1.051549] | 0 | Fails |

Bounds are sorted paired-ratio ranks 4–12 of 15, nominal 96.484375% coverage per comparison, without a familywise guarantee. The gate requires both candidate upper bounds below 1 and at least 10 wins; calibration bounds must lie within [1/1.03, 1.03]. No gain floor, threshold change, extension or repeated capture was introduced. Paired median savings are negative: −0.372423 ms against OFF and −0.381661 ms against accepted. These differ from differences of unpaired median latencies.

Candidate median process peak rises slightly while its maximum falls. This mixed result establishes neither a memory improvement nor a minimum RAM requirement. No larger-shape qualification follows the failed primary gate.

## Profiling and interpretation

After latency, three FIFO-gated four-event counter runs and three separate 199 Hz cycles profiles cover the three variants. Counters use 31 requests and profiles 251, after three primes. Loading/primes are outside their gate. All six captures precede their verification and decode; diagnostic timings do not enter latency decisions.

| Per-request event | OFF | ON |
|---|---:|---:|
| cycles:u | 855,099,044 | 889,081,543 |
| instructions:u | 1,455,142,141 | 1,471,287,763 |
| cache-misses:u | 2,979,754 | 3,236,742 |
| branch-misses:u | 1,239,003 | 1,258,938 |

All counters run 100%. Profiles contain 9,150 accepted, 9,448 OFF and 9,456 ON samples, one PID each and zero lost records. Candidate aggregate CPU time and cycles increase along with latency. These observations do not identify a particular cache, scheduling or RAM-capacity cause.

No separately named `dual_plane` symbol appears in the decoded ON samples. Compilation, source pins, unit tests and the integration build establish candidate construction, but these profiles alone do not establish dynamic reservation acceptance or the fraction of windows taking the new route. Inlining and fallback remain attribution limitations. We reject the measured executable; we do not claim a measured inversion-count saving or a fully isolated explanation for its regression. No new instrumented rerun is presented.

The source-level opportunity was to amortize inversions and avoid repeated base transforms. Sharing traversal and batches did not produce a whole-proof improvement in this implementation. A later approach needs a distinct, testable reduction in actual proof work; neither larger lookup tables nor tuning this failed run's thresholds follows from these results.

## Validation and artifacts

Current native OFF passes 122 core tests; ON passes 124, including two new tests for complete bucket sums, exceptional points, batch tails, capacity, checked costs and fallback. Each passes eight prover/raw-cubic tests. Field source is byte-exact and its inherited 33-test qualification is reused, not rerun or counted as new inputs. The superseded draft's successful validation is separately preserved and is not counted as additional current fixtures.

All 117 capture steps complete. All 7,806 SDK requests verify internally. All 54 fresh 164-byte exports independently verify with Gnark, have correct public-input bindings, and are distinct from the prior proof inventory. All 108 proof/public members are archived; the verified ledger advances 15,757→15,811. All closing source, library, helper and input pins match.

Full source, manifests, licenses, commands, tests, raw observations, proof exports and decoded diagnostics are included. Three raw perf recordings remain local with SHA256 receipts; only ownership of those owned recordings changes after capture for hashing. No owned build, helper, archive or second benchmark overlapped captures. There is no new AWS/Aeglos comparison, phone/ARM/TEE, giant-circuit, portable, concurrent-proof, swap or forced-pressure qualification.

## Historical CPU and Metal references

The supplied M5 CPU/Metal 1x2 references are 37/24 ms. The fresh accepted control is 4.581×/2.972× faster, OFF 4.571×/2.965×, and ON 4.377×/2.839×. These are cross-machine historical reference ratios, not fresh M5 captures.

The complete accepted table below is inherited from the [source174 adoption](../2026-10-28-g2-borrowed-adoption/README.md), including its failed performance gate and explicit engineering exception. This primary-only screen does not replace that all-eight qualification.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.366931 | 37 | 4.422× | 24 | 2.868× |
| 2x2 | 11.539335 | 59 | 5.113× | 30 | 2.600× |
| 4x2 | 16.540233 | 109 | 6.590× | 48 | 2.902× |
| 8x2 | 25.136722 | 189 | 7.519× | 74 | 2.944× |
| 12x2 | 34.858408 | 284 | 8.147× | 101 | 2.897× |
| 16x2 | 41.377043 | 335 | 8.096× | 124 | 2.997× |
| 24x2 | 67.764001 | 528 | 7.792× | 176 | 2.597× |
| merge-24x1 | 67.096516 | 507 | 7.556× | 175 | 2.608× |

The original all-eight CPU 10× goal remains active and unmet, **0/8**. Qualified primary remains 8.366931 ms. No new accepted performance result is claimed.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [current native completion](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [closing pins](artifacts/pins.json), [capture completion](artifacts/runs/result.json), [research and history](artifacts/research.md).
