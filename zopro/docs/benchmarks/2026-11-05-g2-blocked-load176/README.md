# Same-size blocked G2 input loader: native qualification

The experimental loader stores eight points as sixteen contiguous coordinate-word planes. Its payload remains exactly **128 bytes per point**, including a short final block, and encoding uses the caller's existing allocation. **Native correctness qualification passes; no timing improvement or adoption is established.**

The [preceding complete-query census](../2026-11-05-g2-pack-locality176/README.md) found that approximately 81% of full groups touch exactly two logical source blocks. The loader handles aligned loads, single-block permutations, two-block permutations, and general gathers. Every path performs the existing mixed-R256 word-to-limb conversion. Original field arithmetic, generated IFMA source and generator, core, and prover stay byte-exact.

Encoding validates all coordinate fields before its first mutation, then transposes each block with a fixed 8×16-word tile. A partial block uses its actual point count as plane stride and requires no padding. The tile contains 1,024 logical bytes; compiler stack frames, process memory peaks, and minimum RAM have not been measured. There is no expanded radix-52 key, second retained point table, lookup cache, execution plan, or new production worker pool.

## Validation

Two new focused native tests pass on their first execution. They cover empty, short, full, and partial blocks; exact reconstruction; pointer/capacity stability; invalid-input rejection before mutation; all one-through-eight-source-block cases; repeated and reversed indices; partial-block combinations; and 8,192 deterministic random index groups. Every packed limb and scalar word is compared with the original constructor and a separate scalar representation oracle. The other 35 field tests are filtered and inherited. Synthetic field coordinates need not be curve points: this is storage/codec qualification, not a curve, MSM, or proof test.

Three native workflow commands complete successfully: focused tests, coherent library build, and transitive library freeze. Receipt/source review passes on its first execution, checking **244 source/library pins**, protocol and stream hashes, test receipts, exact child source, and recovery of all 176 accepted source hashes. The snapshot has 177 files, with 175 originals exact and one append-only module export. The [bounds argument](artifacts/bounds.md) is source reasoning supported by tests, not a formal machine-code proof.

The accepted SDK and production source remain unchanged. There are no new timings, SDK requests, proof verifications, PMU captures, CPU profiles, AWS comparisons, device tests, or pressure/swap measurements. The proof ledger remains 29,953.

## Next measurement

Replay the complete private index distributions from both census populations against original, fresh-control, and blocked loaders at 1/16/32 workers. Include dispatch, index lookup, radix conversion, and all irregular groups. Use the actual witness-dependent grouped 2×2 root; it is not the first slice of the original key. Shared inputs and caller-local work must reflect their real ownership. Any codec-only measurement excludes setup and must be followed by complete caller/query/SDK measurements that charge conversion and lifetime overlap.

A positive codec result alone does not justify a key-format or point-API change. Typed source ownership, grouped-point emission, loader compatibility, portable access, GPU behavior, full MSM correctness, and resource use still require qualification.

## Accepted CPU and Metal comparisons, inherited

These accepted 32-worker local full-proof results and supplied historical M5 references are **not measurements of this loader**. See the [accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). Latest separate full-SDK evidence remains [finalization/drain composition](../2026-11-05-finalize-drain-bundle176/README.md), preserving its primary gain and failed full gate.

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

Merge54×1 remains 161.883899 ms without a supplied M5 reference. The all-eight CPU10× goal stays active and unmet, **0/8**.

## Artifacts

[Loader](artifacts/blocked_input.rs), [design](artifacts/design.md), [bounds](artifacts/bounds.md), [review](artifacts/review.json), [source audit](artifacts/source-audit.json), [reproduction](artifacts/reproduce.md), [source archive](artifacts/source.tar.gz), [exact receipts](artifacts/receipts.json.gz), [checksums](SHA256SUMS).

Source archive member names and hashes are checked after extraction. The receipt archive preserves exact UTF-8 member bytes, lengths, and hashes. Executables, libraries, point/scalar/key/image/witness arrays, and private input-dependent ordinal transcripts are excluded.
