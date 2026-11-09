# Continue from qualified sparse XYZZ Windows

The full goal is ACTIVE and UNMET: verified single-proof CPU AVX latency at
least 10× faster than supplied M5 CPU for ALL8 shapes; report both CPU and Metal.
Targets met 0/8. This turn is progress: actual sparse G1/G2 Window integration,
checked admission, independent complete Window/Phi/owner/concurrency checks,
restored input contracts, generic fallback and a reviewable source experiment.
There is NO candidate timing, process memory peak, SDK or proof result yet.

Root /home/ubuntu/src/zopro-perf-work/xyzz-window176.
Read design.md, source-audit.json, review-result.json, preparation-note.md.
Final native cores are libs/OFF/libzopro_core.rlib and libs/ON/libzopro_core.rlib;
the ON cfg is zopro_sparse_xyzz, preserving zopro_dense_busy_reset and original
native field1200. Final generic core is libzopro_core-generic.rlib. These are
qualified experiment artifacts, NOT accepted production baselines. Final tests
native-final-tests/generic-final-tests include all new tests, but only changed
G1 owner/contract/concurrency tests rerun there. Earlier complete leaf/G2 tests
and their exact successful binaries/receipts remain retained. Do not rerun old
successful tests or rebuild field libraries for bookkeeping.

Five reversible accepted-source parents, five new children including exact
previously qualified XYZZ helper. Static SparsePoint dispatch into Jacobian or
XYZZ. All G1 borrowed owners, balanced K/H, and G2 Phi owner select once per
operation. Admission uses existing Reservation::analysis and checked capacities;
token spans every feed/parallel task/Phi/final merge/normalization. The ordinary
affine streams, queues, order, geometry and required old forecasts are unchanged.
The denied/disabled/generic paths instantiate Jacobian. G2 map_x retains the
specialized Fq2 mul_by_fp beta mapping, leaving ZZ and ZZZ untouched.

Extra conservative charges cover the extra base field in growing sparse and
combined vectors, an entire old XYZZ allocation during growth, bookkeeping and
ALL windows/parts, not just active workers. Balanced G1 charge 15,733,888 bytes;
G2 Phi 17,307,264 bytes. These are reservation bounds, NOT allocated memory or
process RSS. Logical G1 sparse128 vs96 bytes; G2 256 vs192. No new table, key
copy, production pool, artificial architectural RAM cap or assembly.

Qualification: 66 actual Window cases/curve with both old and new sparse types;
generic repeats same G1 fixtures. Independent Arkworks full-width generator
points and carry-based signed recoding. Queue cases per type/curve:589queued,
17flushes,14packed batches, reuse, identity/equal/opposite, explicit doubling/
cancellation, final affine merge/norm; G2 maps Phi between reused rounds.
G1 full owners20cases, G2 full Phi4cases; admitted and denied processes.
1/4/16/32 workers (53tasks/curve),512KiB stacks;4OS callers/shared8pool, including
8 complete G1 owners and4 complete G2 owners. Token denial/drop/readmission/
unwind/overflow tested;9 invalid fused inputs/backend preserve original contract.

All 28 workflow command receipts are retained:26success/2failures, plus a
source-copy preparation failure. See preparation-note.md. First native test
build failed in Arkworks oracle operator syntax; repaired tests only, preserving
successful core builds. First test process lacked accepted runtime flags, so
two G2 tests returned None; corrected environment reran affected tests. Review
then restored original width13 fused length checks; changed owner cores/tests
were rebuilt/rechecked. No performance capture has started and no failures are
excluded from a latency analysis because there is no latency analysis.

NEXT: freeze and measure COMPLETE actual callers, including allocation/drop,
doubling, normal affine work, dispatch/admission, Phi, final merge/normalize and
memory. Do not use a bare mixed-add microbenchmark to select code. Start from
these exact final qualified runtime bodies. Build an unchanged accepted-source
REF shim, fresh OFF and ON coherently with the original field; include original
required forecasts and the new optional charge. Runtime environment MUST inherit
accepted flags, especially ZOPRO_CPU_G2_PREFIX_RECORDS=1 and DEFER_SIGNED_Y=1.
Measure representative G1 balanced/uniform and G2 Phi complete callers on real
1/16/32-worker pools, fixed affinity and small stacks; include memory-denied
fallback functional checks, allocations/actual RSS and automatic PMU/profiles.
Declare all controls, quality/calibration/nonregression/gain rules and readers
BEFORE any preflight/recording. Keep every positive/adverse observation. A true
complete caller gain is needed before full SDK ALL8+54/fresh Gnark comparisons.
Do not rescue this fixed recipe with curve/worker/shape/threshold/layout sweeps.

Autoresearch loop: current source/profile evidence plus targeted prior negatives
-> distinct useful arithmetic/dependency mechanism -> Rust implementation/model
-> independent math/exception/resource/generic/concurrency checks -> freeze
coherent complete caller protocol and readers -> serialized Rust1.98.1 xtask
-> during ANY preflight/recording ONLY poll the exact confirmed live handle
until terminal (observation timeout/truncation is NOT terminal) -> inspect all
results -> polite report, commit AND push/exact remote verification/checkpoint
-> continue full goal. No questions, Python, subagents, giant LUTs, key expansion,
new production pools, architectural RAM caps, C/ASM rewrite or unchanged rerolls.

Accepted SDK remains /home/ubuntu/src/zopro-perf-work/busy-reset-sdk176/sdk-ON,
SHA7aff5f602a1e250a6b9b28889bf6278b51ec400af8399c7481c36248a8718d01.
Latest accepted local32 ALL8 CPUms:
7.829285503/10.979334806/15.803968871/23.944016903/33.703579161/39.858478065/
66.186709387/65.489902226. PrimaryRSS133.50MiB;16workers9.192236934ms.
Earlier7.803404ms used the SAME accepted executable; no slower build adopted.
Merge54=162.541995323ms,no supplied M5 reference.
M5 CPU37/59/109/189/284/335/528/507;Metal24/30/48/74/101/124/176/175.
Targets3.7/5.9/10.9/18.9/28.4/33.5/52.8/50.7;0/8.
Latest full SDK comparison remains large-class-sdk176, candidate rejected at
ce78a4a069df7bc1a86d8cffceca7ba86668d803. Do not baseline that rejected core.
Canonical ledger NOW43730 unchanged:
/home/ubuntu/src/zopro-perf-work/large-class-sdk176/proof-sha256-after-capture.json
No new AWS/Aeglos/mobile/ARM/huge250/4999/pressure/swap/minRAM/throughput evidence.
No live jobs/cloud after terminal qualification/publication.
