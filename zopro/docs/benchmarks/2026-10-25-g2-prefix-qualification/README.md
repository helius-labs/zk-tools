# G2 prefix-record compatibility and memory qualification

The exact native libraries from the passed [SDK comparison](../2026-10-25-g2-prefix-sdk/README.md) pass compatibility, concurrency, stack and resource qualification. No native prover code is rebuilt. The qualification adds197 internally verified SDK calls and four large proofs, with75 distinct Gnark-verified exports and150 proof/public files. This phase is not a new latency comparison.

Portable x86-64 builds disable IFMA and assembly and pass11 field,89 core, two raw-format and six cubic tests. Eight real Euler/raw OFF/ON processes verify32 SDK calls. The G2 native trace is absent on the portable path. Native compatibility adds raw/preparation-denied and single-worker proofs; primary and merge54 also pass with512KiB and1MiB worker stacks in both modes. Across these functional checks:96 SDK calls,24 independently verified exports. This is not physical ARM/Wasm/phone qualification.

Four32-worker concurrent-caller cases run four callers and two rounds on primary and merge54 with both modes. Mutable SDKs share an immutable mapped key and existing global pool. All32 fresh proofs independently verify, and thread receipts check the pool plus actual callers.

Nine warm pressure cases compare accepted, identical OFF and ON at32 workers with default/retained allocator policies and retained96-worker controls. Each warms at512MiB, then explicitly touches32MiB of controller ballast and lowers the group to128MiB soft/192MiB hard with swap0 before two further proofs. All cases pass without max/OOM/kill events or timeout. They cover45 SDK calls and nine exports.

| Warm case | Variant | Whole-cgroup peak MiB, including ballast |
| --- | --- | ---: |
| default/w32 | accepted | 109.066 |
| default/w32 | off | 111.590 |
| default/w32 | on | 108.031 |
| retained/w32 | accepted | 112.480 |
| retained/w32 | off | 113.426 |
| retained/w32 | on | 110.301 |
| retained/w96 | accepted | 120.332 |
| retained/w96 | off | 123.996 |
| retained/w96 | on | 121.656 |

Six additional cold-start cases begin entirely inside a192MiB hard/soft limit with swap0. An owned key copy charges its file pages to the group. Accepted/OFF/ON pass under both allocator policies:24 SDK calls and six exports. These finite limits establish tested feasibility, not minimum RAM or an architecture-wide cap.

| Large circuit | Cold verified s | Warm verified s | Peak process RSS GiB | Hard cap GiB | Hard-limit reclaim events |
| --- | ---: | ---: | ---: | ---: | ---: |
| 250 | 3.299516 | 2.993476 | 11.215588 |95|0|
| 4999 | 160.111002 | 154.724429 | 94.693420 |95|114676|

Both large images have their hashes checked, followed by fsync/POSIX_FADV_DONTNEED and a mincore check showing zero resident image pages before launch. Each generates one cold and one warm proof from different fixture witnesses, with public inputs bound to those witnesses and both proofs independently verified by Gnark. Zero swap and zero OOM/kill events are recorded. The4999 hard-limit reclaim events show active pressure; the95GiB cap is a successful tested configuration, not a measured minimum. These observations are not a matched speed comparison with earlier Zopro or another engine.

The existing heap forecast remains conservative. Old G2 records contain nine vectors; the new path contains two. For one group, minimum new reserve capacity4 requires eight vectors, still below nine. For two or more groups, reserve growth is bounded by max(4,2g), so the new payload stays below the old9g bound. Tail buffers are unchanged, and the path choice is fixed at scratch construction, preventing both group allocations from retaining capacity in normal SDK use. Added vector/flag metadata is on the stack. Generated code and stack-probing prologues are retained; literal stack adjustments are not treated as whole-call stack bounds. Actual small-stack proofs passed. No new unsafe code or key tables are introduced.

The initial portable build harness expected85 core tests; the actual complete suite passed89 with zero failures. The assertion stopped after those successful tests. The repair reused all four completed native links and both passed portable suites, then continued at the unrun prover tests. A first functional verifier used the parent output directory instead of b0-m4 and found no proofs. Correcting that path independently verified the already-produced proof; no completed SDK call was repeated. Failed scripts/logs and repair receipts are retained. Neither incident was a proving failure or a latency reroll.

All167 accepted production and168 private source pins are unchanged during qualification. Native/portable libraries, binaries, consumer sources and proof hashes are checked. Owned local units are stopped; no AWS resource, source adoption, commit or push is part of this qualification. The original all-eight10x M5 CPU objective remains unmet.

[Build](build-complete.json), [functional cases](functional-complete.json), [concurrency and warm pressure](complete.json), [cold checks](cold-complete.json), [large checks](large-complete.json), [code review](codegen-review.json), [proof inventory](combined-proof-inventory.json), [cleanup](cleanup.json).
