# Batch transfer from local worker queues

Necessary primary SDK screen: **fail**. Batch transfer is **2.18% slower than OFF in the same executable at 32 workers**, with 0/7 paired wins and an individual median interval of [1.016394, 1.038101]. The candidate is rejected. Production remains unchanged, and the original goal of 10× the CPU reference across all eight shapes remains unmet, 0/8.

The fresh OFF executable also fails calibration: it is 8.11% slower than the fresh source control at 32 workers. The larger ON/control difference cannot be attributed entirely to batch transfer. At 16 workers, ON/OFF has a 0.69% positive median estimate with 4/7 wins, but its wide interval fails the nonregression bound. All observations are retained. This is a complete-proof scheduler screen, with no all-shape qualification or adoption.

The private Rayon-core1.13.0 prototype uses Crossbeam-deque0.8.8's existing batch-and-pop API on a successful victim, then advertises remaining local jobs through the original callback. Explicit ZOPRO_CPU_BATCH_LOCAL_STEAL=1 is sampled per registry; OFF/default uses the original single-item extraction. Victim selection, retries, injector/broadcast paths, fences, epoch reclamation, idle rounds, latches and cryptographic source retain their implementations. The upstream32-item transfer bound is unchanged. No unsafe production code, proving-key table, duplicate plan, new pool or proof cache is added. Changed task placement may alter memory lifetimes and wakeups.

This differs from October25 guard batching, which reused a guard across victim probes but still returned a single job. Earlier empty-probe, retained-buffer, St3, idle and shared-ready failures remain closed. The prior C7a profiles motivate this experiment, but no C7a or Aeglos timing occurs here.

## Complete verified SDK requests

Ryzen 9950X, seven rotated/reversed fresh-process blocks. All variants use the same hash-pinned Euler image. Control is freshly built from the exact accepted 172-file source and flags; OFF/ON share a new executable. The historical accepted SDK file is unavailable and is not relabeled as this control. Primary32 uses CPUs 0–31; primary16 uses one logical CPU per physical core on CPUs 0–15. Each has 151 timed calls after three primes. Loading and preparation are outside timing; solving, fresh randomness, proving, internal verification and serialization are inside. No owned helper, build, hashing, archiving or second CPU workload overlaps timed requests.

|Workers|Variant|Median SDK ms|CPU ms/request|Peak RSS MiB|
|---|---|---:|---:|---:|
|32|control|8.358001|191.423|132.75|
|32|off|9.050477|204.256|133.25|
|32|on|9.300137|211.960|137.00|
|16|control|10.091820|123.940|90.00|
|16|off|11.566701|138.889|91.25|
|16|on|11.487321|142.686|92.25|

CPU sums user+system over the warm loop, including search and coordination. RSS is whole-process high water through loading/primes, not minimum RAM.

|Comparison|Paired median ratio|Individual median interval|Wins/7|
|---|---:|---|---:|
|w16:off/control|1.144118|[0.950975,1.202357]|1|
|w16:on/control|1.135536|[0.944448,1.306084]|1|
|w16:on/off|0.993137|[0.944425,1.169923]|4|
|w32:off/control|1.081091|[1.076904,1.086817]|0|
|w32:on/control|1.107286|[1.101398,1.117935]|0|
|w32:on/off|1.021778|[1.016394,1.038101]|0|

Failed checks: **6**. Frozen criteria require OFF/control intervals wholly withinfactor1.03, primary32 ON/control and ON/OFF upper bounds<1 with≥5/7wins, and primary16 upper bounds≤1.02. Individual median intervals use all seven ordered block values,98.4375% nominal coverage conditional on independent/exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusions, extensions or unchanged rerolls. Positive estimates remain in the analysis.

## Profiles and validation

Six separate199Hz FIFO-gated user-cycle profiles follow all latency. All six captures finish before decoding; required independent Gnark runs occur serially after each SDK process. Counters enable after loading/three primes and disable before export. One sampled PID, positive samples and zero lost records are checked per capture. Flat symbol shares are not removable wall-time fractions.

The profiles contain 31,592 samples in total. Primary32 sampled epoch share falls from 9.95% OFF to 8.43% ON, while scheduler share rises from 4.25% to 5.40%. Unprofiled CPU time rises from 204.26 to 211.96 ms per request; voluntary switches rise from 429.71 to 446.25 per request. These accompanying changes do not isolate the cause of the slowdown. The local result supplies no evidence that transferring more jobs from a victim solves the C7a scaling problem.

|Profile|Samples|Epoch %|Scheduler %|
|---|---:|---:|---:|
|profile-w16-control|3168|2.47|1.59|
|profile-w16-off|3811|3.18|1.82|
|profile-w16-on|3959|4.11|2.84|
|profile-w32-control|6649|10.19|4.63|
|profile-w32-off|6906|9.95|4.25|
|profile-w32-on|7099|8.43|5.40|

Runtime validation passes105unit and9integration tests, with two upstream ignored tests retained. Four new tests cover transfer bounds/order,32,768unique items with1/2/8thieves, nested scopes/FIFO/panic recovery and concurrent callers. Candidate119native core,2raw and6cubic tests pass. The unchanged33field-test qualification is inherited, not rerun. Root lock audit preserves all219package entries except Rayon-core's source/checksum, retaining218identical entries and every version/edge. Actual linked libraries and source are pinned before/after capture.

Initial preparation failed an exact-source recovery assertion due to an extra blank line. A test-only type inference error then stopped compilation before tests. The pristine upstream development lock selected older Crossbeam versions; they were aligned with the real runtime before any successful test. A lock-audit helper initially conflated duplicate package names and was corrected to include versions. A workflow schema error used numeric environment values and was corrected before native compilation. Failed sources/logs and repairs remain; no successful tests or SDK captures were repeated.

The independent verifier uses pinned Gnark0.16.3/gnark-crypto0.21.0 and extracts the verifying key from the hash-pinned Euler image. Two new tests cover malformed headers and a known archived proof plus an altered-proof negative control. Reusing that old proof qualifies this new verifier and adds no fresh export to the ledger. A Go toolchain-launch checksum restriction was resolved by using the already installed pinned toolchain directly.

Three four-worker preflights contribute12SDKcalls/3exports. Timing contributes6,468calls/42exports; profiles924calls/6exports: **7,404complete internally verified SDK requests,51distinct fresh independently Gnark-verified exports,102proof/public members**. Public bindings and freshness against12,467prior/new hashes pass. Ledger **12,467→12,518**. No actual AWS resources were created.

A passing screen would still need all-nine performance, portable, concurrency, pressure, phone/TEE and large-circuit qualification before adoption. This screen cannot establish a universal memory bound or whole-objective completion.

## Inherited CPU and Metal comparison

The [accepted172-file comparison](../2026-10-27-clean-fft-adoption/README.md) remains authoritative. Figures below are inherited local observations against supplied historical M5 references, not new all-shape measurements from this screen.

|Shape|Accepted ms|M5 CPU ms|CPU/accepted|M5 Metal ms|Metal/accepted|
|---|---:|---:|---:|---:|---:|
|1x2|8.324853|37|4.445×|24|2.883×|
|2x2|11.460675|59|5.148×|30|2.618×|
|4x2|15.854365|109|6.875×|48|3.028×|
|8x2|24.084626|189|7.847×|74|3.072×|
|12x2|33.693397|284|8.429×|101|2.998×|
|16x2|39.981636|335|8.379×|124|3.101×|
|24x2|66.010670|528|7.999×|176|2.666×|
|merge-24x1|65.086624|507|7.790×|175|2.689×|

Accepted merge54 remains161.341835ms with no supplied M5 reference. The existing full-FFT performance-gate failure and engineering exception remain.

[Design](artifacts/design.md), [declaration](artifacts/timing-declaration.json), [all observations](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [source pins](artifacts/pins.json), [artifact hashes](artifact-index.json).
