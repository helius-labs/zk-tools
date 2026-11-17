# Runtime grouping input audit212

All eight captured G2 scalar arrays are **byte-for-byte identical** across
Boolean OFF/ON and four SDK calls per arm. They also exactly match the retained
175 query fixture. This excludes an input mismatch in this audit and confirms
the fixture's scalar content remains representative for this request. It does
not show that the two execution contexts have equal allocation, hash-state or
cache costs. There is no performance or adoption claim from debugger timings.

|Property|Observed in all eight arrays|
|:---|---:|
|Canonical scalar words|30,089|
|Bytes|962,848|
|Zero values|1,600|
|One values|568|
|Distinct values including zero|26,143|
|Distinct nonzero values|26,142|
|Different words versus fixture|0|

The scalar-array SHA256 is
`5f66ad7f79602b0f843c89b75d71371e0998ce524c048ca92af57d8fafaac13a`.
Equality is checked directly on bytes, with hashes retained as bindings.
Canonical range checks use the modulus from the unchanged BN254 source. The
fixture comparison was an additional postcapture data-quality check, not a
changed performance gate or sample extension.

## Method

The exact retained211 SDK is used without relinking:
`84a472c6cd1a1eca40c9010d330aeea870ceb8e54381384bd163c2b626b3a6fe`.
GDB breaks at the shared scalar_groups entry when its scalar length is30,089,
and reads the raw x86-64 argument pointer before the function prologue. Four
entry hits are required per process. Each OFF/ON process uses32workers, three
primes and one timed SDK call. The debugger interrupts all these calls, so every
SDK time in its output is deliberately ignored. Both fresh exported proofs
pass Gnark verification.

The initial prepared command was corrected during review before any debugger
execution: its request argument had selected the image position in the reused
argv. The original helper/plan remain available. Both actual debugger commands,
both verifiers and the independent receipt/data reader succeed first execution.
The reader checks four receipts, eight stream hashes, entry counts, exact bytes,
canonical ranges, class counts, input/source/binary pins and proof-ledger
separation. GDB's missing optional auto-load-script warning is retained;
auto-loading and debug downloads were explicitly disabled.

GDB15.1 and six dependencies were installed with no existing package upgrades;
the package log is included. No prover runtime source, field kernel, key,
thread pool or memory policy changed. Private scalar dumps remain only under
`/home/ubuntu/src/zopro-perf-work/group-input212/dumps`; they are excluded from
this report along with raw proofs, full ledgers and binaries.

The observed post-prime input alignment modulo64 was48bytes for OFF and32bytes
for ON. This is a property of these debugger processes, not evidence that
alignment caused211's CPU difference or a justification for a padding sweep.
The next discriminating measurement is serial-span instructions and cycles,
with counters initialized before primes and matched observer controls. The
[211 CPU result](../2026-11-17-group-cpu211/README.md) establishes running-time
movement; it does not establish an instruction-count reduction.

The protocol was frozen at18:04:47UTC and the reader result was written
at18:08:44UTC:3m57s, within the15-minute allowance.

Eight SDK calls and two fresh Gnark exports bring the canonical distinct count
to **58,982**. The ledger is
`/home/ubuntu/src/zopro-perf-work/group-input212/proof-sha256-after-audit.json`,
SHA256`616aa4f0fa5259f893875bad99585b78c71366cbaebc65fc044586c85f7d1a5a`.
The complete private root retains the audit. Production194/196 remains accepted;
209 stays closed, and the original all-eight CPU10×goal remains unmet0/8. No
C7a, memory-pressure, portable or concurrent-SDK qualification is added.
