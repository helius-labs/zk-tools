# Matched SDK timing contexts

At 16 workers, identical controls cross the 5% block bound in **7/21 paired blocks** and **8/21 fresh-process blocks**. Both 32-worker contexts have **0/21 crossings**. Sharing a key, SDK and pool does not eliminate the observed 16-worker variation. No candidate is compared or adopted; the preceding [positive fusion signal and failed qualification](../2026-10-27-poseidon-direct-dot/README.md) remain unchanged.

Neutral labels A/B invoke the same complete `sdk.prove` path from the exact eight accepted libraries. A newly linked diagnostic driver runs either seven paired A/B blocks in one process or one neutral stage in each fresh process. The paired case retains one key mapping, SDK object, heap and Rayon pool. Its worker TIDs and allowed CPU masks agree before/after all stages. The fresh case uses the same driver and libraries, with new process/key/pool lifetimes. The driver is not the historical accepted executable, and its absolute timings do not replace accepted latency qualification.

## Matched observations

Ryzen 9950X; primary 1x2; 16 workers on CPUs 0–15 or 32 workers on 0–31. Three repetitions, seven blocks and two neutral labels per context/worker count. Each stage has 17 timed verified SDK requests. Both modes allocate identical diagnostic buffer capacities before capture. Every process has exactly three initial primes; paired stages have no intermediate primes. Worker order, context order and label order alternate by the frozen protocol.

|Context/workers|Median stage ms|B/A geometric|B/A median|B/A min–max|Block crossings /21|Repetition-median crossings /3|CPU ms/request|Maximum lifetime RSS MiB|
|---|---:|---:|---:|---:|---:|---:|---:|---:|
|paired-w16|11.253030|0.981451|1.002246|0.716287–1.178035|7|0|138.277|92.75|
|fresh-w16|11.444101|1.028260|1.016275|0.881239–1.283154|8|1|138.417|80.00|
|paired-w32|8.969234|0.999892|0.998138|0.973836–1.019774|0|0|205.218|133.75|
|fresh-w32|9.009890|0.996896|0.995779|0.975482–1.013635|0|0|206.538|102.25|

A crossing means a block B/A ratio outside factor 1.05, or a repetition's median of seven ratios outside factor 1.03. These are descriptive null checks using the previous bounds, not an optimization acceptance gate. All 84 block ratios, per-repetition medians/geometric means, stage samples, first/last-five medians, MAD/ranges, CPU/faults/switches and process RSS remain in [analysis](artifacts/analysis.json), [rows](artifacts/rows.json) and [process receipts](artifacts/processes.json). There are no retries, exclusions or threshold changes. Ratios compare identical code; neither their sign nor their size is an optimization gain.

One paired 16-worker process spans stage medians from **11.301 to 17.529 ms** while retaining the same SDK and worker identities. Its final-stage/first-stage ratio is 0.645. The corresponding 32-worker paired processes span much narrower ranges. This establishes variation within a process; it does not establish why it occurs or invalidate every earlier measurement.

The methods jointly differ in process/ASLR, key and pool lifetime, warm age, heap history, and whether independent verification occurs between measured stages or after a group. This comparison does not isolate any one of those causes. Fixed output/sample buffers match capacities, but the custom driver and repeated stage boundaries are still diagnostic context. No absence of drift here establishes universal stability. Any bound crossing remains evidence of neutral variation in this protocol, not proof about an earlier untraced run. Prior scheduling traces associated most runnable switches with `sched_yield`; this trial does not repeat them or establish external preemption.

## Timing and proof validation

Each request timer includes witness construction, fresh blinded proving, self-verification and serialization. Sample recording and previous-result ownership updates follow the elapsed read, as in the standard SDK consumer. Stage resource snapshots bracket the loops. After a stage ends, the final proof/public bytes are copied into buffers allocated before capture. No proof file, JSON or standard output is written until all stages in that process finish. Worker broadcasts occur before priming and after all captures. No new pool, individual worker pinning, proof cache, arithmetic path, table, plan, memory policy or production source is introduced.

The six paired processes produce84 stage exports over1,446 SDK calls. The84 fresh processes produce84 exports over1,680 calls. Total **3,126 SDK calls**, including **2,856 timed requests and270 primes**, across90processes and168stages. All168 exported164-byte proofs pass independent Gnark verification, fixed public-input hash checks and fresh identity checks against the complete prior7,009-proof ledger and one another. All336proof/public members are retained and hash checked. The next ledger count is **7,177**; internal primes are not counted as exported identities.

The exact accepted core/field libraries retain their inherited tests and proof qualification; no new core/field build or test run is claimed. The newly linked driver itself is validated through these full verified SDK calls. All worker receipts preserve the expected worker indices/TIDs/allowed masks and a single SDK object. No helper, compiler, hash, archive or independent verifier overlaps a timed process. Helpers compile before captures and analysis begins after all captures. Closing checks rehash all pins and production files.

## Scope

Host/cgroup/CPU-policy snapshots are outside capture; unavailable fields are explicit and do not establish absence of limits or throttling. CPU time includes scheduling. RSS is a process lifetime maximum including load, primes, SDK and diagnostic buffers; it is not minimum RAM. No new cycle or scheduler profile is collected. Production170sources and exactacceptedSDK remain unchanged. No candidate is qualified or adopted and no prior failure is waived.

The [last complete all-shape SDK matrix](../2026-10-26-pairing-x-chain-sdk/README.md) retains all supplied M5 CPU/Metal comparisons. This diagnostic only measures primary1x2 and does not add all-shape qualification. The original all-eightCPU10× objective remains active and unmet. No AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, memory-pressure, concurrent-prover or throughput qualification is added.

Diagnostic executable: `/tmp/zopro-sdk-paired-control/native/control-sdk`; SHA-256 `0694c20a82d7b098533b68883b80c35c839dae506f5c56d0465c38cd3f9db148`.

[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [frozen capture plan](artifacts/capture-plan.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
