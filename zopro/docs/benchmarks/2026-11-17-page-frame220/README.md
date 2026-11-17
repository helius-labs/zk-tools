# Page-aligned scalar-grouping frame220

The layout intervention lowers G2 grouping cycles by **42–44%**, but complete
proof latency is **0.76% slower** than the retained executable and **1.27%
slower** than fresh OFF in this screen. Only one of three pairs wins against
each control. This fixed page-sized-marker implementation is **not adopted**.

|Block|Retained REF ms|Fresh OFF ms|Aligned ON ms|ON/REF|ON/OFF|
|---:|---:|---:|---:|---:|---:|
|0|7.501497|7.464005|7.558665|1.007621|1.012682|
|1|7.555311|7.671965|7.545660|0.998723|0.983537|
|2|7.546755|7.567705|7.724768|1.023588|1.020754|

Times are arithmetic process means over251 complete SDK calls after three
primes. Paired medians are1.007621 against REF and1.012682 against OFF. CPU
ratios are1.000517 and1.000796 respectively; whole-proof CPU service does not
show the large reduction seen inside the grouping interval. Fresh OFF/REF
calibration is1.002776 and passes the declared3% band. There is no minimum gain
floor or sample extension; the full-proof performance decision fails.

## What changed, and what the result supports

The preceding219 live-register capture found that a32-byte key temporary in
the slow grouping context straddles a4KiB boundary, while the fast context's
temporary does not. Both had identical64-byte alignment. This experiment adds
a native-only Rust `repr(align(4096))` one-byte marker and a mutable `black_box`
borrow in the grouping function. It changes stack placement without changing
the grouping algorithm, hash API, map-entry size, scalar inputs, arithmetic,
workspace forecast, pool or heap allocations. It adds no key table or copy.

The actual ON prologue aligns RSP down to a4KiB boundary and reserves0x3000
bytes with Rust stack probes. Its32-byte key store becomes an aligned store
at `[rsp+0xf40]`: the interval3904–3935 stays inside one4KiB page for every
caller stack placement. This property is checked before SDK capture. OFF
retains the old store at `[rsp+0xf0]`; its actual hash-loop instructions match
REF. Two other readonly string-reference displacements differ in the full
REF/OFF function. There is no new live-SP measurement of ON.

|Block|REF group cycles|OFF group cycles|ON group cycles|ON/OFF instructions|
|---:|---:|---:|---:|---:|
|0|7,819,598|7,985,956|4,645,848|1.022583|
|1|8,102,695|8,179,581|4,591,275|1.022460|
|2|7,794,340|8,003,233|4,497,469|1.022668|

Fresh OFF reproduces REF's slow group cost: cycle differences are within2.7%
and instruction counts within0.01%. ON/OFF cycle ratios are0.581752,0.561309
and0.561957. Group CPU medians fall from1.799–1.834ms OFF to1.086–1.108ms ON;
CPI falls from about2.21–2.26 to1.22–1.26.

The declared clean-causal criterion nevertheless **fails**: ON executes about
2.26% more group instructions, exceeding the frozen1% allowance. The changed
prologue/register allocation and improved cache-line alignment are additional
differences, so this does not isolate an exact page-split penalty. It supports
stack-layout-sensitive execution of the identified loop. Neither the large
local cycle reduction nor the removed crossing establishes removable proof
wall time. The MSM join remains about5ms and overall proof latency does not
improve. Overlapping query spans are not exclusive useful-work timings.

## Stack and memory

There is one explicit **pre-capture engineering exception**. The nominal16KiB
frame bound omitted saved registers. Static analysis gives at most16,424bytes
from function entry to the bottom, or16,432 including the return slot:40bytes
above the nominal bound before that slot. The exception is recorded before any
SDK execution in `pre-capture-note.md`; the original protocol is retained.
No source, alignment parameter, performance gate or observation is changed to
obtain this exception. This is about3.14% of the512KiB stacks used in the
new worker tests. Default production worker-stack reservations do not change.

|Whole-process observation|REF|OFF|ON|
|:---|---:|---:|---:|
|Maximum RSS MiB|126.00|128.25|129.25|
|Timed minor faults, three-process range|17,472–18,979|17,086–19,144|17,388–18,344|
|Timed major faults, every process|0|0|0|

Both CPU and RSS guards pass. RSS includes key loading and primes; fault counts
cover the request loop. These observations are not minimum RAM, a pressure
test, or proof that frame padding causes the RSS differences. No page-fault
explanation follows from crossing an address boundary.

## Qualification and execution

Both coherent builds pass all142 native core tests: the inherited141 and one
new test. The new test checks stable groups against an independent BTreeMap,
visits all53 actual workers across1/4/16/32-worker pools with512KiB stacks, and
exercises four OS callers sharing an eight-worker pool. OFF and ON execute the
same mathematical fixtures;142+142 is not284 distinct tests. Generic/ARM,
memory-pressure and full all-shape promotion checks are not claimed.

Only one source parent differs from216;181 source files, all field files and
Cargo dependencies remain exact. Removing the guarded marker and new test
recovers the original source. REF reuses the exact216 SDK; OFF/ON are coherent
builds from the same snapshot, distinguished by the new compiler cfg. SDK
digests:

- REF: `6d9ae8f89e09b4bec8ac7c6a6f795bfbf04594623e4abf39802ada7b4d5d254c`
- OFF: `bd79383e5187f6aa76d05fac135f4f6bbaef30e13bd8cb49e216960914e8d3fa`
- ON: `b67e06aa78d1b99089746dbc743ffe63fafa3c76d46ecdfeb80f9de42c7cf5b9`

Three4-worker smoke processes precede nine32-worker main processes. Three
independent executable backings per arm participate in rotated/reversed
blocks. Boolean peeling is disabled; native trace and internal PMU counters
are enabled in every mode. Stage CPU/cycle/instruction profiles are captured
automatically for every timed request. There are no owned CPU cojobs during
capture, and all main capture finishes before late Gnark verification.

All12 exports pass Gnark;2,298 SDK calls complete. All2,262 recorded requests
pass event, route, placement, receipt and CPU/PMU checks, with100% PMU coverage.
Two independently written readers agree on raw means, proof freshness, paired
performance failures and the failed causal criterion. Twenty-four receipts
and48 stream hashes pass. All build/test/link/capture/verifier/reader commands
pass first execution. The disclosed frame-bound exception is separate from
those successful executions.

Protocol20:10:11UTC to result20:18:52UTC is8m41s, within30minutes. Canonical
exports are **59,074**:
`/home/ubuntu/src/zopro-perf-work/page-frame220/proof-sha256-after-main.json`,
SHA256 `95544b6be55076ff5c759db9007f6576ae400533f58a028a012b7469d373fc83`.
Complete private evidence stays under that root. Raw witnesses, keys, proofs,
full ledgers and binaries are excluded here. No AWS was used. Production194/196
and its [CPU/Metal comparisons](../2026-11-17-certified-all196/README.md) remain
accepted; the original all-eight CPU10× goal remains unmet0/8.
