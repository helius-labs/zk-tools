# Sparse query and serial-leaf service trace on accepted175

Declared before implementation. Prior goal turn PROGRESS: new packed indexed
algorithm/three oracles/7902SDK78Gnark/profiles published63042fbe4b. It reduced
CPU/instructions but regressed paired latency+.826%vsaccepted/+.496%vsOFF. Closed.
Accepted175/SDK20fde remains1x2 8.022165ms/max134MiB; all8CPU10xactive/unmet0/8.

The old27Oct170source arithmetic trace found KH last7/9 graphs at32workers,
but predates accepted175 balancedKH. Reusing that attribution is insufficient.
New diagnostic copies exact175 and wraps CURRENT canonicalA query/borrowed
ordinary windows, fusedKH root/balanced20windows, exclusiveG2 query/phi windows,
and group/indexed/weighted/merge leaves. Whole-query scopes bind grouping.
Explicit integer parent IDs cross every added Rayon closure; never capture
non-Send Scope or inherit a waiting sibling's TLS parent into stolen work.

Record whole query/kernel/window/part wall spans; serial CPU clocks only around
nonforking feeds, pre-normalization tails, phi loops, and merge/group/indexed/
weighted leaf callbacks. No per-batch/reduce-layer/inverse event hooks. Includes
allocation/point movement/bookkeeping inside original leaf bodies, not isolated
field cycles. Query availability markers are entry points after canonical
arguments, G2 ready after decomposition; not executor enqueue timestamps.
Uncovered solving/FFT/scattering/assembly/verification remain outside these
serial service spans and must be described explicitly.

Reuse corrected old recorder semantics, rename service_trace, fixed1024 events
of80bytes per worker/caller, no growth on overflow, report dropped counts.
32workers =>at most2.579MiB payload, plus metadata/snapshots outside request
timers. This is diagnostic-only, not a universal product memory cap. Reuse two
tests for explicit parents/serial CPU/unwind/disabled mode and drop accounting.
Driver clears/snapshots outside each request; fixed graphs at0/15/30 for31round
runs and0 for one-round preflight. Retain all sample/drop counts. No PMU or
optimizer arithmetic/field/scheduling/key/plan changes; inverse patches recover
all175, field17exact/inherit33. Fresh126core (124original+2recorder) and8prover.

One tracing binary/customdriver has runtimeOFF/ON; retained accepted SDK is
separate standarddriver. All32workers/CPUs0-31 and same nativeflags/environment
except diagnosticcfg. Preflight all9OFF plus primaryON=40SDK/10freshGnark; semantic
primary graph check BEFORE main requires one API, rootA6/KH2/G25, actualborrowed
A7/ balancedKH8 /G2kernel3+ready4 and all window coverage, zero drops, valid IDs/
parents/containment/CPU clocks, no serial-with-serial overlap. Then3balanced
rotated/reversed blocks of accepted/OFF/ON,31observed+3prime each=306SDK/9Gnark.
Total346SDK/19freshGnark/38members;17786->17805 after all pins/binding/freshness.
All main captures before their independent verifiers. Main savedONgraphs9;
include preflight graph separately. No exclusions/extensions/successfulreruns.

Calibration all3 OFF/accepted block ratios withinfactor1.05 AND paired median
withinfactor1.03. ON/OFF paired median overhead<=1.05. Both needed for qualified
low-overhead32diagnostic; failure keeps instrumented observations descriptive
and does not permit new causal/speed claims. Fixed positions, zero main drops;
do not grow buffers or pick fastest graph. This does not claim16-core, all8,
AWS/Aeglos, huge/mobile/pressure/minRAM/throughput or product speedup. Preserve
full inherited all8CPU/Metal table/physical16baselineengineeringexception.

All helpers/code audit/pins built before captures; no owned build/helper/hash/
archive/other workload during observations. Source/caller semantic coverage
checked before main. Publish failures/limitations and commit/push. Next kernel
must address observed critical service rather than a totalCPU proxy. Goal full
all8CPU10x unchanged; diagnostic success is not goal achievement.
