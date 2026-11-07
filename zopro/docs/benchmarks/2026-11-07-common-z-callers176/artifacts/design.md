# Complete segmented common-Z caller experiment

Continue a5e9d1908a/8e86ac25d3 without modifying or replaying their qualified
sources, binaries, arithmetic tests or results. No latency has been measured.

Integrate a new segmented G1 finish at the two unweighted production loops
(group_affine_impl and indexed tiles), plus the qualified diagnostic reducer.
Do not modify reduce_layer itself: weighted_sum requires exactly one level per
call. Weighted arithmetic and G2 remain unchanged.

At a loop boundary, select only if all live points across the current task fit
sixteen slots and some group still has at least three points. The first limit
is one eight-lane pair vector; the second guarantees at least two remaining
levels so final normalization can remove an inverse. These rules precede
timing. No threshold, width, worker or layout sweep is selected.

All groups share one product/common Z across the finish. Retain group boundaries,
zero/singleton groups and odd carries; normalize all final outputs with one Z
inverse. Initial identities or equal x at any level decline transactionally and
the original reducer continues from the untouched original frontier. No
speculative output is committed. Never call the standalone single-group kernel
independently for each group; that loses accepted cross-group inverse sharing.

Use a fixed stack frontier and existing field1200/IFMA methods, no new unsafe,
field, key table/copy, duplicate plan, heap scratch or production pool. Read
the private runtime option once in Scratch::default; G2 uses false. Original
source must recover exactly after reversing the recorded edits. Keep original
176 members, current field library and prior frozen diagnostic control pinned.

Qualify segmented arithmetic, zero/singleton/carry/mixed groups, initial and
late exceptions, input immutability, discontiguous frontier slots, actual
production grouping/indexed outputs and 512KiB worker stacks. Reuse unchanged
field and parent qualification; run changed core tests as appropriate.

Before capture, pin the final native/O3/fatLTO/CGU1 complete caller executables,
all helper sources, receipts and prospective gates. Compare old accepted
diagnostic library (original), fresh cfg-disabled control (fresh), and one
candidate executable runtime OFF/ON. Use qualified TLS allocation/CPU/stack/
warm-gated perf harness, with only declared case additions. The original library
is reused, not rebuilt. Same immutable synthetic inputs and expected complete
outputs in all arms; seven old case output digests are inherited.

Fixed cases: tile257,tile384,tile511,tile2048,group35973,groups31x256,tails,
small16,groups3x4,small-mixed. All1/16/32workers. Clock includes work allocation
where the original case does, input conversion/gather/scatter, original
reduction/finish, dispatch and wait. Pool/fixture construction and final oracle
are outside. These are reducer caller waves, not SDK proofs or throughput.
The many-group case is an intentionally ineligible negative control.

Fixed fifteen rotated/reversed matched blocks, no exclusions/extensions or
rerolls. Rank4..12/15 intervals, at least11winning blocks for gains; no minimum
percentage gain floor. Require fresh/original and OFF/original calibration
intervals inside factor1.03 in every context, ON/control upper<=1.03 in every
context, and equal-log all-case aggregate gains at32workers against original,
fresh and OFF (upper<1). Capacity must not exceed corresponding control by>5%
where actually observed; process RSS is reported, not admission/minimum RAM.
Preserve all failed checks and every positive context; no favorable subset
promotion. Any failed gate closes this fixed implementation without SDK trial.

After latency, fixed warm-gated PMU/profiles compare original/ON for tile511 and
groups3x4 at32workers (large material/short-tail cases); exact repetitions and
sampling parameters must be frozen in capture protocol. No owned builds,
hashes, archives, analyses or second workloads overlap recording. On success,
continue actual SDK/all8+merge54, independent Gnark/freshness ledger/resource
qualification before adoption. Goal remains full all8 CPU10x, active0/8.
