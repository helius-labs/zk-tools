# Prospective complete weighted-consumer comparison

Goal active/unmet: single-proof CPU AVX >=10x supplied M5 CPU all8, report CPU and Metal. This experiment measures an actual component, not proof latency or concurrent-proof throughput. Kernel/core/source/generic/resource qualification is inherited from weighted-double-consumer176 without replay. Native ON is the corrected frozen ON-tail library, not initial history. Same original field1200 in accepted-source REF, fresh OFF and final ON. REF has only the common read-only benchmark API appended to accepted176; OFF/ON libraries are reused exactly. Actual caller links O3/native/fatLTO/oneCGU with the original allocator and default Rayon stacks.

Inputs are independent-Arkworks public-generator fixtures, 94720 distinct full-width coefficients and 24 point vectors. They use a deterministic double+fixed-point recurrence, not independent random draws or actual key buckets. Generator endpoint checks, curve checks and weighted coefficient sums bind every expected vector output. Fixtures are benchmark inputs, not a product lookup table. Driver checks every timed output against those independent expected values and checks immutable inputs after the clocks.

Cases: one reducer with2048/4096/16384buckets at1/16/32workers, plus a20window wave of5x2048+15x4096 at1/16/32, plus unchanged512bucket control at32:13conditions. The20window mixture reflects accepted primaryK/H bucket sizes; it excludes fill, other queries, solver/FFT and proof assembly. Both window-level and inner weighted parallelism execute in the same existing global pool. Workers pin toCPU2solo orCPUi at16/32; coordinator inherits the case mask. Worker/TID/CPU and N+1task counts are checked before/after; no secondary production pool.

Every timed wave includes original complete weighted entry, allocations/drop, tree reductions, both required group sums, newELM or oldprojective folds, transfers, exceptions, offset corrections, output collection and normalization. Three checked prime waves and one checked postwave are excluded. Loading/generation/oracle checking is outside timing. Input/vector-output storage is benchmark state and is included in whole-process peakRSS, not proposed prover persistent state.

Primarybalanced20-w32:31matched blocks,151waves/process. Other12conditions15blocks each. Single2048/4096/fallback512 have501waves/process; single16384/balanced20 have151. Rotate/reverse cases and REF/OFF/ON order in each block. All39onewave preflights precede633latency processes. All latency precedes9PMU panels and9cycle profiles:balanced20-w16/w32 andsingle4096-w32, all3arms. Statrounds512balanced/2048single; profilerounds4096balanced/16384single. ControlFIFOenable afterprimes/disable beforechecks. Cycles:u period16M,512mmap pages,IP/CPU/time/period,noDWARF. All recordings end before45ownership/decode/events/metadata/loss commands and any analysis. Total735commands/690callerprocesses. No independent cryptographic proof is created.

Statistic: arithmetic mean of emitted integernanoseconds perprocess; display median of process means. Pair blocks. Original unchanged20kpaired-log bootstrap seed0x551c36a48ee2d971/ranks499,19499. Four-context native32 aggregate uses equal logs and independent within-context resampling at31/15blocks with the same RNG/seed/ranks. It is not an all8proofaggregate or synchronized cross-case sample. Assume exchangeable blocks, no familywise guarantee.

Frozen91numericchecks:
- All13OFF/REFcalibration95intervals wholly within[1/1.03,1.03].
- All26ON/controlnonregression upper95<=1.03.
- All26ON/controlmaximum whole-processRSS<=1.05times controlmaximum.
- Primarybalanced20-w32 gainupper95<1 and>=21/31wins againstBOTHcontrols:4checks.
- Four-contextnative32aggregate gainupper95<1againstBOTHcontrols:2checks.
- BothON/control unchanged512intervals wholly within[1/1.03,1.03]:2checks.
- All9groupedPMUpanels four finite counters and>=99.9%running.
- All9profiles>=1000samples,unknown<=5%,zeroLOST,exact16Mperiod,actualbinary/PID/TID/workerCPUownership/coordinatormask:9checks.

Correctness/source/receipt/identity/placement failures invalidate selection separately. Allcalls/checks retained. No gainfloor, exclusions, extensions, rerolls, favorable case/topology promotion, changed thresholds or post-result compiler/layout/size/tile rescue. Failure closes this fixed8groupconsumer without an unchanged recapture or preauthorized engineering exception. A fullpass only selects later realSDK/all8+54/Gnark/resource/admission/portable/concurrency qualification; it is not adoption or goal completion.

Before any preflight freeze protocol, sources/readers/helpers/order/rounds/inputs/actual optimized executables and prospective review. DuringANYpreflight/recording ONLYpoll the exactconfirmedlivehandle untilterminal;timeout/truncationnotterminal;nohelper/build/hash/decode/analyze/archive/pushoverlap. Rust1.98.1serializedxtask;noPython/questions/subagents/newpool/artificialcap/largepersistentLUT/keyplan/CASM. Savefactualpolitereport,commitANDpush,verifyexactremoteHEAD,updatecanonicalcheckpoint. Rawdata/fullledgers stayprivate. No newAWS/Aeglos/mobile/huge/pressure/swap/minRAM result follows.
