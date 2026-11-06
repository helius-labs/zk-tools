# Solver-to-A streaming component trial

Starting A-query window fills from joined solver epochs improves the 16-worker component estimate but slows the 32-worker component. The predeclared screen fails; production remains unchanged. These are complete **solver→A chain** measurements, not proof times.

|Workers|Original OFF, ms|Streaming ON, ms|Paired ON/OFF median|Rank interval|Wins / 15|
|---:|---:|---:|---:|---:|---:|
|16|2.730213|2.688334|0.983776|[0.976239, 0.993211]|14|
|32|2.388278|2.875706|1.202471|[1.190994, 1.208236]|0|

The 16-worker paired estimate is a 1.62% improvement. The 32-worker estimate is a 20.25% regression. Both observations are retained. The screen required the 32-worker upper bound below parity with at least 10 wins, and the 16-worker upper bound at most 1.02. There is no minimum gain floor, favorable-subset promotion, parameter sweep, capture exclusion, extension or unchanged retry.

Times are medians of 15 process-block medians, each containing 31 interleaved calls per arm. Paired ratios are not ratios of the displayed medians. Sorted ranks 4–12/15 have nominal 96.484375% marginal coverage for the median under exchangeable blocks; no simultaneous coverage is claimed. Worker-context order and within-pair order alternate. Both arms use one executable and loaded key. There is no frozen accepted solver→A executable; this is not a calibration against the accepted full SDK.

## Mechanism and correctness

The original solver and MSM functions remain byte-exact. A private solver clone copies initialized persistent A-query values only after their producing epoch has joined. Readiness preparation checks definite assignment, unique writes and the actual cached direct schedule. It retains one readiness epoch per 1024-point chunk and borrows the existing plan. It creates no duplicate key, execution plan, large lookup table or worker pool.

Each copied scalar chunk is shared across 21 ordinary window actors. Actors retain the existing borrowed Window, signed digits, pending queues, exception handling and arithmetic; they return when their inbox is empty and resume on publication. The existing narrow leading-window tree runs after solving. Chunks can arrive out of original key order, so collision/cache behavior is part of the experiment. Original global scalar grouping remains after solving: admission cancels speculation and selects the original grouped query. The measured primary input declines grouping in both arms.

The original in-place conversion of the full witness remains necessary, so early scalar conversion duplicates work. The clock includes that duplication, copies, resource admission, scheduling, fresh solver/query allocations and complete A reduction. Witness-input construction, key loading and readiness preparation are outside the clock. Quotient FFT, K/H/G2, public correction, proof assembly, verification and SDK parsing are excluded.

Native qualification passes 130 unfiltered core tests, including five new cases for signed/random/boundary scalars, identities, chunk permutations and tails, small pools/stacks, cancellation, invalid publication, producer panic, readiness bounds and narrow reads from otherwise uninitialized storage. The real primary circuit matches the existing independently qualified Arkworks A-MSM result at 1/4/16/32 workers, with uninitialized solver storage forced on. Invalid-witness rejection, explicit admission denial and runtime-configuration fallback pass. The existing fused-query geometry guard declines 2×2 preparation as expected. This component does not qualify concurrent full proofs, arbitrary admitted grouped circuits, portability or memory pressure.

The source snapshot contains 179 files: 171 unchanged originals, five append-only original parents and three new modules. All 176 accepted files recover exactly. Reversing only the publication hooks/signature recovers the original solver body. Production files remain unchanged. Actual native/fat-LTO/one-codegen-unit driver flags are checked before capture; original field qualification is inherited, not claimed rerun.

## CPU work and memory observations

All latency processes finish before four separate warm PMU runs and four cycle profiles, each with 4096 calls. The following are single-run counter observations, without latency confidence intervals:

|Workers|OFF cycles/call|ON cycles/call|OFF instructions/call|ON instructions/call|
|---:|---:|---:|---:|---:|
|16|113,186,381|135,585,192|246,563,444|261,723,765|
|32|173,048,890|252,194,147|263,152,327|305,579,023|

At 32 workers ON uses approximately 45.7% more user cycles and 16.1% more instructions. Cache-miss event counts roughly double at both worker counts. These events do not measure DRAM bandwidth or establish memory-capacity exhaustion. Profiles show more sampled crossbeam epoch work and Window feeding; new actor synchronization and the change in execution order are plausible contributors, not isolated causal measurements. Sample shares include waiting/helping and are not removable wall-time fractions. The original feed retains its partial batches across calls; it does not flush at every chunk boundary.

There are 19 early chunks containing 18,958 of 21,006 scalars. At 32 workers, the median actor-launch count is 278 per call; at 16 it is 194. Median feeds started before the solver function returned are 421/397 respectively, out of 441 possible chunk/window feeds. This marker also includes work after the last joined epoch but before the solver return; it does not prove all counted feeds overlap useful later solver operations or quantify a saved critical path.

The existing resource admission checks a conservative combined requested-byte forecast before speculation and falls back on denial. Forecasts are 102,117,396 bytes at 16 workers and 125,244,820 at 32, including the original full-proof forecast plus persistent windows, weighted scratch, shared scalar copies, queues and launch allowances. This is not a new fixed RAM cap or a rigorous allocator/stack/RSS bound.

Maximum mixed-latency-process RSS is 57.0/75.25 MiB at 16/32 workers. Separate OFF/ON profile-process RSS counters are 60.75/62.75 MiB at 16 and 93.5/93.0 MiB at 32. The diagnostic holds an owned image and all solver vectors through A, and its process counters include loading, preparation and priming. They do not establish full-prover memory savings, minimum RAM, device suitability or pressure safety.

## Records and decision

The 66 successful workflow commands include the native build/test stages, five functional callers, 30 latency callers and the remaining PMU/profile/decode steps. There are 34,628 measured and 34,956 total completed solver→A benchmark calls including priming, postchecks and functional fallbacks. They are not proofs. The canonical independent-proof ledger remains **30,846**; no SDK proof request or new Gnark export is produced.

An initial formatting invocation could not resolve the not-yet-copied perf support module; formatting was completed with child traversal disabled before snapshot/build. The first privileged PMU caller then stopped before preparation/counter enable because `sudo` omitted the required runtime settings. That failed receipt is retained. A separately pinned repair forwards the explicit frozen environment and runs only the remaining 28 commands. No successful latency, test, build or recording is repeated. Four raw profiles contain 173,264 samples with zero lost records; PID/TID/CPU/period/mapping metadata and hashes are checked. A separate checker reconstructs source recovery, pins, command streams, timings, gates, PMU counts and sample ownership/counts, and independently checks raw-profile loss records.

The necessary component gate fails at 32 workers. No full-SDK follow-up or adoption is justified for this implementation. The result closes this actor-based prototype, not every possible solver/MSM fusion. Further work needs a distinct reduction in CPU work or task generation, with a concrete cost model; adding more concurrent readiness jobs is not supported by this result.

## Accepted comparison, inherited

These accepted local full-proof times and supplied historical M5 references are unchanged, not remeasured here. Cross-machine ratios do not establish equal hardware efficiency. The all-eight CPU10× goal remains active and unmet, **0/8**.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37|24|3.7|
|2×2|11.151932|59|30|5.9|
|4×2|16.086386|109|48|10.9|
|8×2|24.179453|189|74|18.9|
|12×2|33.884455|284|101|28.4|
|16×2|40.077381|335|124|33.5|
|24×2|65.850777|528|176|52.8|
|Merge24|65.060937|507|175|50.7|
|Merge54|161.883899|—|—|—|

Accepted source and SDK identity remain those in [the production manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). No fresh AWS/Aeglos comparison, large-circuit, swap, phone/ARM/TEE, concurrent-throughput or minimum-RAM claim follows from this experiment.
