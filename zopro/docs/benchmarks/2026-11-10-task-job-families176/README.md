# Job families in the C7a interval traces

One G2 scalar-decomposition iterator accounts for **99.848% of additional MSM job publications** between 32 and 96 workers in the retained instrumented graphs. These counts are not established for the uninstrumented prover, and the target observer overhead qualification still fails. No speedup or production change is claimed.

This is offline analysis of the [completed C7a diagnostic](../2026-11-10-task-interval-c7a176/README.md). It adds no SDK calls, proof exports, profiling or AWS resources. All 18 main ON processes and all 54 retained graphs are included.

Each process has 37 distinct published-job function pointers. Exactly one common page-aligned translation maps them all to defined text symbols in the measured PIE executable, SHA-256 `7652efb7367834f4c81bf86c4b38e49e5a0080ba9518a3502be9c94636ae50eb`. The 33 relative addresses with publications in phases 2–6 each have one symbol alias; native symbols retain concrete iterator and closure types. All 270 per-graph phase publication totals match the earlier independent phase reader.

|MSM publications across 27 graphs per worker count|32 workers|96 workers|Increase|
|:---|---:|---:|---:|
|All job families|69,645|450,194|380,549|
|G2 `hb_raw` scalar decomposition|39,172|419,144|379,972|
|Other families combined|30,473|31,050|577|

The decomposition job is symbol `0x49b000`, whose concrete iterator is `IterMutProducer<[u64;4]>` with `g2_phi::hb_raw::{closure#0}`. Source contains the unrestricted [in-place phi_split iterator](../../../crates/zopro-core/src/msm_affine/g2_phi.rs#L194). Its publication median is 417 per graph at 32 workers and 15,304 at 96. Totals include all 27 graphs, including large counts in some 32-worker graphs; medians and totals are different summaries. Other MSM families show no comparable increase in these recordings.

Scalar preparation has a separate increase: the two unrestricted A/B gather closures dominate its publications. Their per-family medians are 3,928/4,487 at 32 workers and 13,098/11,163 at 96. The already bounded canonical conversion remains 126 → 127. These medians are not additive.

The history review found the exact `with_min_len(128)` change on this loop in the [October 27 local study](../2026-10-27-g2-scalar-grain/README.md). It showed no confirmed full-proof gain: primary32 paired candidate/accepted 0.997331, interval [0.995288, 1.001898], with failed overall qualification and no regression rows. It measured local 16/32 workers and created no AWS resources. The [October 29 sequential decomposition study](../2026-10-29-serial-phi175/README.md) also failed its full-SDK performance gate. Those outcomes remain in force; neither idea is new here.

The next discriminating test is the existing minimum-grain idea on **C7a with 96 physical cores**, where these traces identify its job family as a scaling lead. Repeating the old local timing campaign would not answer that question. A current-source native candidate, accepted control, fixed 32/96-core target comparison, fresh proof verification and RAM/CPU checks are required. This is a new target condition, not a claim that a prior local failure has passed. Full all-shape qualification remains necessary before adoption. The A/B gather hypothesis is secondary to testing this MSM family.

The observer's 96-worker ON/OFF ratio is 1.095031, above the declared 1.05 ceiling. Adaptive splitting may respond to observation overhead, so these counts cannot establish the cause of the uninstrumented latency loss or predict a speedup. Job counts are not durations; nested execution spans are not exclusive CPU. The original all-eight verified 10× M5 CPU objective remains unmet, with [CPU/Metal context](../2026-11-10-accepted-current-c7a176/README.md). The verified-proof ledger remains 50,208.

The offline decoder workflow passes once with 26 input pins and retained command/exit/output hashes. An initial direct preparation compile typo was corrected before that run; direct compilation is outside the workflow receipt count. Publication review distinguishes 37 addresses per process from the 33 addresses classified within phases 2–6. The publisher rechecks pins, arithmetic totals and receipts without rerunning decoding or proofs. Symbol compression and archived receipt round trips match exact bytes.

[Analysis](artifacts/analysis.json), [reader source](artifacts/decode.rs), [input pins](artifacts/pins.json), [symbols](artifacts/symbols.txt.gz), [ELF headers](artifacts/program-headers.txt), [workflow](artifacts/workflow.tar.gz), [review](artifacts/review.json), [checksums](SHA256SUMS). Raw graphs are in the preceding C7a report; executables remain local with pinned identities.
