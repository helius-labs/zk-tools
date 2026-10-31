# Pre-capture duration revision

This document supersedes only the provisional latency pass counts in design.md.
No performance capture or preflight has run for this candidate. Native tests
have passed; the final matched binaries are being built. The prior studies'
threaded20-pass samples lasted roughly20ms and showed large block excursions.
To improve observation quality, use20passes solo and200passes at16/32workers,
fixed for both shapes and all three arms. Expected durations are roughly
200–330ms rather than20ms for threaded cells; these are expectations, not new
measured candidate results. There is no duration-based stopping or extension.

Everything else remains fixed:15rotated/reversed blocks, three checked primes
and one checked post,18one-pass preflights, six grouped stat panels and six
fixed16Mcycles:u profiles at200passes primary16/32, then30read-only decode/
ownership/metadata/events/loss commands. All latency precedes diagnostics.
There are still330commands and300caller processes;90solo latency processes
times24passes,180threaded times204,18preflights times5,12diagnostics times204
give **41418completepasses**,869778windowfills and51609600checkedprime/post
bucketoutputs. The90solo/180threaded latency processes time37800passes total.

All calibration, nonregression, paired confidence ranks, two-shape aggregate,
wins, resource caveats, affinity/stack, profiling qualifications and no-owned-
overlap requirements in design.md remain unchanged. No minimum gain floor,
exclusions, favorable subset, protocol selection after observing candidate
latency, successful rerun, parameter sweep or unchanged-algorithm rescue.
Native source, flags and existing qualification remain exact. This is a
pre-measurement change to sample duration, not a new prover algorithm.

capture-revision.rs preserves the provisional campaign/analysis/review sources
before updating expected counts and per-pass divisors. The final plan and all
three helpers are frozen before the first performance execution.
