# Bounded SIMD Booth decoding

**The native candidate passes correctness and coherent SDK qualification. Speed is unmeasured; production is unchanged.** Eight independent scalar digits are decoded together into a 16-byte stack array, then consumed by the original per-point bucket operations in their original order. G1 ordinary, balanced K/H and signed low/high G2 feeds use this reader. Short tails use the original scalar readers.

This differs from the rejected [SIMD admission trial](../2026-11-10-simd-bucket-sdk176/README.md): it introduces no conflict tests, busy-state gathers, coordinate predicates or collective admission. It also avoids the input-sized matrix and decode barrier of the rejected [digit-plane trial](../2026-10-31-digit-planes176/README.md). No field operation, point arrival, window geometry, queue/batch boundary, allocator, pool or resource policy changes. There is no new heap buffer, key table or retained execution plan. Cheaper integer decoding is a hypothesis, not evidence for a twofold proof gain.

The opt-in path requires `zopro_simd_booth`, the accepted Booth cfg, x86_64/AVX512F and `no_ifma` unset. It gathers one or two relevant words from each of eight 32-byte rows, performs the overlapping signed extraction, applies G2 component signs and narrows to eight i16 values. G2 magnitude bits exclude the packed sign bit. Missing high words are zero and never gathered; incomplete packets retain scalar reading. The guard-disabled x86-64/no_ifma build succeeds. This is a compile check, not ARM or older-CPU execution.

Independent per-bit oracles pass **8,562,812 digit comparisons** and **219,060 exact 320-bit integer reconstructions**. Inputs comprise 4,868 full-width rows, including every one-bit boundary and its adjacent integers, zero/max/sign patterns and 4,096 deterministic random rows. Widths 1–15, over-top windows, both signed G2 components and full 256-bit G1 rows are covered. A further 11,220 comparisons cover all 20 balanced spans and lengths 0–33. The two standalone integer tests pass.

All **129 unfiltered native core tests pass**. New G1/G2 tests supply 704 window cases and 45,056 bucket-output comparisons with independent Arkworks sums. They exercise 512 KiB requested worker stacks in 1/4/16/32-worker pools, cross-word windows, signs, empty and partial packets, collisions, identities, equal/opposite points and capacities 8/200. Inputs remain exact and batch/queue pointers and capacities remain stable. These pool-context cases are not a concurrent-proof throughput benchmark. Existing query, GLV, balanced-window, resource and solver tests also pass. The unchanged field library's prior tests are inherited.

The private snapshot has 179 files: 172 accepted originals are exact, four parent edits reverse to all 176 original hashes, and three children add the decoder and curve tests. All 176 production files remain exact. Both cores retain dense busy reset and accepted manual cfgs; only ON enables the decoder. Both actual SDKs are coherently relinked through prover and Zolana using original field1200, the same driver, explicit Rust 1.98.1, O3/native/fat LTO/one codegen unit and default SDK worker stacks. The 512 KiB request applies to native qualification only.

| Actual optimized SDK function | OFF bytes | ON bytes | ON static gather / narrow sites |
|---|---:|---:|---:|
| Balanced G1 feed |2,269|3,347|5 /1|
| G2 feed |3,556|4,564|7 /1|

Four selected GNU objdump decodes confirm SIMD packet instructions in actual ON consumers and none in the corresponding OFF bodies. Static sites include alternate branches; they are not dynamic operation counts or cycle savings. Ordinary G1 feed is inlined and is not separately decoded here. The larger code, gathers and iterator state may outweigh the intended saving. Source-local stack values do not establish actual frame peaks, RSS, minimum RAM or phone suitability. No timing or profile is inferred from disassembly.

All 15 serialized workflow commands succeed once: seven native build/test/guard steps and eight coherent SDK build/symbol steps. A separate same-agent reader verifies 409 pin occurrences, actual command argv/exits/stream hashes, source reversals, test populations, cfg selection, SDK bindings, the four decodes and unchanged accepted SDK/47,437-proof inventory. It is not an external audit or a new proof verification.

The initial workflow plan fails parsing before any command because version was omitted. The original plan remains; adding only version:1 enables execution. The first offline reader also fails because it expects unreceipted Cargo build metadata in command.json. xtask intentionally records only managed performance variables. A corrected reader checks that policy and passes; both sources and the original captured failure remain archived. No successful build, test, link or symbol listing is repeated. Native compiler warnings are retained. Direct helper compiles are outside the 15 workflow-command count.

The qualification adds **zero SDK requests, fresh Gnark exports or performance captures**. Next, freeze a complete matched all8+54 SDK comparison plus primary 1/16-worker contexts, exact accepted/fresh controls, order/gates, two raw readers and automatic PMU/profiles before any SDK preflight. Include predeclared independently written executable backings and retain every outcome in light of the [current control diagnostic](../2026-11-10-backing-control176/README.md); choosing the fastest copy is not an optimization. All decoding/gather/iterator/tail costs must be charged. Full-proof verification, calibration, gain, resource and compatibility qualification are still required for adoption.

Accepted local timings below are inherited from the [latest accepted-reference campaign](../2026-11-10-compiler199-sdk176/README.md). M5 CPU and Metal values were supplied separately on different hardware and were not remeasured. These are accepted-build comparisons, not candidate predictions.

| Shape | Accepted CPU ms | M5 CPU ms / speedup | M5 Metal ms / speedup | CPU10× limit ms |
|---|---:|---:|---:|---:|
|1×2|7.863367|37 /4.705×|24 /3.052×|3.7|
|2×2|11.051833|59 /5.338×|30 /2.714×|5.9|
|4×2|15.850652|109 /6.877×|48 /3.028×|10.9|
|8×2|24.050327|189 /7.859×|74 /3.077×|18.9|
|12×2|33.877870|284 /8.383×|101 /2.981×|28.4|
|16×2|39.777696|335 /8.422×|124 /3.117×|33.5|
|24×2|66.208517|528 /7.975×|176 /2.658×|52.8|
|merge24×1|65.473144|507 /7.744×|175 /2.673×|50.7|

Accepted merge54 remains 162.652338 ms without a supplied M5 reference. The full CPU10× objective remains active and unmet, **0/8**. No new AWS/Aeglos/Gnark, ARM/mobile/TEE, huge 250/4999, pressure/swap, minimum-RAM or concurrent-throughput result follows. No cloud resource is created.

[Qualification review](artifacts/review-result.json), [SDK identities](artifacts/identities.json), [emitted-code findings](artifacts/codegen.json), [decision](artifacts/decision.json), [complete authored source/plans/receipts/decodes](artifacts/evidence.tar.gz), [member hashes](artifacts/public-members.json), [checksums](SHA256SUMS). The archive includes both reader versions, preparation exceptions, exact changed source/tests and the next-agent autoresearch prompt. Extraction/member hashes pass. Native executables/libraries, private key/request/witness/proof payloads, unchanged source snapshots and canonical-ledger payload stay local and hash-bound.
