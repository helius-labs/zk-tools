# Continue from sparse-fallback XYZZ arithmetic qualification

Full goal ACTIVE and UNMET: verified single-proof CPU AVX >=10x supplied M5 CPU
on ALL8 shapes, with CPU and Metal comparisons. Targets0/8. Previous goal turn
PROGRESS: full large-class SDK comparison,45938requests/647exports/625414samples,
fixed candidate rejected and report pushed ce78a4a069. This turn PROGRESS: new
Rust complete XYZZ helper, four focused tests against native and generic fields,
independent Arkworks raw/normalized checks, source/receipt reader, report/push.
No latency gain, Window integration or production adoption is established.

Root /home/ubuntu/src/zopro-perf-work/xyzz-fallback176.
Read design.md,xyzz.rs,tests.rs,review-result.json,publication.json. Workflow79919
terminal0/all4commands firstexecution: native testbuild/run, generic build/run.
Four test functions, eight backend executions; each backend checks38208 results
across both curves (19104/curve), SAME fixtures across backends, not doubled
unique mathematical coverage. Per-curve counts are:
16386 pair/raw checks +2718 changing-chain/batch/final-merge checks per curve.
Ignore any informal larger-count claim; reader arithmetic is authoritative.

Independent Arkworks full-width generator weights check identity, equal,
opposite, doubling, cancellation, variable projective scales, noncanonical
identity numerators, changing states, negate, final affine merge and batch
normalization. Nonidentity states check ZZ^3=ZZZ^2 and X=x*ZZ,Y=y*ZZZ in Arkworks,
plus complete normalized outputs. Batch cases0/1/7/8/9/31/128/257 preserve inputs.
Native original field1200 and preexisting generic noASM/noIFMA field are reused,
not rebuilt. All176 production hashes/ledger43730 unchanged;189pins/4exact
command+stream receipts pass separate same-agent reader. Unused test Field
import warning retained without warning-only rebuild. No old suites replayed.

The helper uses EFD Sutherland2008 XYZZ mixed addition and a=0 doubling. Source:
https://www.hyperelliptic.org/EFD/g1p/auto-shortw-xyzz.html
Static base-field counts: mixed XYZZ8M2S vs accepted Jacobian7M4S; doubling
XYZZ6M3S vs accepted2M5S. Doubling is worse and must stay in the full caller.
Batch normalization uses inverse(ZZZ), zi=ZZ/ZZZ, x=X*zi²,y=Y/ZZZ: one shared
inverse and3M1S per output after inverse recovery, like current normalization.
No special unit-Z branch, initial-state tag, affine-stream replacement, queue
change, field primitive, table, key copy, new pool or assembly is introduced.
Prior initial-unit/globalunitZ/full-projective-stream negatives remain closed.
The earlier triple-affine report mentioned XYZZ against affine but did not
implement this sparse fallback representation. Do not transfer its speed claim.

Memory trade: XYZZ128vsJacobian96bytes per G1 sparse accumulator;256vs192 G2.
One extra base field, +33.3% PER SPARSE POINT, NOT whole proof memory. No native
Window forecast/admission, process peak, low-memory/phone or concurrent caller
qualification yet. This targets a small residual path; not a standalone2x route.

NEXT: integrate this exact qualified helper into isolated actual sparse Window
fallbacks, with checked accounting and original denied/generic fallback. Start
from the ACCEPTED176 manifest, NOT rejected large-class source/cores/SDKs.
Consider operation-owner dispatch into statically typed original/XYZZ states;
avoid a hot dynamic branch on every field operation and avoid two populated
point arrays. Charge every additional sparse capacity and normalization buffer
before constructing the changed path; no synthetic global cap. Preserve all
ordinary packed affine kernels, point order, queue rules and geometry. Add
actual G1/G2 Window oracles, Phi high/low X mapping, final merge/normalize,
identity/equal/opposite/reuse, resource denial/overflow and real small-stack
workers/concurrent callers. Generic compile/execution required as applicable.
Do not benchmark just isolated mixed addition while excluding doubling,
allocation, merge, normalization, footprint or dispatch. A complete Window
comparison with original REF/fresh OFF/ON and automatic profiles precedes any
full SDK/all8+54 proof experiment. Declare all controls/gates/readers BEFORE
preflight; keep positives/adverse results, no threshold/worker/curve subset
rescue. If full caller is not useful, close this fixed implementation honestly.

CANONICAL NOW43730 unchanged:
/home/ubuntu/src/zopro-perf-work/large-class-sdk176/proof-sha256-after-capture.json
No new SDK/proof exports in this stage. Do not seed from43109/43083/43073/etc.
Accepted SDK /home/ubuntu/src/zopro-perf-work/busy-reset-sdk176/sdk-ON remains
7aff5f602a1e250a6b9b28889bf6278b51ec400af8399c7481c36248a8718d01.
Latest actual accepted ALL8CPUms7.829285503/10.979334806/15.803968871/23.944016903/
33.703579161/39.858478065/66.186709387/65.489902226;primaryRSS133.50MiB;
merge54=162.541995323ms,noM5reference;primary16=9.192236934ms.
M5CPU37/59/109/189/284/335/528/507;Metal24/30/48/74/101/124/176/175.
10xtarget3.7/5.9/10.9/18.9/28.4/33.5/52.8/50.7;0/8. Earlier7.803404primary is
same accepted binary in the preceding campaign, not a newer adopted build.
No newAWS/Aeglos/ARM/phone/huge250/4999/pressure/swap/minRAM/throughput evidence.
No live jobs/cloud after terminal qualification/publication.

Autoresearch: source/profile+specific history -> distinct useful work/dependency
mechanism -> Rust implementation/model -> independent math, exceptions,
resource/generic/concurrency qualification -> frozen coherent complete caller/
all8+54 SDK controls, automatic profiles and readers BEFORE ANY preflight ->
serialized Rust1.98.1 xtask -> during ANY preflight/recording ONLY poll exact
confirmed live handle until terminal (timeout/truncation NOT terminal) -> verify
ALL observations -> polite factual report, commit AND push/exact remote verify/
checkpoint -> continue full goal. No questions/Python/subagents/large persistent
LUT/key expansion/new production pool/artificialRAMcap/CASM/unchanged rerolls.
Difficulty is not a blocker or completion. Next turn must implement/measure
useful Rust progress, not restate this report or do a broad historical audit.
