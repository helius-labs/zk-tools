# Scalar fused Poseidon dot products

The scalar fused-dot candidate is **8.23% slower than the fresh control** in the paired primary32 comparison, with 0/3 winning blocks. A physical16 control block also fails calibration. The screen does not advance; the candidate is retained and production is unchanged. No minimum positive-gain threshold is used.

The original scalar Poseidon executor now pairs adjacent dot-product terms with the field library's existing `Fr::sum_of_products`, sharing one Montgomery reduction between two products. An odd final term retains ordinary multiplication. This applies to dense matrices and each partial round's rest term for recognized widths 2–13, including the smallest circuit. S-boxes, constants, prefixes, sparse updates and every original wire/slot/A/B/C write remain unchanged.

This changes one original source file and adds one private helper with two tests. The other 169 originals remain byte-identical. No field primitive, runtime experiment flag, scheduler, point/constant table, copied plan, heap scratch, thread pool, image format or resource policy is added. It uses the existing scalar backend and does not reduce independent-task parallelism.

The preceding [parallel grouping result](../2026-10-27-poseidon-parallel/README.md) prompted a structural check: every wholly-width-three eligible task contributes to the first width-three frontier, so the earlier model's maximum bounds its count. All eight original circuits have at most 52 such tasks per epoch. The previous grouping guard requires more than 64 at 32 workers and therefore cannot enable for those eight circuits. [Context audit](artifacts/context-audit.json) checks the saved census and its hash. This new scalar substitution has no batch-occupancy requirement; no speedup is inferred from the census.

## Complete SDK measurements

The exact historical accepted binary, a fresh unmodified build and the fused candidate use the same standard SDK consumer, compiler flags, allocator policy and inputs. Request timers include witness construction, fresh blinded proving, self-verification and serialization; key loading and three primes occur outside capture. Ratios pair process medians within rotated/reversed fresh-process blocks. CPU time includes scheduling; process lifetime RSS includes loading and primes and is neither minimum RAM nor mode-specific working memory.

|Case|Accepted ms|Fresh ms|Fused ms|Fused/accepted|Fused/fresh|Saving vs fresh µs|Wins vs fresh|Fused RSS MiB|Fused CPU ms/request|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|1x2-w16|10.503055|10.254917|10.237584|0.915562|0.952810|-115.157|1|78.25|129.745|
|1x2-w32|8.642115|8.606477|9.152437|1.083007|1.082257|-384.916|0|99.50|190.098|

The screen has three blocks and 17 timed requests per process at primary 16 and 32 workers. Its declared rule requires fresh/accepted calibration within factor 1.05 in every block and factor 1.03 for paired medians, a primary32 improvement against both controls in at least two of three blocks, and both cases within 2% nonregression against both controls. Any failure remains included. There are no successful repeats or discarded cells.

Calibration failures: `["1x2-w16 block2 fresh/accepted=0.9242924598746899"]`.

Primary improvement failures: `["1x2-w32 fused/accepted=1.0830071523319502,wins1","1x2-w32 fused/fresh=1.0822570762835324,wins0"]`.

Nonregression failures: `["1x2-w32 fused/accepted=1.0830071523319502","1x2-w32 fused/fresh=1.0822570762835324"]`.

The full stage and cycle profiles did not run. No arithmetic-only benchmark was used to select or reject this candidate. The [last complete all-shape SDK matrix](../2026-10-26-pairing-x-chain-sdk/README.md) retains the full supplied M5 CPU/Metal comparison and remains the performance reference outside this short screen.

Static inspection finds a **2,861-byte out-of-line dot helper with five direct call sites** in the candidate executable. [Code-generation audit](artifacts/codegen-audit.json), [helper assembly](artifacts/dot.asm.txt), and [call sites](artifacts/dot-call-sites.txt) retain that evidence. This identifies a possible cost of the generic slice helper; it does not establish the cause of the measured regression. The later slow primary32 blocks also have more involuntary switches and less summed CPU time. Those counters do not establish external preemption or justify excluding blocks. A distinct fixed-width implementation could test the call-boundary hypothesis; this unchanged candidate will not be rerun.

## Validation and scope

All 111 native core tests, two raw tests, six cubic tests, generator check, and portable/no-IFMA consumer checks pass. New helper tests compare empty/odd/even dot products through length 13 with separate multiplication/addition across boundary and random input families, and exercise actual width-three dense/partial constants. Existing independent symbolic fixtures compare 384 complete-buffer executions across widths 2–13, normal/reversed operands, omitted/materialized C, and adversarial inputs; 48 malformed recognitions and 48 tail-error comparisons are retained. Field source and prior qualification are unchanged; no new field-suite run is claimed.

All nine circuits pass a four-worker preflight for both fresh and fused binaries: 72 SDK requests and 18 independent Gnark exports. Every exported proof from subsequent measured stages is also checked for its 164-byte length, pinned public-input hash, fresh identity against the complete preceding ledger, and independent Gnark verification. The inventory does not count internal priming proofs as exported identities.

Total **432 SDK requests**, **36 distinct independently verified exports**, **72 proof/public members**; next proof-ledger count **6982**. Full stage executed: **false**; new cycle profiles: **0**. The full all-eight CPU10× objective remains active and unmet; no production adoption follows from this report.

No owned helper, build, hash or archive runs alongside SDK timing/profile captures. All pins and proof members are checked again at close. No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, memory-pressure, concurrent-prover or throughput qualification is claimed.

fresh executable: `/tmp/zopro-poseidon-fused-dot/native/fresh-sdk`; SHA-256 `d847ae9fd2ae52c6c74172a95004780eaa5481ca2a5f8889d7c9d0063dcd1eaa`.

fused executable: `/tmp/zopro-poseidon-fused-dot/native/fused-sdk`; SHA-256 `bffba6406ea1833f06362e078a9fccb2dba183128bdc2d870eba872c1f9f51d1`.

[Design](artifacts/design.md), [implementation audit](artifacts/implementation-audit.json), [screen analysis](artifacts/screen-analysis.json), [native tests](artifacts/native/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
