# Poseidon independent-task census

Existing ordinary solver epochs expose substantial width-three Poseidon work that could fill eight SIMD lanes in larger circuits. **Primary 1×2 has no full eight-lane group in this model**, so this mechanism alone offers no demonstrated path to its 3.7 ms target. No vector kernel or performance gain is claimed. Production is unchanged.

The private diagnostic reads the prepared schedule after existing exact Poseidon recognition. The two inserted module/call lines can be removed to recover both original files byte for byte; the other 168 originals remain unchanged. It changes no arithmetic, writes, scheduling, dependency, admission, key image or execution plan. Census state uses fixed 14-entry arrays and borrowed iterators. Source tagging guards remain active for oversized plans and unsupported tasks.

Each ordinary epoch excludes refined containers. Each refined wave is analyzed separately. Within one such independent set, depth d selects the d-th recognized hash of each task after its scalar prefix, then groups equal widths. Coverage is the sum of full four/eight-lane groups at those abstract frontiers. This preserves hash order within each task. It assumes regrouping at each depth and sufficient progress through scalar prefixes; it does not measure simultaneous runtime readiness, fixed-chunk occupancy, speedup, optimal batching or a wall-time fraction.

|Shape|Recognized t=3 hashes|Potential full4 hashes|Potential full8 hashes|Full8 coverage|Peak abstract t=3 frontier|Max chain, any width|Refined t=3 hashes|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|52|4|0|0.00%|6|2|0|
|2x2|99|80|8|8.08%|8|2|0|
|4x2|194|188|152|78.35%|12|2|0|
|8x2|385|376|368|95.58%|20|2|0|
|12x2|575|556|520|90.43%|28|2|0|
|16x2|769|748|736|95.71%|36|2|0|
|24x2|1151|1116|1104|95.92%|52|2|0|
|merge-24x1|1093|1076|1072|98.08%|49|2|0|
|merge-54x1|2903|2848|2728|93.97%|110|3|0|

Counts are per prepared plan, not timings. One plan is observed per process. Identical 16/32-worker totals: **true**. Every width's total matches the existing independent recognition counters. All width-three hashes in these cases are in ordinary epochs; refined waves contain none. This supports investigating a bounded ordinary-epoch implementation before modifying refined execution. Other widths and tails remain in the raw records.

The four new core tests distinguish eight hashes in one dependent chain from eight independent tasks, verify mixed widths and tails, exclude refined containers, preserve wave separation, and reject malformed tags while honoring the operation-count limit. All 113 core tests, two raw tests, six cubic tests, generator and portable/no-IFMA consumer checks pass. Field code and its prior qualification are unchanged; no new field-suite run is claimed.

All nine supplied circuits are exercised at 16 and 32 workers, with CPUs 0–15 and 0–31 respectively. Each of 18 standard SDK processes runs three primes and one additional request: **72 internally verified SDK calls and 18 distinct independently Gnark-verified exports**, with 36 proof/public files. Every proof is 164 bytes, has the pinned public fixture and is fresh against 6,928 earlier identities; next ledger **6,946**. All source/library/binary/input/tool pins and 170 production hashes are checked after capture. No owned build, helper, hash or archive overlaps capture. No successful capture is repeated or excluded. The SDK is `/tmp/zopro-poseidon-frontier/native/on-sdk`, SHA256 `b61206d37d7e1dbeea6bdcb241caa7d9f0172c85722f93ee0ef263e0de3655f0`.

Next, a component must include the original wire, slot and A/B/C writes, scalar-prefix work and dispatch costs. It must test actual task arrangement: abstract regrouping coverage can exceed what a fixed batch implementation achieves. Reusing existing epoch task storage and bounded scratch avoids another plan or a large table. Concurrent scalar tasks already use CPU threads, so packed throughput can reduce task parallelism. Those tradeoffs require measurement. An implementation following this model leaves 1×2 as a negative control; its full-proof nonregression remains required, while improvement must be demonstrated in the affected scope. The model does not prove the absence of eight-lane work under every legal interleaving. No adoption follows from this census.

The [latest borrowed-G1 report](../2026-10-26-borrowed-g1/README.md) retains its experimental 8.191548 ms primary result and failed 16-worker check. The [latest full matrix](../2026-10-26-pairing-x-chain-sdk/README.md) retains all supplied M5 CPU and Metal comparisons. This census supplies no new latency comparison. The original all-eight CPU 10× goal remains active and unmet. No new AWS/Aeglos, phone, TEE, giant-circuit, minimum-RAM, swap or throughput result is claimed.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [census source](artifacts/candidate/crates/zopro-core/src/solver/sparse_poseidon/census.rs), [source audit](artifacts/source-audit.json), [capture protocol](artifacts/capture-plan.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
