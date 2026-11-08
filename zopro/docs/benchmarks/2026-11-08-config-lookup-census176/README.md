# Complete-request configuration lookup census

**Configuration lookups allocate little data in this diagnostic.** The primary32-worker request makes1,893 lookups,118 successful Rust allocations and119 requested bytes within those lookups. Most reads are unset options and allocate nothing. This is not evidence of a substantial configuration-allocation memory problem or a twofold optimization. Lookup CPU/lock/search cost is not measured; no cache or prover change is selected.

The new diagnostic covers all supplied shapes plus merge54 and primary solo. Every process makes three primes and two observed complete SDK requests, including fresh proving, internal verification and serialization. All ten final exports pass independent Gnark verification. The canonical exported-proof inventory advances **35,171→35,181**. Production176, accepted SDK6d83 and accepted proof timings remain unchanged.

## Counts

Calls are two observed requests per process, displayed separately where they differ. Allocation columns are identical between the two repetitions. They count successful Rust GlobalAlloc allocation, zeroed-allocation and reallocation events within lookup scopes; byte volume sums requested sizes/new reallocation capacities and is not net or live memory.

|Shape/workers|Lookup calls, rep0 /rep1|Present lookups, each|Lookup allocation events, each|Lookup requested byte volume, each|
|---|---:|---:|---:|---:|
|1×2/32|1,893 /1,893|118|118|119|
|1×2/1|1,467 /1,467|118|118|119|
|2×2/32|2,447 /2,444|151|151|154|
|4×2/32|7,296 /7,296|407|407|409|
|8×2/32|8,253 /8,253|407|407|409|
|12×2/32|7,932 /7,915|216|216|218|
|16×2/32|8,666 /8,651|216|216|218|
|24×2/32|12,132 /12,085|401|401|402|
|merge24/32|10,033 /10,036|216|216|218|
|merge54/32|19,931 /19,892|586|586|586|

No unknown dynamic option name or non-UTF8 configuration value occurs in the observed requests. Var/var_os statuses and returned lengths/capacities are preserved separately. Environment values themselves are not recorded.

For primary32 rep0, the recovered caller/Rayon threads make11,274 total Rust allocation/zeroed/reallocation events requesting96,428,346 cumulative bytes. The118 lookup events and119 lookup bytes are a subset of those totals. This comparison concerns event counts and requested volume, not CPU fractions, allocator-internal activity, peak/live memory or minimum RAM. No inference that a lookup is free follows from an absent allocation.

|Selected option reads|Primary32, rep0|Primary1, rep0|Merge54/32, rep0|
|---|---:|---:|---:|
|TREE_VECTOR_INVERSE|1,101|675|3,382|
|DIRECT_G2|47|47|12,851|
|G2_PREFIX_RECORDS|108|108|579|

TREE_VECTOR_INVERSE and DIRECT_G2 are unset in these captures, so they use their existing native defaults and cause no observed allocations. Prefix-record reads are present and account for108 primary allocations,108 bytes. The primary captures also include the other existing options. All names/sites and their complete metrics remain in [analysis](artifacts/analysis.json).

The small repetition differences shown above occur in TREE_VECTOR_INVERSE read counts; the corresponding allocation totals stay equal. Scratch initializer frequency can depend on the task decomposition. These observations are retained without forcing repetitions or worker counts to match or claiming changed arithmetic work.

## Observer and scope

A private178-file snapshot retains162 accepted files byte-exact. Reversible call substitutions in14 core source files plus two diagnostic modules recover all176 original hashes. Each of73 fully qualified std::env::var/var_os sites calls the same standard function once and returns the original result. Dynamic cpu_option names receive a separate name index; site and name counters overlap and must not be added together. There are87 declared name slots including the unknown slot and a test key. No global configuration cache, option/default change, unsafe getenv shortcut, point/key/solver-plan copy or pool is introduced.

The diagnostic TrackingAllocator forwards to the original allocator, recording successful allocation/zeroed/reallocation events. A lookup scope tags only allocations made while the original standard environment function executes. Returned strings may be dropped later; deallocations and retained/live objects are not measured. Totals cover recovered Rust caller/Rayon-thread events, excluding libc/kernel allocation internals and unobserved threads. There is no new allocator routing policy or payload prefix.

Fixed thread-local counters use15,424 payload bytes per participating thread, plus platform TLS overhead. Site/name metadata is fixed read-only diagnostic data, independent of key/circuit size. Counter snapshots and serialization occur with counting disabled, outside requests. The extra observation, owner lookup and allocation hooks perturb optimization and execution: these are counts in the instrumented program, not accepted-machine instruction counts or latency measurements. No timings or RSS values are emitted.

Most hot-path flags already live in scratch. This census does not justify globally memoizing options or breaking configuration changes between requests. A future owner-lifetime change would need its own semantic and complete-query/full-proof performance case; allocation counts alone are insufficient. The primary optimization target remains ordinary MSM work with enough parallelism, rather than a large allocation-saving claim for these lookups.

## Validation and provenance

The new focused native test passes on its first execution;125 existing tests are filtered and inherited. It checks missing/present/empty/non-UTF8 results, actual allocator observation inside lookup scopes, enable/reset behavior, cross-thread isolation and changes between reads. Original field/arithmetic source and qualification are inherited; the diagnostic introduces no new arithmetic formula. Installed rustc/rustdoc1.98.1 are explicitly pinned.

A coherent native release library set and SDK driver build once. The final link has O/native CPU/fatLTO/CGU1 checked before capture. Ten processes produce50 complete SDK requests,20 observed populations and ten fresh164-byte proof/44-byte public-input exports. All captures finish before any external Gnark invocation. No other owned build/helper/hash/archive/analyzer/verifier workload overlaps captures. No successful old test, model, observer, recording or verifier is rerun.

Independent review passes on its first execution:186 source and220 capture pins;23 completed command receipts; all176 source reversals; reconstruction of all20 raw per-thread populations; site/name/status/allocation conservation; complete size/public/image/verifier bindings and freshness against the full prior proof inventory. It verifies the resulting35,181-entry union without rerunning a request or cryptographic verifier. It does not independently reimplement std environment behavior, the allocator or Gnark. [Review](artifacts/review.json), [source audit](artifacts/source-audit.json), [observer](artifacts/config_census.rs), [site map](artifacts/sites.json).

Preparation, focused test, coherent build/link, capture, independent verification, analysis and review succeed on their first executions. There are no discarded populations, sample extensions, timing decisions, cache integration, production adoption or new twofold speed claim. [Design](artifacts/design.md), [protocol](artifacts/protocol.json), [native plan](artifacts/native-plan.json), [capture plan](artifacts/capture-plan.json).

## Accepted CPU and Metal comparisons, inherited

The census measures none of these times. The same accepted binary previously measured7.915/7.929/7.996ms for local1×2; its latest observation is8.003ms. Variation remains unexplained, with no adopted regression established.

|Shape|Accepted local ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.002770|37 /4.623×|24 /2.999×|3.7|
|2×2|11.241463|59 /5.248×|30 /2.669×|5.9|
|4×2|15.933816|109 /6.841×|48 /3.012×|10.9|
|8×2|24.234311|189 /7.799×|74 /3.054×|18.9|
|12×2|33.855565|284 /8.389×|101 /2.983×|28.4|
|16×2|40.071563|335 /8.360×|124 /3.094×|33.5|
|24×2|65.917462|528 /8.010×|176 /2.670×|52.8|
|merge-24×1|65.305023|507 /7.764×|175 /2.680×|50.7|

Source: [preceding comparison](../2026-11-08-query-control-codegen176/README.md). Seven nonprimary rows remain from the earlier full campaign. Merge54 remains162.240109ms without a supplied M5 reference. The all-eight verified CPU10× goal remains active and unmet, **0/8**. No new AWS/Aeglos, huge-circuit, mobile/ARM/TEE, RAM-pressure/swap, minimum-RAM or concurrent-throughput result is added.

[Source/receipt/count evidence](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json) and [checksums](SHA256SUMS) preserve the new observer and proof bindings. Every UTF-8 payload hash and gzip roundtrip is checked. Private points/scalars/keys/images/requests/witness/proof/public arrays, full ledgers, libraries/binaries and unchanged full production source are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/config-lookup-census176`; new canonical inventory: `/home/ubuntu/src/zopro-perf-work/config-lookup-census176/proof-sha256-after-capture.json`.
