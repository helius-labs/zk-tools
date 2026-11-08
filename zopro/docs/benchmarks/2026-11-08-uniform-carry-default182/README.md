# Combined arithmetic: default-source preparation and saved16-worker audit

The combined arithmetic now has a clean default-source build, reproducible field generation and **297 passing test executions** across native and generic configurations. All seven inspected native arithmetic functions retain the measured private candidate's instruction/IFMA counts. This is preparation for adoption, **not a new latency measurement or source promotion**. The prior full performance gate remains false because of primary16 uncertainty; the original all-eight CPU10×goal remains unmet,0/8.

## Default-source change

The183-file snapshot starts from the exact182-file measured parent and adds one permanent field-test child. It removes the two experiment predicates from the existing ISA guards. The remaining feature, architecture, AVX and no_ifma predicates remain, as do ordinary fallback expressions. This enables the arithmetic by default under the existing accepted native build flags; it does not remove the project's other existing build settings. A truth-table audit checks3,456 cases across all27 edited guard occurrences. The eight-lane generated arithmetic and four-lane source arithmetic retain their parent bytes, apart from the generated module footer; the uncarried constructor and subtraction bodies remain byte-exact.

Caller fixtures are now private cfg(test) modules. Library diagnostic exports are removed, and raw field-inspection methods are test-only. The new permanent Arkworks oracle checks65,728 outputs covering canonical-X sums, lazy minuends, both fields and radices, normalized raw bounds and logical field export. Existing coupled point, sign-mask, tail, reuse,512KiB-stack and1/4/16/32-worker tests run on the new default artifact. G1's1600-byte/alignment64 record remains unchanged.

The existing field generator now emits the eight-lane child-module footer and passes its --check command with pinned Rust1.98.1 rustfmt. No new generator implementation or Python helper is introduced. No new lookup table, key copy, retained solver plan, heap buffer, production pool, runtime option or architecture-wide RAM cap is added. New whole-process memory or pressure benefits are not claimed.

## Validation and actual artifacts

|Suite|Native|Generic noASM/noIFMA|
|---|---:|---:|
|Field library|39|12|
|Core library|129|101|
|Prover library|2|2|
|End-to-end cubic prover|6|6|
|Total test executions|176|121|

All eight suites are unfiltered and pass without exclusions. The cubic cases cover named assignment/proving/verification, wrong inputs, raw/legacy/in-place image loading and format behavior. This stage adds no independently Gnark-verified export or full SDK request; unit/integration proof calls are not counted as those exports. The canonical unique-export inventory remains **35,863**, and no old proof, test artifact, capture, decoder or timing analysis is replayed for bookkeeping.

Fifteen new workflow commands complete: generator consistency, six library suites, two coherent eight-library SDK build/link pairs, two cubic integration suites, one full native disassembly and one symbol listing. Both bundles'16libraries are frozen by content hash; original library paths still match after integration tests. Final links use pinned Rust1.98.1, native or generic target flags, fat LTO and one codegen unit. The two new experiment cfg flags are absent. Generic builds use no_asm/no_ifma and the original fallback branches; this is native x86 generic qualification, not execution on a phone, ARM or TEE.

The new native SDK has no caller-fixture or raw diagnostic symbols. Its six G1 bodies retain800IFMA sites each and the borrowed G2 body retains1450, with every selected whole-function instruction count unchanged from private ON. The hot borrowed signed G1/G2 bodies retain4437/8489instruction sites. Equal static counts and arithmetic-body identity do not prove equal layout, dynamic costs or latency, so the private candidate's7.869400ms is not assigned to this default SDK.

The independent reader rechecks source/binary/frozen-library pins, all15receipts and stream hashes, raw unfiltered test summaries and names, unchanged production176 sources, arithmetic-body identity, seven code-count comparisons and the saved-data audit below. It inherits the detailed cfg parser and crypto implementations rather than claiming another independent crypto verifier. An initial reader counted only single-line test results and missed three tests whose diagnostics split their output; a repaired reader binds the successful summary and every unique test-start name. The original reader and repair note remain. No underlying test/build/generator/decoder/arithmetic/analysis or threshold changes or repeats.

## What the saved16-worker data now tells us

All45previous processes and6,795timed request samples remain. Every first16-worker process follows a32-worker latency process; every middle/last process follows a16-worker process. Thus the earlier position association is confounded with a worker-count/affinity-context transition. This is a recorded sequence, not proof that the transition caused the slowdown.

|Arm,15processes each|Correlation: wall mean vs involuntary switches/request|Correlation: wall mean vs minor faults/request|
|---|---:|---:|
|Accepted|0.987742|0.108912|
|OFF|0.996662|-0.105647|
|Candidate|0.998400|0.269818|

Slow processes also have lower CPU occupancy and less user CPU time. These observations support investigating scheduling and yielding; they do not identify an arithmetic, allocator, kernel, interference, governor or cache cause. Per-process aggregate rusage does not locate individual off-CPU intervals. The separate profiles/counter panels are not traces of these latency processes.

Rayon1.13's local source calls thread::yield_now while idle. The [Linux6.8 reference scheduler](https://github.com/torvalds/linux/blob/v6.8/kernel/sched/core.c#L6254) defaults running-task switches to nivcsw; its [yield syscall calls schedule](https://github.com/torvalds/linux/blob/v6.8/kernel/sched/core.c#L8018). Consequently an explicitly yielding task can contribute to this counter. That is a source-based explanation of counter ambiguity, not an audit of the exact Ubuntu kernel binary or evidence that all counted switches were yields. A high involuntary-switch count alone does not establish external host interference.

Five contiguous temporal bins cover every recorded request without a cutoff. The candidate's first-position median bin means are9.658/11.075/11.755/9.642/9.837ms: the slow work is not uniformly confined to the first few requests. These samples do not justify simply dropping initial observations or adding a few primes to rescue the gate. Full trajectories and counters remain in the audit artifact; no alternate filtered gain estimate or changed selection is produced.

The prior full gate therefore stays false. Its real primary32 and original-eight aggregate gains are retained, alongside the unresolved16-worker calibration/nonregression risk. No scheduling-policy change, unchanged timing reroll or adoption decision follows from this descriptive audit.

## CPU and Metal comparisons retained from the measured parent

The default artifact is unmeasured. This table preserves the private parent's actual complete-SDK observations and supplied M5 references on different hardware; primary32 comes from the unchanged pilot and the other shapes from the full-shape study. These are not timings of this default build.

|Shape|Private-parent local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.869400|37 / 4.702×|24 / 3.050×|3.7|
|2x2|11.018191|59 / 5.355×|30 / 2.723×|5.9|
|4x2|15.922784|109 / 6.846×|48 / 3.015×|10.9|
|8x2|24.003699|189 / 7.874×|74 / 3.083×|18.9|
|12x2|33.729083|284 / 8.420×|101 / 2.994×|28.4|
|16x2|39.706864|335 / 8.437×|124 / 3.123×|33.5|
|24x2|65.792319|528 / 8.025×|176 / 2.675×|52.8|
|merge-24x1|64.546908|507 / 7.855×|175 / 2.711×|50.7|


Private-parent merge54 remains161.197772ms without a supplied M5 reference. Accepted production176 and SDK6d83 remain unchanged. The all-eight CPU10×goal is not met by any supplied shape; the primary target is3.7ms. No new AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM, service-concurrency or latency result is claimed.

Next: qualify the default SDKs' complete request, fallback and resource behavior, then assess this actual artifact's performance under a prospective protocol. Any later adoption decision must preserve the original failed gate and state the16-worker limitation explicitly. CPU scheduling causality requires suitable tracing; saved correlations do not settle it. This roughly1% parent gain is useful incremental evidence, well short of the original10×goal.

[Source audit](artifacts/source-audit.json), [qualification](artifacts/qualification.json), [codegen](artifacts/codegen.json), [saved-data audit](artifacts/tail-audit.json), [review](artifacts/review.json), [reader repair](artifacts/review-repair.md), [publication](artifacts/publication.json), [evidence archive](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
