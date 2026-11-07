# Current solver eight-term dot-product coverage

The current accepted solver retains **11,422 eight-term dot products per valid primary solve**. Across one solve of each of the nine retained workloads, the count is **1,085,089**. This establishes a real caller for the proposed horizontal accumulation before Montgomery reduction. No arithmetic operator, timing result or speedup is established yet; production remains unchanged.

The older raw-plan census counted 25,634 primary dots. Current recognized execution bypasses 14,212 of those opportunities. Reusing the old count as current coverage would overstate this target. This diagnostic follows the current admitted schedule and recognized-task dispatch instead of merely scanning every original constraint.

## Current-dispatch counts

Counts below are deterministic metadata-derived calls on a valid full solve, not executed field-operation measurements. The last column excludes constraints executed by recognized Poseidon spans. They are separate workloads, not one aggregate proof.

| Workload | Original raw-plan dot8 opportunities | Bypassed by recognized execution | Current interpreter dot8 calls |
|---|---:|---:|---:|
| 1×2 | 25,634 | 14,212 | 11,422 |
| 2×2 | 44,459 | 24,555 | 19,904 |
| 4×2 | 81,022 | 45,170 | 35,852 |
| 8×2 | 155,235 | 87,161 | 68,074 |
| 12×2 | 228,905 | 128,663 | 100,242 |
| 16×2 | 302,574 | 171,033 | 131,541 |
| 24×2 | 450,457 | 255,015 | 195,442 |
| Merge 24 | 431,707 | 240,205 | 191,502 |
| Merge 54 | 965,637 | 634,527 | 331,110 |
| Sum across workloads | 2,685,630 | 1,600,541 | 1,085,089 |

All nine cases use 32 workers and fused-sum mode 2. Their actual cached schedules are prepared by the ordinary key loader; none takes serial fallback. Recognized spans include the existing widths 3/4/5/8 where present. The census reads the existing schedule and never builds an alternative one.

## Method and checks

The isolated source starts from the exact accepted 176-file manifest. Four parents receive append-only, diagnostic-cfg code, and one new child implements the census. All 176 original files are recoverable and remain unchanged in production. No field arithmetic, recognition, schedule or execution body changes. The consumer maps the same pinned basis images through the normal prover loader; it does not solve, prove or consume a witness. Existing request and public-input hashes bind the workload provenance without creating a new export.

The walker follows outer tasks, refined waves and the current tagged-task format. Untagged and width-zero operations use the ordinary interpreter. Recognized blocks skip the same encoded span length as the current dispatcher. An exactly-once packed bitset covers every original op, including skipped blocks. For each raw constraint, a metadata-only cursor reproduces the existing eight-term, pair and single-term selection rules. Counts conserve both operations and sum terms: raw-plan totals equal dispatched plus recognized-bypassed totals, and `8*dot8 + 2*pairs + single` equals the sum-term count in each category.

One new unit test passes, covering empty/seven/eight/sixteen-term lengths, modes 0/1/2, last/internal stores, wire-zero, coefficient-four and non-Chain boundaries. Existing field/core/solver tests and fixture/proof qualification are inherited, not rerun. Every loaded coefficient's Montgomery words are below the modulus. This is not yet a complete provenance proof for every witness operand of a future arithmetic helper.

Plan serialization is streamed into a constant-sized SHA-256 sink before and after the census; lengths and digests agree. No second serialized plan or proving key is retained. The transient visitation bitset is diagnostic memory, not a new production allocation or a RAM bound. All nine image/request/public hashes, source pins, native flags and consumer identities are bound before the dataset reads.

The new consumer uses O3, native CPU targeting, fat LTO and one codegen unit. It is a diagnostic executable, not a replacement SDK. Independent review checks 13 successful native/census/analysis command receipts, nine image results, 1,230 source-pin occurrences, source recovery, current selection/tag literals and all count conservation. It does not independently execute a second solver parser or trace. The first reviewer had an incorrect source-text assertion naming `self.write_image` rather than `p.write_image`; only that failed review stage was repaired. Original source/logs remain. No successful census, test, build or old oracle was repeated.

## Arithmetic question selected

The existing `solver.rs::dot8` computes eight lane-wise products and then horizontally sums their reduced results. The proposed operation would sum complete integer products across lanes before one scalar R256 Montgomery reduction. It needs only one final field sum, which distinguishes it from bulk curve kernels that need eight independent results. It also differs from the existing horizontal-limb reduction after multiplication, rejected scalar three-product Poseidon code and closed CIOS recurrence experiments.

The [relevant paper](https://flokrieger.github.io/papers/ches26_spielman.pdf) studies delayed reduction for 128-bit Spielman-code fields. Its representation and gains do not transfer directly to BN254. No external implementation is copied. This is an independent proposed transfer to a specific current caller, with no point table, copied key, retained plan or new pool.

Next: independently qualify the full integer product/sum, canonical-input provenance, R256 scaling, quotient cancellation, carry bounds and final corrections before any native operator. A successful arithmetic model still needs actual pack/product/horizontal/reduction code generation, matched native caller measurements and verified full-SDK all-shape performance/resource checks. These counts neither predict a twofold gain nor satisfy the original goal.

## Accepted performance: adoption result versus later observations

The accepted SDK is still `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`.

Its [adoption benchmark](../2026-10-30-g2-vector-sign-adoption176/README.md) measured primary **7.928660 ms**. A later baseline reported **7.915095 ms**; the latest [matched comparison](../2026-11-07-pairing-half-constant176/README.md) observed **8.041470 ms** for the same accepted binary. These are different runs/protocols, not a slower code version being adopted. None is a new measurement from this census.

The table below preserves the adoption benchmark and the supplied M5 CPU/Metal references. Later observations remain in their own reports rather than replacing the adoption result.

| Shape | Local adoption ms | M5 CPU ms | M5 Metal ms | CPU 10× limit ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 52.8 |
| Merge 24 | 65.060937 | 507 | 175 | 50.7 |
| Merge 54 | 161.883899 | — | — | — |

All-eight CPU 10× status remains 0/8. Cross-machine references do not establish equal hardware efficiency. This stage creates zero proofs, solves, timings or profiles; the canonical verified-proof ledger remains 32,196. It supplies no fresh AWS/Aeglos, mobile, large-circuit, pressure/swap, minimum-RAM or throughput result. No AWS resource is created.

[Evidence archive](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json), and [archive checksum](artifacts/SHA256SUMS) retain the design, observer source, changed parents, driver, plans, receipts, counts and review. Images, witness/request/public/proof payloads, executable/library blobs and targets are excluded. Absolute paths identify the working environment; reconstruct them before executing plans elsewhere.
