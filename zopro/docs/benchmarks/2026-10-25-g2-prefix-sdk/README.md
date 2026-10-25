# G2-only inversion-prefix records: full SDK comparison

The G2-only candidate passes its declared SDK performance gate. Primary ON latency is8.606433ms; the all-nine geometric paired ratios are0.992587 against accepted and0.995119 against the identical OFF binary. No source is adopted by this performance campaign.

This is a new G2-only hypothesis after the [combined G1/G2 component](../2026-10-25-prefix-records-component/README.md) failed. That original failure remains unchanged. The G1 candidate is absent. Native G2 streaming stores an inversion prefix and norm, then rereads original coordinates on reverse. Combining prefix and inverse once before scaling both complex numerator components saves one elementary Fq product per full point. Scalar tails use the original code verbatim. A batch's distinct bucket indices preserve original inputs until their unique reverse writes.

| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF | ON peak RSS MiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1x2-w32 | 8.691594 | 8.729365 | 8.606433 | 0.987556 | 0.985917 | 111.25 |
| 2x2-w32 | 11.871071 | 11.824303 | 11.704247 | 0.985037 | 0.993575 | 262.25 |
| 4x2-w32 | 16.210727 | 16.366611 | 16.254490 | 1.002915 | 0.996816 | 303.75 |
| 8x2-w32 | 24.605202 | 24.550508 | 24.425422 | 0.990989 | 0.991987 | 504.75 |
| 12x2-w32 | 34.993626 | 34.339081 | 34.292122 | 0.978349 | 0.995931 | 695.00 |
| 16x2-w32 | 41.316511 | 40.686282 | 40.622383 | 0.979404 | 0.998289 | 823.00 |
| 24x2-w32 | 68.656512 | 69.321064 | 68.749216 | 0.995727 | 0.998504 | 1208.22 |
| merge-24x1-w32 | 68.006565 | 67.526008 | 67.368191 | 0.996304 | 0.996502 | 1148.50 |
| merge-54x1-w32 | 173.836034 | 177.492547 | 176.897239 | 1.017610 | 0.998615 | 1854.48 |
| 1x2-w16 | 10.192205 | 10.286362 | 10.145567 | 1.005394 | 0.986575 | 76.00 |

Primary gate: True; per-case nonregression: True; OFF calibration: True. All original observations and failed conditions are retained. The gate requires a positive primary paired median with at least4/7 wins against both controls, all-nine geometric improvement against both, each of ten cases within2% paired median and5% in every block, and OFF within3% of accepted. There is no minimum gain floor or unchanged reroll.

Seven fixed rotated/reversed blocks compare accepted, candidate OFF and ON over all nine32-worker cases plus a16-worker physical-core-pinned primary control. Each process excludes key loading and three prime SDK calls, then measures31 complete fresh requests. Requests include witness generation, proof randomness, proving, internal verification and serialization. The standard six-argument driver is unchanged. Native CPU, fat LTO and one codegen unit match accepted. No owned build, profile or second benchmark overlaps timing. Whole-process RSS includes loading and priming and is not minimum required RAM.

All nine shapes have OFF/ON four-worker functional preflights; a missing-option primary32 case confirms the experimental default is OFF. First-use traces confirm actual G2 stream selection on every shape, including full vector batches. Tracing is absent from timing. Native validation passes106 core, two raw-format and six cubic tests with the runtime option enabled. The two added tests check complete-curve formulas, short/zero lengths, tails, signs, canonical and inverse modes, repeated scratch use and actual dispatch/control behavior. Field source is unchanged.

The campaign completes7,216 SDK calls and229 distinct independently Gnark-verified final exports. All458 proof/public files match hashes and public fixtures; proof hashes are fresh against1,473 previous exports. Production167 and private168 source pins, inputs, libraries and binaries are checked before and after. G1 algorithm bodies are restored to accepted; additional private scratch fields and code layout can affect generated code, so the fresh OFF control remains required.

The new path reduces only G2 streaming group-record payload from nine vectors to two. It adds one vector header and three bool fields to native private scratch; the unused old group vector stays unallocated on the new path. The component's72KB-to16KB capacity result is not a full-prover memory bound. Resource-forecast and generated-stack review, portable behavior, caller concurrency, cold/warm pressure transitions, and actual250/4999 qualification remain outstanding before any promotion. The runtime option is read at scratch construction; immutable key data, solver image format and proving-key representation are unchanged.

| Shape | Candidate local ms | Supplied M5 CPU ms | CPU speedup | Supplied Metal ms | Metal speedup |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 | 8.606433 | 37 | 4.299x | 24 | 2.789x |
| 2x2 | 11.704247 | 59 | 5.041x | 30 | 2.563x |
| 4x2 | 16.254490 | 109 | 6.706x | 48 | 2.953x |
| 8x2 | 24.425422 | 189 | 7.738x | 74 | 3.030x |
| 12x2 | 34.292122 | 284 | 8.282x | 101 | 2.945x |
| 16x2 | 40.622383 | 335 | 8.247x | 124 | 3.053x |
| 24x2 | 68.749216 | 528 | 7.680x | 176 | 2.560x |
| merge-24x1 | 67.368191 | 507 | 7.526x | 175 | 2.598x |

All-eight10x CPU target achieved: False. Supplied M5 numbers are historical references, not fresh Apple measurements. No M5 reference was supplied for merge54. No AWS, Aeglos or physical-phone comparison, source adoption, commit or push is part of this campaign. Accepted production remains the [small-core entry baseline](../2026-10-25-small-core-entry-adoption/README.md).

[Protocol](protocol.json), [results](results.json), [raw observations](rows.json), [scope audit](scope-audit.json), [proof hashes](proof-inventory.json), [build](build-complete.json), [source](source/crates/zopro-core/src/msm/g2_prefix.rs).
