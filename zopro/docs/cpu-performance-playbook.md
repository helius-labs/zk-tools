# CPU performance playbook

Keep useful improvements moving toward adoption while preserving correctness,
bounded memory and honest measurements. Follow the [xtask workflow](../xtask/README.md)
for execution. The [candidate register](cpu-performance-candidates.json) records
the production baseline and selected pending or closed candidates. It is manually
maintained, not an enforced xtask state machine. Historical reports retain their
original protocols and decisions.

## Keep the goal, experiment and candidate separate

The research goal remains verified single-proof CPU AVX latency at least 10×
faster than each supplied M5 CPU reference, with CPU and Metal comparisons for
all eight supplied shapes. Report merge 54×1 separately, without inventing an
M5 reference. A completed report or a failed experiment does not finish that goal.

Identify every candidate by its source snapshot, configuration, toolchain,
dependency/build receipts and measured SDK digest. Retain its binaries and
evidence. A cleanup, cfg removal, changed build configuration or relink that
produces a new artifact gets a separate record, linked to its parent. Equivalent
source operations do not establish equivalent performance.

Record four independent conclusions, each with its tested scope and evidence:

|Area|Record|
|:---|:---|
|Correctness|Passed, failed or untested; tests, proof bindings and external verification|
|Resources and compatibility|Memory, allocation, portability, concurrency and other applicable checks; inherited versus newly measured coverage|
|Performance|Passed declared criteria, failed declared criteria, inconclusive, or measured only as a control; identify host, shape, workers and comparator|
|Adoption|Pending with concrete remaining work, adopted with commit, or not adopted with an explicit reason|

Never compress these into an unexplained “candidate failed.” A performance
qualification failure does not imply an implementation bug. A bound that cannot
exclude a regression is inconclusive about that regression; it is not proof of
zero benefit or proof of a slowdown. It still fails a declared nonregression gate.

## Finish qualification of a promising candidate

1. Review existing source, profiles and experiment history. Write a distinct
   hypothesis and the whole-proof effect it could plausibly explain.
2. Implement the smallest Rust change. Check relevant correctness and resource
   behavior, including portable and concurrent behavior where affected. Reuse
   applicable qualification when its source and assumptions remain valid;
   identify what is inherited. Avoid tests that only mirror the implementation.
3. Build and pin coherent consumers. Use a short, verified SDK screen when useful.
   A microbenchmark or operation count alone does not establish a proof speedup.
4. When a candidate shows a useful gain, list its missing adoption requirements
   and finish those requirements on that candidate. Do not require a cosmetic
   cleanup or unrelated refactor before proceeding. If integration requires a
   changed build, track it separately and preserve the earlier candidate.
5. Freeze a suitably sized qualification protocol before its first preflight:
   actual consumers, inputs, comparisons, timing boundaries, worker counts,
   order, samples, uncertainty method, resource limits and decision criteria.
   Compare adoption performance against the accepted production baseline.
   State the role of additional controls explicitly; they do not automatically
   become adoption vetoes. If choosing between two candidates requires a
   head-to-head criterion, declare it prospectively.
6. Run the missing qualification, verify outputs and make an explicit adoption
   decision. Once the candidate meets its declared release requirements, apply
   the tested change, check that the production configuration matches, commit
   locally. Push only with user authorization; publication is currently withheld.
   Do not postpone a qualified improvement because it falls short of
   the broader research goal or because another cleanup might look nicer.

Retain qualified small improvements. There is no minimum percentage gain floor.
Report absolute milliseconds or microseconds saved, paired ratios, uncertainty,
CPU use and memory. A high-core gain does not automatically improve the best
worker-count latency; report both. Do not demand that a candidate beat Aeglos,
reach 2×, or meet the full 10× goal before adopting a smaller qualified gain.

## Make rejection cheap and reserve qualification for promising changes

Do not start every candidate with the full all-shape performance campaign.
Correctness qualification, exploratory screening and release qualification answer
different questions. Use the smallest measurement that can answer the current
question; keep all eight shapes in the final goal and release requirements.

1. **Check the mechanism first.** State which work disappears, which cost is
   added, and what observation would refute the hypothesis. Inspect generated
   code when relevant. Measure the actual caller with representative inputs,
   including packing, scratch and reduction costs. A faster isolated field
   primitive is insufficient. For a scheduling change, inspect worker activity
   and the critical path; aggregate CPU samples are not a critical-path estimate.
   Static instruction counts, frame sizes and operation counts are explanatory
   evidence, not general latency vetoes. A correctness or resource violation can
   stop a prototype; an uncertain codegen tradeoff usually calls for a short
   actual-caller timing. A screen stopped on a surrogate leaves latency unmeasured.
2. **Run a short full-proof screen.** Freeze the exact consumers, a small fixed
   sample count, relevant worker counts and representative cases before capture.
   On this host retain all three predeclared executable backings and balance
   ordering. Include a memory-sensitive case when the change can affect resources.
   Verify exported proofs. This screen is for rejecting an expensive idea or
   choosing a follow-up question, not declaring adoption or a precise small gain.
   Preserve inconclusive results as inconclusive. Do not extend a screen until
   a favorable answer appears.
3. **Profile the surviving mechanism.** Collect counters or samples that can
   distinguish the proposed explanation from added work elsewhere, with observer
   checks. Reuse valid prior tests and retained captures. If the mechanism is
   refuted and whole-proof benefit is absent, close the candidate without a
   full all-shape timing run. A profile needed to test the original mechanism
   belongs in the first stage, not after expensive qualification.
4. **Qualify a promising exact build.** Run the missing all-shape correctness,
   performance, portability and resource checks against accepted production.
   Use a separately frozen, adequately powered protocol. Finish qualification
   before adoption; report CPU and Metal comparisons for all eight supplied
   shapes and merge 54×1 separately. An exploratory success does not relax these
   requirements.

Record preparation, capture, analysis and total elapsed time, plus the decision
each stage enabled. Prefer reusable xtask commands and existing readers over a
new campaign framework. A source change that increases instructions or stack
traffic may still help scheduling or instruction delivery, but it needs a short
test of that benefit before hundreds of full-proof processes.

Gate dependent stages on checked command exits and prerequisite review results.
The main-capture launcher must refuse to run until smoke semantic checks and
the independent smoke reader succeed. Do not launch it merely because a smoke
command was invoked, or after a shell sequence whose earlier failure was hidden
by a later successful command. Preserve and disclose any sequencing deviation;
an offline reader repair cannot retroactively satisfy an unmet pre-capture gate.

## Prioritize changes that remove work

Before choosing the next experiment, make a compact cost model from the current
production implementation and profiles. Count field operations, point operations,
passes over memory and synchronization where feasible. Separate measured work
from hypotheses. Estimate how much complete-proof time the proposed change could
plausibly remove, and account for its added work and memory. Amdahl-style estimates
are upper bounds only when their critical-path assumptions hold.

Use this model to prioritize algorithmic changes: fewer scalar/point operations,
sharing work already repeated within a proof, less expensive representations or
fewer passes and synchronization boundaries. Check previous experiments and
primary papers/code before repeating an idea. Preserve Groth16 semantics,
fresh blinding, portable fallback and bounded memory; a genuinely different proof
system or protocol is a separate compatibility decision, not a drop-in speedup.

After a mechanism-directed variant fails, require new evidence before trying
another annotation, threshold or helper boundary in the same family. Document
why the next hypothesis differs and what a short experiment will resolve. This
is research prioritization, not a minimum gain floor: retain and finish adopting
small improvements that already have credible evidence. For an AWS discrepancy,
a local result screens an idea but cannot establish that it fixes C7a behavior.

## Explore beyond the current implementation

Maintain two tracks: finish candidates with credible gains, and test structural
alternatives. For the next three new exploratory decisions, devote at least two
to a different algorithm, representation or execution dependency, rather than
another parameter or helper variant. This is a research allocation rule, not an
adoption threshold; existing promising candidates still receive qualification.

Before building, write five short items in the existing experiment record:
the assumption being challenged, the predicted observable change, the cheapest
test that distinguishes explanations, the added memory/setup cost, and the
decision each possible outcome enables. Give the first screen a preparation and
measurement time allowance. If it overruns, simplify the question or explicitly
justify continuing. Elapsed time is not evidence that an idea failed.

After two unsuccessful variants in one mechanism family, choose a different
family unless new evidence changes the hypothesis. Consult retained negative
results first. A changed constant, language or function boundary alone is not a
new hypothesis. Do not require a speculative prototype to look like production
code before screening it. Keep it isolated, preserve output checks, and charge
its setup, conversion, merge and allocation costs at the actual caller boundary.

For the C7a gap, separate excess arithmetic work from poor parallel execution.
Use matched input and timing boundaries, worker-count curves, CPU time and
latency for both implementations. Select a targeted intervention that can
distinguish the remaining explanations. Local experiments can reject ideas;
they cannot establish a C7a fix. Lower operation counts are neither necessary
nor sufficient for lower latency, and a scheduler sample is not automatically
avoidable overhead. Reuse trustworthy captures before commissioning new ones.

Measure iteration by time to a resolved question and verified proof time saved,
not reports, tests or candidates produced. Reuse the Rust/xtask harness and
inherited qualification. Keep rejected screens to a short decision plus raw
evidence; reserve full campaigns and integration work for credible survivors.
At each three-experiment review, state what assumption changed. If none changed
and no candidate advanced, change the question before generating more variants.

For exploration, keep one short record with the hypothesis, exact artifacts,
fixed screen, result and next decision. Reuse raw receipts and existing readers;
do not commission a new reporting framework or adoption-sized evidence bundle
before the first discriminating measurement. Diagnostic ablations may deliberately
remove work to test an explanation, but label them non-production: their times
are not valid proof speedups or automatically attainable bounds. Production
correctness and resource requirements still apply to any candidate being adopted.

### Current execution queue

The [52-bit FMA/RZ prerequisite241](benchmarks/2026-11-18-fma52-rz241/README.md)
is distinct from the old51-bit RNE field route: it uses embedded RZ/SAE and the
current five-limb shape. Native exactness passes, but raw-product integer-storage
and prepared-input ratios are about2.00 and1.42, with0/3wins in every context.
No field/SDK follow-up is selected. Preserve both measured FMA negatives; do not
start another rounding/limb-width variant without new mechanism evidence.
These are component results, not C7a or full-proof penalties. Third consecutive
STS failure leaves239 unrun; its sources/package/payload and existing exports
were rechecked intact. Canonical62191 and the adopted implementation stay unchanged.

Current production is paired weighted233/adopted236, source188; canonical export
inventory is62191. The [current C7a package239](benchmarks/2026-11-18-adopted-c7a239/README.md)
is ready with four verified functional exports, but AWS SSO remains expired.
Its cloud measurements are unrun. Keep completed local measurements and the
separate frozen local/remote roots; do not replay smokes for bookkeeping.

The [final-only G1 endomorphism screen240](benchmarks/2026-11-18-late-phi-kh240/README.md)
tests a distinct execution plan:20 independent component/span reductions and
one final phi, rather than the old175 bucket transforms/point partitions.
Primary32 query latency regresses9.28%,0/3wins, with10.58% more paired CPU.
All388 query outputs match the independent real-input oracle; threaded RSS
guards pass. Close this exact plan without a GLV window/partition sweep or SDK
follow-up. No current adopted gain is invalidated. The next meaningful AWS
comparison reuses239 after valid credentials; further local exploration needs a
different work/dependency hypothesis. Entries below retain historical scopes.

The [compact G2 prefix record](benchmarks/2026-11-17-g2-compact-prefix202/README.md)
reduces record payload640->512bytes using existing field packing, but primary32
query latency regresses2.34%,0/3wins. It fails both local SDK progression and the
predeclared local rule for selecting a C7a test. All816 query checks and four
native test functions pass; production and ledger58257 are unchanged. Close this
record recipe without a packing/layout sweep. Smaller storage alone did not
improve the complete caller, and no C7a or whole-proof result is inferred.

The [exact adopted SDK C7a campaign](benchmarks/2026-11-17-accepted-c7a201/README.md)
now covers all eight supplied shapes plus merge54 at32/96 physical cores.
Primary ON is11.098/12.515ms; retained Aeglos Sorted12.143/8.311ms. The adopted
certificate flag improves primary32 versus its same-SDK OFF control (paired
0.986170, interval upper0.999130);96 remains inconclusive. All224 new distinct
exports pass independent Gnark verification; canonical inventory58257. All16
M5 worker-context checks fail, so the all-eight goal remains0/8. Source unchanged.
The last three studies change the scaling assumption:199 rules out a cache-
capacity-only explanation;200 shows more tasks can improve96 without lowering
best latency;201 shows96 helps24x2/merge54 while primary worsens. Do not describe
the problem as a universal high-core failure or solely RAM capacity. Select a
distinct complete-caller work/dependency reduction; no unchanged baseline rerun,
tree/width sweep or next algorithm is queued. All owned cloud resources are gone.

The [unshifted G2 tree C7a test](benchmarks/2026-11-17-bucket-tree-c7a200/README.md)
closes another scaling-only route. Primary96 improves 23.90% with 9/9 wins and no
retained table, but best tree latency 4.916 ms loses to streaming 4.870 ms at32;
grouped best latency regresses about 7%. Only one of three progression checks
passes. All 1552 remote queries match retained oracles; production is unchanged.
More independent tasks can help high-core execution without improving the best
configuration. Do not repeat this exact tree with a band/width sweep. Next work
must change the cost of the complete caller, rather than task count alone.

The [G2 footprint audit](benchmarks/2026-11-17-g2-code-footprint199/README.md)
does not select a compact-code candidate. Both hosts have 32 KiB L1I; the 51.9 KB
symbol is not one hot loop. Two IFMA-loop components total 27.8 KB and hold 2047/2049
retained kernel samples. Their static union with entire helper/feeder bodies is
37.3 KB, which is not measured residency or a causal cache model. Do not reopen
outlining/loop-bound variants on whole-symbol size alone. Continue toward a
distinct complete-caller arithmetic or dependency reduction; no algorithm queued.

The [central G2 denominator check](benchmarks/2026-11-17-g2-central-equality198/README.md)
is closed after its short query screen: primary32paired0.997817but only2/3wins,
below its fixed3/3progression rule. Profiles show0.47–0.60%fewer whole-caller
instructions and mixed cycles; all16quality guards pass. This does not reopen
the screen or establish a proof gain. Stop further equality/identity/bounds
metadata variants as the main breakthrough track. No runtime change is adopted;
choose a distinct reduction in complete-caller arithmetic or algorithmic work.

The [prepared G2 C7a screen](benchmarks/2026-11-17-shifted-c7a197/README.md)
closes the exact masked188recipe on both tested hosts. At96physical workers its
primary query improves7.35%(9/9wins),but best prepared latency5.868ms loses to
streaming4.875ms;grouped32regresses32.85%. Better high-core scaling alone does not
meet the single-proof objective. No table/width/worker/layout rescue sweep is
queued. Choose a distinct reduction in complete-caller work; no next algorithm
is selected. This adds no full-proof/Aeglos comparison or SDK194 AWS measurement.

The [generated-proof certificate](benchmarks/2026-11-17-certified-all196/README.md)
passes all33 local performance/resource checks and portable release checks; its
exact runtime is adopted with `ZOPRO_BENCH_CERTIFIED_OUTPUT=1`, alongside accepted
fusion and indexed batching. Primary16 saves a paired34.055µs(0.385%);primary32
gain remains inconclusive. Keep the52–64ms primary cold certificate cost visible:
this is an explicit preparation opt-in, with ordinary validation without the flag.
Start from this179file production baseline and retained SDKbcc8e077. No cosmetic
rebuild or certificate/layout rescue sweep is queued. The all-eight CPU10xgoal
remains unmet. Choose a distinct substantial work/dependency question against the
retained profiles; maintain one active qualification and at most one next question.

The previous queue entry missed the completed
[solver-to-A streaming trial](benchmarks/2026-11-06-solver-a-stream176/README.md).
Its 32-worker solver-to-A chain regressed by 20.25%; do not implement that recipe
again or repeat the readiness census. Likewise, the promising inversion-count
model for [borrowed bucket trees](benchmarks/2026-11-07-borrowed-bucket-tree-pilot176/README.md)
was followed by a full-proof regression. These are measured negative results.
The earlier queue incorrectly said uniform scaled prefixes stopped at codegen.
The subsequent [CFG audit](benchmarks/2026-11-07-uniform-prefix-cfg176/README.md)
led to a [complete-SDK pilot](benchmarks/2026-11-07-uniform-prefix-pilot176/README.md):
ON/OFF was0.9910 with interval0.98368–1.000012. This is positive but insufficiently
qualified evidence, not an unmeasured candidate or a measured regression.
The [combined default carry version](benchmarks/2026-11-08-uniform-carry-default-performance183/README.md)
then passed all32-worker checks but failed three16-worker checks; the
[short-map composition](benchmarks/2026-11-09-prefix-short-sdk186/README.md)
also has completed full-proof results and failed nine checks, including memory.
Preserve these separate decisions. Do not rerun them as a first clock, mistake
a native-stage report for the latest status, or adopt a component from a
post-result subset. Search references to a candidate's report and artifact root
before treating it as unfinished.

The [ordinary-stage diagnostic](benchmarks/2026-11-12-ordinary-elision178/README.md)
now resolves one local priority: primary32 complete G2 query changes from2.993ms
to1.407ms with ordinary fills replayed, versus2.667ms with their downstream
reductions replayed. All1632 query oracles pass. These are fixed-input diagnostic
times, not valid production gains, full-proof bounds or a C7a explanation.
Ordinary filling takes priority over another top/reduction-only variant.

The [borrowed-pair follow-up](benchmarks/2026-11-12-borrowed-pair178/README.md)
implemented that dataflow change and failed its screen: primary32 is3.118ms
versus2.917ms streaming and3.092ms staged tree. All2609query oracle checks pass;
the failure is performance. New/staged counter panels show0.98%more instructions
and13.61%more cache-miss events. Staging-helper samples fall while arithmetic-helper
samples rise. A removed source copy was not a net cost reduction in this caller.

The [joint search/publication analysis](benchmarks/2026-11-12-search-overlap178/README.md)
finds that most recorded search residence in all54 historical C7a graphs occurs
without pending publications. At96workers the whole-request median is75.85%;
only0.33% overlaps at least96 pending jobs. The observer failed its overhead
qualification and phi grain has since changed; these are research leads, not
current-source attribution or removable latency.

The [solver-phase intervention](benchmarks/2026-11-12-phase-search178/README.md)
now restricts private-deque searching to ceil(N/3) existing workers during solving
and restores full participation before scalar preparation.113scheduler tests and
97freshGnark exports pass. Its local screen fails three gain checks:primary32
ON/OFF is0.999933;2x2 is1.004676. All nonregression/RSS guards pass. Profiles show
only a small cycle reduction, not a local breakthrough. Keep this screen closed.
The matched [C7a32/96 experiment](benchmarks/2026-11-12-phase-search-c7a178/README.md)
is complete and negative: 23/28 screen checks passed, all RSS guards passed,
124 fresh exports verified, and both cycle profiles passed quality checks.
Accepted primary latency is 11.178852/12.714640 ms at 32/96 workers; ON is
11.366803/12.729304 ms. Primary96 paired ON/OFF is 1.001355 with 4/9 wins.
Epoch/scheduler symbol shares remain about 61–62% of sampled cycles, but this
does not identify removable wall time or the next causal intervention. Close
the solver-only search intervention; no divisor sweep is queued. All owned
AWS resources were removed. No capture or successful analysis needs replaying.

The [destination-round screen](benchmarks/2026-11-12-bucket-round178/README.md)
replaces tree intermediates and collision handling with one accumulator per bucket,
advancing independent destinations in rounds. All1229 complete-query checks pass,
but primary32 rounds/streaming is1.0048 with1/3wins and threaded nonregression
checks fail. It beats the staged tree while still using more CPU than streaming.
Close this exact topology without a bucket-size sweep or full-SDK campaign.
Another ordinary-fill proposal must address remaining work or dependency cost;
removing storage/queue bookkeeping alone has not established a production gain.

The [accepted-build scaling diagnostic](benchmarks/2026-11-12-accepted-scaling179/README.md)
adds local 1/2/4/8/16/32-worker evidence without rebuilding. Primary is 87.94,
8.76 and 7.71 ms at 1/16/32 workers. In three separate counter blocks, 32/1
instructions grow about 5.4%, while user cycles grow about 62%; most proportional
cycle growth is in cycles/instruction. The local SMT curve does not reproduce
C7a's additional-physical-core behavior or establish redundant scheduler work.
Do not select another local scheduler variant from aggregate CPU time alone.
Native radix-52 bucket storage also already has a measured 22–27% regression;
it is not an untried conversion-elimination proposal.

Canonical exports are now 55209 at phase-search-c7a178/canonical-final/proof-sha256.json,
SHA-256 64f44d498212b79be20498597f39fd6b5eb2e063004816f9f2d5c73f524cd350.
The C7a package's frozen 55028 seed remains historical. Its 124 new exports were
merged with the later local 55085 ledger after local transport verification. The
[host finalizer](benchmarks/2026-11-12-phase-search-c7a178/artifacts/resume-ledger.md)
passed actual target integration and preserved both proof-history branches.
Ordinary-fill research remains available,
but no next implementation is selected. Previous tree, plane and two-bank
results remain closed or inconclusive at their documented scope.

The [retained Aeglos stage decomposition](benchmarks/2026-11-12-scaling-divergence180/README.md)
now narrows the scaling question to MSM. Across all fifteen November 10 blocks,
1x2 MSM mean time falls 8.337837→3.900750 ms at32→96, saving4.437088 ms;
witness time rises1.288977→1.625243 ms. MSM improves and witness worsens in
every block for both1x2 and2x2. These are historical stage timers, not current
Zopro timings or exclusive CPU. The October25 Aeglos Windowed control uses the
same pool but slows26.808818→55.113735 ms for1x2: the pool alone does not explain
Sorted scaling. Do not attribute this mixed algorithm/preparation difference
solely to shifted tables. Next obtain matched coarse MSM substages/work geometry
for current Zopro and pinned Sorted/Windowed at32/96, with explicit observer
checks. Avoid another aggregate profile audit, per-task trace, solver search
sweep or unchanged joint-window-queue retry. No new capture or implementation
is supplied by the retrospective decomposition; canonical55209 remains unchanged.

Both coarse observers are now integrated into full-request harnesses. The
[local phase comparison](benchmarks/2026-11-15-msm-local180/README.md) passes
all16 calibration/observer gates at16/32 threads, with231 fresh Gnark exports
including integration. Canonical55440 supersedes55209. Zopro reference1x2 is
7.710818ms/115MiB at32; Aeglos Sorted is12.792590ms/427.45MiB. Zopro's query
span shrinks6.606→5.087ms with SMT; Sorted's MSM stays near10.2ms. Windowed
accumulation/sum savings are offset by merge planning/execution. This locates
local phase costs but does not explain C7a's physical-core reversal or establish
a production gain. The panel used one backing per binary: calibration is scoped
to those files, not backing independence. Carry the prescribed backing replicas
into the target comparison. Existing Aeglos native qualification was reused;
no successful SDK capture or verification needs replaying. The
[C7a capture](benchmarks/2026-11-15-msm-c7a180/README.md) is now complete:
220 fresh exports verify on target and host; canonical55664 supersedes55444.
Zopro REF is11.257745ms32/12.986700ms96; Sorted REF11.871786/8.097657ms.
All Zopro and ON/OFF checks pass. Sorted96 FRESH/REF fails one interval and one
backing guard:62/64 overall,10/10 environment checks. Fresh Sorted traces cannot
be assigned exactly to the retained96 reference; their own OFF/FRESH and ON/OFF
controls pass. Zopro solve/preparation/FFT add1.943684ms while query span saves
0.371650ms, consistently across9/9blocks. Its actual G2 late-callback tail grows
0.442806->1.296600ms; callbacks are not exclusive arithmetic or removable time.
Fresh Sorted accumulation saves3.916824ms; Windowed serial merge planning adds
18.823415ms. These observations prioritize concrete pre-query passes and G2
dependencies, not another general pool rewrite. No new implementation selected;
prior solver-search, weighted groups and fused-product overlap recipes stay
closed. All owned cloud resources are cleaned. Production unchanged; no rerun
of the successful capture or relaxed calibration gates is queued.

The subsequent [preparation/G2 diagnostic](benchmarks/2026-11-15-prep-g2-detail181/README.md)
is complete locally and on C7a. All 114 new exports verify; canonical 55778
supersedes 55664. Local calibration passes 16/16; C7a passes 15/16 plus all ten
environment guards. C7a OFF/REF at 96 cores exceeds the interval bound, so its
fine substages describe the diagnostic build, not exact accepted attribution.
ON/OFF passes at both counts. In that build, canonical conversion and A/B gathers
account for 0.819689 ms of the 0.822236 ms preparation increase. Reclamation is
negligible. G2 itself gets faster (7.044007 to 6.630071 ms), but G1 improves more;
only 0.042770 ms remains after the final G2 window. Most of its 1.280190 ms late
tail lies inside ordinary window work, not the final collection return.
All owned resources are cleaned. Next target an actual preparation boundary or
ordinary G2 fill dependency; do not repeat prior gather, grouping-bypass,
independent-plane or routed-bucket recipes unchanged. No algorithm is adopted.

At the next checkpoint, report one resolved question, the evidence that changed
the decision, and elapsed preparation/capture/analysis time. An unchanged
explanation after another three variants calls for a different question, not
another playbook expansion. These queue entries are current decisions; replace
them as work completes rather than accumulating another experiment history here.

## Handle a failed follow-up without losing its parent

A new variant's failure applies to that variant and campaign. Preserve the
parent's successful checks and its outstanding requirements. Decide separately
whether new evidence directly contradicts the parent's results, under matching
conditions, and record that evidence if it does. Do not silently transfer a
child's failure or success to its parent.

Use retained captures and receipts first. Rechecking existing data is distinct
from generating new proofs. A candidate measured as a control can supply useful
evidence, but a newly chosen comparison is retrospective analysis, not a newly
passed prospective gate. Do not silently add that comparison to the old protocol
or claim the control has completed full adoption qualification.

Keep failed protocols closed. Do not extend their samples, loosen thresholds,
drop inconvenient contexts, change comparators, or repeatedly rerun unchanged
screens until they pass. If required evidence remains inconclusive, state the
missing question and declare a separate, adequately sized follow-up before
collecting data. Reuse completed tests and captures; additional work must resolve
that question, rather than rebuild the entire campaign for bookkeeping.

Plan uncertainty and sample sizes with the number of required comparisons in
mind. Report marginal intervals as marginal; do not imply joint confidence.
Avoid an unnecessarily large set of underpowered vetoes. This guides future
protocol design and does not relax any completed experiment's declared rules.

## Preserve measurement and resource discipline

- Use the Rust xtask workflow and exact build receipts. Keep portable and native
  builds separate. Resolve actual Cargo libraries and proc-macro artifacts, and
  use the pinned toolchain for SDK linking. Never nest xtask under itself.
- For small-change comparisons on this Ryzen host, distribute each consumer
  across a fixed, predeclared set of independently written, byte-identical
  executable files. Record complete hashes and device/inode identities; retain
  every copy and outcome. Pair or balance backing replicas in the analysis.
  Do not select the fastest copy or use copying to reopen a failed gate. The
  [full-SDK backing diagnostic](benchmarks/2026-11-11-gather-backing177/README.md)
  reproduced a large file-associated latency difference despite identical bytes.
  It does not establish a hardware mechanism or guarantee that any particular
  number of copies removes the effect. Diagnose large backing interactions
  before attributing their size to source changes.
- During any SDK preflight or recording, only poll the confirmed live handle
  until authoritative completion. No owned build, helper, hashing, analysis,
  reporting or push may overlap it. A timeout or truncated output is not
  evidence of completion. Run isolated latency and profiling phases separately.
- Use automatic profiles when needed to answer the hypothesis. Validate observer
  overhead before treating instrumented behavior as representative. Sample shares,
  job counts, faults and context switches are not removable wall time or syscall
  counts. Reject causal claims the evidence cannot support.
- Independently verify fresh exported proofs, public-input bindings and sample
  accounting. Extend the current verified-export ledger; transport rechecks do
  not add distinct proofs. Preserve failures and resume completed work without
  replaying successful science.
- Preserve universal bounded-memory behavior. Avoid giant lookup tables, expanded
  key plans, new production pools and artificial architectural RAM caps. Record
  benchmark container limits separately from prover requirements. Peak RSS is
  not minimum RAM. Do not infer phone, ARM, pressure, huge-circuit or concurrent
  throughput results from unrelated qualification.
- On AWS, use only scoped owned benchmark resources, bounded lifetimes and
  adequate credentials. Retrieve and verify results, then promptly remove and
  check the owned VM, volume, temporary image tag and transfer objects. Do not
  act on historical resource IDs.

## Close an experiment and continue the goal

Publish a short factual report with exact candidate identities, results and
limitations. Preserve raw evidence, and use respectful comparisons with Gnark,
Aeglos and other projects. Label historical comparisons and supplied references.
Commit each completed experiment locally and update the candidate register and
current handoff. Push only when the user authorizes publication, then verify the
remote commit. Current authorization does not permit pushing. Preserve historical
report decisions.

The handoff must identify the production baseline, each pending candidate and its
remaining requirements, the current proof ledger, live handles/resources, actual
external blockers and the next concrete action. Close a pending promising
candidate's qualification or record why it is deferred before replacing it with
another variant or speculative optimization. Reporting is a checkpoint in the
research loop, not its endpoint.

Report a stale blocked goal status separately from an actual external blocker.
After the user resumes work, continue authorized independent work and reset the
blocker audit. Do not invent an authentication, approval or budget obstacle, or
mark the full goal achieved because an experiment passed. If the available goal
tools cannot reactivate its recorded state, say so; do not create a replacement
goal or claim that the metadata was repaired.

## Current example: G2 scalar grain

The [experimental build](benchmarks/2026-11-11-phi-grain-c7a176/README.md), SDK
`c88a3528…`, passed all twelve primary checks at 32/96 C7a workers, including
the 96-worker improvement. It was retained as PARENT in the later all-shape
run, and those control measurements were reviewed retrospectively. A separate
[31-block prospective qualification](benchmarks/2026-11-11-phi-grain-parent176/README.md)
then passed all forty adoption checks. The exact experimental runtime and a
separately validated native build selector are now adopted. No cleanup build
was substituted for the measured SDK.

The [default build](benchmarks/2026-11-11-phi-grain-clean176/README.md), SDK
`96915d56…`, passed correctness, exported-proof verification and all thirteen
RSS guards, but passed only 58 of its 66 performance/resource checks. Its
performance qualification failed and it is not adopted. This does not reject
the experimental build. Its failed protocol remains unchanged. The adoption
record preserves the distinction between retrospective control review, fresh
prospective qualification, default-selection evidence and production source
mapping. Future experiments start from the newly adopted baseline.
