# Serial grouping instructions and cycles214

Both paths retire about **3.61 million user instructions**, while cycles differ
by roughly **43–46%**. The grouping CPU-time shift therefore corresponds to
execution cost, rather than a comparable reduction in retired instructions.
This does not identify a cache, alignment, SMT, frequency or allocator cause.
The actual inputs and grouping admission were equal in the separate212/213
audits. No new full-proof speedup or adoption is established.

|Block|OFF instructions|ON instructions|ON/OFF instructions|OFF cycles|ON cycles|ON/OFF cycles|OFF/ON CPI medians|
|---:|---:|---:|---:|---:|---:|---:|:---|
|0|3,613,423|3,613,663|1.000066|8,267,010|4,470,081|0.540713|2.287/1.238|
|1|3,613,887|3,614,153|1.000074|7,876,555|4,520,405|0.573906|2.178/1.252|
|2|3,613,519|3,614,301|1.000216|8,296,653|4,544,226|0.547718|2.296/1.256|

Each count and CPI is a process median; the median of per-record CPI need not
equal a ratio of displayed medians. CPU medians remain1.860617ms OFF versus
1.090864ms ON; elapsed1.958837/1.185437ms. These instruction/cycle counters
exclude kernel/hypervisor execution; thread CPU time includes kernel running
time. The events bracket a serial span and include small read/parser overhead,
not a perfect instruction attribution to one Rust function.

## Capture and validation

A copy of211adds one PMU module and modifies its trace parent only:180of181
prior runtime files remain exact, with one new module. All179production files
and all BN254 package files remain unchanged. No grouping, solver, field,
point-table or thread-pool algorithm changes. The diagnostic SDK digest is
`74604011e4fc33805b08305400ec5d993e0e167e889042a85fdecea291f97ab2`.

Every worker opens two owned perf-event FDs before key loading/primes. Event IDs
bind cycles/instructions to their read-group positions. All four BooleanOFF/ON
×traceOFF/ON modes run as root with initialized counters; traceOFF skips stage
reads/stores. This avoids mixing privilege/setup differences into observer
comparisons. FDs close on ownership drop or process exit. The initial perf CLI
capability probe succeeded; no global perf policy was changed.

Four4-worker smokes precede12primary32-worker processes, using three independent
binary backings and three rotated/reversed blocks. Main processes perform
three primes and251timed SDK calls. All main capture finishes before late
Gnark verification; no owned CPU cojobs overlap capture. All6focused native
tests pass,134other tests are filtered; broader209coverage is inherited.

All16fresh exports pass Gnark;3064SDK calls complete. The readers check
32receipts,64stream hashes, source/backing/proof bindings, event/CPU records,
PMU initialization/root UID, monotonic deltas and scheduling coverage. All1508
active PMU records have **100% coverage**;1508observer-OFF records are zero.
Both observer checks pass factor1.05: median traceON/OFF wall ratios0.997100
(BooleanOFF),1.004818(ON). All build/test/capture/verifier/readers pass first
execution. Protocol18:20:24UTC to result18:28:13UTC is7m49s, within20minutes.

A controlled scalar-layout intervention is now justified as a test:212observed
different alignments, but that observation alone is not causality. Compare two
fixed offsets with the same storage capacity/allocation path, record actual
addresses modulo64 and PMU costs, and check complete SDK behavior. Do not start
an alignment sweep or claim a hardware cause from these counters alone.

Canonical distinct verified exports are**59,000**:
`/home/ubuntu/src/zopro-perf-work/group-pmu214/proof-sha256-after-main.json`,
SHA256`ab79c6d417c61ad2817e42608fabaa6b54f544aabab18b0a168b3c97b26489db`.
The private root retains complete evidence. Raw inputs, proof bytes, full
ledgers and binaries are excluded here. No C7a, pressure, portable or SDK
concurrency qualification is added. Production194/196 remains accepted;
209stays closed and the all-eight CPU10×goal remains unmet0/8.
