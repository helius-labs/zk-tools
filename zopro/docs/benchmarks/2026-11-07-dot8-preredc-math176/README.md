# Eight-term product accumulation before reduction: integer model

The integer model passes **63,136 cases** across the BN254 scalar and base fields. It checks complete raw products, horizontal accumulation, packing, R256 quotient cancellation, carries and canonical output against independent BigUint calculations. This qualifies the proposed arithmetic for an isolated native implementation. **No native operator, speedup or production change is established.**

The [current-dispatch census](../2026-11-07-dot8-dispatch176/README.md) finds 11,422 primary interpreter dot8 calls. The proposed operation sums eight complete integer products before one scalar Montgomery reduction, replacing the current eight lane-wise reductions followed by a horizontal sum. It needs no heap allocation, retained plan, key copy or point lookup table. Compiler stack usage and complete caller performance remain unmeasured.

For canonical operands below p and R=2^256, both primes satisfy p<R/4. Thus T=sum8(a*b)<8p²<R²/2. With q=(-T*p^-1) mod R, the unreduced value u=(T+qp)/R<3p<R. Two final conditional subtractions suffice. The model constructs ten raw radix52 coefficient vectors, horizontally accumulates eight lanes, normalizes and packs all512bits, then performs four scalar cancellation rows. It independently checks the entire integer T, q, u and final modular result. Raw coefficient bounds are below72*2^52, with carry normalization below73*2^52. Lazy inputs below2p are outside this contract.

| Field | Cases | Zero corrections | One correction | Two corrections |
|---|---:|---:|---:|---:|
| Fr | 31,568 | 22,345 | 6,597 | 2,626 |
| Fq | 31,568 | 22,261 | 6,663 | 2,644 |

Each field has 146 unique canonical boundaries and their 21,316 cross-product vectors, twelve explicit zero/extreme/alternating/isolated-lane cases, 8,192 deterministic random vectors and 2,048 dependent-result vectors. Examples for all correction counts are retained. These are arithmetic populations, not observed witness distributions. The release model enables overflow checks. The lock, build and model commands all succeeded on their first execution; the model is not replayed for publication. Offline review verifies 367 source-pin occurrences, the three command receipts and output hashes, population conservation and bound assertions. It is not a second model execution or a cryptographic verifier.

The source provenance audit establishes canonical inputs for ordinary valid SDK assignments: checked field constructors, validated solver coefficients, scalar arithmetic, inverse/bit hints and recognized Poseidon writes preserve reduced Montgomery words. The old dot8 also returns a canonical scalar. The raw decoder and public unchecked field constructors do not validate arbitrary caller-supplied words; existing field arithmetic already requires reduced inputs. This stage adds no public input validation and makes no claim for malformed unchecked values.

Next: implement an isolated complete native pack/product/horizontal/reduction consumer, qualify it against this model, inspect emitted instructions and stack, then measure matched complete-caller performance. A local kernel gain still requires verified full-SDK all-nine performance/resource checks before adoption. Removing vector reductions may lose to horizontal transfers or scalar carries; no speed prediction follows from the model.

Accepted performance remains the [adoption benchmark](../2026-10-30-g2-vector-sign-adoption176/README.md), using SDK SHA256 `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. Primary adoption measured **7.928660 ms**, a later baseline **7.915095 ms**, and the latest matched comparison **8.041470 ms** for the same binary. These are distinct observations; no slower version was adopted.

| Shape | Local adoption ms | Supplied M5 CPU ms | Supplied M5 Metal ms | CPU 10× target ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 52.8 |
| Merge 24 | 65.060937 | 507 | 175 | 50.7 |

Merge54 adoption was161.883899ms, with no supplied M5 reference. All-eight CPU10× status remains **0/8**. Cross-machine references do not establish equal hardware efficiency. This model creates zero proofs, solves, profiles or benchmark observations; the canonical verified-proof ledger remains32,196. No new AWS/Aeglos, mobile, large-circuit, pressure/swap, minimum-RAM or throughput result is supplied.

[Evidence](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json) and [archive checksum](artifacts/SHA256SUMS) include source, fixed design, provenance audit, lockfile, plans, command receipts, examples and review. No target directories, executable/library blobs, proving fixtures or witness/request/public/proof payloads are included. Absolute paths identify the working environment and must be reconstructed elsewhere.
