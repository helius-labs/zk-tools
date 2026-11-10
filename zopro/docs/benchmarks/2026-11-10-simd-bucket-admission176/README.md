# SIMD admission of ordinary bucket updates

**The new native G1/G2 dispatch path passes correctness and source/link qualification. Speed is unmeasured; production is unchanged.** Both optimized SDK consumers are built for a matched comparison. The accepted local 1×2 full-proof result remains **7.783594 ms at 32 workers**, and9.016864 ms at16 workers.

The candidate checks eight incoming updates together. Hardware conflict detection rejects repeated destinations; vector masks reject pending buckets, zero first X limbs and equal first X limbs. A passing packet appends eight ordinary borrowed updates directly to the existing batch. Every rejection uses the original scalar state transitions, with already-extracted digits. This covers ordinary G1, balanced K/H G1 and G2 Phi feeds. It adds no point table, expanded key, retained execution plan, worker pool or heap buffer.

## Invariants and memory scope

Admission requires eight actual inputs, eight nonzero digits, room for all eight updates, in-range destinations, distinct bucket indices and no pending destinations. Nonzero first X limbs imply the original identity check cannot hold; unequal first limbs imply full X inequality. Therefore every admitted input would take the original ordinary-add branch. There is no affine mutation before the decision. Appending in input order and executing only when the eighth append fills the batch preserves the original batch boundary and queue handling.

Conflicts are checked on bucket indices, not bitmap-word indices. Different destinations sharing one bitmap word remain eligible; scalar OR writes preserve every bit. Partial, zero, identity, equal/opposite, colliding and insufficient-room packets retain the original path, including execution within a rejected packet. Window geometry, queues, arithmetic, prefix records, signs, scalar storage, allocator and resource policy stay original.

Fixed stack arrays hold the packet and admission inputs. This source-level lack of new heap storage is not a whole-program stack or minimum-RAM bound. The actual G1 admission function reserves232 bytes in its prologue; the G2 enclosing feed reserves728 bytes. These are selected frame reservations, not full call-depth peaks or additional bytes versus a measured control. New native tests execute with requested512 KiB worker stacks. No memory-pressure, swap or phone claim follows.

Selection requires the private configuration plus compile-time AVX512F/CD inside the existing native path. The enabled source compiles with AVX-512 conflict detection explicitly disabled, retaining original dispatch. This is a compile guard, not execution on an older CPU or ARM device. No new architectural RAM cap is introduced.

## Qualification and emitted code

The actual native conflict/mask predicate matches an independent scalar predicate in **66,626 cases**, including every eight-lane busy/zero/equality mask, all pairwise duplicate positions, shared bitmap words and65,536 deterministic random cases. That population also runs in the coherent full-core suite; it is not counted as a second distinct population.

All **130 unfiltered native core tests pass**. New G1 and G2 tests contribute704 window cases and **45,056 independent Arkworks bucket-output comparisons**. They cover1/4/16/32-worker pools, lengths0–1025, eight-input boundaries, capacities8/200, mixed signs, identities, equal/opposite points and stable batch/queue pointers and capacities. Test-only counters confirm284 admitted packets for each curve. Existing full-query, balanced-window, sign, exception, reuse and threaded tests also pass. The original field library is reused; unchanged field suites are inherited rather than rerun.

The private snapshot has179 files:171 accepted originals stay byte-identical, five parents have checked inverse edits and three children add the predicate and curve tests. Reversal recovers all176 accepted-source hashes. Both release cores explicitly retain `zopro_dense_busy_reset`. The original field1200, solver, FFT, proving data and downstream source remain unchanged. OFF excludes packet admission; ON enables it. Prover, Zolana and SDK libraries are rebuilt coherently. Actual final links use O3, native targeting, fat LTO and one codegen unit.

|Actual ON function|Bytes|Static instructions|Conflict-instruction sites|
|---|---:|---:|---:|
|G1 `Window::admit_packet`|2,771|575|1|
|G2 enclosing `Window::feed`|8,561|1,711|1|

The disabled consumer has no named packet/admission functions. Test counters are absent from both release symbol lists. G2 admission is inlined into its existing feed symbol. The G1 helper and G2 enclosing function have different scope; their counts are not comparable operation costs. Bounds checks, cold calls, compiler copies and code size can offset the intended dispatch saving. Static instructions, call sites and stack operands do not establish executed work, spills, latency or a speedup.

The separate same-agent reader verifies42 successful build/test/decode receipts and614 pin occurrences, native test markers, source reversals, feature selection and actual optimized SDK identities. Its corrected execution adds one successful receipt. The original reader fails on a wrong emitted-owner assumption:24 retained `hb_raw`-related decodes contain no conflict instruction. A supplemental decode finds the actual G2 feed; only the offline reader changes. Total through review is **43 successful workflow commands and one retained reader failure**. No successful test, native build, symbol listing or decode is repeated for that correction.

A direct preparer compile also initially fails on overlapping Rust mutable borrows before setup runs. The original source is retained; moving metadata assignment before the argv borrow fixes only preparation. That direct compile is outside the44 workflow receipts, and no filesystem stderr receipt was captured for it. An unused-import warning remains in the reader logs; it does not trigger cleanup-only rebuilding.

## Prior evidence and next measurement

The preceding [compact ternary query trial](../2026-11-10-packed-ternary176/README.md) remains rejected. Targeted review also found an existing [fused epoch SDK trial](../2026-11-07-fused-epoch-sdk176/README.md) with no gain and [detailed span tracing](../2026-11-07-g2-ternary-span176/README.md) that failed threaded calibration. Neither is repeated. The present change targets collective ordinary-input admission, rather than field arithmetic, scalar recoding, queue policy or another intrusive observer.

A fresh primary-source search located [PSSM](https://www.mdpi.com/2079-9292/15/16/3660), which aligns independent scalar-multiplication operations for SIMD. Its reported speedup does not compare against this prover's existing SIMD Pippenger MSM. The indexed primary text was available, but fetching the complete page returned HTTP429. No external implementation or performance factor is imported. LogJumps already has a local full-SDK negative and is not a new proposal.

**No SDK preflight, timing, PMU/profile capture, Gnark export or proof is added in this checkpoint.** Next, freeze a matched complete-consumer comparison with accepted REF, fresh OFF and ON; charge every admission/packing/fallback cost. Actual SDK measurements cover the complete queries as well as their interaction with the prover. Use all eight supplied shapes plus merge54, primary1/16/32-worker contexts, fixed order/gates, automatic CPU profiles and a separate raw reader frozen before preflight. Seed every new proof export from the canonical44,265-proof inventory and verify independent Gnark bindings/freshness. A correctness or instruction result cannot qualify adoption.

The candidate must pass current-source full-SDK nonregression, gain, resource and verification checks before any default change. Larger code and extra rejection work may lose; no prospective gain or twofold improvement is claimed.

## Accepted CPU and Metal comparisons

These accepted local full-proof observations are inherited from the [latest SDK campaign](../2026-11-10-frontend-overlap176/README.md). M5 CPU and Metal values were supplied separately on different hardware; no device was remeasured here.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.783594|37 /4.754×|24 /3.083×|3.7|
|2×2|10.932731|59 /5.397×|30 /2.744×|5.9|
|4×2|15.757088|109 /6.918×|48 /3.046×|10.9|
|8×2|23.973295|189 /7.884×|74 /3.087×|18.9|
|12×2|33.827269|284 /8.396×|101 /2.986×|28.4|
|16×2|39.900917|335 /8.396×|124 /3.108×|33.5|
|24×2|66.041083|528 /7.995×|176 /2.665×|52.8|
|merge24×1|65.717994|507 /7.715×|175 /2.663×|50.7|

Merge54 remains162.170158ms without a supplied M5 reference. The verified CPU AVX10× goal remains **active and unmet,0/8**. Accepted production/SDK and the canonical44,265 exports remain exact. No new AWS/Aeglos, huge250/4999, ARM/mobile, pressure/swap, minimum-RAM or concurrent-throughput result follows.

[Qualification review](artifacts/review-result.json), [decision](artifacts/decision.json), [design](artifacts/design.md), [predicate](artifacts/packet.rs), [admission template](artifacts/method-template.rs), [native curve tests](artifacts/window-test-template.rs), [source audit](artifacts/source-audit.json), [SDK identities](artifacts/identities.json), [commands](artifacts/receipts.json), [continuation](artifacts/next.md), [evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Raw helpers, inverse edits, actual changed children, plans, source pins, streams and decoded bodies remain in the indexed UTF-8 archive. Executables, libraries, private key/witness/proof arrays, unchanged source snapshots and canonical ledger payloads are excluded.
