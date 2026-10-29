# Current streaming-kernel operation and fetch profiles

The unchanged accepted SDK (175 source files) completed three operation and three instruction-fetch profiles on the local Ryzen 9950X. This adds completion/retirement and instruction-class observations to the previous load-only IBS studies. No optimization is adopted.

## Findings

Streaming arithmetic has substantial tag-to-completion delay on IFMA instructions. Demand loads almost never miss L2, and completed instruction fetches usually have short latency. This makes shortening field-arithmetic dependency chains the next investigation. This is a hypothesis from the combined service trace and hardware observations, not a proven causal bottleneck or forecast speedup.

The preceding [service trace](../2026-10-29-service-trace175/README.md) put both K/H and G2 on the finishing path. G1feed below combines all `stream_g1_impl` instantiations, including A and K/H; IBS alone does not separate their query contexts. G2feed is the current borrowed `apply<FAST=true>` kernel.

|Kernel|Median op samples/process|Tag→completion mean, cycles|Completion→retirement mean, cycles|L1 misses/load ops|L2 misses/load ops|RAM samples, all3blocks|Valid demand-miss mean, cycles|
|---|---:|---:|---:|---:|---:|---:|---:|
|G1feed|36072|46.87|23.78|18.245%|0.128%|0|13.74|
|G2feed|34940|37.07|23.95|8.430%|0.402%|1|15.89|

Load denominators use precise raw load operations; software-prefetch counts are zero in both feed groups. Miss latency includes only demand loads with valid DC-miss data and excludes uncached accesses. These fractions describe samples, not shares of wall time or memory bandwidth.

|Kernel|Instruction|Median samples/process|Tag→completion mean, cycles|Completion→retirement mean, cycles|
|---|---|---:|---:|---:|
|G1feed|`vpmadd52luq`|6284|60.88|10.91|
|G1feed|`vpmadd52huq`|6341|58.76|13.07|
|G1feed|`vpaddq`|3786|60.77|11.74|
|G1feed|`vpsrlq`|2191|61.73|10.87|
|G1feed|`vmovdqa64`|2683|24.78|41.86|
|G1feed|`vpxor`|1115|0.00|71.81|
|G2feed|`vpmadd52luq`|5398|54.93|11.30|
|G2feed|`vpmadd52huq`|5430|51.37|14.71|
|G2feed|`vpaddq`|3276|45.62|15.78|
|G2feed|`vpsrlq`|1709|49.62|15.43|
|G2feed|`vmovdqa64`|3293|17.94|42.68|
|G2feed|`vpxor`|1012|0.00|65.53|

These are waits from tagging through completion, including operand and resource availability, **not intrinsic instruction execution latencies**. A zeroing XOR can complete immediately yet wait for earlier operations to retire. The same selection/sampling bias prevents interpreting the arithmetic means as exclusive stalls or summing them into removable proof time. Memory-operand instructions can comprise multiple ops; raw load/store flags identify sampled ops. Stack-addressed operands do not by themselves establish compiler spills.

|Kernel|Median fetch samples/process|Completed fetch mean, cycles|Completed fraction|I-cache misses/all fetch samples|Op-cache misses/all fetch samples|
|---|---:|---:|---:|---:|---:|
|G1feed|3487|0.154|99.65%|0.725%|7.915%|
|G2feed|4126|1.668|91.89%|10.472%|28.526%|

G2 has more I-cache/op-cache misses than G1. Completed-fetch latency omits incomplete fetches, which remain counted explicitly; low completed means cannot rule out frontend stalls. Fetch samples are separate from retired-op samples. No generic frontend-stall alias or unsupported event interpretation is used.

## Protocol and verification

Six fixed fresh processes, 32 workers on CPUs 0–31, three priming proofs plus 61 sampled complete SDK requests each. Blocks alternate op/fetch, fetch/op, op/fetch; period 524288, `ibs_op/cnt_ctl=1/` and `ibs_fetch/rand_en=1/`. FIFO acknowledgments enable after key loading/priming and disable before export. No owned helper, build, hash, archive or second CPU workload overlapped capture; every capture finished before any independent verifier or decoder. All 30 workflow steps passed. No successful capture or verifier was rerun, no rounds extended or observations excluded.

**384 SDK requests internally verify; six fresh 164-byte exported proofs independently pass pinned Gnark, image/public binding and freshness checks. Ledger 17805→17811, 12 proof/public members.** Operation samples: 517,186; fetch samples: 67,569; zero lost records. Exact raw/script PID/TID/ns/IP joins, op weight/source agreement and raw-vs-pretty field checks pass. All SDK instruction addresses resolve by exact mangled symbol+offset into the pinned PIE disassembly. Both feed groups have zero imprecise or saturated/invalid latency pairs. Three imprecise user records outside these feeds retain sample-count attribution limits documented in [postcapture review](artifacts/postcapture-review.json).

Four new Rust decoder tests pass, plus one focused incomplete-fetch regression after a capability decode exposed perf's omitted invalid field. A new non-SDK 32 MiB SHA256 fetch capability yields 31 samples; the existing 239 op records validate the new decoder without repeating that workload. Original prover arithmetic/runtime tests are inherited because source and executable are unchanged. Initial closeout encountered root-owned perf data; ownership changed after capture, then closeout completed with original bytes and no rerun.

SDK medians under op sampling were 7.991270/8.157612/8.154106 ms; fetch 8.054909/7.973145/8.012890 ms. These are diagnostic, without unprofiled latency or observer-overhead qualification. No RSS improvement, minimum RAM, AWS/Aeglos, phone/ARM/TEE, huge-circuit, concurrency or throughput result follows.

Register semantics follow [AMD's Zen4/Zen5 IBS derived events](https://docs.amd.com/r/en-US/57368-uProf-user-guide/7.9.3.2.4.-IBS-Derived-Events), the installed6.8.0-147 [header](artifacts/references/installed-amd-ibs.h), and [upstream Linuxv6.8 IBS handling](https://github.com/torvalds/linux/blob/v6.8/arch/x86/events/amd/ibs.c). Upstream is a reference, not asserted byte-identical to Ubuntu's runtime kernel. Current hardware exposes Zen4 IBS extensions; no old-family erratum is assumed.

## CPU and Metal comparison

Inherited accepted local timings against the supplied historical M5 references; not newly measured by this diagnostic. Accepted primary peak RSS remains 134 MiB. The prior 16-worker baseline-calibration engineering exception remains; the full performance gate must not be relabeled passed.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

The CPU 10× goal remains active and unmet for all eight shapes (0/8). Accepted 54→1 is 162.250464 ms without a supplied M5 reference. No production code or accepted measurement changes.

Whitespace review found three trailing blank lines in two original test logs and the selected objdump excerpt. Those artifacts remain byte-exact; authored Rust source has no whitespace findings.

[Design](artifacts/design.md), [declaration](artifacts/declaration.json), [summary](artifacts/summary.json), [all observations](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [source/tool pins](artifacts/pins.json), [raw local-only hashes](artifacts/raw-local-only-sha256.json). Raw perf data/register dumps/scripts and complete 49 MiB disassembly stay local; selected streaming disassembly is archived.
