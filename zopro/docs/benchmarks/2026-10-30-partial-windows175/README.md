# Partial scalar-window sharing: count screen

This restricted local-pair layout is not selected for native integration. Its algebra and necessary opportunity gate pass, but generic G2 bucket-tree additions decrease only **0.174% on 1×2** and **0.033% on 2×2**, with roughly 0.95 million and 0.91 million row-pair comparisons. No prover source or accepted timing changes.

## Method and scope

Independently implement a disjoint-pair restriction of digit reuse described in [Hrabar’s 2026 Waterloo thesis](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403), section 5.2. After the current whole-scalar grouping decision, split each existing point-range half into consecutive tiles of at most 64 rows. Examine every original-row pair across 22 signed Phi2/Booth columns. Prefer more shared nonzero digits, then lower indices; each original row participates at most once. Remove its common digits from both residual rows and feed them once with the derived sum Pi+Pj. There is no recursive, negative, cross-tile or cross-part pairing. This is not a reproduction of the thesis’s full CRLC algorithm or published measurements.

The two saved fixture populations already have independent complete-MSM oracle receipts. Their hashes, canonical fields, on-curve inputs and scalar representations are rechecked. The accepted grouping policy declines 1×2 and admits 2×2 (52,274 rows become 28,852). Grouped point cancellation is not determined by scalar metadata and is not modeled. The admitted grouping case assumes its existing reservation is available; this is not a pressure test.

## Counts

| Shape | Post-group rows | Selected / genuine partial pairs | Generic bucket-tree additions before | Net saved after derived sums | Net reduction | Row-pair comparisons | Proposed extra payload KiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 30089 | 215 / 178 | 531627 | 926 | 0.174182% | 947536 | 146.930 |
| 2x2 | 28852 | 180 / 180 | 551405 | 181 | 0.032825% | 907850 | 137.312 |

Primary savings split into 747 additions from 37 whole-duplicate pairs and 179 from 178 genuine partial pairs. The 2×2 result has 181 additions saved from 180 genuine partial pairs and no remaining whole-duplicate pairs. The layout examines 20,845,792 and 19,972,700 signed column pairs. Occupied magnitude buckets and indexed-top signed bins stay occupied; this supports the generic noncancelling arrival/tree-addition proxy. One derived point addition is charged per selected pair. Actual curve exceptions, inversions, borrowed queues, tree topology, phi cost, complete-query finishing and wall time are not modeled. These percentages are not predicted proof-time gains or rigorous latency ceilings.

The count gate has no minimum percentage threshold and **passes**. Choosing not to build this layout is a separate engineering decision based on its small work reduction and search volume. Neither a native slowdown nor the impossibility of other clustering designs is established. No timing sweep or successful test repetition is performed.

## Correctness and memory

Four standalone native-model tests pass. They cover 8,195 deterministic/edge canonical scalars plus signed 126-bit component boundaries; full/partial/negative/zero digits; inactive identities; empty, odd, tail and partition-boundary populations; and deterministic disjoint selection. Each selected rewrite conserves every original signed coefficient exactly. Both fixture populations reconstruct their Phi components and full Fr scalar exactly. Two independent Fr-token populations check every output bucket/sign bin and the weighted full sum against the original scalar dot product. There are 82,432 checked bins per population per token stream. These field-token and symbolic checks do not qualify a new real-curve kernel. Unchanged production field/core suites are inherited, not rerun.

A hypothetical runtime layout needs 4N residual-mask bytes plus 12M pair-record bytes and 128M derived-point bytes: 150,456  / 140,608 requested bytes for these two populations. It borrows original key points. This excludes Vec headers/capacity rounding, tile and normalization scratch, allocator overhead, stacks and existing query workspace. No production layout, admission/release behavior, minimum RAM, mobile, huge-key or concurrent-proof feasibility is qualified. The count helper’s allocations are analysis storage, not prover RSS. Masked residuals are kept as explicit digits in the model; ordinary scalar recoding is not assumed to preserve them.

## Related work and next selection

A wider history search after the count located older Hrabar reviews and negative global-window search results. This report adds a restricted current-fixture census; it does not discover the thesis anew or reopen the old serialized search. Sparse whole-scalar grouping with singleton references and repeated-only point sums was also already implemented and measured, including a borrowed revision; its original full gate failed. Neither candidate should be rerun unchanged. [History audit](source/history-audit.md).

Return to retained full threaded caller/SDK profiles and select a distinct cost-removal mechanism before building another reuse or arithmetic kernel. These counts do not explain the AWS/Aeglos gap.

## Accepted proof results, unchanged

The original all-eight 10× M5 CPU objective remains **0/8**. CPU/Metal columns use supplied M5 reference timings, not fresh Apple measurements. Local accepted results are from the saved 32-worker campaign; merge54 has no supplied M5 reference.

| Shape | Local ms | Peak RSS MiB | M5 CPU ms / speedup | M5 Metal ms / speedup | CPU10× target ms |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.022165 | 134.00 | 37 / 4.61× | 24 / 2.99× | 3.7 |
| 2x2 | 11.335969 | 275.00 | 59 / 5.20× | 30 / 2.65× | 5.9 |
| 4x2 | 15.945544 | 315.00 | 109 / 6.84× | 48 / 3.01× | 10.9 |
| 8x2 | 24.090485 | 512.50 | 189 / 7.85× | 74 / 3.07× | 18.9 |
| 12x2 | 33.844855 | 707.25 | 284 / 8.39× | 101 / 2.98× | 28.4 |
| 16x2 | 39.931557 | 826.25 | 335 / 8.39× | 124 / 3.11× | 33.5 |
| 24x2 | 66.233309 | 1328.75 | 528 / 7.97× | 176 / 2.66× | 52.8 |
| merge-24x1 | 65.273224 | 1175.50 | 507 / 7.77× | 175 / 2.68× | 50.7 |
| merge-54x1 | 162.250464 | 1831.87 | — | — | — |

Latest retained C7a 1×2: Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96 workers, best tested settings in that campaign. This is a configuration-specific comparison. All nine local/AWS rows, engine worker counts, RSS and original eight reference ratios are retained exactly in [accepted rows](accepted-rows.json); see the [complete snapshot](../2026-10-30-current-bench-snapshot175/README.md). No new AWS, CPU profile, performance capture, witness solve, SDK request or independent proof export occurs. Distinct verified-export ledger remains 25,231.

## Receipts

A source-pinned Rust xtask workflow completes compile-tests, four model tests, compile-probe and count-two-fixtures once, with 427 input pins. All 175 production sources match the accepted manifest; the reused library snapshot recovers those same sources after removing only two diagnostic export suffixes. Copied grouping, Phi-split and Booth functions match their qualified source checks. Every command exit and stdout/stderr hash is checked.

Initial preparation incorrectly read a binary manifest member as UTF-8 and stopped before a workflow existed. The original helper/log is retained; the correction hashes bytes. An initial invocation selected xtask’s perf subcommand instead of workflow and was rejected during configuration parsing, before any step/output directory. The unchanged plan then completed via workflow. No successful native command or count was repeated.

[Machine-readable counts](data.json), [declared design](source/design.md), [model source](source/probe.rs), [setup source](source/setup.rs), [publisher](source/publish.rs), [source hashes](source-hashes.json), [exact workflow receipts](workflow-receipts.tar.gz), [checksums](SHA256SUMS). Raw fixture points/scalars, unblinded query results, libraries and executables are excluded.
