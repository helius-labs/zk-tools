# Count-first solver dependency construction

The new constructor reduces admitted temporary graph capacity by approximately
**20×**, while retaining the same dependency graph. For merge54 the construction
charge falls **143.12 → 7.18 MiB**. This is a memory-construction result;
new C7a latency has not been measured and the96-core primary problem remains open.

The [preceding C7a screen](../2026-11-18-ready-solver246/README.md) found useful
2×2 and merge54 latency gains, but failed the primary96, calibration and memory
checks. Those results remain unchanged. This follow-up removes the temporary
edge-pair vector sized for all possible reads. It counts deduplicated producer
edges first, admits exact adjacency storage, then fills it in a second pass.
Existing ownership/read scratch is reused. Runtime scheduling and arithmetic
source bodies remain unchanged; a new build can still have different codegen.

Two existing resource-admission tokens cover base arrays and exact edge storage.
Either admission failure drops all construction scratch and preserves the
previous solver. Per-proof atomic counts retain their independent admission.
No fixed RAM cap, new pool, key table or copied operation plan is introduced.
The prototype remains private and opt-in; production files are unchanged.

## Functional and memory observations

The workspace machine runs one process per old/new arm for1×2,2×2 andmerge54
at32logical CPUs. All six processes prime three proofs and export the next one.
These are functional/activation observations, not a latency screen. All six
fresh exported proofs independently pass Gnark; leaf/edge/root counts match.
No owned CPU job overlaps these SDK captures.

|Circuit|Old construction charge MiB|New charge MiB|Old retained bytes|New retained bytes|Old lifetime RSS MiB|New lifetime RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1×2|3.780|0.193|3832|3896|63.50|63.50|
|2×2|6.568|0.332|5444|5508|112.25|120.75|
|merge54|143.118|7.179|103264|103328|1600.34|1530.29|

Charges describe requested allocation capacity, not measured physical RAM.
The additional64bytes of retained charge cover the separately admitted edge
allocation. Process RSS includes loading, preparation, primes and allocator
history. Merge54's single observation is70.04MiB lower;2×2 rises8.50MiB despite
lower construction charge. These are not universal peak/minimum-RAM estimates
or physical-pressure, phone, swap or concurrent-throughput qualification.

The preserved graphs have97leaves/84edges for1×2,140/143 for2×2 and2694/4166
for merge54. An independent BTreeMap/BTreeSet reference now checks every edge,
predecessor count and root during the actual refined and tagged solver-vector
tests. Five focused native tests pass, including publication/exactly-once,
concurrent reuse, error/panic handling, refinements through96workers and all12
tagged Poseidon widths. Generic no-ASM/no-IFMA compilation passes. This is
compile/test coverage, not ARM or mobile execution.

New SDK SHA-256:
`2d47de55858b89ff4a48f46753aec91a1183b802cd0a6d083e122c55ba8cd407`.
Direct old-constructor control:
`1e0cdbfa3b0ee307b74fa670d93ef15c3c685825e287adcaaf3a405326061549`.

This stage adds24SDK calls and six distinct verified exports. Canonical
inventory is64307, SHA-256
`60bdbcb13137800eeb8fa9e9b31edaa93a96ca0fd9d0f010ee6f8445beba900a`.
No cloud instance is created and no successful preceding capture is repeated
for bookkeeping. The old constructor is a fresh paired functional/memory
control for this new construction change.

Next is a fixed fresh C7a comparison preserving the old ready-constructor,
accepted SDK and new disabled controls, with32/96-core contexts and automatic
profiles. No full-suite adoption, new CPU/Metal speedup or all-eight10× success
is claimed. The full objective remains active and unmet. Current broader
CPU/Metal evidence remains in the linked preceding report and its references.
This construction qualification is committed locally; not pushed.

[Protocol](artifacts/protocol.md), [functional/memory review](artifacts/local-review.json),
[native plan](artifacts/build-plan-v2.json), [checksums](SHA256SUMS).
Source, tests and preparation tooling are included; private keys, requests,
proof bytes, binaries and raw placement/PID/TID streams are excluded.
