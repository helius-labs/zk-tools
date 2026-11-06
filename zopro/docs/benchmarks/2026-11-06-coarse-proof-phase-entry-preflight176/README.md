# Coarse recorder worker-entry correction: preflight coverage

**The explicit worker-entry handoff produces complete primary and 2×2 graphs. The all-shape preflight stops at 4×2 because its ordinary CPU path is not instrumented yet.** No latency block, PMU panel or cycle profile runs. This diagnostic revision is closed without performance qualification or adoption; source, qualified libraries, successful receipts and failure are preserved.

The [previous attempt](../2026-11-06-coarse-proof-phase-preflight176/README.md) lost caller context when the existing small-core-entry route recursively moved primary solving onto a Rayon worker. This revision passes the owning Arc through a private helper parameter, then borrows it into canonical solving and query callbacks. It installs no worker TLS, pool or key plan. A fixed entry-hop counter in actual primary graphs reads one. Each saved graph passes its recorded parent, timestamp, CPU and ownership checks. There are two saved complete preflight graphs, not a calibrated schedule or performance sample set.

## Native qualification and capture

The changed integration passes 130 unfiltered core tests, two prover unit tests and six cubic integration tests. The four earlier recorder cases are reapplied to this integration and one new explicit-entry test covers 1/4/16/32-worker pools with 512 KiB stacks: five distinct new recorder cases overall, not nine. Coherent libraries and actual final O/native/fat-LTO/one-codegen-unit flags are verified. Source recovery reverses transport edits before prior diagnostic hooks and recovers all 176 originals; 174 remain byte-exact and one new module gives 177 files. Original field, solver, FFT and MSM arithmetic are unchanged.

The first successor capture completes 14 new commands: seven callers and seven independent Gnark verifications. It also reuses the exact accepted primary caller, verification and public proof from the initial attempt, with pinned binary, inputs, arguments and streams. The next caller, enabled 4×2, exits 101 in its first priming request because canonical phase state is zero. The workflow is terminal; it has no result.json. No latency block or hardware recording starts, and no preflight clock is used as a performance result.

The original key constructor enables canonical-wire solving only when the largest query length is below 65,536. The 4×2 image follows the existing general solve_inner → compute_parts → cpu_msms route. The recorder hooked solve_canonical_wires and therefore misses that alternative pipeline. This is an attribution coverage failure, not a proving or format-size failure. The next separate revision must instrument the ordinary CPU pipeline and preserve the all-shape preflight requirement. Requiring only favorable graphs would not complete that diagnostic scope. This failed source and its successful stages will not be restarted.

## Memory, proofs and scope

The enabled primary process reports an 888-byte recorder payload with 18 fixed slots, excluding Arc bookkeeping, allocator overhead, compiler frames and up to three saved snapshots. Metadata does not grow with key size or worker count. Request-local ownership is tested; this does not measure concurrent throughput or a production minimum-RAM bound. No field, key or witness value is recorded. The saved coarse wall spans include nested helping and waiting; they are not exclusive CPU or removable-time estimates.

Seven new caller processes each complete three priming requests and one preflight request; the failed 4×2 process internally verifies its first request before graph validation stops it. Thus 29 new SDK requests verify internally. The reused accepted receipt describes four earlier requests and is not counted again. Seven new 164-byte exports pass pinned Gnark, image/public/VK identity and uniqueness checks, advancing the distinct-proof ledger **30,710 → 30,717**. The reused proof is already in that prior ledger. The inventory contains eight qualified proof rows (one reused), and the archive contains their 16 proof/public members. No failed-caller proof export exists or is claimed independently verified.

Failure review checks 789 pin occurrences, 20 successful new qualification/link/capture commands and the failed caller, as well as the two complete saved graphs and actual entry-hop marker. Successful proof outputs and stream hashes are retained. Planned capture counts are not executed counts. No proving optimization, accepted latency change, AWS/Aeglos, phone, pressure, minimum-RAM, swap or concurrent-throughput result is claimed.

Accepted SDK6d83 and full-proof times remain unchanged; all-eight CPU10× remains active and unmet, 0/8. The following measurements and supplied historical M5 references are inherited.

|Shape|Accepted ms|M5 CPU ms|M5 Metal ms|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37|24|3.7|
|2x2|11.151932|59|30|5.9|
|4x2|16.086386|109|48|10.9|
|8x2|24.179453|189|74|18.9|
|12x2|33.884455|284|101|28.4|
|16x2|40.077381|335|124|33.5|
|24x2|65.850777|528|176|52.8|
|merge24|65.060937|507|175|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference.

[Failure review](artifacts/failure-review.json), [source recovery](artifacts/pre-native-review.json), [design](artifacts/design.md), [proof inventory](artifacts/proof-inventory-after-failure.json), [ledger](artifacts/proof-sha256-after-failure.json), [public exports](artifacts/public-proofs.hex.json.gz), [source](artifacts/source-snapshot.hex.json.gz), [receipts/helpers](artifacts/receipts.json.gz), [checksums](SHA256SUMS). Archives round-trip exact bytes, lengths and hashes. Executables, libraries and private proving inputs remain local with hashes. Reproduction requires the pinned toolchain, dependencies and private inputs. Private root: `/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-entry176`.
