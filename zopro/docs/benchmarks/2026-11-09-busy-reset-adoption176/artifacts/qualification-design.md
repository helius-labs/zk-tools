# Bitmap reset: remaining SDK qualification

Goal remains verified single-proof CPU AVX >=10x supplied M5 CPU ALL8 with CPU+Metal comparisons. busy-reset-sdk176 passed79/79 declared checks. This stage qualifies its exact libraries before promotion; no new latency gate or unmeasured arithmetic/topology change is introduced. Prior native field/core/Window suites and full SDK performance are inherited without replay.

Reuse measured native ON core/prover/zolana and original-field1200. Reuse the parent-qualified generic noASM/noIFMA core and generic field, then link coherent generic prover/zolana/fixed-placement SDK. Link the byte-exact previously qualified shared-key concurrent consumer against native ON. Add a separate mixed-key concurrent consumer:four callers/2proofs each sharing one existing global32worker Rayon pool, each with its own mapped image/SDK. Inputs1x2/2x2/merge54/1x2. Shared-key concurrent cases1x2and54 useoneArc-mappedimage/fourSDKcallers. Check37loaded/33idle TIDs and everyproof/publicbinding. These are functional tests,notthroughput.

Functional scope fixed before SDK calls:
-Native1/4workers,requested512KiB stacks,primary+54,basis/raw/preparation-denied:12processes,48SDKrequests,12exports.
-Native4workerprimary optouts PACKED/DIRECT/CANONICAL_BATCH/VECTOR_INVERSE:4processes,16requests,4exports.
-Generic4workers all8+54basis plusprimary/54raw+denied:13processes,52requests,13exports.
-Two native shared-key concurrent processes:16requests/exports.
-One native separate-map/mixed-key concurrent process:8requests/exports.
Total32SDKprocesses,140requests including3primes forsinglecallerprocesses,53fresh exports. All32SDKprocesses precede53Gnarkchecks:85workflowsteps. Canonical41739->41792 if all independently verified and unique. No successful old proof/decoder/model/field suite is rerun. Fixed-placement driver inherited byte-exact. Optional preparation zero is an existing fallback setting,notan architecturalRAMcap orminimumRAM measurement.

Source/resource review verifies onlytworesetedits/testsuffixes versusaccepted176,exactnative/generic/librarylinkidentities,unchanged Windowfieldlayout/capacityinitializers/typed workspace and admission code. Newreset usesexistingVec.fill ororiginalloop andnoallocation. ObservedfullSDK5%RSSgates are inherited,notrecastas hardRAM bounds. No newcgroup/swap/phone/ARM/4999/250/stackpeak/minRAM claim. Genericx86 runtime is not ARM execution.

BeforeanySDKfunctionalrun freezehelpers,inputs,binaries,order,conditions,proofverificationreader and41739priorledger. DuringANYpreflight/recording onlypoll exactlivehandleuntilterminal;no helper/hash/build/report overlapsSDKcalls. Allrecording endsbeforeofflineproofchecks/analysis. Rust1.98.1serializedxtask/noPython/questions/subagents. Source/script failures retainactualreceipts;successfulbuilds/casesare reused ratherthanunchangedreruns. Publishfactualresult,commitANDpush/exactremoteverify. Promotion must preservemeasurednativebody/defaultselection andrecord acceptedsource/artifacts;full10xgoalstill0/8.
