# Bounded serial work inside expanded FFT leaves

Every 32-worker speed and regression condition passes, but the overall
screen remains unqualified: the physical-16 OFF control differs from
accepted by a factor of 1.040054, beyond the fixed 1.03 calibration bound.
That failure is retained. The candidate is preserved without adoption or an
unchanged timing rerun. The original all-eight tenfold CPU goal remains unmet.

## Change

The private module retains the existing 32-way packed FFT passes and the
expanded `Vec<Pair>` leaves of at most 1,024 vectors. Inside each expanded
leaf, binary forks stop at a power-of-two cutoff computed once per transform:
`floor_power_of_two(clamp(vectors / (2 * workers), 64, 1024))`.
The cutoff is carried through recursion. When there are at least 128 vectors
per worker, this exposes at least two terminal chunks per worker.

Terminal leaves call the existing DIF/DIT quad and within-pair arithmetic
in serial loops, without Rayon operations or heap allocation. They bypass
the nested multi-stage fused leaf execution; the outer packed 32-way passes
retain their kernels. An audit recovers the original OFF function body and
the 32-way geometry, expanded-vector allocation/load/store/drop after removing
only the experimental dispatches. This is a different boundary from the
[previous cheap-pass grain trial](../2026-10-26-fft-pass-grain/README.md).

Private cfg `zopro_serial_fft_leaves` and explicit
`ZOPRO_CPU_SERIAL_FFT_LEAVES=1` select ON. A process-local `OnceLock` reads
the option once; absence/zero select OFF. No new key table, buffer, copied
point array, plan, dependency or per-proof heap allocation is introduced.
Additional binary-fork frames hold references and scalar parameters, with no
large stack array. Scheduling can still change allocation lifetimes and peaks.

## Full SDK matrix

Ryzen 9950X; seven rotated/reversed fresh-process blocks, accepted versus the
same new executable OFF and ON. All nine shapes use 32 workers on CPUs 0–31,
31 measured requests after three primes. The 16-worker primary uses CPUs 0–15, one
logical CPU per physical core, and 101 measured requests. The larger 16-worker
sample was declared before implementation because earlier observations
varied across controls; its regression and calibration bounds were retained.

Loading and preparation are excluded. Fresh solving, randomness, proving,
internal verification and serialization are timed. All latency completes
before separate profiles. No owned build, profiler, CPU benchmark or bulk
audit overlaps latency. Times below are medians of process medians; ratios
pair corresponding blocks, so ratios of displayed medians differ.

| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF | M5 CPU / ON | M5 Metal / ON |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1x2-w32 | 8.646072 | 8.638903 | 8.604017 | 0.988177 | 0.993968 | 4.30x | 2.79x |
| 2x2-w32 | 11.695492 | 11.784155 | 11.684529 | 0.993532 | 0.987475 | 5.05x | 2.57x |
| 4x2-w32 | 16.197786 | 16.045586 | 16.006727 | 0.997432 | 0.992221 | 6.81x | 3.00x |
| 8x2-w32 | 24.908450 | 24.794680 | 24.750650 | 0.997198 | 0.988178 | 7.64x | 2.99x |
| 12x2-w32 | 34.611273 | 34.287845 | 34.310896 | 0.998111 | 0.991480 | 8.28x | 2.94x |
| 16x2-w32 | 40.898107 | 40.851396 | 40.722918 | 0.998905 | 0.990740 | 8.23x | 3.04x |
| 24x2-w32 | 68.546921 | 68.387492 | 67.392952 | 0.981242 | 0.990423 | 7.83x | 2.61x |
| merge-24x1-w32 | 67.352853 | 67.292361 | 66.294877 | 0.984292 | 0.982868 | 7.65x | 2.64x |
| merge-54x1-w32 | 177.089653 | 176.442303 | 173.395150 | 0.982445 | 0.981940 | — | — |
| 1x2-w16 | 11.018018 | 10.626684 | 10.545843 | 0.996463 | 0.984644 | — | — |

Primary at 32 workers wins 6/7 against both controls. The original-eight paired geometric
ratios are 0.990246 against accepted and 0.995147 against OFF. Merge-54 wins 7/7
against accepted and 5/7 against OFF. All 32-worker cases satisfy the fixed
bounds. The physical-16 case's ON/accepted and ON/OFF paired ratios also improve, but
its OFF calibration fails; more requests did not resolve that mismatch.
These observations do not establish its cause or qualify universal adoption.

The declaration requires positive primary 32 and original-eight geometric
results against both controls, at least four primary wins, every case within
2% paired-median and 5% per-block regression, and OFF calibration within 1.03.
No minimum gain floor or later exclusions are used.
[All failed checks and observations](artifacts/pilot-result.json).

The M5 CPU references are 37/59/109/189/284/335/528/507 ms and Metal references
24/30/48/74/101/124/176/175 ms in the supplied shape order. They are historical
references, not fresh matched-machine M5 runs. None of the eight candidate
results reaches tenfold CPU speedup. No M5 reference was supplied for merge-54.

## CPU time, memory and profiles

The following CPU values are medians of process-loop CPU per request; RSS
is the maximum ordinary latency-process peak and includes loading/priming.

| Case | CPU ms accepted | CPU ms OFF | CPU ms ON | Peak MiB accepted | Peak MiB OFF | Peak MiB ON |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1x2-w32 | 200.23 | 199.53 | 196.94 | 111.50 | 112.75 | 111.75 |
| 2x2-w32 | 271.52 | 269.67 | 269.76 | 274.25 | 273.75 | 268.75 |
| 4x2-w32 | 427.92 | 421.17 | 419.82 | 319.50 | 312.75 | 322.50 |
| 8x2-w32 | 684.82 | 681.02 | 683.88 | 515.25 | 524.75 | 514.00 |
| 12x2-w32 | 967.34 | 952.42 | 952.25 | 730.00 | 703.50 | 708.25 |
| 16x2-w32 | 1142.03 | 1146.46 | 1136.36 | 867.75 | 855.25 | 839.25 |
| 24x2-w32 | 1972.04 | 1961.85 | 1926.29 | 1310.01 | 1297.94 | 1351.50 |
| merge-24x1-w32 | 1874.42 | 1868.40 | 1846.96 | 1212.80 | 1189.66 | 1176.21 |
| merge-54x1-w32 | 4884.22 | 4821.20 | 4757.05 | 1892.96 | 1895.89 | 1907.81 |
| 1x2-w16 | 129.83 | 130.10 | 130.03 | 89.25 | 90.75 | 86.50 |

Nine separate 999 Hz monotonic-clock captures contain 300,972 samples, with
one process per capture and no lost records. Primary profiled CPU is
198.95 ms OFF and 198.02 ms ON; merge-54 is 4951.84 ms OFF and 4800.42 ms ON.
Primary epoch share falls from 9.41% to 8.42%, while the scheduler category
rises from 3.52% to 3.95%. Merge-54 epoch share falls from 2.21% to 1.75% and
scheduler from 2.18% to 2.04%; FFT remains about 42% of its sampled work.
The 12x2 capture does not show a CPU reduction:969.17 ms OFF versus 969.97 ms ON.
These are modest changes, not a broad CPU breakthrough. Inlining limits
attribution; sampled user-cycle shares are not removable wall time.
[Complete profile observations](artifacts/perf/report.md).

No peak establishes minimum RAM or pressure feasibility. The trial adds no
buffer, but measured peaks can still vary with scheduling and allocator
state. Portable, real memory-pressure, concurrent whole-SDK, large-circuit
and clean-integration qualification remain outstanding. There is no new
C7a, Aeglos, TEE or phone measurement here.

## Verification and retained failure

Native OFF and ON each pass all 112 core tests. Three new tests check cutoff
bounds, independent scalar-polynomial inverse coefficients before forward
identities, ordinary/twisted roots and admitted cached directions, concurrent
callers sharing roots, and 512 KiB worker stacks. Raw-image tests pass two and
cubic tests pass six. Field kernels are unchanged; the prior 33-test field
qualification is inherited, with no new field-suite run claimed.

The first OFF suite passes 111 tests and fails a test that assumes both
optional root caches were admitted. Clearing column vectors retained their
reservation tokens, and two peak builders need not fit in the existing
preparation budget. The correction changes only test fixtures: replace the
whole roots object to release storage and credit, exercise cached forward,
release it, then exercise cached inverse. No budget or runtime algorithm
changes. The failed source, logs and executable hash remain; the original
executable is retained locally. The final build uses a new source snapshot
and target directory. No SDK or proof timing preceded that correction.
[Failure review](artifacts/failure-v1/review.json).

All-nine OFF/ON preflight at four workers completes 72 SDK requests and 18
independent Gnark exports. Fixture/freshness validation runs inside that
workflow and again as a required perf check before timing. Latency adds
8,610 requests and 210 exports; profiles add 510 requests and nine exports.
Total: **9,192 SDK requests, 237 distinct verified exports and 474 proof/public
members**. Public fixtures and fresh proof identities pass against 3,045
prior exports; the 18 preflight identities also guard timed/profiled exports.
The final review rehashes every member and checks run/verifier exit codes
and stdout/stderr receipts. All 460 performance input pins pass.

The final candidate has 173 source pins:170 accepted files, the frozen
CLI/helper, and the private module. Source, actual linked libraries, binaries,
images, original verification-key inputs, fixtures, verifier and orchestrator
are pinned. Accepted production's170-file manifest remains unchanged.
No AWS resource is created by this experiment.

[Protocol](artifacts/experiment.json), [closeout](artifacts/closeout.json),
[source audit](artifacts/source-audit.json), [patch](artifacts/source.patch),
[build](artifacts/native-v2/result.json),
[proof inventory](artifacts/proof-inventory.json),
[next design to audit](artifacts/next-design.md).

Full local experiment: `/tmp/zopro-serial-fft-leaves/`.
Raw perf captures and expanded symbol streams are retained locally, omitted
from Git, and included in the artifact hash index. Two existing source test
fixtures are hashed separately from the 474 new export members. Executables
and compiler caches are represented by source/build/link and failure receipts.
