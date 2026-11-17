# Boolean G2: whole-proof phase diagnostic210

The Boolean point sum finishes before Phi in **753/753** traced primary proofs.
The complete G2 caller and its parallel A/fused-KH peers still take about5ms.
This explains the limit of the isolated208result: this integration does not
shorten the observed whole-proof parallel stage. It does not identify a sole
hardware, allocator or scheduler cause, or establish that all G2 optimizations
are ineffective. The closed[209 full-SDK screen](../2026-11-17-boolean-proof209/README.md)
is not extended or rescued by these diagnostic samples.

|Elapsed phase,ms|Boolean OFF|Boolean ON|
|:---|---:|---:|
|Solver|1.092352|1.091290|
|Public correction/canonical gather|0.106832|0.106771|
|Product|0.519821|0.518278|
|A caller|4.971135|4.998878|
|Fused KH caller|4.994519|5.009207|
|Complete G2 caller|4.997124|5.020087|
|Whole parallel MSM stage|5.013685|5.034214|
|G2 grouping decision|1.906881|1.124783|
|Phi caller|3.112837|3.876439|
|Guard/select/zero|—|0.016642|
|Boolean tree caller|—|0.190209|

Each value is the median of three process-level phase medians, not an additive
proof decomposition or an exclusive CPU cost. Rayon callers can execute other
work while joining. These timestamps establish start/return boundaries; they do
not measure arithmetic-only critical paths or idle workers. All753traced records
per arm use fusedKH. G2 returns last in179/753OFF and219/753ON records; a peer
returns last in574/753 and534/753 respectively. Median peer-after-G2 gaps across
processes are12.353µs OFF and7.224µs ON. Faster Boolean sums are not supported as
the next late-dependency target by these observations.

The grouping/Phi time split shifts by roughly0.8ms while total G2 time remains
near5ms. Both callers use the same retained scalar_groups symbol. This does not
show that hashing became intrinsically faster or Phi arithmetic became slower;
serial CPU work, waits and the surrounding parallel workload remain to be
separated before selecting an intervention. Exact early grouping rejection is
already covered by the[2026-10-27 experiment](../2026-10-27-grouping-prefix-cutoff/README.md),
so it should not be rediscovered as a new optimization.

## Method and validation

An isolated209source copy adds a24-slot atomic timestamp buffer and wrappers
only. The driver resets inside its request timer before calling the SDK,
snapshots after the timer stops, and prints records after the measured loop.
There is no per-event allocation or printing, new pool or arithmetic change.
The fixed251-round driver storage is about48KiB; the global events occupy192B.
This shared trace is for one sequential diagnostic driver, not a concurrent
production API.177of209's180source files remain exact; three files gain
instrumentation and one timestamp module is added. All179production files and
all BN254 package files remain unchanged.

The diagnostic SDK SHA256 is
`4d7c1cfc4733f995e4542a5741eb406bd24ee0ff50fb69cd4b388925eb6f3123`.
Its four modes combine BooleanOFF/ON and traceOFF/ON in the same binary.
Three independent executable backings rotate across three reversed/rotated
blocks at32workers. Four4-worker smoke processes precede12main processes;
main processes run three primes plus251timed SDK requests. Main captures finish
before late Gnark verification, without owned CPU cojobs. Certificate, fused
preparation and indexed-bundle options stay enabled throughout.

Both predeclared observer checks pass: median traceON/traceOFF wall ratios are
0.994701 for BooleanOFF and0.996730 for BooleanON, within factor1.05. This limits
observer disturbance at the whole-request level; it does not establish precise
sub-percent differences between phases. There is no new adoption latency claim.

All5focused native tests pass, including event reset/order and inherited209
query/oracle/guard/metadata cases;134other core tests are filtered. All16fresh
exports pass Gnark and3064SDK calls complete. The reader independently checks
32execution/verifier receipts,64stream hashes, source/backing pins,1508traced
records including two smoke records, event ordering/containment, phase medians,
finish order, observer decisions and ledger disjointness. Producer and reader
pass first execution. The preparation helper initially rejected a whitespace
spelling while locating a source call; the original helper is retained and the
matcher was fixed before build/capture. A later ad hoc jq count expression was
corrected after a syntax/type failure; the successful Rust readers and scientific
artifacts were unchanged. All native build/test/capture/verifier steps pass on
their first execution. Protocol17:37:28UTC to confirmed result17:44:46UTC is7m18s,
within its20-minute allowance.

Canonical distinct verified exports are**58,964**:
`/home/ubuntu/src/zopro-perf-work/boolean-phases210/proof-sha256-after-main.json`,
SHA256`34f60e381bf9259290fa47b26fd5e9478e7624ea1557caa01ef2c068ed7f2d8b`.
The private root retains complete evidence; this archive excludes raw keys,
points/scalars, proof bytes, full ledgers and binaries. This is local native
diagnostic evidence, not C7a, portable, huge-circuit, pressure or SDK concurrency
qualification. Production194/196 remains accepted, with0/8CPU10×targets met.
