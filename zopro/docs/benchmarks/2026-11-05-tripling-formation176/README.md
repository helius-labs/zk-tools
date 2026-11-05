# Complete tripling formation: cost budget fails

The complete direct-tripling implementation fails every declared 16-/32-thread formation budget. At32workers, per-point formation costs **1.806× the 200-point G1 addition reference**, against a1.342×budget, and **2.199× the G2 reference**, against1.814×. The352-point references are faster per point and give larger ratios. No native query or SDK follow-up is justified for this measured formation design. Production remains unchanged.

The preceding [bounded LFG count screen](../2026-11-05-lfg-tiled-count176/README.md) remains a passing optimistic count screen: it charged one formation addition but omitted the formation doublings and implementation costs. This study supplies a concrete complete formation kernel and measures its cost. A count pass is not a latency pass. The literal shifted-point table route and universal chunk-local grouping route remain separately closed.

## Implemented formula and correctness

For affine a=0 input(x,y), beta=2y,a=3x²,z=a²−3x·beta²; for nonzero z compute d=beta²/z,t=2a/z+d²,x3=x+beta²t,y3=beta(a+beta²d)t−y. The arithmetic is seven field multiplications,four squares and one batch inversion. Infinity is preserved; z=0 maps to identity. A full vector group containing a zero norm uses the complete scalar formula, keeping zero out of the product chains. Scalar tails are handled explicitly. The implementation reuses current lane primitives and eight-lane root inversion; it adds no field primitive, key lookup table, retained plan or pool to production.

All128native core tests pass (125original+3new), including exhaustive toy a=0 curves with2-/3-torsion, vector/scalar/Jacobian agreement over tails/identity/sign/reuse, and both caller modes. Separate Arkworks checks pass for3,712G1 and3,712G2 inputs/outputs, including empty and partial groups, signed points, identities,1024/1025points, on-curve checks and G2 subgroup checks. Field/prover qualification is inherited; neither is newly rerun.

The initial native compiler stopped before tests because the toy-prime list needed an expliciti64type. Only that test annotation changed. The original source,pins,protocol and failed compiler streams are retained; runtime formula bytes are otherwise exact. The corrected six-step qualification/library/link/oracle workflow completes once. No successful tests, oracle or performance capture were repeated.

## Measurement

Ryzen9950X, soloCPU2 /physical CPUs0–15 /logical CPUs0–31. One coherent nativeO3,fatLTO,one-CGU executable contains the new1024-point formation caller and unchanged borrowed signed addition kernels at200/352points. Fixtures are synthetic finite generator multiples; the reference's mutable buckets evolve, while formation rewrites3P of immutable input. Both have checked output oracles outside timers. This is not same-work SDK calibration or a whole-proof critical-path comparison.

Three checked prime phases precede18preflight processes and15fixedrotated/reversed blocks across18contexts. Per worker:200-point references use8,192iterations,352-point references4,656,formation1,600,approximately1.638million points each. Mean worker elapsed nanoseconds per point is the component metric. All292processes verify output/affinity/stable capacity with zero timed allocations. Warm reused scratch excludes fresh proof allocation/drop and pending-window ownership work, making this an optimistic formation screen. All latency finishes before four automated cycles:u profiles. No owned compilation,hashing,analysis or other CPU workload overlaps capture.

|Curve / workers|Add200 ns/point|Add352 ns/point|Tripling1024 ns/point|
|---|---:|---:|---:|
|g1/solo|21.608|20.216|37.599|
|g1/physical16|23.605|22.174|41.138|
|g1/logical32|34.506|32.542|62.245|
|g2/solo|39.104|37.639|87.188|
|g2/physical16|43.983|42.317|97.131|
|g2/logical32|72.341|70.277|159.146|

Displayed values are medians of15process metrics; paired ratios need not equal their quotients. Bounds are order statistics4–12of15, nominal96.484375%marginal coverage conditional on independent exchangeable blocks, no familywise guarantee. Exact budgets come from the count screen:(448422−423425)/18626 forG1 and(633620−582968)/27919 forG2. The upper bound must fit the corresponding budget against BOTHreference sizes at16/32workers. All8checks fail; solo remains diagnostic. No threshold,sample count,geometry or subset changed after capture.

|Ratio|Median|Lower|Upper|Budget|Pass|
|---|---:|---:|---:|---:|---|
|g1/logical32/triple1024/add200|1.806058|1.797383|1.811478|1.342049|false|
|g1/logical32/triple1024/add352|1.912751|1.905402|1.918315|1.342049|false|
|g1/physical16/triple1024/add200|1.749574|1.741895|1.757992|1.342049|false|
|g1/physical16/triple1024/add352|1.861669|1.852434|1.869048|1.342049|false|
|g1/solo/triple1024/add200|1.738035|1.723950|1.752325|1.342049|false|
|g1/solo/triple1024/add352|1.871555|1.846325|1.877559|1.342049|false|
|g2/logical32/triple1024/add200|2.199024|2.196010|2.203291|1.814248|false|
|g2/logical32/triple1024/add352|2.263194|2.257589|2.270543|1.814248|false|
|g2/physical16/triple1024/add200|2.209031|2.202686|2.228499|1.814248|false|
|g2/physical16/triple1024/add352|2.301238|2.286304|2.307607|1.814248|false|
|g2/solo/triple1024/add200|2.222160|2.210458|2.244461|1.814248|false|
|g2/solo/triple1024/add352|2.320207|2.299804|2.335356|1.814248|false|

## Memory and profiles

The1024-point kernel retains248KiB of G1 group-record capacity or448KiB of G2 group-record capacity per worker. Those are requested cached-record payloads, not complete live workspace: input/output/fixture arrays, incoming points,references,Vec headers,growth/allocator/stacks and other proof work are additional. Their actual layout and per-proof admission are not integrated. The following RSS counters cover the entire benchmark process,including setup,priming,oracles and independent worker fixtures; they are neither minimum RAM nor standalone kernel/production peaks.

|Curve / workers|Add200 RSS MiB|Add352 RSS MiB|Formation RSS MiB|
|---|---:|---:|---:|
|g1/solo|2.250|2.500|2.750|
|g1/physical16|2.500|6.250|10.250|
|g1/logical32|2.750|10.250|18.750|
|g2/solo|2.250|2.500|3.500|
|g2/physical16|2.500|6.250|22.250|
|g2/logical32|3.500|10.750|42.250|

Four16-million-cycle user-mode profiles contain4,492samples,total. All have one expected executable PID,permitted worker/main TIDs and CPUs,and zeroLOSTrecords. Three are below the declared1,000sample descriptive threshold and remain limited without reruns. Only G2formation qualifies that sample/unknown screen. Symbol periods include inlined payload; no absolute period coverage,CPI,cache/DRAM cause or removable wall-time attribution is established.

|Profile|Samples|Unknown share|Qualified|
|---|---:|---:|---|
|profile-00-g1-1024-logical32|867|0.000%|false|
|profile-00-g1-200-logical32|476|0.210%|false|
|profile-00-g2-1024-logical32|2164|0.046%|true|
|profile-00-g2-200-logical32|985|0.203%|false|

Static linked assembly includes IFMA in both formation and original reference kernels. That establishes compiled construction; source multiplication counts or symbol size alone do not explain the measured ratios. The result closes this particular formation-based bounded design. It does not prove all tripling formulas or all low-memory uses of the LFG sets are slow. No unchanged reroll,packing/layout rescue or favorable-curve promotion follows.

## Validation and accepted comparisons

Independent review checks all pins,315successful command receipts and stream hashes,the preserved initial compiler failure,source recovery,all raw worker integer timing sums and declared matched-block gates,and profile ownership/counts. There are21,874,656measured kernel invocations plus10,915,392priming invocations:32,790,048total complete calls. This is not a proof count. The private177-file source has174of176originals exact,two append-only parents and one child;all original bytes recover. Production arithmetic,solver,schedules,keys and accepted176source remain unchanged.

Accepted SDK SHA-256 remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. The following local accepted SDK observations and supplied historical M5 references are inherited,not remeasured. All eight CPU10× targets remain unmet.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37 / 4.667x|24 / 3.027x|3.7|
|2x2|11.151932|59 / 5.291x|30 / 2.690x|5.9|
|4x2|16.086386|109 / 6.776x|48 / 2.984x|10.9|
|8x2|24.179453|189 / 7.817x|74 / 3.060x|18.9|
|12x2|33.884455|284 / 8.381x|101 / 2.981x|28.4|
|16x2|40.077381|335 / 8.359x|124 / 3.094x|33.5|
|24x2|65.850777|528 / 8.018x|176 / 2.673x|52.8|
|merge-24x1|65.060937|507 / 7.793x|175 / 2.690x|50.7|

Merge54 remains161.883899ms without a supplied M5 reference. Latest full-SDK latency evidence remains epoch-clock176;ledger29,461unchanged. No new SDK/Gnark proof,AWS/Aeglos,pressure/swap,phone/ARM/TEE,250/4999or concurrent proof-throughput result is added. Goal all-eight CPU10× remains active.

## Artifacts

[Design](artifacts/design.md), [formula/callers](artifacts/formation.rs), [driver](artifacts/probe.rs), [Arkworks oracle](artifacts/oracle.rs), [analysis](artifacts/analysis.json), [raw rows](artifacts/rows.json), [reviewer](artifacts/review.rs), [review](artifacts/review.json), [source audit](artifacts/source-audit.json), [native plan](artifacts/native-plan.json), [capture plan](artifacts/run-plan.json), [pins](artifacts/pins.json), [codegen index](artifacts/codegen.json), [assembly](artifacts/codegen.json.gz), [all command receipts](artifacts/receipts.json.gz), [source archive](artifacts/source.tar.gz), [source member hashes](artifacts/source-members.json), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS). Source177regular members match extraction hashes;both JSONgziparchives roundtrip exactly. Executables,libraries,rawperf.data,private keys/images,witnesses and actual point arrays are excluded.
