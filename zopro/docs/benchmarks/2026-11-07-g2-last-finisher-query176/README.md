# G2 partition completion: complete-query results

**The last-finisher scheduler is not adopted.** Primary32 does not establish a gain, and grouped2×2G2 at32workers regresses against both controls. Both16-worker B cases improve; those scoped positives remain in the results. The declared gate fails15/42 checks.

The [native-qualified scheduler](../2026-11-07-g2-last-finisher176/README.md) submits original G2 window/partition fills directly to one existing Rayon scope. The last part completes the original merge and weighting. Arithmetic,widths,two-partgeometry,scalar grouping,points and the pool remain. This study tests complete queries,not proof latency or a kernel-only loop.

REF is a newly built accepted-SOURCE reference with the same diagnostic adapter,not accepted SDK6d83. OFF contains the inactive candidate source and adapter;ON selects the scheduler. All three binaries are distinct,optimized,native,fatLTO/oneCGU consumers linking originalfield1200. Old coalesce-query binaries with the deficient extern-only link profile are not used. Four cached actual1×2/2×2A/B fixtures and independent Arkworks full-MSM outputs match the prior reviewed bindings;no witness export,converter or oracle execution is repeated.

## Decision and all timings

Two unchangedAcontrol-calibration,eight nonregression and five gain checks fail. Every B control-calibration check passes. This is a calibrated B comparison but not a globally calibrated study. A is unchanged contextual work;its failures do not establish a G2arithmetic cause.

PrimaryB32 medians are3.021033msON versus3.031019msOFF and3.023735msREF. The paired estimates are0.055%and0.264%lower,but both intervals include parity,with8/15wins. Grouped2×2B32 is2.37%slower thanOFF and1.08%slower thanREF,with1/15and2/15winning blocks and intervals above parity. These are G2query times,not1×2proof times.

At16workers,primaryB improves1.07%againstOFF and1.12%againstREF;grouped2×2B improves2.12%and2.73%. All four intervals are below parity. ALL-B6 improves0.525%againstREF,but lacks a gain againstOFF. Solo B wins all15blocks againstREF while losing all15againstOFF. The full comparison against both controls is retained;no favorable worker/shape subset is selected.

| Declared scope/control | ON/control ratio [marginal interval], wins |
|---|---|
| PRIMARY-B32/ON/OFF | 0.999446245 [0.991468367, 1.013921980], 8/15 |
| PRIMARY-B32/ON/REF | 0.997355919 [0.991591989, 1.007694955], 8/15 |
| B32/ON/OFF | 1.011701593 [1.006481449, 1.026339941], 3/15 |
| B32/ON/REF | 1.004384291 [0.999739869, 1.015866196], 6/15 |
| ALL-B6/ON/OFF | 1.000802304 [0.998898697, 1.003854231], 5/15 |
| ALL-B6/ON/REF | 0.994751863 [0.992524995, 0.996231847], 14/15 |


Each displayed time is the median of15process medians over31timed queries. Paired ratios use corresponding blocks and need not equal ratios of displayed medians. All samples and loop means are preserved.

| Context | REF ms | OFF ms | ON ms | ON/OFF [interval], wins | ON/REF [interval], wins | OFF/REF [interval], wins |
|---|---:|---:|---:|---|---|---|
| 1x2-A-w1 | 14.720183 | 14.701592 | 14.675509 | 0.998922514 [0.997212460, 1.000043158], 11/15 | 0.997579380 [0.996376238, 0.999097019], 14/15 | 0.998001653 [0.997174090, 1.000930632], 9/15 |
| 1x2-A-w16 | 1.546723 | 1.539632 | 1.536004 | 0.997032681 [0.988743706, 1.008167712], 10/15 | 0.995738086 [0.986483941, 1.002219634], 11/15 | 0.995568473 [0.990996449, 1.006083732], 9/15 |
| 1x2-A-w32 | 1.191943 | 1.186163 | 1.188297 | 1.002269944 [0.990111641, 1.025849465], 6/15 | 1.008384287 [0.980507457, 1.027777660], 6/15 | 1.001879609 [0.975739918, 1.016465170], 7/15 |
| 1x2-B-w1 | 37.522200 | 37.056858 | 37.366345 | 1.008233114 [1.007585558, 1.010170053], 0/15 | 0.996583541 [0.994591543, 0.997280335], 15/15 | 0.987184334 [0.986469824, 0.989772359], 15/15 |
| 1x2-B-w16 | 3.443160 | 3.438652 | 3.399908 | 0.989265512 [0.977661014, 0.998380836], 12/15 | 0.988770785 [0.980605390, 0.994297634], 14/15 | 0.996359833 [0.983805820, 1.003248828], 10/15 |
| 1x2-B-w32 | 3.023735 | 3.031019 | 3.021033 | 0.999446245 [0.991468367, 1.013921980], 8/15 | 0.997355919 [0.991591989, 1.007694955], 8/15 | 1.002408941 [0.984757047, 1.009159852], 7/15 |
| 2x2-A-w1 | 14.560967 | 14.527820 | 14.520195 | 0.998695680 [0.996428143, 1.000840732], 10/15 | 0.997365572 [0.994375943, 0.999203676], 14/15 | 0.997463081 [0.995797054, 1.000465413], 11/15 |
| 2x2-A-w16 | 1.884451 | 1.888990 | 1.882209 | 0.994096845 [0.971588903, 1.004212415], 9/15 | 0.993108975 [0.973123526, 1.053963019], 10/15 | 0.996395289 [0.978558794, 1.054136374], 8/15 |
| 2x2-A-w32 | 1.519382 | 1.508993 | 1.501649 | 0.990001973 [0.976837875, 1.032364941], 9/15 | 0.990238829 [0.973477966, 0.999293164], 12/15 | 0.992018295 [0.963410164, 1.001820496], 11/15 |
| 2x2-B-w1 | 39.951225 | 39.500966 | 39.685341 | 1.004659582 [1.002838422, 1.004886669], 0/15 | 0.992494772 [0.991239956, 0.994185549], 15/15 | 0.988106428 [0.986809674, 0.989359569], 15/15 |
| 2x2-B-w16 | 3.934500 | 3.922264 | 3.824408 | 0.978805354 [0.972515337, 0.986935180], 14/15 | 0.972735295 [0.970934234, 0.977624408], 15/15 | 0.994915325 [0.984709890, 1.001657911], 11/15 |
| 2x2-B-w32 | 3.523342 | 3.476103 | 3.564358 | 1.023744793 [1.016391056, 1.039240122], 1/15 | 1.010827571 [1.002163971, 1.020993299], 2/15 | 0.987443765 [0.976267742, 0.995438289], 14/15 |


All failed checks:

- `1x2-A-w32/ON/OFF/nonregression`
- `1x2-A-w32/ON/REF/nonregression`
- `1x2-B-w1/ON/OFF/nonregression`
- `1x2-B-w32/ON/OFF/nonregression`
- `2x2-A-w16/OFF/REF/calibration`
- `2x2-A-w16/ON/REF/nonregression`
- `2x2-A-w32/OFF/REF/calibration`
- `2x2-A-w32/ON/OFF/nonregression`
- `2x2-B-w32/ON/OFF/nonregression`
- `2x2-B-w32/ON/REF/nonregression`
- `ALL-B6/ON/OFF/gain`
- `B32/ON/OFF/gain`
- `B32/ON/REF/gain`
- `PRIMARY-B32/ON/OFF/gain`
- `PRIMARY-B32/ON/REF/gain`


## Resources and CPU observations

The scheduler adds no point/key copy,lookup table,solver plan,new pool or unsafe block. Its3,080-byte control-vector payload is charged before scalar mutation. Scoped Rayon task heap allocations,allocator overhead and stacks are additional and are not a complete added-memory qualification. The observed whole-process RSS includes fixture parsing,primes and allocator history;it is not minimum RAM,an exact lifetime peak or a pressure test. Grouped2×2B32 reaches69.00MiB ON versus54.00MiBOFF and60.50MiBREF across latency processes;this does not isolate the cause or establish a fixed new footprint.

| Context/arm | Max process RSS MiB | CPU ms/query | Minor faults/query | Voluntary switches/query | Involuntary switches/query |
|---|---:|---:|---:|---:|---:|
| 1x2-A-w1/REF | 9.500 | 14.722323 | 0.065 | 2.000 | 2.097 |
| 1x2-A-w1/OFF | 9.500 | 14.700258 | 0.065 | 2.000 | 2.097 |
| 1x2-A-w1/ON | 9.500 | 14.676516 | 0.065 | 2.000 | 2.097 |
| 1x2-A-w16/REF | 19.000 | 17.103419 | 17.774 | 62.742 | 0.129 |
| 1x2-A-w16/OFF | 19.000 | 17.087129 | 16.871 | 63.032 | 0.161 |
| 1x2-A-w16/ON | 18.750 | 17.070065 | 16.226 | 62.581 | 0.194 |
| 1x2-A-w32/REF | 22.750 | 24.871194 | 43.065 | 76.613 | 0.581 |
| 1x2-A-w32/OFF | 25.250 | 24.744935 | 43.710 | 77.226 | 0.613 |
| 1x2-A-w32/ON | 25.250 | 25.094387 | 43.613 | 79.129 | 0.710 |
| 1x2-B-w1/REF | 15.500 | 37.519871 | 0.065 | 2.000 | 2.161 |
| 1x2-B-w1/OFF | 15.500 | 37.074323 | 0.065 | 2.000 | 2.194 |
| 1x2-B-w1/ON | 15.500 | 37.359129 | 0.226 | 2.000 | 2.258 |
| 1x2-B-w16/REF | 27.500 | 45.417742 | 24.000 | 117.258 | 0.065 |
| 1x2-B-w16/OFF | 28.250 | 44.954935 | 26.613 | 118.839 | 0.065 |
| 1x2-B-w16/ON | 26.500 | 43.967419 | 20.774 | 98.710 | 0.129 |
| 1x2-B-w32/REF | 35.500 | 69.725226 | 35.129 | 199.000 | 1.161 |
| 1x2-B-w32/OFF | 35.250 | 70.299323 | 49.032 | 201.355 | 1.194 |
| 1x2-B-w32/ON | 34.000 | 69.497935 | 30.903 | 185.806 | 1.065 |
| 2x2-A-w1/REF | 13.000 | 14.565871 | 0.065 | 2.000 | 2.129 |
| 2x2-A-w1/OFF | 13.000 | 14.525484 | 0.065 | 2.000 | 2.065 |
| 2x2-A-w1/ON | 13.000 | 14.520194 | 0.065 | 2.000 | 2.065 |
| 2x2-A-w16/REF | 21.250 | 17.749774 | 20.742 | 60.742 | 0.065 |
| 2x2-A-w16/OFF | 22.750 | 17.703355 | 22.065 | 59.484 | 0.097 |
| 2x2-A-w16/ON | 22.750 | 17.654677 | 20.516 | 59.000 | 0.097 |
| 2x2-A-w32/REF | 30.250 | 26.158419 | 58.323 | 75.419 | 0.581 |
| 2x2-A-w32/OFF | 30.000 | 26.156452 | 62.677 | 76.129 | 0.645 |
| 2x2-A-w32/ON | 33.750 | 26.257484 | 60.871 | 76.484 | 0.548 |
| 2x2-B-w1/REF | 27.250 | 39.967194 | 0.065 | 2.000 | 2.161 |
| 2x2-B-w1/OFF | 27.250 | 39.507129 | 0.065 | 2.000 | 2.194 |
| 2x2-B-w1/ON | 27.250 | 39.702839 | 0.065 | 2.000 | 2.194 |
| 2x2-B-w16/REF | 44.250 | 48.727000 | 22.000 | 109.226 | 0.097 |
| 2x2-B-w16/OFF | 43.750 | 48.171677 | 20.613 | 109.290 | 0.129 |
| 2x2-B-w16/ON | 43.750 | 47.892194 | 17.065 | 105.581 | 0.161 |
| 2x2-B-w32/REF | 60.500 | 75.510355 | 76.806 | 181.677 | 1.419 |
| 2x2-B-w32/OFF | 54.000 | 75.525484 | 76.452 | 181.097 | 1.129 |
| 2x2-B-w32/ON | 69.000 | 77.096452 | 75.419 | 214.871 | 1.290 |

Fault/switch values are medians per timed query across15latency processes. Minor faults are mappings/first touches,not measured diskI/O or fault-service time. These counts do not by themselves explain the elapsed differences.


All12PMU panels qualify with numeric counters and at least90%running coverage. They are separate single observations,not confidence intervals or independent matched latency replicates. Primary32 instructions/cycles remain near parity versusOFF;the grouped32panel grows. A shorter dependency/join structure did not establish the required latency gains.

| PMU context | ON/OFF instructions | ON/OFF user cycles | ON/REF instructions | ON/REF user cycles |
|---|---:|---:|---:|---:|
| 1x2-B-w1 | 1.001683371 | 1.008683504 | 1.001685129 | 0.995168892 |
| 1x2-B-w16 | 0.999651327 | 0.983125187 | 0.998484471 | 0.968758658 |
| 1x2-B-w32 | 0.999905251 | 0.995095011 | 1.000590081 | 0.991202063 |
| 2x2-B-w32 | 1.003775526 | 1.010421086 | 1.001590609 | 1.008938453 |


All12profiles qualify:88,831samples,zeroLOST,maximumunknown share1.0494%. Fixed16Mcycles:u periods,512pages,warmFIFO and executableMMAP/PID/FORK/registeredpinnedworkerCPU are checked. The coordinator may run on any allowedCPU. Perf-resolved symbols do not supply independentASLR decoding,absolute cycle coverage or exclusive wall-time attribution.

| Profile | Samples | Unknown share | Epoch category share |
|---|---:|---:|---:|
| cycles-1x2-B-w1-OFF | 1900 | 0.3684% | 0.0000% |
| cycles-1x2-B-w1-ON | 1914 | 0.5747% | 0.0000% |
| cycles-1x2-B-w1-REF | 1921 | 0.6247% | 0.0000% |
| cycles-1x2-B-w16-OFF | 6943 | 0.7346% | 3.9752% |
| cycles-1x2-B-w16-ON | 6805 | 0.9552% | 3.5562% |
| cycles-1x2-B-w16-REF | 7030 | 0.8393% | 4.0256% |
| cycles-1x2-B-w32-OFF | 9942 | 1.0159% | 10.8831% |
| cycles-1x2-B-w32-ON | 9897 | 1.0205% | 9.6191% |
| cycles-1x2-B-w32-REF | 9915 | 0.9581% | 10.6404% |
| cycles-2x2-B-w32-OFF | 10805 | 0.9903% | 10.2545% |
| cycles-2x2-B-w32-ON | 10991 | 0.9917% | 10.7634% |
| cycles-2x2-B-w32-REF | 10768 | 1.0494% | 10.1690% |


The primary32 epoch category falls from10.88%OFF to9.62%ON,while grouped2×2B32 rises from10.25%to10.76%. Neither movement is an exclusive/removable wall-time budget or a causal diagnosis. Inlining,code placement,allocation,scheduling and cache effects are not isolated. Generic cache counters do not prove DRAM bandwidth or capacity exhaustion.

## Protocol, source and correctness

12contexts:actual1×2/2×2A/B×1/16/32workers,REF/OFF/ON,15fixed rotating/reversing matched blocks. The copied original driver changes only the diagnostic module name and two metadata output fields. Loading,point parsing,oracle setup,pool construction and3checkedprimes are excluded;original grouping/admission,allquerywork,Bwordreset,normalization,allocation/drop and pool install/join are timed. Each timed output,prime and onepostcheck matches the cached independent full-MSM oracle. Point arrays remain unchanged. Inputs are fixed actual fixture queries,not a varying witness stream or concurrent whole proofs.

All ON B processes observe the actual selected raw/groupedPhi route and successful completion of11windows with3,080controlbytes. The grouped case has28,852classes;controls observe no scheduler admission. A's Bmetadata stays zero. The TLS observations are checked and their costs are included in this diagnostic consumer;they are not claimed free or a production-source benchmark. No per-point/task trace or new pool is added.

All eight new bridge/link/observation-test commands pass once. The one new metadata-isolation/reset test passes,with129 inherited core tests filtered out. The earlier129-test scheduler/math qualification and original field qualification are reused,not rerun. REF recovers all176 originals after removing adapter edits;the candidate also recovers the qualified177-file scheduler after removing observation additions. Actual final link flags and exact libraries are audited before first execution.

The complete661-command workflow,finish and independent reviewer each pass on first execution. It contains36preflights+strictchecker,540latency,12PMU,12profiles and60offline operations.600newqueryprocesses perform24,510completequerypasses:22,110timed(including36incidentalpreflightpasses),1,800primes and600postchecks. None is anSDKrequest or Gnark proof;the31,387-export ledger is unchanged. The36preflight clocks are excluded from performance decisions. Four unique independent oracles are inherited,not reexecuted.

All latency precedes diagnostics,and all recording precedes decoding/analysis/hashing/archive work. A gated launcher saves the actual process handle/checkpoint before starting collection. No unrelated owned compute overlaps latency/recording.519bindings freeze sources,coherent libraries,binaries,tools,inputs,oracles,protocol and reviewer before capture. No successful native,math,test,fixture,profile or timing stage is repeated for bookkeeping.

The42predeclared checks are12OFF/REFcalibration intervals within[.97,1.03],24ONvsboth nonregression upper bounds≤1.01,and six gain checks(PRIMARY-B32,B32two-shapeequal-log,ALL-B6equal-log×bothcontrols) requiring high<1and≥11/15wins. Sorted zero-based ranks3/7/11of15 give marginal intervals without a familywise guarantee. No minimum gain floor,excluded observation,addedblock,changedthreshold or reroll. The independent reviewer reconstructs every timing check and checks every raw caller,oracle/dispatch/worker row and command stream. PMU/profile parsing has one implementation and is not independently reparsed.

The candidate is closed without a whole-SDK,portable or pressure trial. There is no new AWS/Aeglos,phone,huge-circuit,swap,minimum-RAM or throughput result/resource. This rejects the tested continuation policy,not all possible parallel MSM algorithms. [Decision](artifacts/decision.md).

## Accepted full-proof CPU and Metal comparison, inherited

These are the prior matched local32-worker full-proof times of the unchanged accepted SDK6d83,not this query candidate's proof performance. The all-eight CPU10× goal remains active and unmet0/8.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | CPU10× target ms |
|---|---:|---:|---:|---:|
|1×2|7.915095|37|24|3.7|
|2×2|11.169812|59|30|5.9|
|4×2|16.085882|109|48|10.9|
|8×2|24.517520|189|74|18.9|
|12×2|33.838626|284|101|28.4|
|16×2|40.083672|335|124|33.5|
|24×2|65.506198|528|176|52.8|
|merge24|65.200982|507|175|50.7|

Accepted merge54 remains161.718956ms without a supplied M5 reference. Historical adopted primary7.928660ms/merge54161.883899ms remain labeled historical. No new candidate all-eight proof timing is inferred from query ratios.

[Analysis/all ratios](artifacts/analysis.json),[independent review](artifacts/review.json),[native/source/link review](artifacts/native-review.json),[protocol](artifacts/protocol.md),[statistics](artifacts/statistics.md),[identities](artifacts/identities.json),[profile symbols](artifacts/profile-symbols.json),[archive index](artifacts/archive-index.json),[local raw/bulk hashes](artifacts/raw-and-bulk-index.json). Source/helpers/receipts round-trip exact UTF-8 bytes and hashes. Rawperf and bulk decoded streams remain local with hashes. Executable/library,actualpoint/scalarfixtures,key/image/witness and full proof-ledger payloads are excluded. Local root:`/home/ubuntu/src/zopro-perf-work/g2-last-finisher-query176`.
