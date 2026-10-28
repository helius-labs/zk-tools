# Adopted-source stage and CPU diagnostic

Source174 and SDK81033 are immutable. No prover or dependency rebuild, source
instrumentation, optimization, executable copy or layout selection. Kernel
uprobes/uretprobes trace six coarse function pairs and one small-path callsite
pair in the exact adopted ELF. File offsets are derived from executable PT_LOAD
segments, not assumed equal to virtual addresses. The small join call at VA
0x441a33 targets0x497460 and returns at0x441a38; retain actual disassembly. Trace
arguments contain no witness/key data. Only the owned zopro174stage group is
registered/removed; no trace ring, global event enable, sysctl or scheduler change.

Trace SDK, solve_inner, solve_in_with_slots, product_values, canonical wrapper,
G2raw_exclusive and small fused-query join. Pair entries/exits by event and TID
using stacks, retaining main/worker recursion. Select the innermost core span
containing the unique solver and product; require those on its TID. Partition
that span into pre-solver, solver, gather/public/reclaim gap, FFT product and
post-product tail. Report the direct small join only where its markers exist;
missing alternate-path markers are unavailable, not zero. SDK outside innercore
includes wrapper wait, witness/proof assembly, verification and serialization;
it does not isolate each of those costs. Join/query spans include nested Rayon
work and waiting and are not attributable CPU time.

One four-worker traced primary preflight validates actual event counts/containment
before the fixed diagnostic. Its one fresh export also independently verifies.
Then three complete rotated/reversed process blocks cover primary16/32,16x2/32
andmerge54/32, both OFF and ON. Every process uses the same SDK and root identity,
environment and affinity,3primes+31verified SDK calls. OFF has probes disabled;
ON uses acknowledged warm-loop FIFO gates and records every probe (period1)
plus user cycles at199Hz with per-event freq override. Use one shared FIFO pair.
No owned compile/helper/hash/archive/otherbench overlaps captures; decode and
verification are serial commands outside them. No exclusive-host claim.

All25exports require independent Gnark verification,164-byte length, exact public
binding and freshness beyond15028known hashes; prospectiveledger15053.820SDK
requests include primes. Require one sampled PID/no lost records, complete
trace pairs and exactly31SDK/solver/product spans per diagnostic. Mandatory
small join/G2 markers on both primary contexts; optional alternate-path spans
on the larger shapes remain explicit. Validate PMU traceperiod1/cyclesfreq199
from each perf evlist, not just command intent. Keep all three blocks and
OFF/ON overhead observations. Do not substitute instrumented times for the
accepted8.366931ms or claim an optimization from a diagnostic.

Assign sampled user-cycle period by the global wall-phase intervals. It locates
CPU activity during each phase, including work stealing and inlined symbols;
phase shares are not removable latency, exclusive query work, or DRAM bandwidth.
Retain unknown/outside attribution and all raw events. Include inherited all8
M5CPU/Metal table and0/8tenfold status. No new AWS/Aeglos, mobile/ARM/TEE, giant,
swap, minimum-RAM or pressure claim. Commit and push the completed diagnostic.

References consulted for tool semantics: Linux kernel uprobe tracer documentation
https://docs.kernel.org/trace/uprobetracer.html and perf record/probe manuals
https://man7.org/linux/man-pages/man1/perf-record.1.html
https://man7.org/linux/man-pages/man1/perf-probe.1.html . A separate true-command
syntax check confirms cycles/freq=199/u selects hardware cycles,freq1,value199;
its9startup samples are not SDK/prover evidence or counted exports.
