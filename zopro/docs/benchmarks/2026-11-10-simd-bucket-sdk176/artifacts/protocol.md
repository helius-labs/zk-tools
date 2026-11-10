# SIMD bucket admission in complete SDK proofs

The native SIMD admission implementation is qualified and pushed at
46ede153a3d380855c7f860696bf7a3a66677804. Its130native tests,704new curve
Windows/45056Arkworks bucket outputs,66626predicate cases, feature-disabled
compile, actual optimized SDK links and source/code review are inherited.
Do not rerun them for bookkeeping. No performance result exists for it.

The candidate conservatively admits eight independent ordinary updates using
collective index-conflict, busy-bit and first-X-limb checks. Rejected packets
retain original scalar transitions and reuse their extracted digits. Field,
batch boundaries, queues, digits, windows, solver, FFT, allocation policy and
pool remain original. Fixed stack arrays add no heap/table/key/solver plan.
Code and rejection work can outweigh dispatch savings; no gain is predicted.

Measure actual complete SDK requests, which cover every complete query and its
interaction with solving/FFT/assembly. REF is the exact accepted SDK7aff from
busy-reset-sdk176. OFF and ON are the already built SDKs in
simd-bucket-admission176, SHA b08a3ed139614350e9194d81b284a657d50b6abbd3fba5377b09262bf2a3839e
and692ec7fbbefc4fefe63c8adb85357d6778ece7adc1c49124c92fc17f0992ce6d.
Do not copy or rebuild them. Both use original field1200 and dense busy reset;
only ON enables packet admission. All use the identical placement driver,
native/O3/fatLTO/oneCGU, fixed per-worker affinity and512KiB requested stacks.

Before performance, all nine shapes at32workers plus primary at1/16workers
must pass OFF/ON SDK preflight and fresh independent Gnark verification:
22processes, three primes and one observed/exported request each,88requests.
Bind image/request/public/proof/VK identities and unique fresh proof hashes.
Seed ALLexports from frontend-overlap176/proof-sha256-after-capture.json,
canonical44265; successful preflight advances to44287, capture to44800 only
after actual verification/inventory checks. These counts are requirements,
not assumed results. Preflight clocks select no performance result.

Prospective matched comparison: ALL8+merge54 at32workers and primary at1/16;
15fixed rotated/reversed blocks, all3arms in all11contexts.31observed requests
per process after3primes.495latency processes. Unit is each process arithmetic
mean of integer-rounded SDK nanoseconds; paired ratios use matching blocks.
Sorted zero-based ranks3/7/11 give bounds/median, nominal96.484375%marginal
coverage conditional on independent exchangeable blocks; no familywise or
stationarity guarantee. No gain floor, exclusions, extensions, rerolls,
reordering, favorable subset, worker/parameter rescue or changed threshold.

Freeze all77checks:11OFF/REF calibration intervals within[1/1.03,1.03];
22ON/control nonregression upper<=1.02; primary32 and equal-log ALL8 upper<1
and>=10/15wins against BOTHcontrols;22RSS guards ONmax<=1.05*controlmax+1MiB;
and9PMU+9profile quality checks. Merge54 has no M5reference and is an explicit
nonregression/resource condition. Every failure and positive observation stays.

After ALLlatency, record fixed grouped PMU and cycle profiles for primary16,
primary32 and2x232, all3arms. Stat31requests/profile251requests after3primes;
FIFO gates cover observed calls. cycles:u/instructions:u/cache-misses:u/
branch-misses:u grouped,>=99.9%running. Profiles16Muser-cycle period,512pages,
>=1000samples,<=5%unknown, zeroLOST, exact PID/TID/CPU/MMAP/event ownership.
One-shot counters/profiles are descriptive, not exclusive costs or causes.
After all recording, decode each new recording once and independently verify
all513capture exports.1071capture commands:495latencies+18diagnostics+
45offline+513Gnark.19422capture requests plus88preflight=19510;535unique
exports including22preflight. Export count differs from internally made proofs.

Source/compiler/deps/binaries/inputs/seed/order/gates/producer ANDseparate
raw reader must be frozen BEFOREfirst preflight. During ANYpreflight/recording
ONLYpoll exact confirmed live handle until terminal. Timeout/truncation is not
terminal. No overlapping owned helper/build/hash/analysis/report/push/workload.
Analyze only after recording is terminal. Preserve failed stages; no successful
native test, build, oracle, capture or decoder replay for bookkeeping.

Native/source qualification does not adopt production. Full verification,
resource and portability/concurrent-caller limits remain explicit. RSS includes
loading/primes/allocator history and is neither minimumRAM nor pressure/swap
qualification. Generic/CD-disabled compile is not ARM/mobile execution.
Report CPU ANDMetal comparisons for ALL8 and retain original verified CPUAVX
>=10xM5CPU objective ACTIVE/UNMET until actually achieved. Current accepted
primary32 is7.783593581ms; all8targets remain unmet. No new AWS/Aeglos run.
Commit ANDpush completed experiments with polite factual reports.
