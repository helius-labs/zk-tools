# Measure the qualified initial-affine accumulator state

Goal ACTIVE/UNMET: verified single-proof CPU AVX >=10x supplied M5 CPU for all
eight supplied shapes; report comparisons against CPU and Metal. Targets met0/8.
The preceding clarification-only reply was no progress. This experiment makes
concrete progress: a distinct proof-owned state implementation, native OFF/ON
and generic ON qualification, eight coherent native artifacts, source/receipt/
link reviews, report and exact verified commit/push. Speed is UNMEASURED.

Root /home/ubuntu/src/zopro-perf-work/initial-unit176. Read design.md,
source-audit.json,review-result.json,link-review.json,publication.json. Initial
workflow19111terminal1: two native core libraries succeeded; OFF test compile
failed E0277 in the independent Arkworks G2 test oracle. Test-only componentwise
Fq2 correction retained in test-repair.json and before-test-type-fix/. No runtime
change; both completed core libraries reused. Remaining workflow8174terminal0,
six commands: native OFF/ON test builds+runs and generic ON test build+run.
Downstream SDK77158terminal0/six commands. Fourteen successful workflow commands
and one retained failed compile. Review helper needed binary-safe handling for
unchanged source files (review-before-binary-source.rs retained). Repair helper
had one compile-time borrow fix (repair-tests-before-borrow.rs retained). These
manual helper attempts do not have complete xtask receipts. No scientific work
was replayed for either reader/helper correction.

Actual tests11: native OFF4/native ON4/generic ON3. Native OFF/ON each138 Window
cases; generic ON69, total345 executions using reused fixtures. Per Window test:
12 dimensions plus53 actual broadcast workers across1/4/16/32 pools with
requested512KiB stacks and four OS callers sharing eight-worker pool. G1 state
checks7924/call-family, G2 checks7993 including69 endomorphisms. Native ON per
curve logged1608 tagged updates and469 tagged finish joins,4036 other updates,
472 queued operands. These are fixture counts, not proof workload eligibility
or saved field operations. Generic tests execute both-curve helper oracle plus
G1 Window; native-only G2 Window is unavailable under noIFMA. Independent helper
2048 outputs per arm, exact accepted Jacobian checks,60 synthetic index-boundary
checks per arm. Populations reused across arms; not independent proof exports.

Only three accepted source files differ (msm_affine.rs and borrowed_pending.rs/
g2_borrowed.rs), all176 originals recover exactly; one new private initial_unit.rs
module. Field source and field library are the accepted originals. Window
structs, vector dimensions/capacities, arithmetic elsewhere, dense bitmap reset,
solver/FFT, schedule, allocation/forecast policy, driver and stacks unchanged.
No new runtime heap/table/pool/key plan/cache. High index bit marks ONLY initial
nonidentity affine provenance. Existing-index update decodes it, uses complete
unit-Z formula if marked and clears it after every update, including identity.
Finish consults tag for still-fresh entries. G2 phi changes X only. Do not infer
unit state after any addition/double/cancellation. Domain buckets<2^31 leaves MAX
as sentinel; larger domains preserve original u32 encoding. No new index cap.
Costs include masking/domain/flag checks, clearing stores and finish lookup;
math savings at most3M+2S once per eligible accumulator. No speed prediction.

Coherent native core/prover/zolana libraries root/libs/{OFF,ON}; SDK root/sdk-OFF
and sdk-ON, all identities in link-review.json. Both use exact original field
/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate/libzopro_bn254-1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713.rlib.
Do NOT use rejected global-unit-Z fields from unit-z-native/shared176. Original
fixed-placement worker-placement-control176/driver.rs and support/perf.rs,
nativeO3/fatLTO/oneCGU, original stacks+allocator. Release qualification helpers
absent (OFF.nm/ON.nm already generated; do not re-decode for bookkeeping).

NEXT concrete action is a frozen complete SDK comparison, not another broad
audit/prototype or unchanged field/Window suite replay. Suggested new capture
root /home/ubuntu/src/zopro-perf-work/initial-unit-sdk176. Reuse qualified SDKs
and source snapshot above, not rebuild them. REF must be exact accepted
/home/ubuntu/src/zopro-perf-work/busy-reset-sdk176/sdk-ON. Donor protocol/helpers
/home/ubuntu/src/zopro-perf-work/unit-z-sdk176 has full matched all8+merge54,
primarysolo/16, automatic PMU/profiles and79 declared checks. Copy statistics.rs,
sampling.rs,metadata.rs,counter.rs byte-exact. Adapt provenance to THIS root's
three reversible changes plus new module, source_pins/repair-pins and actual
two-success+failed-test-compile / six remaining / six SDK receipts. Fields stay
original1200 for BOTH arms. OFF core literal accepted body, ON cfg state variant;
both retain dense bitmap reset. Donor field/global-method reviewer is inapplicable.

Freeze protocol and all helper/reader/prospective code before ANY SDK preflight.
Preserve donor complete all8+54/primarysolo16 gate79 and baseline calibration,
memory guard, fresh binding/164-byte proof/Gnark verification, original fixed
placement and SDK interface. Donor52preflight commands=26exports/104SDKrequests;
main1263commands=591latency+9PMU+9profile SDKprocesses,45offline+609Gnark. Primary
32/16:31blocks*151requests; other cases15blocks*31 exceptsolo11requests. No
gain floor, observation exclusions, repeated blocks, subset rescue or extension.
All-recordings-finish-before-offline-analysis/verification. Do NOT infer speed
from native test duration, operation counts, NM sizes or successful qualification.

CANONICALNOW42438 unchanged:
/home/ubuntu/src/zopro-perf-work/unit-z-sdk176/proof-sha256-after-capture.json.
Seed future exports from THIS, not41829/41803/41792. This stage0 SDKrequests,
0newGnarkexports,0latency/PMU/profiles. Future full schedule would advance
42438->42464->43073 if unchanged26+609 fresh exports actually verify. Never
increment ahead of evidence. No live jobs/cloud after this completed stage.

Accepted production manifest:
/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-09-busy-reset-adoption176/artifacts/production-source-manifest.json.
Accepted latest REF observations from the completed unit-z-sdk176 campaign:
1x2 7.803149ms;2x2 10.960964;4x2 15.869955;8x2 24.036033;12x2 33.735567;
16x2 39.937712;24x2 66.114430;merge24x1 65.391550. Primary RSS132.75MiB;
merge54 162.727253ms. M5CPU37/59/109/189/284/335/528/507;Metal24/30/48/74/
101/124/176/175;CPU10x targets3.7/5.9/10.9/18.9/28.4/33.5/52.8/50.7.
All remain unmet. Already sub8; historical7.699867differentcampaign/statistic.
No newAWS/Aeglos/ARM/phone/huge250/4999/pressure/swap/minRAM/throughput result.
The global shared unit-Z recipe remains CLOSED/rejected (+3.20% primary); so
does old affine bucket-occupancy tagging. Do not resurrect either by curve,
shape, worker, compiler, layout, inlining or threshold selection.

Autoresearch loop: inspect actual code/profile and relevant failed mechanisms;
choose a distinct work/dependency/resource hypothesis; implement in Rust; run
targeted independent functional/resource/generic/concurrency qualification;
freeze coherent complete SDK all8+54 controls/protocol/readers and automatic
profiles BEFOREANYpreflight; serialize Rust1.98.1 xtask; during ANYpreflight or
recording ONLY poll the exact confirmed live handle until terminal (timeout or
truncated observation is not terminal); then analyze every observation; write
polite factual report, commit AND push, verify exact remote HEAD, checkpoint;
continue full goal. No questions/Python/subagents/large persistent LUT/key
expansion/new production pool/artificial RAMcap/C or ASM rewrite. Don't rerun
unchanged successful work for bookkeeping. Small gains matter, but full goal
is unchanged and completion requires verified latency targets for all8.
