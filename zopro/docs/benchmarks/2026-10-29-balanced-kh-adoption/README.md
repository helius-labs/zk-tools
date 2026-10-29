# Balanced K/H production adoption

Adopt the tested variable-width K/H windows under the existing native entry guards. Primary at 32 workers measures **8.022165 ms versus 8.127910 ms**, paired **1.131% faster** with 15/15 wins; process peak RSS is **134.00 versus 135.00 MiB**. This is an incremental gain, with no general RAM reduction claimed.

Adoption is an **explicit engineering exception**: the full performance gate remains **false** solely for physical 16 OFF/accepted calibration. All candidate gain/nonregression checks and all 32-worker calibration checks pass. The separately declared final-artifact gate is true. Physical 16 improvement versus accepted is uncertain; its interval crosses parity. The previous private-candidate and earlier production exceptions remain recorded.

Only the borrowed K/H parent and one new private module change. Twenty 12/13-bit signed windows retain the complete bucket algorithm and conservative memory forecast. The private experiment flag and trace are absent. Of 174 original files, 173 are byte-exact; reversing the parent edits and removing the child recovers all 174. The final 175-file source matches the actual tested artifact, with unchanged accepted native flags. No retained key table, additional input-sized buffer, pool or execution plan is added; stack span metadata is 320 bytes on 64-bit, with compiler frames additional.

## Validation and limits

Fresh actual-artifact native 124 core/8 prover and portable 101 core/8 prover tests pass, including both balanced scalar/point oracles. Field 33 native/11 portable evidence is inherited from identical source. Thirteen fresh portable real-proof cases cover all nine basis shapes and primary/54 raw/denied. All **28,000 SDK requests** and **505 fresh Gnark exports** pass; all source/library/proof pins close. The independent ledger is **17,005**. Promotion adds no new proof/test capture; all 175 copied source hashes must match the qualified snapshot.

Private-candidate small-stack/opt-out/concurrent/configured-memory checks are inherited by source equivalence; they are not new clean-binary captures. All eight 192 MiB cases had zero high/max/OOM events, so active throttling was not established. The historical forced-high baseline timeout and unexecuted candidate remain limitations. There is no fresh giant 250/4999, phone, ARM, TEE, swap, minimum-RAM or AWS/Aeglos qualification. Separate CLI build is unverified; the actual standard SDK engine is qualified.

54→1 measures 162.250464 ms versus 161.689371 ms accepted; its paired +0.912% estimate and interval crossing parity are retained, with upper 1.013465 inside the declared 1.03 bound. RSS observations are mixed across shapes.

## Supplied M5 comparisons

These are actual final-artifact 32-worker measurements and supplied historical M5 references.

| Shape | Adopted ms | M5 CPU ms | CPU/adopted | M5 Metal ms | Metal/adopted | CPU 10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.022165 | 37 | 4.612× | 24 | 2.992× | 3.7 |
| 2x2 | 11.335969 | 59 | 5.205× | 30 | 2.646× | 5.9 |
| 4x2 | 15.945544 | 109 | 6.836× | 48 | 3.010× | 10.9 |
| 8x2 | 24.090485 | 189 | 7.845× | 74 | 3.072× | 18.9 |
| 12x2 | 33.844855 | 284 | 8.391× | 101 | 2.984× | 28.4 |
| 16x2 | 39.931557 | 335 | 8.389× | 124 | 3.105× | 33.5 |
| 24x2 | 66.233309 | 528 | 7.972× | 176 | 2.657× | 52.8 |
| merge-24x1 | 65.273224 | 507 | 7.767× | 175 | 2.681× | 50.7 |

The original all-eight CPU 10× goal remains **active and unmet, 0/8**. Primary requires 3.7 ms. No M5 reference was supplied for 54→1. [Full experiment](../2026-10-29-balanced-kh-production/README.md), [decision](artifacts/decision.json), [production source manifest](artifacts/production-source-manifest.json), [inherited compatibility](../2026-10-29-balanced-kh-compatibility175/README.md).
