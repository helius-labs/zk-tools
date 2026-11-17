# Serial G2 grouping: CPU versus elapsed time211

The roughly0.74ms grouping-time shift seen in210is predominantly **running CPU
time**, rather than a shorter blocked/descheduled wait. All three paired blocks
show this pattern. It does not establish fewer instructions, intrinsically
faster hashing, a hardware cause or a full-proof speedup.209remains closed and
not adopted; these are separate diagnostics, not extra209selection samples.

|Grouping median,ms|Boolean OFF|Boolean ON|
|:---|---:|---:|
|Elapsed|1.916767|1.181980|
|Thread CPU|1.820040|1.085074|
|Elapsed minus thread CPU|0.100324|0.097979|

Values are medians of three process medians. The median of paired per-request
differences need not equal a difference of displayed medians. CPU time includes
user/kernel running time and stalls while scheduled; the residual is not an
exact mutex/scheduler classification.

|Block|ON−OFF elapsed,ms|ON−OFF CPU,ms|ON−OFF residual,ms|
|---:|---:|---:|---:|
|0|−0.747501|−0.738453|−0.003036|
|1|−0.734787|−0.734586|−0.000181|
|2|−0.738143|−0.741048|−0.002510|

The existing grouping path is serial: it samples/hashes current scalars and
performs resource admission, without a Rayon fork. Thread clocks bracket that
path inside the210elapsed markers. Worker tags match at both ends for every
traced record. Disassembly of both inspected callers places the same
scalar_groups call immediately before group_end; there is no intervening Phi
call or visible hoisted Phi computation in those brackets. This narrows the
question but does not establish that runtime input, allocation and cache state
are equivalent. The next cheap check is actual scalar equality at the shared
function entry in the exact retained binary, before another causal PMU claim or
optimization. Exact early grouping rejection already has an older experiment;
it is not a new proposal.

## Method and evidence

An isolated210copy changes three source files to add four atomic CPU-clock/
worker-tag slots and driver observations.178of181runtime files remain exact;
all179production files and all BN254 package files remain unchanged. No
arithmetic, solver, grouping rule, memory policy, point table or pool changes.
The native diagnostic SDK SHA256 is
`84a472c6cd1a1eca40c9010d330aeea870ceb8e54381384bd163c2b626b3a6fe`.

The fixed four-mode design is BooleanOFF/ON ×traceOFF/ON in the same binary,
three independent backings and three rotated/reversed blocks. Four4-worker
smokes precede12primary32-worker main processes. Main processes run three
primes and251timed SDK calls; all main capture precedes late Gnark verification.
No owned CPU cojob overlaps capture. CPU clocks useCLOCK_THREAD_CPUTIME_ID;
traceOFF records are zero. CPU delta is checked against elapsed plus10µs for
cross-clock overhead/accounting tolerance, without clipping residuals.

All6focused native tests pass,134other tests are filtered; broader209native
coverage is inherited. All16fresh exports pass Gnark,3064SDK calls complete,
and1508traceON/1508traceOFF CPU records pass integrity checks, including smoke
records. The reused independent reader checks32receipts,64stream hashes,
event/route/source/backing/phase/ledger bindings. A separate CPU reader checks
new source deltas, per-record thread clocks, worker identity and CPU medians.
All build/test/capture/verifier/reader steps pass first execution.

Both observer checks pass the declared factor1.05 bounds: median traceON/OFF
wall ratios0.996634(BooleanOFF) and0.997217(ON). This bounds whole-request observer
effects; it does not qualify a new latency gain. Protocol17:51:27UTC to result
17:57:29UTC is6m02s, within20minutes. No C7a, portable, physical-pressure,
huge-circuit or concurrent-SDK qualification was added.

Canonical distinct verified exports are now**58,980**:
`/home/ubuntu/src/zopro-perf-work/group-cpu211/proof-sha256-after-main.json`,
SHA256`25f3486018e2702859911f4e047fb105d57020648a30aa46d2579e6262ad6e2b`.
Complete private evidence stays at that root; raw keys/scalars, proof bytes,
full ledgers and binaries are excluded from the report archive. Production194/196
remains accepted, and the original all-eight CPU10×goal remains unmet0/8.
