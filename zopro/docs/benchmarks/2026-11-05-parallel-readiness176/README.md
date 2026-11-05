# Query readiness at actual parallel solver barriers

Both retained circuits have ready A/B query chunks before the solver's final epoch. This is a structural opportunity for earlier MSM work, **not an implementation, latency gain or adoption**. The loaded 1×2 plan has 16 epochs: the first has 63 root tasks, most later epochs have only two, and the last two have one. The 2×2 plan has 77 root tasks initially, four through most middle epochs and two at the end. This measures available solver-task parallelism, not actual CPU utilization or idle time.

After primary's first epoch, 6,901 A operands and 9,542 B operands are available. Two complete A chunks and five complete B chunks of 1,024 points are available. The strict key-order prefixes contain only 1,542/2,248 operands. Chunks could therefore expose work without arbitrary per-point reordering. Dispatch, grouping, chunk lifetime and bucket costs remain unmeasured.

## Scope and validation

The earlier [source-order census](../2026-10-30-wire-readiness175/README.md) explicitly left parallel-epoch readiness open. This observer reads the schedule already prepared by the current accepted Key loader, including bounded epochs, local refinement and sparse-Poseidon tags. It does not substitute a reconstructed scheduler. Both currently loaded programs match the earlier serialized images byte for byte. Each operation appears once; independent original initialization/unique-write checks and a separate solver-image reader validate 771,105 external producer/read schedule edges.

The source snapshot has 177 files: 173 of 176 originals remain exact, three parents gain reversible diagnostic exports, and one child adds the observer. All 176 original bytes recover. Arithmetic and scheduler bodies remain exact. Three focused observer tests and three reader tests pass. Existing full field/core/prover qualification is inherited; no fresh full-suite or portable claim is made. No witness is solved and no proof, timing sample or cloud resource is created. The observation arrays are disposable offline state, not a proposed retained execution plan.

Two helper failures are retained. An inherited link helper expected unused perf.rs metadata after successfully compiling the census executable. Metadata was completed from that existing executable. A separate test link omitted the SHA dependency required only by the SDK-main imports; a minimal model-test entry removed that unrelated import. Four successful checkpoints were reused, and no successful build, test or census was repeated. Eight commands complete successfully, with two known helper failures. Independent closeout checks all 285 pins, command stream hashes, 68 query/epoch rows and unchanged production source.

Readiness below is after an outer epoch's tasks have joined. Operation counts are not time weights: primary's first epoch already completes 10,219 of 31,061 operations. Task-internal readiness and elapsed publication times are not measured. Values past their final solver read may still be needed by public correction or other proof consumers; this does not authorize early in-place canonicalization of live Montgomery storage. Folded K and H are excluded.

## Readiness by barrier

Counts are ready points (percent of that query). Complete chunks use fixed size 1,024; the final partial chunk is counted at its actual size.

|Circuit / query|Epochs joined|Operations completed|Ready in any order|Strict key-order prefix|Points in complete chunks|Complete chunks|
|---|---:|---:|---:|---:|---:|---:|
|1x2/A|0|0|141 (0.7%)|141 (0.7%)|0 (0.0%)|0|
|1x2/A|1|10219|6901 (32.9%)|1542 (7.3%)|2048 (9.7%)|2|
|1x2/A|4|17633|12027 (57.3%)|3340 (15.9%)|9216 (43.9%)|9|
|1x2/A|8|22807|15484 (73.7%)|4992 (23.8%)|11790 (56.1%)|12|
|1x2/A|15|30443|20593 (98.0%)|7900 (37.6%)|18958 (90.3%)|19|
|1x2/A|16|31061|21006 (100.0%)|21006 (100.0%)|21006 (100.0%)|21|
|1x2/B|0|0|142 (0.5%)|142 (0.5%)|0 (0.0%)|0|
|1x2/B|1|10219|9542 (31.7%)|2248 (7.5%)|5120 (17.0%)|5|
|1x2/B|4|17633|16676 (55.4%)|4911 (16.3%)|13312 (44.2%)|13|
|1x2/B|8|22807|21844 (72.6%)|7380 (24.5%)|19849 (66.0%)|20|
|1x2/B|15|30443|29475 (98.0%)|11724 (39.0%)|28041 (93.2%)|28|
|1x2/B|16|31061|30089 (100.0%)|30089 (100.0%)|30089 (100.0%)|30|
|2x2/A|0|0|232 (0.6%)|232 (0.6%)|0 (0.0%)|0|
|2x2/A|1|14990|10148 (27.7%)|1803 (4.9%)|4096 (11.2%)|4|
|2x2/A|4|27463|18831 (51.4%)|3601 (9.8%)|13312 (36.3%)|13|
|2x2/A|8|37575|25593 (69.9%)|5253 (14.3%)|22301 (60.9%)|22|
|2x2/A|15|52847|35811 (97.7%)|8161 (22.3%)|33565 (91.6%)|33|
|2x2/A|16|54083|36637 (100.0%)|36637 (100.0%)|36637 (100.0%)|36|
|2x2/B|0|0|231 (0.4%)|231 (0.4%)|0 (0.0%)|0|
|2x2/B|1|14990|13759 (26.3%)|2602 (5.0%)|6144 (11.8%)|6|
|2x2/B|4|27463|25678 (49.1%)|5265 (10.1%)|22528 (43.1%)|22|
|2x2/B|8|37575|35784 (68.5%)|7734 (14.8%)|31794 (60.8%)|32|
|2x2/B|15|52847|51046 (97.7%)|12078 (23.1%)|48178 (92.2%)|48|
|2x2/B|16|54083|52274 (100.0%)|52274 (100.0%)|52274 (100.0%)|52|

All 17 barriers per query are retained in [the raw aggregate results](artifacts/analysis.json). The prospective condition—an ordered prefix that fails to advance past its input-ready terms before the final epoch—is false for all four populations. That only preserves an ordered-stream design as a possibility; it is not a passing performance gate. Fixed ready chunks provide a separate opportunity with more available points.

## Prepared task geometry

|Epoch|1×2 ops / root tasks|2×2 ops / root tasks|
|---|---:|---:|
|0|10219 / 63|14990 / 77|
|1|3508 / 5|6085 / 8|
|2|1872 / 3|3074 / 5|
|3|2034 / 3|3314 / 5|
|4|1426 / 3|2616 / 5|
|5|1280 / 2|2560 / 4|
|6|1190 / 2|2380 / 4|
|7|1278 / 2|2556 / 4|
|8|1248 / 2|2496 / 4|
|9|1222 / 2|2444 / 4|
|10|1280 / 2|2560 / 4|
|11|1188 / 2|2376 / 4|
|12|1282 / 2|2564 / 4|
|13|776 / 2|1552 / 4|
|14|640 / 1|1280 / 2|
|15|618 / 1|1236 / 2|

Primary has 97 root/leaf tasks and 44 tagged leaves; 2×2 has 140 root/leaf tasks and 74 tagged leaves. The prepared snapshots add no leaf count through local refinement. The observation uses 32 configured workers but does not measure realized utilization or predict a speedup from filling the remaining worker capacity.

## Next implementation question

A useful prototype would publish only fully ready query chunks at solver barriers and preserve pending bucket state until all chunks are consumed, so that chunking does not repeat weighted reductions. It must preserve scalar grouping or measure the cost of losing cross-chunk groups, retain original G2 high/low semantics, bound any scalar staging/queue metadata, and reserve the complete live workspace. It must not duplicate keys or solver plans. Elapsed epoch readiness and a verified whole-proof comparison are still needed. The earlier post-solver product-overlap failures remain unchanged; moving work into low-parallelism solver epochs is a different scheduling boundary. This census does not establish a twofold path or justify adoption.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker SDK observations and supplied M5 references follow; this census remeasures none of them. Accepted SDK SHA-256 is `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. All eight CPU10× targets remain unmet.

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

Accepted merge54 remains 161.883899 ms without a supplied M5 reference. Latest full-SDK measurements remain in the earlier integer-LogJumps screen; accepted source176 and the 29,255-proof ledger are unchanged. No AWS/Aeglos, RAM peak/minimum, pressure/swap, phone/ARM/TEE, 250/4999 or concurrent-throughput result is added. The all-eight10× objective remains active.

## Artifacts

[Design](artifacts/design.md), [all counts](artifacts/analysis.json), [closeout review](artifacts/review.json), [observer](artifacts/observer.rs), [independent reader](artifacts/model.rs), [driver](artifacts/probe.rs), [source recovery](artifacts/source-audit.json), [initial plan](artifacts/build-plan.json), [final remaining plan](artifacts/final-plan.json), [final pins](artifacts/final-pins.json), [raw command receipts](artifacts/receipts.json.gz), [source archive](artifacts/source.tar.gz), [source member hashes](artifacts/source-members.json), [publisher](artifacts/publish.rs), and [checksums](SHA256SUMS). The source archive contains exactly177 regular members with checked extraction hashes; the receipt JSON gzip roundtrips exactly. Private key/solver images, per-operation stamps, executable/library binaries and witnesses remain local.
