# Early G2 prefix-factor evaluation

Prospective10November2026. Goal unchanged: verified CPU AVX single-proof
>=10x supplied M5CPU for all8 shapes, report CPU+Metal; active/unmet0/8.
Prior goal turn PROGRESS: exact offline instruction mapping pushed232871748f;
no new timing, no inherited live owned job/cloud. Acceptedprimary32 7.783593581ms.

Source/dataflow review rejects the proposed reload helper before implementation.
ax/bx/ay needed after slope comprise30ZMM-sized words at source level. A helper
that drops these and reloads them repeats six radix52 pack operations and1536B
of raw coordinate reads, plus up to640B return storage and320B inverse/factor
state transfers. The exact ABI is compiler-dependent; source counts are not a
spill or runtime-cost proof. Prior whole-loop/per-product outlining failed.

Distinct fixed mechanism: move factor=record_prefix*inverse and inverse*=norm
to the beginning of each G2 reverse8-point group, before a/b coordinate assembly,
packing and differences. Original source has ax/bx/ay/d/dy (up to50source-level
ZMM-sized values) created before these two products. Both products depend only
on records[j] and inverse. Valid input arithmetic commutes with point gathers;
bucket writes, record order, prefix recurrence, signs and scalar-tail bridge
remain exact. No new calls, operations, key copies, tables, heap/record/state
buffers, pool, layout or admission changes. G1 and field source bytes exact.
This is source-level lifetime scheduling, not proof LLVM will improve it.

Isolated181-member snapshot from fused-incoming-dy176: only quarter_kernels.rs
changes; reverse statement movement recovers all181. Reuse byte-exact qualified
field library and its196608independentBigUint+old-path comparisons. Do not rebuild
or rerun unchanged field arithmetic. New coherent optimized core must pass the
four existing prefix/completeWindow/every-worker+sharedpool/layout tests, and
24conversion allocation cases plus fullWindow request counts. Inspect new linked
G2 assembly and compare function size/mnemonics/stack accesses with prior exact
binary before clocks. No speed inference from source liveness/static counts.

If qualified, one frozen four-arm completeWindow campaign: REF retained accepted
API-only; BASE newcoreoriginalWindow; OFF retained fused-incoming-dy176 candidate;
ON earlyfactor. OFF intentionally differs from accepted and is mechanismcontrol.
BASE/ON share new executable. All exact independent-Ark fixtures reused:15045points,
width12,column0,batch200;G1/G2×workers1/16/32,CPU i/solo2,512KiBstacks. Three
checked primes and17timedwaves perprocess;15fixed rotated/reversed context/arm
blocks. FreshWindowconstruction/feeds/queues/scalarpaths/normalization/fulloutput
comparison/checksum/drop included; inputload/hash/poolsetup excluded. One Window
perworker perwave, not wholeMSM/proof/serverthroughput.

Same48prospective checks:6BASE/REFcalibration factor1.03;18ONnonreg upper<=1.02
against BASE/OFF/REF;3primaryG232 +3equal-logG1/G232 gains upper<1 and>=10/15wins
against all3;6RSSmaxcontrols*1.05+1MiB;12PMU/profilequality. Fixedranks4..12/15,
nominal96.484375% individual median coverage assuming exchangeable independent
blocks, nofamilywise guarantee. No exclusions/extensions/gainfloor/unchanged
reroll/favorableworker-group/parameterrescue after clocks.
24preflight360latency before6PMU+6profiles2048waves/FIFOwarmgates/period16M/
512pages,>=1000samples/unknown<=5%/zeroLOST/PIDTIDCPU/MMAPchecks/PMU>=99.9%,
then30offline steps=426steps/396callers/906752checkedWindows. Freeze all source,
deps,tools,binaries,inputs,order,gates,producer/rawreader beforefirstpreflight.
During ANY preflight/recording ONLYpoll exactconfirmedlivehandle untilterminal;
no owned helper/build/hash/analyze/report overlaps. Timeout/truncation notterminal.

CommitANDpush completed experiment/exactremoteverify; preserve everyfailure and
positive result. Polite factual report includes ALL8 CPU+Metal inheritedexplicitly.
Passingcomponent requires fullquery/ALL8+54SDK/freshGnark/resources beforeadoption.
Canonical44265 unchanged; seed allfutureexports frontend-overlap176/proof-sha256-
after-capture.json. Rust1.98.1+serializedxtask; noquestions/subagents/Python/CASM/
giantLUT/expandedkeys/solverplans/newproductionpool/arbitraryRAMcap/newAWS.
