# Interleaved shared-inverse products in the R260 G2 reverse group

Prospective10November2026. Goal remains verified CPUAVX single-proof >=10x M5CPU
ALL8 with CPU+Metal comparisons; ACTIVE/UNMET0/8. Previous goalturn PROGRESS:
earlyfactor scheduling improves G2 andaggregate butCLOSEDFAIL1/48,pushed71cdb9e882.
No inherited live owned build/recording/cloud. Acceptedlocal32primary7.783593581ms.

Prior-art check:2026-10-25-dual-cios-component implements interleaved shared
operand products only for8lane MIXED/R256; its R260 arms retain ordinarymul.
R256component improves2.58%pair butdual-cios-sdk fails primary/OFF win-count,
despite small positive proof estimates.2026-10-29-interleaved-complex175 instead
interleaves a complex fused-sum kernel and loses .303%aggregate. Do not import
those speed estimates or repeat their unchanged recipes. Current trial targets
nativeFq/R260 and newly early prefix-factor/next-inverse pair, not G2component
numerator scaling. Document code provenance; no newperformance claim fromnovelty.

New private Wide2::mul_pair interleaves two5rowR260 CIOS products sharing inverse.
Each stream exactly matches ordinaryproduct_cios(false) with cancelcarry. No
summation across outputs, representation change or partiallyreducedoperand.
Bothstreams inputs<2p, output REDC<p+4p^2/2^260<2p; eachcoefficient/carry bound
unchanged because streams independent. All5cancellationrows use52bits (noR256
final48pack). Nativequartermoduleonly; guards/genericportable unchanged. G2
earlygroupuses inverse.mul_pair(prefix,norm). Separate oldprefix*inverse exact
representative mustagree even operandorientation changes. G1source unchanged.
No newLUT/table/keycopy/solverplan/heap/recordbuffer/layout/pool/admission changes.
Twelve localaccumulators may increase register/stack pressure and lose; no2xclaim.

Before clocks: new independentBigUint exactREDC quotient and fieldmod oracle,
oldpath exactrepresentatives,boundary lazy0/p/2p-1,powers52j±1 andcarry corners,
all boundarypair combinations,4096randomgroups,4096output-fedgroups, twooutputs
8lanes each; unchangedinputs andstrict<2p outputs. Four existing completeprefix/
Window/worker+sharedpool/layout tests oncoherentnewfield/core;24conversion cases
plusfullWindowallocationpanel. Full source reversal of2children recovers181member
parent,179exact; all176production unchanged. Nativeold/new pairprobes retain
fullassembly/instructions/IFMA/calls/stack; staticcounts notcaller speed.

Ifqualified, onefrozen fourarm actualWindow campaign:REFretainedacceptedAPI-only;
BASEnewcoherentcoreoriginalWindow;OFFretainedearlyfactorcandidate;ONsharedpair.
BASE/ONsharebinary; OFFmechanismcontrol intentionallydiffersfromaccepted. Same
independentArkfixturebytes15045points/w12/column0/batch200;G1/G2×1/16/32workers,
CPU i/solo2,512KiBstacks.3checkedprimes17timedwaves,15rotated/reversedblocks.
Windowconstruction/feeds/queues/scalarpaths/normalization/fulloutputequality/
checksum/drop timed;inputload/hash/poolsetup excluded. Wave oneWindowperworker,
notwholeMSM/proof/serverthroughput. No oldscientificfixtures/testsreplay unless
neededtonewcodequalification;unchangedfieldoperations inherited.

48prospectivechecks unchanged:6BASE/REFfactor1.03cal;18ONnonreg upper<=1.02vs
BASE/OFF/REF;3G232+3equal-logG1/G232gainupper<1 and>=10/15winsvsall3;6RSS
maxcontrols*1.05+1MiB;12quality. Ranks4..12/15 nominal96.484375% individual
coverage,independentexchangeableblocks,no familywise guarantee. No exclusions,
extensions,gainfloor,thresholdchange,unchangedrerun orfavorableworker/group rescue.
24preflight360latency BEFORE6PMU+6profiles2048waves/FIFOwarmgates/period16M/
512pages/1000samples/unknown<=5%/zeroLOST/PIDTIDCPU/MMAP/PMUcoverage>=99.9%,
then30offline=426commands396callers906752checkedWindows. Freezeallsource/tools/
deps/binaries/inputs/order/gates/producer/rawreader beforefirstpreflight.
DuringANYpreflight/recording ONLYpoll exactconfirmedlivehandle untilterminal;
nohelper/build/hash/analyze/reportoverlap;timeout/truncationnotterminal.

CommitANDpush completedexperiment/exactremoteverify. Preservefailures/positives;
polite factual ALL8CPU+Metal comparisons inheritedexplicit. Passingcomponent
thenfullquery/ALL8+54SDK/freshGnark/resources beforeadoption. Canonical44265
unchanged;seedallfutureexports frontend-overlap176/proof-sha256-after-capture.json.
Rust1.98.1+serializedxtask;noquestions/subagents/Python/CASM/giantLUT/expandedkeys/
solverplans/newproductionpool/arbitraryRAMcap/newAWS. Fullgoal nevernarrowed.
