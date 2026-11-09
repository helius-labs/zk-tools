# Complete-affine ring phases: native correctness and work screen

**The new schedule passes its structural screen and focused native correctness checks. Performance is unmeasured and the candidate is not adopted.** Ordinary bucket slots remain 2,731 per part rather than the old ring's 8,193. Mandatory affine completion removes modeled generic scalar spills but introduces long dependency chains and more inversions than the old ring. Accepted production remains unchanged at **7.857170 ms** for local 1×2.

## Fixed change

The [preceding unit-phase screen](../2026-11-09-ring-unit-phases176/README.md) found 10,746 overflow and 1,777 phase-boundary spill points on primary. This implementation completes both paths using the existing affine batch and queue: execute pending work, scan the queue backwards, swap-remove free destinations, execute and repeat until empty. Each round removes at least one of at most 199 queued operands. Normal full-batch processing keeps the original LIFO take. No production Window field, heap buffer, incoming-point transform, retained key/point/scalar plan or pool is added.

The same one-array identity processes unit 2, applies φ to live affine and exceptional projective accumulators, processes unit 1, applies φ again, then processes unit 0. All pending contributions finish before each transform. Original add handling preserves identities, doubling and cancellation. Radix 128, nineteen positions, two partitions, batches of 200, the original three-plane indexed top, coordinate axes, weighting, recoding and field kernels are retained. Unit filtering precedes coordinate/identity access; three code-row inspections per ordinary position are still required.

Earlier [guarded queue draining](../2026-10-29-queue-drain175/README.md) and [normal-take scanning](../2026-10-30-g2-queue-scan175/README.md) had different schedules and failed complete qualification. Their outcomes remain unchanged. No isolated speed gain transfers to this combination. [Model design](artifacts/design.md), [native scope](artifacts/native-design.md).

## Structural work and limitations

The necessary screen declared before execution requires bounded algebra/accounting tests, no generic spills/normalization, the specified coordinate geometry, a vector-product proxy below accepted and no increase in root inverse calls versus accepted on both retained populations. It passes. These conditions permit resource/native correctness investigation; they are not a runtime or adoption gate. The noncancelling model does not predict native elliptic-curve exceptions. Raw2×2 is diagnostic because production groups that population.

|Population / recipe|Generic affine pairs|Root inverse calls|Vector proxy|Projective mixed + finish|Normalized points|Unit-map Fq products|
|---|---:|---:|---:|---:|---:|---:|
|1x2-B / Accepted|632442|6587|116322150|1178|429|82486|
|1x2-B / Old ring|572574|2608|104454800|676|83|188084|
|1x2-B / Complete phases|573250|4070|105126350|0|0|346550|
|2x2-B / Accepted|1082397|8911|198460450|2236|851|83338|
|2x2-B / Old ring|955437|4394|174701450|997|46|190324|
|2x2-B / Complete phases|956434|6406|175804600|0|0|378218|

The proxy 100V+150F counts nominal eight-lane product/fused-product sites and omits scalar work. New primary/raw 2×2 queue checks are **64,188/96,687**, completion rounds **1,182/1,519**, and overflow drains **54/5**. Both reach **199 rounds** in one completion; **854/1,399 affine batches contain only one pair**. These serial chains explain why removing projective spill is insufficient by itself. No speedup is inferred.

**Arithmetic ledger correction (9 November 2026):** the accepted comparator normalizes once per ordinary window and part: 10×2=20 calls. Its earlier covered ledger counted 40. The accepted rows below are corrected; the original frozen review remains intact, with [revised values and source binding](artifacts/accepted-normalization-erratum.json). Raw operation histograms, measured timings, tests and selection decisions are unchanged.

The extended source ledger charges scalar tails, unit maps, mixed projective additions and normalization, including empty finish boundaries, beta-squared setup and ring axis maps. Full vector operations expand into eight logical events. Scalar and vector throughput differ; these are neither instructions nor cycles. Weighted projective postprocessing, linear operations, packing, memory and scheduling remain unpriced, and their geometry differs between accepted and ring methods.

|Population / recipe|Covered Fq products|Covered fused2 events|Scalar Fq squares|Scalar root inverses|
|---|---:|---:|---:|---:|
|1x2-B / Accepted|4806797|3183702|336|6607|
|1x2-B / Old ring|4310769|2873206|320|2644|
|1x2-B / Complete phases|4530736|2863707|5158|4106|
|2x2-B / Accepted|8062818|5453357|328|8931|
|2x2-B / Old ring|7069126|4791575|312|4430|
|2x2-B / Complete phases|7340483|4779972|4468|6442|

Primary ordinary Fq products rise versus the old ring while fused2 events decline slightly; inverse calls and tails also increase. The lower storage and absence of generic spills therefore do not establish a CPU gain. [All counts](artifacts/analysis.json), [independent replay and ledger](artifacts/review.json).

## Ownership and native checks

The checked optional reservation precedes code/bucket allocation and leaves original inputs intact on denied admission or arithmetic overflow. It charges all eighteen ordinary two-part parents, the leading two-part parent, both suspended indexed-top parents/results, 38 bytes per scalar-code row, nineteen two-axis outputs, integer routing data and maximum nonforking scratch per worker. Parent storage is independent of worker count. This is additional to the prover's unchanged base forecast.

Coordinates alone total **14,681,856 bytes**, versus **39,850,752** for the old ring if all two-part outputs coexist. This omits other ownership and is not measured RAM. For the primary population's 30,089 rows, the conservative complete optional forecast is:

|Workers|Checked optional capacity bytes|MiB|
|---|---:|---:|
|1|49486548|47.194|
|4|80558052|76.826|
|16|204844068|195.355|
|32|370558756|353.392|

The forecast deliberately includes the leading shape's conservative streaming/exception leaf even though the actual leading reducer is indexed. This overcharge remains visible; coordinates alone must not be presented as a full memory bound. Neither forecast is RSS, minimum RAM, stack usage, concurrent-process or pressure/device qualification. Generic unsupported paths retain the original fallback.

Four model-workflow commands and seven corrected native-workflow commands finish successfully once. Two new abstract tests check signed/cancelling payload conservation, queue termination and batch bounds; five inherited model tests are filtered. Three new native tests cover actual-curve phases/collisions/exceptions and ownership geometry. Two existing full-query/admission tests are reapplied to changed code, for **five focused native tests total**. Complete queries cover 1,024 and 2,051 points with 1/4/16/32 workers, eight owned pools and 512 KiB worker stacks. The original generic noASM/noIFMA guard and optimized native core compile. Old map/codec and unchanged weighted-plane tests are inherited rather than replayed. There is no new field, prover or SDK build.

The first native compile fails before any native test because Fr lacks Sum. The repair uses an explicit fold. Inspection also replaces coefficients 1..1000 with 1001..2000 in the specifically generic-collision test, avoiding the predictable equality 1+2=3; the separate exception test retains equal/opposite/identity coverage. Initial sources, pins and failed receipt are preserved and reverse-checked. The native review helper's initial delimiter compile error is also preserved; its corrected first execution passes. No successful scientific stage is repeated. [Repair](artifacts/native-test-repair.json), [native review](artifacts/native-review.json).

The separate same-agent readers check 223 model pins, 201 native pins, all eleven successful command/stream receipts, all 176 recovered production sources, exact original field library 1200 and integer table bytes, every new stream/batch histogram and 36 window diagnostics, all 20 capacity rows, admission order and unchanged Window fields. New stream replay uses native MAP/direct division and VecDeque instead of the producer's recoder/Vec. Shared qualified Phi arithmetic and tree helpers remain inherited. Native resource composition is source-audited; it is not a second independent allocator model. Generator tests are not arbitrary-key independent proof verification.

## Accepted CPU and Metal comparison, inherited

The following local Ryzen 9950X timings with 32 workers come from the completed singleton-elision SDK campaign's REF arm. They are unchanged accepted results, not timings of this candidate or new M5 measurements.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

The goal remains active and unmet on all eight shapes. The next step is a prospectively frozen whole-query comparison with original/fresh controls and automatic profiles; SDK, Gnark and resource qualification across all eight shapes plus merge54 is still required before adoption. This stage runs no timing/profile/SDK requests or new proof exports; the unique verified ledger remains 40,154. It adds no AWS/Aeglos/phone/ARM/huge-circuit/pressure/swap/minimum-RAM/throughput result and owns no cloud resource.

[Continuation](artifacts/next.md), [source audit](artifacts/native-source-audit.json), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
