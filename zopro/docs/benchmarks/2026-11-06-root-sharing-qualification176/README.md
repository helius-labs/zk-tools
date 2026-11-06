# Optimistic scalar-root sharing: arithmetic qualification

**The bulk arithmetic passes correctness checks; performance is not measured.** The optimized driver is linked, but no request broker or production service is implemented. The next experiment asks whether a ready-root SIMD bulk can outperform independent inversions even with coordination costs removed.

The optimistic bulk arithmetic is qualified, and its timing driver is linked. No caller clock has run. There is no request broker, concurrent service or production change.

The immutable accepted field library is reused byte-exact, without a field/core/SDK rebuild or old-suite rerun. The new wrapper combines two/four SIMD vectors using the accepted eight-lane product-tree inverse. It needs8/14 vector multiplies, one scalar root multiply and one scalar inverse at16/32roots. Solo will use the exact individual-inversion loop. One independent oracle process passes12,544 bulk-output configurations and49 invalid-root-position rejections. The general Montgomery model passes776 batches and37,248 root outputs for b=0..96, with exact one inverse plus3(b−1) multiplies for nonempty inputs. All outputs are canonical and match Arkworks.

There were four native command attempts: oracle build/run passed first time; the first timing-driver link failed because helper libraries lacked LTO bitcode; the repaired driver link passed using the existing native serde_json/sha2 libraries. Source, accepted field and O3/native/fatLTO/one-CGU flags were unchanged. The successful oracle was not rebuilt or rerun. The initial repair-plan helper expected a result.json that xtask does not write on failure; its source/binary/logs remain under repair-initial, and only that helper was corrected to bind completed.json. This did not repeat a native or mathematical command. Independent review checks both pin sets, all four actual command attempts/stream hashes, the two library-reference changes, optimized final-driver identity and unchanged source176. It passed once.

The state-size model is not an implementation or race proof. A64-byte slot overlays four input/result atomic words with status/generation, plus cache-line-sized pending masks and leader state. On this64-bit host its requested payload totals1,216bytes at16workers,2,240at32 and6,400at96, excluding allocator rounding and leader stack. A single worker can omit it. Three Fq arrays plus usize IDs would add104bytes per leader root in a straightforward scalar implementation; this is not the compiler frame or total stack peak. Input/result publication alone requires at least16 payload memory operations per request plus state and shared atomics. None is charged in the proposed optimistic timing arm.

The ownership audit found a material constraint: current leaf kernels do not fork with scratch live, and the workspace forecast multiplies max leaf scratch by worker count. Parking preserves that assumption but delays workers; helping nested leaves while waiting needs a new lifetime/admission bound. A real proof-owned service also needs solo behavior, nonzero-domain checks, safe publication/reuse, overlapping-proof isolation and panic/cancellation handling. No such concurrency guarantee is claimed by the sizeof model or arithmetic oracle.

The next stage is a separate prospectively frozen ready-root arithmetic wave at1/16/32workers, two identical individual-inversion controls and one optimistic shared arm. All input roots are already available to the coordinator and other workers are parked, so the SIMD bulk has favorable resources and pays no request/wait/ownership overhead. Start/end barriers, actual packing/unpacking and output sums remain measured. This is a useful necessary implementation screen, not a mathematical lower bound on every sharing algorithm. A loss closes this implementation before broker work; a win only selects further coordination feasibility, never a proof-speed claim or automatic adoption.

Existing G2 norm sharing, joint-frontier, global phase, chain and queue negatives remain closed. A targeted search found no matching scalar-root worker service; that is not proof of novelty. All-eight CPU10× remains active/unmet0/8. Accepted source176/SDK6d83, local1×2 7.928660ms, inherited132.75MiBRSS/notminimumRAM and proofledger30846 are unchanged. There is no new proof, timing, AWS/Aeglos/Gnark, huge/mobile/pressure/swap or throughput result.


## Why the optimistic screen comes first

Current workers can invert their own roots immediately. A coordinator can save summed arithmetic yet serialize independent work and lengthen the proof path. The current [workspace forecast](../../../crates/zopro-core/src/msm_affine/workspace.rs) assumes leaf kernels do not fork with live scratch. Making waiting workers execute arbitrary nested leaves would require a different lifetime/admission bound. Parking avoids that new retention, but workers stop progressing. No throughput improvement or twofold latency claim follows from the batch-inversion identity.

At 16 roots the wrapper multiplies two eight-lane vectors, uses the accepted lane inverse, then recovers both inverse vectors. At 32 it uses two vector pairs, a common root and a reverse tree. All roots are in the same BN254 Fq field, including G2 denominator norms; FFT Fr roots are outside scope. Root extraction and scalar inversion retain the accepted implementation, including canonicalization. No new scalar backend, IFMA multiplication, representation or key table is added.

|Ready roots|Vector multiplications|Scalar root multiply|Scalar inversions|
|---|---:|---:|---:|
|1|0|0|1|
|16|8|1|1|
|32|14|1|1|

These are source operations, not instruction or latency predictions. The existing field library is immutable; the wrapper is a standalone feasibility consumer. Both independent controls will use the same native executable and identical individual-inversion code. They are repeated controls, not separately compiled accepted-SDK references. The shared arm runs one existing benchmark worker on CPU2 and parks the others, including its SMT sibling. It pays for arithmetic, packing/unpacking, output sums and start/end barriers; it pays no request publication, leader election, queue, polling or result handoff. That favors this implementation but is not a lower bound on every possible asynchronous strategy.

## Independent arithmetic and state checks

The fixture contains 256 public rows of 32 canonical roots, 256 KiB of input data, generated from hashes and positive/negative boundary rows. It contains no precomputed inverse lookup used by timed code and is not a proposed prover table. Output qualification covers 12,544 active-root configurations at 1/16/32 roots, each checked for canonical form, multiplicative identity and equality with independent Arkworks inversion. All 49 possible invalid-root positions are rejected. A separate general batch model checks 776 cases and 37,248 root outputs across sizes 0 through 96 and eight patterns. It verifies the optimized 3(b−1) multiply count plus one inverse for nonempty batches. Empty batches perform neither. These are configurations, not a claim of 49,792 distinct root values or a new mathematical algorithm.

The conceptual state model is a sizing calculation only. A 64-byte aligned slot overlays input/result words and state/generation; cache-line pending-mask words and a leader header scale with worker count. No instance services requests, and no concurrency, safe reuse or cancellation protocol is qualified. The driver creates benchmark threads only, with 512 KiB configured stacks; that is not measured stack use or a production RAM bound.

|Workers|Requested conceptual state bytes|Three Fq arrays + IDs payload bytes on a straightforward leader|
|---|---:|---:|
|1|256|104|
|16|1216|1664|
|32|2240|3328|
|96|6400|9984|

A single worker can omit the shared state entirely. The table excludes allocator rounding and compiler temporaries/spills and is not an actual service memory peak, full-proof RSS or minimum RAM. A real protocol must account for at least 16 payload loads/stores per request, release/acquire publication, shared atomic traffic, ownership, overlapping proofs and panic/cancellation. Those costs may erase any arithmetic saving.

## Build and review evidence

All 176 accepted production source files remain exact. The accepted native field rlib SHA256 is `1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713`; it is reused, not rebuilt. Initial qualification pins 307 identities. Oracle build and execution succeed on first attempt. Driver linking initially fails because the ordinary helper serde/sha libraries contain no fat-LTO bitcode. Only those two library references change to the already frozen native dependencies; driver source, accepted field and O3/native/fatLTO/one-CGU flags remain exact. The repaired final link succeeds, with 317 repair pins. No successful oracle, field/core/prover suite or profile is repeated.

The first repair-plan helper also assumed a result file existed after failure. Its source/binary/logs are retained; the corrected helper binds the actual completed-step file. This changes metadata preparation only. Independent review checks all four native command attempts, actual argv/exits/stream hashes, source/field identities and the two library changes. It passes once. One unused fixture helper warning in the oracle is preserved without a cleanup rebuild. The final driver SHA256 is `40efd3811d9616e0f50ef46e5ada2b23b4622dedee51e82f4b9a2e9f7e0b96d3`. It is linked and has not been executed.

No timing protocol has been frozen yet. The [next stage](artifacts/next.md) specifies the prospective 1/16/32-worker contexts, identical controls, 15 paired blocks, all timing checks and automatic PMU/profiles to freeze before the first driver/preflight clock. A loss closes this ready-root implementation before broker engineering; a win selects further ownership/coordination feasibility, not production adoption or a full-proof speedup. Existing shared-norm, frontier/phase, chain and queue results remain distinct and closed.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.





## Artifacts

All new wrapper/fixture/driver/oracle sources and raw build/repair/review receipts are public as checked UTF8 members. Accepted sources and dependencies are bound by hashes. No executable/library blobs, private keys/images/witnesses/proofs, inverse-result table or third-party full text is embedded. There is no new performance, AWS/Aeglos/Gnark, huge/mobile/pressure/swap/minimum-RAM or throughput result. All eight CPU10× requirements remain active and unmet.

[Design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [oracle result](artifacts/oracle.json), [review](artifacts/review.json), [bulk wrapper](artifacts/bulk.rs), [driver](artifacts/driver.rs), [fixture](artifacts/fixture.rs), [build plan](artifacts/build-plan.json), [build pins](artifacts/build-pins.json), [repair](artifacts/driver-repair.json), [repair plan](artifacts/driver-repair-plan.json), [repair pins](artifacts/driver-repair-pins.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS).
