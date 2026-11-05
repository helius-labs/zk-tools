# Proof-only PGO for the composed CPU prover: qualification

The composed finalization/queue-drain source has a fresh proof-only compiler profile and a successfully built uninstrumented PGO SDK. **Native correctness qualification passes; candidate latency is not measured and no change is adopted.** The next experiment is the fixed four-arm full-SDK comparison described below.

This combines two previously measured approaches. The [plain composition](../2026-11-05-finalize-drain-bundle176/README.md) improved primary against its fresh control but failed its full gate. [Source175 PGO](../2026-10-29-proof-pgo-qualification175/README.md) had small primary/seven-shape gains and failed physical16 calibration; its [C7a results](../2026-10-29-proof-pgo-c7a175/README.md) also remain separate. These prior failures are preserved. Their gains are not assumed additive, no old profile is reused, and this stage makes no twofold-speedup prediction.

## Source and compiler

All **179 source files are byte-exact to the plain composition**. Its six reversible original-file edits and three children preserve 170 accepted originals exactly and recover all 176 accepted hashes. Existing field arithmetic, generated IFMA code and generator remain unchanged. The addition here is compiler PGO, with no new arithmetic formula, proving-key table, retained point copy, solver plan, worker pool, or RAM cap.

Rust 1.98.1 and matching LLVM 22.1.8 build separate GEN and USE targets. Explicit Cargo target selection excludes host build scripts from target instrumentation. GEN uses atomic profile counters; that instrumentation can perturb scheduling and is not claimed negligible. USE consumes the one merged profile and has no profile-runtime symbols or counter/coverage sections in the checked SDK.

Qualified USE SDK: `817d07cd7aadc5586d0a8472dcce18ee7fe12f5f8802e0a283d6212070c95203`, retained locally at `/home/ubuntu/src/zopro-perf-work/bundle-proof-pgo176/native-use/candidate-sdk`. Actual final-link argv contains `-O`, `target-cpu=native`, `lto=fat`, `codegen-units=1`, the composed cfg flags, and the absolute profile-use path.

The process follows the [rustc PGO workflow](https://doc.rust-lang.org/rustc/profile-guided-optimization.html). Actual tool hashes, source, flags, raw/merged profiles and emitted-counter audits are retained. Documentation is a method reference, not speed evidence.

## Fresh proof-only training

One 32-worker process on CPUs 0–31 runs for each of all nine supplied shapes, including merge54. Image loading, SDK construction and three complete priming requests precede counter reset. Eleven fresh requests follow reset; one profile dump occurs before exporting their proofs and formatting output. All nine captures finish before independent verification and merging. Profiles receive equal weights, without a primary weight, variant selection or extension.

The **126 SDK calls verify internally**, including 27 excluded primes. All **99 fresh 164-byte exports independently pass Gnark**, expected public/image bindings, and uniqueness against the 29,953-entry ledger. The new ledger has **30,052** entries. These 99 exports belong to **GEN**, not the optimized USE SDK; USE full-SDK preflight/latency exports remain future work.

Nine fresh raw profiles produce 6,239 function records. Audited selected counters are exactly **99 SDK proves, zero image loads and zero SDK constructions**, cross-checked against request/export counts. Generic maximum block counters are not treated as arbitrary function-entry counts. The training driver asserts the runtime selector enabled before loading and prints its true state after dumping. Training times are not performance results.

## Native tests and coverage audit

USE freshly passes 35 field tests once, 128 core tests with the selector off and 128 with it on, and 8 prover tests in each mode (two raw-image plus six cubic). All runs are unfiltered. Field tests have no selector-dependent behavior and are not repeated. Mathematical cases are inherited across compiler variants; no new mathematical-case count is claimed. These checks do not establish portable, phone, pressure, or huge-circuit behavior.

Ten selected nonzero profile records cover the SDK prove entry, G1 stream, balanced feed, G2 apply, GLV component/digit/G2-image helpers, and guarded G1/G2 flush callers. No missing-profile warning matches those selected records in the normal library/SDK consumer logs, and no detected hash-mismatch/out-of-date warning appears. All 6,765 missing-function warning lines remain retained, including test and unseen-driver warnings. This is a scoped record/warning check, not complete branch attribution or optimization-remark coverage.

## Preserved harness failures

The initial setup requested parent metadata field `review_passed`; the parent uses `passed`. It stopped before creating the source snapshot, native protocol or build. Correcting only that lookup completed the unfinished setup. [Initial setup](artifacts/initial-setup/repair.md).

After all nine training captures and 99 Gnark checks, the original final audit successfully merged/decoded profiles and passed the 99/0/0 checks, then required separate named records for `pair_g1`, `joint_g2` and `drain_queued_batches`. Those definitions have no separate emitted records. A separately pinned auditor checks actual emitted helper/caller records bound to exact source/cfgs and the immutable enabled selector; it explicitly records the missing names and its coverage limits. The original workflow remains failed at that audit step. No capture, verifier, LLVM merge/show, or successful test is repeated. [Audit repair](artifacts/audit-repair.md).

The first offline qualifier also compared the full declared environment to the intentionally filtered command receipt. It stopped before producing qualification output. The correction applies the exact xtask managed-variable predicate; full protocol environment and output/profile identities remain checked. The source shows all variables are passed to `Command::envs`, while only the receipt display is filtered. No process environment changes. [Qualifier repair](artifacts/initial-qualification/repair.md).

Final qualification checks **1,064 source/input pins, 120 successful workflow commands and the one preserved failed workflow command**, source recovery, tests, proof/ledger identities, profile records, optimized argv and GEN/USE runtime distinctions. The two direct setup/qualifier failures are additionally preserved outside that workflow count. Qualification passes after these metadata/coverage repairs; no arithmetic repair or timing rerun occurs.

## Fixed next comparison and limits

Compare four arms: accepted SDK6d83; a freshly linked SDK using original coherent libraries; the frozen plain composed SDK enabled; and this new PGO composition enabled. The fresh original is a **link control**, not a fresh source compilation. The plain composition deliberately changes behavior and is not calibration-equivalent to accepted.

Use all eight reference shapes plus merge54 at 32 workers and primary at 16 physical workers, with 15 rotated matched blocks. Measure 151 primary32, 301 primary16, or 31 other requests after 3 primes. Original-link/accepted calibration must fit factor 1.03. Candidate non-regression upper bounds must be ≤1.03 in every context against all three controls. Primary32 and original-eight geometric aggregates require upper <1 and ≥10/15 wins against all three. No gain floor, exception transfer, observation removal, unchanged reroll, profile extension, or favorable-shape subset is allowed. Automatic warm-interval counters/profiles and independent Gnark exports follow the declared latency capture. Passing selects additional compatibility/resource/final-artifact qualification; it does not automatically adopt the compiler artifact or complete the goal.

No new full-proof timing, AWS/Aeglos comparison, portable/device, pressure/swap, minimum-RAM, concurrent throughput, or 250/4999 qualification is added here. Compiler profiles are offline build inputs. Runtime allocation behavior and the inherited per-proof GLV scratch require full resource qualification before adoption. Production and the accepted SDK remain unchanged.

## Accepted CPU and Metal comparisons, inherited

These accepted 32-worker local full-proof results and supplied historical M5 references are **not measurements of this PGO candidate**. [Accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json).

| Shape | Accepted local ms | M5 CPU ms | CPU/local | M5 Metal ms | Metal/local | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 4.667× | 24 | 3.027× | 3.7 |
| 2×2 | 11.151932 | 59 | 5.291× | 30 | 2.690× | 5.9 |
| 4×2 | 16.086386 | 109 | 6.776× | 48 | 2.984× | 10.9 |
| 8×2 | 24.179453 | 189 | 7.817× | 74 | 3.060× | 18.9 |
| 12×2 | 33.884455 | 284 | 8.381× | 101 | 2.981× | 28.4 |
| 16×2 | 40.077381 | 335 | 8.359× | 124 | 3.094× | 33.5 |
| 24×2 | 65.850777 | 528 | 8.018× | 176 | 2.673× | 52.8 |
| merge24×1 | 65.060937 | 507 | 7.793× | 175 | 2.690× | 50.7 |

Merge54×1 remains 161.883899 ms without a supplied M5 reference. The full all-eight CPU10× goal remains active and unmet, **0/8**.

## Artifacts

[Design](artifacts/design.md), [qualification](artifacts/qualification.json), [training audit](artifacts/training-audit.json), [linked-code audit](artifacts/code-audit.json), [coverage scope](artifacts/profile-record-coverage.json), [source audit](artifacts/source-audit.json), [reproduction](artifacts/reproduce.md), [source archive](artifacts/source.tar.gz), [profile archive](artifacts/profiles.tar.gz), [proof/public exports](artifacts/proofs.tar.gz), [exact receipts](artifacts/receipts.json.gz), [checksums](SHA256SUMS).

Archive member names/bytes/hashes are checked after extraction and receipt UTF-8 round trip. Executables, frozen libraries, key/image/witness/scalar/point arrays and raw perfdata are excluded. The explicitly included profile data and proof/public exports contain build evidence and the 99 verified outputs, not private proving-key arrays.
