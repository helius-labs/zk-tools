# Build weighted trees as initialized prefixes

Goal remains verified single-proof CPU AVX >=10x supplied M5CPU for ALL8,
with CPU+Metal comparisons. Active/unmet0/8. No budget/questions/subagents.
Rust1.98.1/xtask; commit AND push completed experiments, polite factual reports.

Mechanism: weighted_affine currently resizes each tree to2n-1 initialized
infinity points, overwrites the first n with inputs, then weighted_sum
writes every internal point before its first read. The new private cfg
zopro_weighted_live_prefix clears/reuses the existing Vec, reserves the same
2n-1 capacity, extends only n initialized leaves, and appends saved even
points before each reduction level. Every live slice contains initialized
points. Use safe Vec push; no uninitialized reference/set_len/raw pointers.
All existing arithmetic, reduction/layer/start/count order, inverse modes,
point layout, allocation envelope and thread pool remain unchanged.

Change ONLY weighted_affine's initialization and dispatch plus a new child
copy of weighted_sum with Vec argument/live-length assertion/appending.
Original weighted_sum and all other callers remain exact. OFF expands to
original runtime statements. Both native curves and generic code participate;
no field/kernel/generator mutation or primitive build/test replay.
No new key/scalar/point-array copy, persistent cache/table, execution plan,
production pool, C/ASM rewrite or artificial RAM cap. Existing saved even
points are still copied once into the same positions. Maximum tree payload
unchanged; compiler stack and actual performance remain to be measured.

Source/history search found compact/packed/weighted tree representations,
policy-owner snapshots and page-initialization experiments, without a matching
safe initialized-prefix-only route in the read summaries. No broad novelty
claim. Those negative algorithms remain closed and are not combined here.

Native qualification BEFORE clocks: both-curves generator-weighted/original
reference comparisons, byte/limb equality of final complete tree, all level
boundaries1..1024powersoftwo, linearfullwidth/mixedequal-opposite/identity/
completecancellation patterns, vector/scalar inversions, growing/shrinking
reuse and unchanged input/capacity. Check actual1/4/16/32worker indices,
512KiBrequestedstacks and nested joins for both curves. New ON wholecore
suite once; OFF onlynewsharedoracles; inherit unchangedfield qualification.
Compile generic noASM/noIFMA. Reverse edits recoverall176sourceparents.

Then coherent OFF/ON Prover/Zolana/SDK links with exact originalfield1200,
actual nativeO3/fatLTO/oneCGU. Verify actual machine setup routes show the
intended initialization change before clocks. No speed claim from fewer
source stores; Vec capacity checks and code generation can offset it.

First performance should measure complete SDK requests, because this is
an initialization/dataflow-only change ratherthan an isolated arithmetic
operation. Freeze controls acceptedSDK6d83/freshOFF/ON, exactbindings,
ALL8+54/primary16 correctness/coverage, 16/32worker cohorts, pairedblocks,
calibration/no-gain-floor, resources, automatic PMU/profiles and prospective/
resultsreaders BEFORE ANY recording. A passing first screen selects full
ALL8+54/admission/portable/concurrent qualification before adoption; it does
not redefine the fullgoal. Do not adopt a candidate from source counts.

All latency beforePMU/profiles;all recording beforeofflineGnark/analysis.
DuringANYrecording includingpreflight ONLYPOLL exactconfirmedlivehandle;
nootherownedCPUwork/build/helper/hash/decoder/verifier/report/archive/push.
Never rerun successful unchanged suites/captures/decoders for bookkeeping.
Seed canonical37633ledger fromquery-neutralization-sdk176 fornew exports.
Latestaccepted1x2=8.031248742ms,4x2=15.961623323ms;other6inherited;
historical7.915095 remainsvalid. No adoptedregression fromroughly8ms.
