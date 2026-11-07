# Historical batch-affine source comparison

The inspected CycloneMSM draft supplies no new arithmetic reduction for the accepted ordinary G1 path. Its normal affine batch uses **5M+1S per point plus one shared inversion**. Zopro's full SIMD groups use six field-product operations per lane, including the square, plus shared-inverse overhead. No implementation or performance trial is selected by this source audit.

## Source and comparison

The source is [gnark-crypto PR249](https://github.com/Consensys-Incorporated/gnark-crypto/pull/249), a **closed, unmerged historical draft**, at [head 33b4576](https://github.com/0x0ece/gnark-crypto/blob/33b4576efd99457757fe103ff99a5f8429b3a368/ecc/bls12-377/multiexp_affine.go). It is not the current Gnark CPU prover or current gnark-crypto MSM implementation. The complete file was read; its Git blob 417838a31072830bcdbdf4ce6b607d735127ad44 and SHA256 identities are retained.

The draft's BatchInvert makes one forward and two reverse multiplications per element. Normal BatchAdd adds a numerator multiplication, slope square and y-coordinate multiplication: 5M+1S total. Its doubling branch adds a square. The source comment's 5M excludes the ordinary square; treating it as a five-product replacement would undercount work.

Accepted G1 has two products per full-group forward step and four in reverse. It uses eight lane-wise chains and the existing shared-inverse helper, so its constant inversion overhead differs from the draft's scalar chain. Tails and exceptional paths are outside this ordinary-group count. These arithmetic units, curves, representations and SIMD implementations have different costs; equal formulas do not imply equal runtime.

The draft scheduler fetches fixed candidate batches of 40 or 100, filters conflicting destinations by scanning the current batch, and appends conflicts to a pointer queue. After input exhaustion it consumes and requeues conflicts. Processed queue entries remain stored and no hard bound is implemented in this draft; the author explicitly discusses bounding it before a production contribution. Zopro instead fills bounded batches using busy bitsets and retains its bounded queue/projective fallback. No queue implementation is ported here.

The author's reported 10–20% gain concerns large BLS12-377 populations, `N≥2^22`. It is not transferred to the 30k-point BN254 primary query. Prior Zopro [bounded queue scanning](../2026-10-30-g2-queue-scan-count175/README.md), drain and routed-round experiments stay closed. This review identifies source differences, without reopening those implementations or asserting that either scheduler is universally preferable.

## Checks and limitations

A read-only Rust/xtask helper checks source identities, named function intervals, branch-specific multiplication/square sites, queue operations, selected accepted source expressions and all 176 unchanged production hashes. It saves function-body hashes and derived arithmetic counts. No upstream Go code is compiled, executed or adopted.

The first helper incorrectly expected two static BatchAdd multiplication sites: there are three, with two in mutually exclusive lambda branches. The corrected helper distinguishes the ordinary executed branch. A second attempt stops because its internal pin path still names the original helper manifest; the final helper accepts the active manifest explicitly. Both earlier source versions and failed receipts remain. Only this failed source-audit stage is repeated; no successful prover qualification, capture, model or verifier is replayed. The final audit passes. It is a source assertion check, not an independent cryptographic or machine-code proof.

There are no new prover builds/tests, SDK calls, Gnark exports, profiles, latency or memory measurements. Production176/SDK6d83 and canonical proof ledger 31,913 remain unchanged. No cloud resource is created. No phone, pressure, concurrent-proof or 250/4999 qualification follows.

The inherited local 32-worker 1×2 observation remains **7.915095 ms**, versus supplied M5 CPU 37 ms and Metal 24 ms; merge54 remains 161.718956 ms without an M5 reference. [All eight accepted CPU/Metal comparisons](../2026-11-07-whole-field-census176/README.md) remain unchanged. The verified all-eight CPU 10× goal is active and unmet, **0/8**.

[Artifacts](artifacts/) preserve the own helper versions, source identities, design, derived counts, plans, pins and exact receipts. Full upstream source/diff stays local with links and hashes; private key/image/request/witness/proof payloads and executable/library blobs are excluded.
