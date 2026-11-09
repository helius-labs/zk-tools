# Exact-query work removal: first full-SDK diagnostic capture

This is a diagnostic, not a proving optimization. Cached query durations
cannot count toward the 10x goal or fresh-work M5/Aeglos speed. Production,
accepted SDK6d83 and the qualified field1200 remain exact. Native state and
core/source/driver qualification are inherited from query-neutralization176,
commit34fd5a5d7f. Successful suites and the seven decoded root bodies are
not replayed. Finish inspecting actual Rayon callback bodies before clocks.

Ten arms: accepted, OFF (fresh inactive native control), m0 (active/no query
removal), m1..m7. A=1,G2=2,KH=4. One session per process. Prime1 runs original
queries and learns; prime2/3 and all measured requests use the fixed mask.
The mask scope, key/witness matching/hash/lookup and original remaining work
are included in each full SDK request. Output proof retains fresh blinding
and native verification. Report priming and timed query coverage separately.
Payload576B, controller944B before Arc/allocator; no arrays/key copies.

Preflights: ALL8+merge54 at32 workers and primary1x2 at16, all ten arms,
100 processes, each3primes+1timed. Timings excluded from comparisons.
Each active process must show zero faults/declines/misses, exact requested
hits/original computations, path/topology and entry-hop counts. Full-shape
coverage cannot be inferred from the native cubic test. The SDK verifies
inside each call; independent Gnark checks occur after ALL recording to
avoid overlapping verifier CPU work. This resolves the earlier design's
ambiguous 'Gnark before latency' phrase in favor of its explicit rule that
all recording precedes offline verification. Any correctness/coverage
failure invalidates the affected experiment; retain failures, no reroll.

Latency: contexts1x2-w16,1x2-w32,4x2-w32;15matched blocks each,31timed
requests after3primes for every arm. 450 processes. Rotate context order
left by block mod3, reverse on odd3-block groups. For context index i,
rotate ten-arm order left by(block+i) mod10, reverse on odd10-block groups.
All100preflights, then ALL450latencies, then ALL60PMU/profiles, then offline.

Controls: paired process means. For each context, bootstrap paired log ratios
with20,000 resamples, SplitMix64seed0x551c36a48ee2d971, sorted ranks499/19499.
OFF/accepted calibration AND m0/OFF observer intervals must both lie within
[1/1.03,1.03]. Do not drop16workers or relax limits after seeing data.
Retain median process means, geometric paired ratios, intervals, wins and
all15raw ratios for every mask against m0. Calibration failures weaken
causal interpretation; they never select a faster observer or new trial.
No shortened-arm adoption/gain gate. Effects are descriptive for the changed
schedule, not additive exclusive fractions or an immutable lower bound.

Resources: native whole-process RSS maxima and timed user/systemCPU, minor/
major faults, voluntary/involuntary switches perrequest; masks are not given
memory caps. Report relative to m0, with no minimum-RAM/mobile inference.
CPU affinity0-15 or0-31 and RAYON_NUM_THREADS16/32, original production env,
exact pinned basis/Lagrange/image/request inputs. No new runtime pool.

Automatic diagnostics: each of3contexts×10arms gets one gated grouped PMU
panel (31timed) and separate warm cycles profile (501timed), both3primes.
cycles:u,instructions:u,cache-misses:u,branch-misses:u; counterrunning>=99.9%.
Profiles period16,000,000cycles,512mmap pages;>=1000samples,>=95%assigned to
actual process/pool threads,<=5%unknown,zero LOST. Report all qualityfails,
no extension/selection. One panel/profile has no repeated-PMU uncertainty
and does not establish causal DRAM/cache/spill stalls or criticalpath time.
Allgating/inputidentity/PID/worker/CPUcoverage must be checked after capture.

610new exports, one LAST proof perprocess (not every SDKrequest):
400preflight SDKrequests+15300latency+1020stat+15120profile=31840 total,
including priming. Seed exactcanonical37023ledger fromtree-policy-sdk176;
610fresh unique exports independentlyGnarkverified -> expected37633.
All610verifiers then150profile ownership/decode/events/metadata/loss steps.
No new process/clock during analysis. Frozen prospective/results reader,
analyzer, fullplan, binary/inputpins and resource/statistical limits exist
beforeANYpreflight. SerializedRust/xtask. DuringANYrecording ONLYPOLL exact
confirmedlivehandle; nootherownedCPUwork/build/helper/hash/decoder/verifier/
report/archive/push. No rerun of successful old tests/verifiers/decoders.

Next optimization must remove real work identified by this study; never
adopt cached-query timing as success. Fullgoal stays active0/8: verified
singleproofCPUAVX>=10x suppliedM5CPU all8 andCPU+Metal comparisons. Native
qualification/report alreadypushed; commitANDpush this capture regardless
ofperformance outcome. No budget/questions/subagents/cloudresources.

Harness resource addition before any recording: copy the qualified driver
exactly and add only ru_majflt to the support module's output JSON. Relink
OFF/ON consumers against the exact qualified libraries; no library or suite
rebuild/replay. Capture pins bind both new consumers and their source/link
receipts. The accepted immutable6d83 consumer lacks majorfault reporting,
so its majorfault field is unavailable; never infer zero. Other arms report
that actual counter. Inspect the newly linked callback bodies as well.
