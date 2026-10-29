# Selective fused-Fq arithmetic outlining

Declared before source/build/capture. Accepted175/a1585b86/SDK20fde remains
8.022165ms qualified primary; all8CPU10x goal ACTIVE/unmet0/8, target3.7ms.
Ledger17083 after rejected ordinary-balanced pilot f72531d6. Production stays
unchanged during this study. No questions/cloud resources/subagents.

Actual accepted native nm lists G2borrowedapplyFASTtrue at51950bytes, other
G2streams50396/51273 and G1signedborrowed26230bytes. simd::FqBatch wraps
Ifma8<FqConfig,true>; Fq2lanes_norm uses sum2 and lanes_mul uses difference2
and sum2. These invoke the always-inline product/CIOS body. This suggests a
code-footprint hypothesis; it does NOT establish instruction-cache pressure
or predict latency. Function calls may add loads/spills/ABI costs and lose.

One fixed private selector zopro_g2_fused_outline forwards ONLY
LANES8 && MIXED && FUSED && C::P==FqConfig::P to an inline(never) helper.
That helper calls the unchanged always-inline product_cios body. Ordinary
products, Fr, R260, four-lane and unsupported paths preserve original code.
No field formula, normalization, range, representation, vector layout,
consumer, scratch, key/solver/image, allocator, pool or forecast changes.
No new unsafe/handwritten assembly/heap/table/point copy/plan. Rust ABI can
increase bounded stack traffic; frames/minimumRAM remain unmeasured.

Copy exact175 originals; change only ifma.rs template dispatch/newhelper and
regenerate ifma8.rs with the unchanged official generator.173 original files
must remain exact. Unedit template then official-regenerate to recover both
old kernel hashes and all175 accepted files. Generator/reverse-check receipts
precede build. Native OFF/ON use baseacceptedflags/plusprivatecfg; each must
pass all33field,124core and8raw/cubicprover tests. No new mirrored arithmetic
test: existing independent Arkworks/lazy/fused/chain and complete-point suites
exercise the changed dispatch. Verify generated consistency. Freeze coherent
8direct/transitive/procmacro libraries before standardSDK linking.

Before any performance, inspect actual linked artifacts: OFFhas no outline
helper; ONhelper mono must be Fq/Ifma8/MIXEDtrue/FUSEDtrue and G2borrowedapply
must call it. Save selectedfunction sizes and disassembly. Do not impose a
code-size/gain floor or pick a fastest layout; structural route check only.
No source variant/tuning sweep follows a performance failure.

Use fixed actual-SDK pilot (no isolated primitive speed claim): all9shapes×3
accepted/OFF/ON preflights+Gnark BEFORE45primary32latencies.15fixed rotated/
reversed fresh-process blocks,151requests after3primes, full witness/fresh
randomness/proving/SDKverification/serialization timed; loading excluded.
All45latencies beforetheir45Gnark;3FIFO4eventstat+3FIFO199Hzcycleprofiles
before6Gnark/9decode.165capturesteps,7902SDK/78freshGnark156members,
prospectiveledger17083->17161 ONLYafterclosingpins/binding/hash/freshnessaudit.
Everyhelper/consumer/PMUfeasibility/pin fixed BEFOREcapture. No ownedbuild/
helper/hash/archive/secondworkload overlaps captures. Noexclusivehostclaim.

Gain requiresONupperpairedratio<1and>=10/15winsagainstBOTHcontrols;
OFF/acceptedcalintervalwithin[1/1.03,1.03].Rank4..12nominal96.484375%under
independentexchangeableblocks,nofamilywiseguarantee. No minimumgainfloor,
exclusion, extension, successfulcapturerepeat, changedthreshold orfastestcopy.
Failure closesexactcandidate;pass only permits separatedeclaredall8+54/
physical16/portable/resource/cleanartifact assessment, NOTadoption/goalcomplete.
Reportfreshprimaryand inheritedacceptedall8CPU+Metal with scopesexplicit.
PrioracceptedfullgateFALSEphysical16calibration exception retained. Commit/
push completedexperimentincludingnegative. No freshAWS/Aeglos/mobile/ARM/
TEE/giant2504999/swap/minRAM/activepressure claim from this screen.
