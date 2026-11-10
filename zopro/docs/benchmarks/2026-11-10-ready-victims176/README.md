# Advertised private-deque victim discovery

A private Rayon prototype replaces a scan of every worker with a bitmap of workers advertising private deque work. **114 Rayon tests pass, with two ignored; all three bounded publication models pass under a privately repaired checker.** This qualifies a candidate for further integration. It establishes no latency or memory-peak improvement, and production remains unchanged.

The preceding AWS audit found increasing sampled period around epoch pinning and participant scanning at 96 workers. This mechanism changes how potential victims are discovered. It is distinct from the earlier per-victim empty check: a search loads bitmap words and visits observed set bits, while owners pay additional shared atomic updates. Shared-word contention, extra owner checks and changed sleep frequency can outweigh the saved searches. Earlier negative scheduler results remain negative.

Each existing registry allocates one `AtomicUsize` per `ceil(workers / usize::BITS)`, plus the bitmap object; each worker adds a `Cell<bool>` to avoid redundant updates. At 96 workers on this host, the bitmap words contain 16 bytes. That is source payload, excluding object layout, allocator overhead and alignment; it is not measured RSS or a minimum-RAM requirement. There is no fixed worker-count limit, new pool, key table or solver plan.

Only a private deque's owner changes its bit. It publishes after `Worker::push` and before the unchanged internal-job notification. After a private pop, it clears only if its own deque is empty. Thieves never clear another worker's bit. Per-word release RMWs preserve other bits; search iterators acquire-load words, mask padding and retain cyclic order among observed candidates. A word containing the start index may be read twice around wraparound. Concurrent new work can be discovered in a later search.

The external injector, broadcast queues, Crossbeam queue protection, latch handling, retries and sleep notifications remain unchanged. Owner-private and broadcast stealers are different fields and are treated separately. Rayon explicitly permits a local push to miss waking peers under some races, relying on the owner eventually popping its job. This prototype does not claim a stronger peer-wakeup guarantee. The conditional sleep event counter is not used as a complete publication generation.

|Qualification|Observed result|Scope|
|---|---|---|
|Changed Rayon library and integration suite|114 pass, 2 ignored, eight test suites|Includes four new bitmap/pool tests|
|Cyclic bitmap reference cases|12,192 start/pattern cases|15 worker counts through 1,001; word boundaries and padding|
|New pool integration|11,711 leaf executions|1/2/4/16/32/65/96 workers, nested joins, FIFO, broadcasts, external injection, two registries, unwind/reuse|
|Original three bounded models with private checker|3 pass|Preemption bound 2; branch limit 10,000|
|Atomic store/swap control with private checker|1 pass|The same control fails under stock Loom 0.7.2|
|Private checker's atomic test suite|10 pass, 1 ignored|Includes the imported atomicity regression tests|
|Bitmap-disabled Rayon compilation|Pass|Experimental module and fields are cfg-excluded|

Ignored tests are not passes. These host tests do not qualify ARM, a phone, a complete prover consumer or concurrent-proof throughput. The bitmap source is identical under standard and Loom atomics. The bounded models cover separate-owner bit preservation, job conservation using an abstract one-slot queue and visibility after an observed sleepy job-event. They are not a formal Crossbeam deque or complete Rayon sleep/latch model, nor an unbounded liveness proof.

The first stock-Loom run passes two models and fails the abstract queue's conservation assertion. Repeating only that failed case reproduces it. An event-log diagnostic then shows a lost value around a store/swap sequence. A separate minimal atomic control, containing no bitmap code, also fails. This is consistent with the reported store/RMW atomicity limitation in [Loom issue #254](https://github.com/tokio-rs/loom/issues/254).

For qualification only, a private copy of Loom 0.7.2 imports the exact two-file change from [unmerged PR #391, commit c673a8d](https://github.com/ZobyTwo/loom/commit/c673a8d800f3a8163586e8a6177ddafd01f2e665). The patch adds the missing ordering constraint and regression cases. Both modified files reverse byte-exactly to cached upstream, with 75 other files unchanged, excluding resolved lock metadata. The unchanged control and three original models pass with that checker. No prover dependency or runtime algorithm is changed to obtain these model results. The patch is not represented as merged upstream or as a formally validated checker; its status and the bounded models limit this evidence.

The original native Rayon qualification is not replayed after the checker-only change. Reexecuting the models is necessary because the checker changes. All original failed outcomes remain in the evidence; they are not relabeled as passes. Fifteen workflow commands execute: eleven pass and four fail, including the deliberate stock-checker control and failed-case diagnostics. One successful reverse-patch subcommand belongs to the independent review. Preparation/helper compiles are separate from that count.

The diagnostic crate accidentally reused the original model crate's Cargo package/target identifier and overwrote its test executable. Original sources, receipts and binary hash remain, but that original executable is unavailable. The diagnostic executable is retained, and all later checker/control/model targets are separate. This limitation is explicit in the review rather than silently accepting a changed binary pin. The original successful model build is not replayed to reconstruct the missing executable.

Before any runtime test, source review removes one extra parenthesis in the new pool test. Offline lockfile preparation excludes the copied lockfile from its mutable-output input pins, then freezes resolved locks before qualification. One review-helper compile also needs a JSON expression syntax fix before the reader runs. Earlier sources and notes are retained; none of these preparation repairs changes runtime code or repeats successful native qualification.

Independent closeout verifies 671 current pin occurrences, reconstructs the one historical pre-fix test input, identifies the unavailable original model executable, checks all fifteen command receipts and stream hashes, and reverses both changed Rayon parents to their exact upstream bytes. Forty other upstream files remain exact, excluding resolved lock metadata. Model sources and the bitmap body remain byte-identical across the checker change. All 176 accepted production files and the canonical 46,167-proof ledger remain unchanged.

The next step is coherent full-prover integration and prospective complete-proof benchmarking against accepted and fresh bitmap-disabled controls. That must include calibration, all eight supplied shapes plus 54→1, primary at 16/32 workers, CPU and memory observations, switches/faults, automatic profiles and fresh independent Gnark verification before adoption. No clock, gate, worker subset or acceptance decision is chosen from a favorable result here: there are **zero new proof requests, exports, prover latency measurements or CPU recordings**.

The following accepted observations are inherited and separate. Local uses the accepted Zen5 build at 32 workers; AWS uses accepted Zen4 at each circuit's fastest tested count. Supplied M5 CPU and Metal references are historical observations on different hardware. The original verified CPU AVX single-proof 10× objective remains active and unmet: **0/8 on either host**.

|Circuit|Local accepted ms|AWS accepted ms|M5 CPU ms|M5 Metal ms|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|
|1×2|7.772288|11.243892|37|24|3.7|
|2×2|10.929386|15.263659|59|30|5.9|
|4×2|15.731643|19.121785|109|48|10.9|
|8×2|23.965774|25.894916|189|74|18.9|
|12×2|33.742953|33.778290|284|101|28.4|
|16×2|39.704242|37.977699|335|124|33.5|
|24×2|66.034077|53.297583|528|176|52.8|
|24→1|65.387417|56.324672|507|175|50.7|

Merge54 has no supplied M5 reference. Accepted local remains 162.096312 ms; AWS best Zopro is 120.757128 ms versus Aeglos 171.908504 ms. AWS best 1×2 remains Zopro 11.243892 ms at 32 workers versus Aeglos 7.902802 ms at 96. No new AWS resource or comparison is added here.

Evidence: [bitmap implementation](artifacts/ready.rs), [source edits](artifacts/source-edits.json), [independent review](artifacts/review.json), [source audit](artifacts/source-audit.json), [design](artifacts/design.md), [checker provenance](artifacts/checker-provenance.json), [authored sources, frozen plans and receipts](artifacts/evidence.json.gz), [publication checks](artifacts/quality.json), [checksums](SHA256SUMS), [preceding AWS instruction audit](../2026-11-10-aws-coordination-sites176/README.md), [accepted AWS comparison](../2026-11-10-accepted-current-c7a176/README.md), [accepted local measurements](../2026-11-09-busy-reset-adoption176/README.md). Raw private solver/key/witness/proof data, executables and library blobs are excluded. Complete third-party trees and the imported checker patch remain local, with identities and upstream links.
