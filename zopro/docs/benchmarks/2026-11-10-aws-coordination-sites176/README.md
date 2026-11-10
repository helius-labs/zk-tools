# C7a coordination instruction attribution

In the saved accepted 1×2 C7a profiles, **84 of 97 sampled threads** have more than half their sampled period in Crossbeam epoch functions at 96 workers; **none of 33** do at 32 workers. Epoch's overall period share rises from **8.77% to 53.08%**. The instruction locations identify pin publication and participant scanning as specific areas to investigate. They do not measure idle time, fence cost or recoverable proof latency. No runtime change or speedup is adopted.

This uses the four existing accepted-Zopro/pinned-Aeglos captures from the completed AWS campaign. No profile, benchmark, proof or successful decoder is rerun. Equal 251-request profile windows support comparing their descriptive totals, but absolute sampled-period coverage remains unvalidated. There is one capture per engine/count, without a confidence interval for these shares.

|Profile|Samples|Sampled threads|Epoch-majority threads|Epoch sampled period share|
|---|---:|---:|---:|---:|
|Zopro / 32 workers|13,039|33|0|8.77%|
|Zopro / 96 workers|39,821|97|84|53.08%|
|Aeglos / 32 workers|17,214|40|N/A|N/A|
|Aeglos / 96 workers|27,015|104|N/A|N/A|

The 97 Zopro TIDs include the caller. Thread counts describe sampled participants, rather than a census of all process threads. Epoch-majority is a descriptive threshold, not a latency gate. Aeglos does not use this Rust epoch machinery; N/A is not an assertion that its scheduling has no overhead.

The dominant Rayon-linked `with_handle` instance grows from 906 samples / 14.23 billion period units to 14,038 / 229.77 billion. Its two hottest locations are at offsets +0x52 and +0x61, respectively near the global-epoch load and immediately after the local-epoch locked compare/exchange. The second location loads the pin counter. This is not evidence that the counter load itself causes the cost: sampling skid, preceding instructions and coherence effects remain possible.

`Global::try_advance` grows from 255 samples / 4.10 billion period units to 6,446 / 104.51 billion. Its hottest location, +0xa7, follows a load of another participant's epoch at +0xa0. The linked code traverses participants and checks whether their pinned epochs allow advancement. `Global::collect` itself has 0 and 22 samples. These locations distinguish the scan and pin paths from simply executing deferred destructors, without proving the reason for their growth.

|Static region, assigned after inspecting disassembly|32-worker period units, billions|96-worker period units, billions|
|---|---:|---:|
|Zopro pin-publication fast paths, both sampled instances|12.989|215.850|
|Zopro participant-list/epoch scan regions|3.927|101.520|
|Aeglos pool polling region|46.845|105.272|

The region boundaries are post hoc source/disassembly interpretation, not frozen performance gates or causal time attribution. For pinning they cover offsets [0x4b,0x7b); for advancement, [0x40,0x80) and [0xa0,0xc1); for Aeglos polling, [0x78,0xa2). Complete instruction rows and period weights are retained.

Aeglos's hottest `Pool::work(unsigned int)` location is the branch at +0x7d after `pause` and a poll-loop counter test. Its linked loop checks a shared published state, with clock/yield and eventual blocking paths also present. The polling-region attribution narrows the earlier function-wide observation; it still does not establish removable wall time or make the whole function an idle-time measurement. Other Aeglos work remains outside this selected function.

The previously measured unprofiled Zopro CPU service grows from 247.072 to 771.838 ms/request as workers rise 32→96, while latency worsens 11.243892→13.502813 ms. Aeglos CPU grows 327.646→518.202 ms while latency improves 11.870012→7.902802 ms. Those measurements remain unchanged. This audit supports investigating work search and epoch participation at high worker counts; it does not establish RAM capacity as the cause. Earlier empty-victim, empty-collection, fused-pin, single-advancer, retained-deque and bounded-search experiments keep their recorded failures. A lower epoch share alone has already proved insufficient for adoption.

The next implementation needs a distinct mechanism and a progress argument. One question is whether existing work-advertisement information can avoid a full victim search when no deque work is available. The current sleep event counter increments conditionally when sleepers are announced, so it is not a complete publication generation and cannot simply be used as that gate. No such optimization is selected or implemented by this report.

Two exact executables, their ELF load mappings, four original perf files, four saved decoded streams and accepted production sources are hash-bound. The producer selects all sampled defined epoch functions, the three other Rayon functions with highest combined period, and the sampled Aeglos pool worker. It accounts for every sample and matches **34,521 selected samples to exact instruction starts**, with zero selected interior/unresolved addresses.

The independent reader reaggregates all **97,089 samples**, recomputes selection, instruction and per-thread totals, and independently intersects symbol-range constraints to recover a unique page-aligned load bias for each profile. All four match the producer's MMAP/ELF calculation. Both readers use the same saved perf-script streams and nm/objdump extraction; this is not an independent event decoder or compiler proof. General Aeglos defined-symbol coverage is limited: 13,550/18,035 own-executable samples at 32/96 lack an unambiguous nonzero-sized nm match. They remain in thread and overall totals; the selected pool function resolves fully.

Two new parser tests and one new symbol-ambiguity test pass. Nine workflow commands execute: eight pass, and the original audit fails on duplicated C++ demangled names at different addresses. The repair explicitly excludes seven nonunique names from defined-symbol matching and retains their samples as unmatched; Rust has none. The successful two symbol listings and seven Zopro disassemblies are reused with exact argv and stream hashes. Only the one previously unexecuted Aeglos disassembly is added. The original source/failure, repaired source and all ten successful read-subcommand receipts remain. A preparation-only string-anchor failure also remains documented; it executes no audit or benchmark. Helper compilation commands are outside that nine-command count.

All 176 production files remain unchanged. There are zero new proof requests, exports, latency measurements or recordings; canonical verified exports remain 46,167. The existing local Docker base image supplied the pinned Aeglos executable through an unstarted extraction container, which was removed. No cloud resource is created. This report makes no new memory-pressure, minimum-RAM, mobile, large-circuit or throughput claim.

The accepted full-proof observations below are inherited, separate from this audit. Local uses accepted Zen5 at 32 workers; AWS uses accepted Zen4 at the fastest tested count for each circuit. Supplied M5 CPU and Metal references are historical observations on different hardware. The full verified CPU AVX single-proof 10× goal remains active and unmet, **0/8 on either host**.

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

Merge54 has no supplied M5 reference. Its accepted local observation remains 162.096312 ms; AWS best Zopro is 120.757128 ms versus Aeglos 171.908504 ms. The complete AWS comparison, resource observations, quality limits and cleanup are unchanged.

Evidence: [instruction and thread analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [design](artifacts/design.md), [source, listings and command receipts](artifacts/evidence.json.gz), [publication checks](artifacts/quality.json), [checksums](SHA256SUMS), [original AWS comparison and profiles](../2026-11-10-accepted-current-c7a176/README.md), [accepted local measurements](../2026-11-09-busy-reset-adoption176/README.md).
