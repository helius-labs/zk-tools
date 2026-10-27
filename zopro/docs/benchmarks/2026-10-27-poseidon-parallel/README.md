# Parallel across-hash Poseidon component

In-place chain-length grouping is **18.05% / 18.00% slower** than the two scalar task controls over the eight affected cases, with 0/7 winning block aggregates. It is also **21.86% slower** than plain batching over the affected ragged cases. Identical-control calibration passes in all 24 cases. This grouping strategy is retained as a negative experiment and is not integrated into the prover. The [earlier 2.45× single-thread throughput result](../2026-10-27-poseidon-across8/README.md) remains a positive component result.

The arithmetic is byte-identical to that earlier complete-write eight-lane kernel. This experiment adds actual Rayon execution and sorts existing independent task descriptors by width-three chain length before fixed groups of eight. A predeclared backlog guard enables grouped batching only when eligible tasks exceed twice the worker count. Underfilled cases retain scalar task execution. Raw/unsupported groups preserve individual-task parallelism. There is one fixture Plan and no new lookup table, copied execution image or production thread pool.

## Fixed comparison

Ryzen 9950X; 16 workers individually pinned to CPUs 0–15 (one per physical core), or 32 workers to CPUs 0–31 (including SMT siblings). Each case creates one pool and one probe, enters the pool once, and runs seven paired blocks of control A, control B, plain original-order batching, and guarded grouped batching in rotated/reversed order. Both controls use the same preserved scalar task routine. Input data, pool and probe addresses remain shared within a case.

Construction, descriptor sorting, reset, three warm calls and full interpreter comparisons occur outside capture. Timed loops include task iteration, tag checks, prefixes, arithmetic, every original wire/slot/A/B/C write, Rayon scheduling and completion. Sorting is measured separately; its exclusion makes these results insufficient to establish total setup/request cost. No per-iteration pool installation or output checks occur inside the loop. Case keys use `g` for groups of eight tasks (1/4/16), `w` for workers, `c` for materialized C, and `h` for uniform one-hash tasks (0) or ragged one/two/three-hash chains (1). Each stage runs `512/g` epochs.

|Case|A µs/epoch|B µs/epoch|Plain µs/epoch|Grouped µs/epoch|Grouped/A paired|Grouped/B paired|Wins A/B|
|---|---:|---:|---:|---:|---:|---:|---|
|g1-w16-c0-h0|20.172|20.193|17.833|20.393|1.021315|1.014930|0/7;1/7|
|g1-w16-c0-h1|37.098|37.247|81.781|37.161|1.001940|0.996760|4/7;5/7|
|g1-w16-c1-h0|20.257|20.393|18.336|20.422|1.009802|1.002347|1/7;4/7|
|g1-w16-c1-h1|38.146|38.220|77.836|38.292|1.005225|1.001615|1/7;3/7|
|g1-w32-c0-h0|27.829|27.830|18.098|28.057|1.007812|0.997631|0/7;3/7|
|g1-w32-c0-h1|53.363|53.468|77.621|53.656|1.003414|0.999923|2/7;4/7|
|g1-w32-c1-h0|28.098|28.311|18.448|28.250|1.000002|0.991037|4/7;4/7|
|g1-w32-c1-h1|54.096|54.117|77.924|53.892|0.997047|0.993998|5/7;4/7|
|g16-w16-c0-h0|91.662|91.353|76.314|76.017|0.826363|0.822671|7/7;7/7|
|g16-w16-c0-h1|177.750|178.440|145.389|195.967|1.102567|1.095931|0/7;0/7|
|g16-w16-c1-h0|92.525|92.919|97.483|98.789|1.066656|1.065494|0/7;0/7|
|g16-w16-c1-h1|178.970|179.297|156.750|230.837|1.306253|1.307082|0/7;0/7|
|g16-w32-c0-h0|78.792|78.051|92.527|91.143|1.107447|1.162728|1/7;0/7|
|g16-w32-c0-h1|152.565|154.228|200.967|203.314|1.336602|1.289363|0/7;0/7|
|g16-w32-c1-h0|81.863|81.496|110.899|110.712|1.332826|1.347308|0/7;0/7|
|g16-w32-c1-h1|158.446|158.191|219.251|236.819|1.505671|1.482513|0/7;0/7|
|g4-w16-c0-h0|29.189|29.235|31.027|29.128|1.002948|0.993839|4/7;4/7|
|g4-w16-c0-h1|61.475|61.930|101.111|61.921|1.004625|1.001779|3/7;3/7|
|g4-w16-c1-h0|29.751|29.782|40.036|29.659|1.001449|0.999069|4/7;4/7|
|g4-w16-c1-h1|62.240|62.461|101.566|62.233|1.003831|0.998430|4/7;4/7|
|g4-w32-c0-h0|26.661|25.968|36.830|26.143|0.970591|0.993968|4/7;4/7|
|g4-w32-c0-h1|54.540|53.938|107.516|55.354|1.029235|1.047780|3/7;1/7|
|g4-w32-c1-h0|27.501|27.382|40.782|27.543|1.034099|1.042701|3/7;4/7|
|g4-w32-c1-h1|56.127|56.467|104.901|55.732|0.960231|0.963574|5/7;4/7|

Displayed medians are independent; ratios pair blocks and use their geometric mean. All seven ratios, control median/geometric comparisons, resource observations and failures are retained in [analysis](artifacts/analysis.json). The predeclared calibration checks require each case's median B/A within factor 1.03 and geometric B/A within factor 1.05; all pass. Grouped mode must meet all-case nonregression and improve every affected case plus its aggregate. Those speed checks fail. There is no minimum positive-gain threshold: the omitted-C uniform 16-worker affected case improves by about 17%, but that does not qualify the other cases or justify a posthoc shape selector. Even underfilled grouped mode, which invokes scalar work, has several small failed nonregression comparisons; none are excluded. No unchanged rerun, threshold relaxation or successful-repeat selection occurs.

## Profiles and interpretation

After all 24 paired latency processes (672 stages), four separate warm-FIFO-gated 999 Hz `cycles:u` captures execute the 128-task, 32-worker, omitted-C, ragged fixture for 4,096 epochs each. Every capture contains one PID, only its own worker/main TIDs, and zero lost records. No helper, compiler, hash or archive runs alongside the captures. Raw root-owned recordings remain local, protected and hashed.

|Mode|Samples|Scalar Poseidon period %|Across8 period %|Rayon core period %|User CPU ms|System CPU ms|Voluntary switches|
|---|---:|---:|---:|---:|---:|---:|---:|
|control-a|20031|74.321|0.000|17.695|19757.139|679.370|14342|
|control-b|20042|74.659|0.000|17.358|19774.708|687.603|15358|
|plain|16044|41.057|14.824|32.674|13722.451|1364.027|150148|
|grouped|15847|3.175|38.583|41.928|12391.967|1451.492|177520|

Grouping increases vector-executed hashes from 128/255 to 248/255, reducing scalar fallback to seven hashes. Scalar Poseidon drops from 41.057% of period in plain batching to 3.175% in grouped batching, yet the paired latency test regresses. In the separate grouped profile, user CPU decreases while system CPU, Rayon period share and voluntary switches increase versus scalar controls. The SIMD arithmetic saving is real, but work granularity and completion cost matter. Period shares are exclusive sampled user-cycle categories; they are not wall fractions, and the observations do not establish that a particular sleep policy is the cause.

The guard counts tasks before packing: 128 eligible tasks become only 16 batch jobs, which can occupy at most half of 32 workers. Chain-length sorting also puts the longest homogeneous batches at the tail. The distinct-task count and service-time distribution need analysis before another integration attempt. This is evidence against this fixed grouping rule, not evidence that Rust, SIMD or the retained arithmetic cannot help. The separate long profile happens to put grouped ahead of plain; it uses a different iteration count and instrumentation and does not override the declared paired latency result.

## Validation and limits

All 116 native core tests, two raw tests, six cubic tests, generator and portable/no-IFMA consumer checks pass. Four new tests cover disjoint writes and initialized reads, complete parallel outputs across pools of 1/4/16/32 workers, descriptor-only reordering with unchanged serialized Plan, and untagged/unsupported-width fallback. The main parallel test covers 96 fixture configurations and all three execution modes (288 complete-buffer comparisons), in addition to the unchanged parent tests. Every one of the 676 measured stages matches the original interpreter before and after capture. Before/after affinity receipts agree for all workers in all 28 processes.

Sorting and timed allocation/reallocation call counts are both zero. The largest observed process lifetime RSS is **53.45 MiB (54,736 KiB)**, including fixture construction, expected-output copies, all modes and worker stacks. It is not a mode-specific peak or a whole-prover/minimum-RAM claim. Resource medians include process CPU time, faults, context switches and sorting time in the full analysis.

Four original files contain only six diagnostic cfg-line insertions; removing them recovers every original byte. The other 166 originals, all 170 production sources and the exact accepted SDK remain unchanged. Eight-lane math and parent reference tests are byte-identical; field qualification is inherited, with no new field-suite run claimed. No native test, compile or capture failures occurred. No new SDK requests or Gnark exports occur; the proof ledger remains **6,946**.

The [latest full matrix](../2026-10-26-pairing-x-chain-sdk/README.md) retains all supplied M5 CPU/Metal comparisons. This component supplies no new proof latency. The original all-eight CPU 10× goal remains active and unmet (0/8). No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM or server-throughput qualification is claimed. Small reproducible full-proof gains remain eligible for adoption; this candidate fails because of measured regressions.

Executable: `/tmp/zopro-poseidon-parallel/native/probe-sdk`; SHA-256 `3e8108eb624b3e21d650b13ca208a5671e82d50bb390ae01fd0d471fac08af5d`.

[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [native tests](artifacts/native/core/tests.json), [measurement protocol](artifacts/measurement-plan.json), [rows](artifacts/rows.json), [affinity](artifacts/affinity.json), [raw local hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).
