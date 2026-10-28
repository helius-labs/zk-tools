# Four-chain norm inversion before borrowed G2 finishing

Prospective private candidate on accepted174/SDK81033, no speed claim.
The preceding norm mechanism was found to duplicate the older G2-norm trial;
report correction edb4c0093 is pushed. That history miss is not justification
for a further norm rerun. The corrected mixed timings and earlier rejection
stand. No symmetric-norm candidate code enters this source copy.

Source audit of Aeglos pinned0290613e8eeca17a4ce88f6f2d726e8e06a5385d:
cpu/msm_impl.hpp Lane1::invert_all uses4chains then materializes inverse
denominators before finishing; Lane2 converts denominators to base-field norms.
VMont uses normalized lazy limbs below2p and plain CIOS. Its PairBatch keeps
more point-coordinate records, and its phase scheduler differs. No Aeglos
performance claim or copied implementation is imported. Primary sources:
https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm_impl.hpp
https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/lanes/vmont.hpp
Raw inspected source stays local; report retains source URLs and hashes only.

History distinctions: 2026-10-22 G1chains interleaved2/4 in retained G1 point
loops and was rejected under its old3%floor; no failed verdict is changed here.
2026-10-26 ownedG2finish left forward/inverse recurrence unchanged, consumed
incoming X to separate output Y, and regressed. 2026-10-26 G2prefix-tree changed
bucket topology with larger point arrays. New candidate does none of these:
only borrowed G2 affine batches n>=64 select it, original fallback below64.

Fixed algorithm: gather each norm into existing prefix_records[j][1]; form4
interleaved prefix products in record[j][0]; combine four roots with a binary
tree; use the accepted central inversion and exact existing scalar-tail block;
distribute the inverse to4roots; reverse just the norm recurrences, overwriting
the now-dead record[j][0] with1/norm. Then the original full-point reverse pass
uses this factor, without maintaining an inverse recurrence amid point values.
Borrowed key/input coordinates stay immutable; distinct destinations preserve
reverse reads. Record[j][1] keeps its norm. No new vector allocation/header,
point buffer, table, key, plan, pool, delay, scalar or field primitive changes.
Exactly9extra FqBatch products/batch:3root products+2root inverse factors+
4individual inverses. Repacking and multiple scratch passes may lose. Stack
temporaries increase; unchanged heap capacity/forecast is not stack/RSS proof.

One fixed4chain schedule only; no2/4/width/padding/prefetch sweep, no source or
layout reroll. New private module plus2original declaration/dispatch edits,
172originals exact/reverse174, candidate175. Privatecompilecfg selects ON.
OFF retains original body; prepend cfg return on the existing nonpanicking
first-statement line and append module declaration to preserve old line numbers.
No symmetric-norm/old-experiment runtime present. Native supported guards only;
unsupported dispatch, grouping, scalar preparation and field sources exact.

OFF/ON full unfiltered124core +8prover tests each before capture; two new tests
check each stored norm inverse/point result with complete scalar references and
scratch reuse across full/tail lengths. Existing independent MSM/generator and
borrowed-G2 edge tests remain. Field source/qualification inherited, no new
field suite claimed. Freeze coherent native rlibs/proc macros, actual flags/
features checked. SeparateOFF/ONSDKs plus exact retained accepted81033 control.

Fixed7rotated/reversed blocks×4cells(primary16/32,16x2/32,merge54/32)×3variants.
12one-roundpreflights independentlyGnark verify BEFORE84latency151primary/
31others+3primes;84Gnarkafter. Primary32gain upper<1 with>=5/7wins against
OFFANDaccepted, OFF/acceptedboundsfactor1.03. LargerONuppers<=1.03; their
calibration separately reported. Physical16descriptive. Seven-ratioextrema
medianbounds98.4375%nominal underindependentexchangeableblocks,nofamilywise.
No gainfloor/exclusions/extensions/successfulcapturerepeat. Passingonlyselects
furtherall8/clean/portable/resourcesqualification, not adoption/goalcompletion.

After all latency/verification:6FIFOgroupedPMUcaptures primary16/32×3v,
31requests+3primes, {cycles:u,instructions:u,cache-misses:u,branch-misses:u},
allactualrunning>=99.9%. Then6separate199Hzcycles:u profiles251requests+3primes,
onesampledPID/zeroLost each. All capture BEFOREdiagnosticverification/decoding/
hashes/archive. No owned build/helper/hash/archive/verifier/second benchmark
duringproofcapture. Rootperf variants vsnormaluserlatency disclosed; noglobal
kernelsetting/installedprobes. Counter/profiletimes do not replace latency.

Total9672internallyverifiedSDKrequests,108fresh164Bexports/216members,
ledger15433->15541. Acceptedall8CPU+Metal table retained, goalACTIVEUNMET0/8.
No AWS/Aeglos/mobile/ARM/TEE/giant/minRAM/swap/pressure/concurrentproof claim.
No prediction of2x gain. No production mutation. Commit AND push completed
experiment, negative too; noquestions/subagents.
