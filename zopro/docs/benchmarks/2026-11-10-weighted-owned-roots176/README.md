# Reuse owned bucket storage for weighted roots

**The new implementation passes native and SDK-link qualification. Performance is unmeasured; production is unchanged.** It consumes temporary bucket vectors and stores each group's nine roots in that group's first nine existing entries. This removes the preceding candidate's separate root array and per-call admission check through a different ownership contract. The [previous slower recipe](../2026-11-10-weighted-root-phases-sdk 176/README.md) remains rejected. No speedup is claimed here.

Nine terminal callers pass owned results of `state.finish()`, `affine_buckets`, indexed reduction or final bucket merges. No caller uses the vector afterward. Immutable key/scalar borrows remain unchanged. The new `weighted_affine_owned(Vec)` wrapper lends an exclusive slice internally; disabled, short, unsupported partial-group and G2 paths call the original borrowed reducer, whose body stays byte-exact. All guards run before mutation.

For eligible G1 inputs of at least 2,048 buckets in complete 256-bucket groups, each root job first copies its group into the original 511-slot tree/scratch. It computes the unchanged nine roots, then writes only the first nine bucket entries. After that pass completes and its scratch is released, a second pass gathers up to eight root prefixes for the unchanged SIMD double-add fold. Both weighted and unweighted partials/root 0 and original chunk-offset corrections are retained. The input allocation is consumed and freed after reduction; this mutates temporary buckets, not key points.

The parent bucket and partial-output capacities already appear in the existing forecast. The G1 leaf forecast adds 6,176 logical bytes for fixed fold objects; G2 adds zero and overflow declines. There is no separate heap root array, new optional-allocation token, OS headroom query or admission lock in this entry. Preparation, allocator and parent workspace implementations remain original. Existing tree/partial allocations can still cause allocator/OS work; total allocation counts are not measured here. Logical accounting is not an emitted stack, RSS or minimum-RAM bound. There is no key table, retained plan, production pool or architectural RAM cap.

The optional preparation budget excludes required proof scratch. A zero optional budget therefore still permits this route, which adds no heap root parent; an explicit disabled resource-schedule flag retains the original route. This changes the allocation requirement rather than bypassing admission for an allocation that still exists. Extra barrier, root stores/reloads, gathers, packing, inversions, corrections and drop remain costs for complete-proof timing.

Five serialized native commands pass once: fresh OFF/ON cores, ON test compilation, **131 unfiltered core tests**, and portable-disabled compilation. Four new tests cover:

- 65 full-group cases across five populations, checked through the owned entry, direct mutable-slice route, both partial outputs and original borrowed reference against independent Arkworks oracles. Sizes include non-eight group counts and partial SIMD tiles.
- Unchanged bucket pointer/length/capacity, unchanged247 nonprefix entries per group and unchanged external input points;19 decline-before-write cases and20 original-valid tail cases. This does not newly qualify arbitrary non-power-of-two tails in the original fallback.
- 53 actual workers across1/4/16/32 pools requesting512 KiB stacks and12 calls sharing an eight-worker pool; phase completion before folding.
- Original G2 behavior with an independent oracle, resource-disabled fallback, zero optional budget, six forecast boundaries and checked overflow.

These are case families, not distinct proof inputs or server throughput. The unchanged roots/kernel algebra and field qualification are inherited without replay. Portable compilation is not ARM/mobile execution. The current 127 other core tests also pass with the changed terminal calls.

Eight further commands coherently link fresh OFF/ON prover, Zolana and SDK consumers and inspect demangled symbols. All **13 scientific commands succeed once**. Final ON contains private `::msm::weighted_owned::` symbols; OFF does not, and both omit test route counters. Dynamic SDK execution, latency and process peak have not been measured. Original field 1200, all accepted cfgs including dense busy reset, native/O3/fat-LTO/one-CGU SDK links and default SDK worker-stack behavior are retained.

The snapshot contains 170 exact originals, six reversibly changed parents and three children. Reversing changes recovers all 176 production hashes, including the binary fixture as bytes. The separate same-agent readers check 202/212 pin occurrences,13 raw command/exit/stream receipts, source recovery, nine caller replacements, original borrowed body, unchanged roots/kernel, test markers, cfgs and coherent final consumers. This is a source/test cross-check, not an external cryptographic audit.

Preparation exception: the first source selector matches three `affine_buckets` sites, including a reference test, while expecting two production returns. It stops before editing that file or starting any native command. The failed helper/source/streams and partial snapshot are retained. Qualifying the selector with `return` preserves the reference test; a fresh snapshot and five-command plan then complete. No successful scientific build, test, recording or proof is replayed. The failed full partial tree remains private; the failed helper and streams are archived.

No SDK request, preflight or Gnark export is added; the checked canonical proof inventory stays **49,363**. Next is a prospective complete-SDK ALL8 plus merge 54 comparison at 32 workers, primary also 1/16, with both actual raw readers, automatic counters/profiles, memory guards and all gates fixed before preflight. This local change alone does not establish a twofold gain or completion of the full objective.

Accepted REF values below are inherited unchanged from the preceding complete campaign. M5 CPU/Metal are supplied historical measurements on different hardware and were not remeasured. The new candidate has no timing column.

| Shape | Accepted REF ms | M5 CPU ms / speedup | M5 Metal ms / speedup | CPU10× limit ms |
|---|---:|---:|---:|---:|
|1x2|7.827060|37 / 4.727×|24 / 3.066×|3.7|
|2x2|10.928922|59 / 5.399×|30 / 2.745×|5.9|
|4x2|15.793934|109 / 6.901×|48 / 3.039×|10.9|
|8x2|23.996602|189 / 7.876×|74 / 3.084×|18.9|
|12x2|34.000511|284 / 8.353×|101 / 2.971×|28.4|
|16x2|39.766683|335 / 8.424×|124 / 3.118×|33.5|
|24x2|66.285987|528 / 7.965×|176 / 2.655×|52.8|
|merge-24x1|65.442208|507 / 7.747×|175 / 2.674×|50.7|

Accepted primary remains 7.827060 ms /133.25 MiB in that campaign, primary 16 is 9.170142 ms and merge 54 is 163.340201 ms /1921.91 MiB without an M5 reference. The CPU AVX10× ALL8 goal remains active and unmet, **0/8**. No new AWS/Aeglos, ARM/mobile/TEE,250/4999, pressure/swap, minimum-RAM or concurrent-proof-throughput result is added; no cloud resource is created.

[Native reader](artifacts/review-result.json), [SDK reader](artifacts/sdk-review-result.json), [consumer identities](artifacts/identities.json), [source and exact receipts](artifacts/evidence.tar.gz), [archive member hashes](artifacts/public-members.json), [checksums](SHA256SUMS). Evidence includes changed parents/children, tests, design, plans/pins, failure records, all native/link receipts and the next-agent autoresearch prompt. Native executables/libraries, unchanged full trees, key/request/witness/proof payloads and full ledgers remain private and hash-bound.
