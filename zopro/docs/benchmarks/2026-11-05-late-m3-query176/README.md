# Late multiplier G2 complete-query trial

The native trial provides **no basis for adoption**. With corrected native optimization, the candidate's paired ON/OFF ratio is **2.754× at 16 workers and 2.594× at 32**; every primary timing block is slower. The declared reference-calibration gate also fails in four multithreaded contexts. These observations do not establish accepted-baseline latency or a full-proof regression. Production remains unchanged; no SDK follow-up is justified for this fixed candidate.

The earlier [count screen](../2026-11-05-lfg-multiplier-planes176/README.md) remains true: G2 generic additions fall 633,620→606,872 (4.22%). Its omitted projective, recoding, queue, normalization and layout costs were unqualified. This trial implements the complete query and shows that this necessary count gate was insufficient. The earlier [per-input tripling method](../2026-11-05-tripling-formation176/README.md) remains separately rejected.

## Implementation and memory policy

Each fixed width13/M13 ordinary window computes the multiplier-one weighted sum, releases bucket owners, computes the multiplier-three sum, and combines `S1 + 3*S3`. Original high→phi→low order and two point partitions remain within each multiplier plane. The direct leading window uses multiplier one only. It uses dense4,096 bucket positions with holes and the original weighted tree, original borrowed queue/batch arithmetic, and the original signed indexed leading reducer. It forms no per-input triples or key-derived point table.

The original complete dispatcher performs global grouping once. Admitted grouping, including 2×2, bypasses the candidate; only declined queries in the existing 1024..65536 range can enter. The runtime flag is explicitly opt-in. Resource denial returns before optional allocation or input mutation. Canonical words remain immutable inside the candidate; the complete caller resets its exclusive input and includes reset/affine normalization in the timer.

A temporary `[u16;20]` digit buffer is40bytes per input, **1,203,560bytes** for primary B. Existing input words remain live. Additional admission conservatively charges codes, all suspended window/part outputs, indexed top arrays, exceptional projective normalization, weighted scratch and workers×the largest nonforking leaf. The unchanged original proof forecasts remain charged as well; this does not relax existing pressure headroom. No arbitrary global RAM ceiling, new production pool, key cache or retained execution plan is introduced. Overflow/denial is tested, but full-proof pressure, allocation lifetime peaks, minimum RAM and mobile/ARM are not qualified.

The accepted176-file source is isolated:172 originals byte-exact, three appended parents, one reversible dispatcher insertion and one new child. All176 original bytes recover; field and original point kernels are unchanged. The default path retains original grouping/fallback.

## Link audit and preserved first attempt

The first consumer helper copied a legacy `rustc_flags` dependency injection. That replaced the workflow's default optimization flags rather than appending the SHA2 dependency. The library builds were native/release, but the driver command omitted `-O`, native CPU targeting, fat LTO and one codegen unit. Its 313-step capture and first analysis remain preserved as **disqualified native timing evidence**. The preliminary timing rejection was retracted when this audit found the error.

The repair changes only linking and the dependent capture: `additional_rustc_flags` preserves all native defaults. Both corrected link-command receipts explicitly contain `-O`, `target-cpu=native`, `lto=fat`, `codegen-units=1`. The source176 reference is freshly frozen from the old coherent original control libraries and the exact original driver; it is not a historical optimized executable or the accepted SDK. The fresh OFF/ON arms share one corrected executable. No source, successful core test, native library build, sample count, declared gate or geometry was changed or repeated. Both attempts and their distinct binary identities are retained. The historical coalesce consumer executable used the same legacy link pattern; its native-driver qualification is not inherited here.

## Corrected complete-query measurements

Ryzen9950X: one worker pinnedCPU2, physical16 pinned0–15, logical32 pinned0–31. Three checked primes precede18preflight processes;15fixed rotated/reversed blocks contain31timed calls per process, followed by one postcheck. Every timed, prime and postcheck output agrees with the retained independent Arkworks complete-MSM hash, and key point inputs remain unchanged. Preflight also confirms candidate route313/digit payload on primary ON and original bypass elsewhere. These are standalone complete G2 queries in the original dispatcher, not full proofs or simultaneous independent proof requests.

|Query / workers|Reference ms|Fresh OFF ms|ON ms|Paired ON/OFF median [lower,upper]|Reference calibration|
|---|---:|---:|---:|---:|---|
|1x2-B-w1|37.470506|36.975143|87.923416|2.378976 [2.373839, 2.382920]|pass|
|1x2-B-w16|3.691887|3.507871|9.690858|2.753739 [2.722133, 2.811799]|fail|
|1x2-B-w32|3.281868|3.075093|7.891225|2.594241 [2.544044, 2.608267]|fail|
|2x2-B-w1|40.101023|39.704178|39.694197|1.000167 [0.998542, 1.001490]|pass|
|2x2-B-w16|4.237364|3.951296|3.980294|1.011767 [0.993866, 1.015469]|fail|
|2x2-B-w32|3.861911|3.550429|3.521053|0.998078 [0.981596, 1.008470]|fail|

Bounds are ranks4–12of15, nominal96.484375% marginal coverage under independent exchangeable blocks, without a familywise guarantee. Displayed latency medians need not divide to the paired-ratio median. Reference calibration requires OFF/reference bounds within[0.97,1.03] in all6contexts. The four16/32-worker contexts fail: freshOFF is materially faster than the freshly linked reference. This limits accepted-baseline inference; it is not a new adopted OFF optimization. Primary16/32 also independently fail the gain gate (upper<1). Grouped2×2 retains its declared safety bound (upper≤1.03). No failed calibration was rerolled, widened or replaced with a favorable subset.

|Query / workers|OFF CPU ms/query|ON CPU ms/query|OFF process RSS MiB|ON process RSS MiB|
|---|---:|---:|---:|---:|
|1x2-B-w1|36.974613|87.921032|15.50|15.50|
|1x2-B-w16|45.179903|104.698968|26.00|45.25|
|1x2-B-w32|70.204774|151.654032|35.50|63.25|
|2x2-B-w1|39.703774|39.693452|27.25|27.25|
|2x2-B-w16|48.232871|48.258387|43.75|43.75|
|2x2-B-w32|75.335387|75.361742|61.75|60.75|

RSS is the maximum whole-process rusage counter among latency processes, including fixture loading, priming, allocation and oracles. It is not the candidate's isolated workspace peak or minimum RAM. Primary32 rises35.50→63.25MiB in this standalone caller. No major/minor-fault service time or causal DRAM conclusion is inferred.

## Profiles and model limits

All latency finishes before the4FIFO-gated cycles:u profiles, fixed16-million-cycle period after priming,200measured queries each. All4pass the descriptive sample/unknown screen, have expected PID/worker TIDs/CPU bounds/executable mapping and zeroLOSTrecords. No absolute period coverage or instruction/CPI comparison is established.

|Corrected profile|Samples|Unknown share|Qualified|
|---|---:|---:|---|
|cycles-1x2-B-w16-OFF|2768|0.506%|true|
|cycles-1x2-B-w16-ON|6658|1.202%|true|
|cycles-1x2-B-w32-OFF|3970|0.781%|true|
|cycles-1x2-B-w32-ON|9052|1.381%|true|

At32workers, borrowed signed addition samples decrease1,623→1,122, while projective mixed addition increases71→1,427, reduce_layer531→1,427, and base-field inversion279→872; candidate batch_to_affine has791samples. These samples describe the same number of complete queries but include inlined work and scheduling; they are not exact operation counts or a decomposition of wall time. The signature is consistent with lower borrowed-add work being overwhelmed by other paths, rather than the count saving being false.

The model counted formal noncancelling occupancy and generic affine pairs. It did not reproduce actual pending-queue collisions, spill-to-projective work, exceptional normalization or per-batch inverse costs. The native fixed width13 uses the existing352-point batch heuristic, versus200atwidth12, over nonuniform allowed digits. These facts motivate an exact branch/queue census if it can improve the next screen. The profiles alone do not identify how much mixed-add cost comes from collisions versus equal-x exceptions or other reductions. A changed batch/queue policy would be a new preregistered experiment, not a rescue or unchanged repeat of this result.

## Correctness, reproducibility and next decision

The full unfiltered native core suite passes128tests (125original+3new) once. New tests exhaust carry inputs−1..8193, reconstruct signed Phi2 components over field edges/powers/random inputs, and check complete generator-sum oracles at1024/2051points and1/4/16/32threads, with identities/repeated/opposite points/zero/±1/full-width scalars. Overflow/admission denial and empty multiplier planes preserve inputs. Real retained fixtures carry independent Arkworks on-curve/subgroup/canonical-scalar/full-MSM provenance; all20,712complete queries across both attempts match their output hashes. This count includes primes/postchecks and is not a proof count.

Independent review passes354unique pins,632successful command receipts/stream hashes, original source recovery, native-link argument checks, all raw integer timing reconstruction/gates and independently reparsed symbol counts. Successful core tests/library builds are reused, not repeated. Corrected capture completes313steps/292callers on first execution after the link repair. Both the initial disqualified and corrected captures remain available. No production change, full-SDK follow-up or runtime adoption follows this fixed candidate. Before another native design, audit actual queue/projective work so the next cost screen accounts for the paths that defeated this one; do not treat a small generic-addition reduction as a proof-latency prediction.

## Accepted CPU and Metal comparisons, inherited

Accepted SDK SHA-256 remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`.
No full-proof latency is remeasured here. All8CPU10×targets remain unmet.

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

Merge54 remains161.883899ms without an M5 reference. Latest full-SDK latency remains epoch-clock176; canonical proof ledger29,461unchanged. No fresh SDK/Gnark/AWS/Aeglos/250/4999/device/pressure/concurrency performance result is added. Goal all-eight CPU10× remains active.

## Artifacts

[Implementation](artifacts/late_m3.rs), [scratch forecast](artifacts/workspace-suffix.rs), [original design](artifacts/design.md), [corrected design](artifacts/corrected/design.md), [source audit](artifacts/source-audit.json), [native protocol](artifacts/native-plan.json), [link repair](artifacts/corrected/repair-link.rs), [corrected link plan](artifacts/corrected/consumer-plan.json), [corrected capture](artifacts/corrected/capture-plan.json), [corrected analysis](artifacts/corrected/analysis.json), [corrected symbols](artifacts/corrected/symbols.json), [initial disqualified analysis](artifacts/analysis.json), [reviewer](artifacts/review.rs), [review](artifacts/review.json), [all receipts](artifacts/receipts.json.gz), [source archive](artifacts/source.tar.gz), [source member hashes](artifacts/source-members.json), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS). All177regular source members match extracted hashes; receipts gzip roundtrips exactly. Executables,libraries,raw perf.data,private actual key/image/witness/scalar/point arrays are excluded.
