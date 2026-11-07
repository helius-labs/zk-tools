# Single-vector common-Z G1 finish qualification

The new standalone SIMD algorithm passes correctness qualification. **No latency result, complete-caller integration or production adoption is established.** Accepted local1×2 full-proof latency remains7.915095ms.

This implements the [qualified common-Z construction and cost screen](../2026-11-07-common-z-product-tree176/README.md) for at most sixteen live G1 points, one eight-lane pair vector at the first level. Butterfly pair/quad/oct products form a broadcast denominator product and every all-other cofactor using five vector multiplications. The algorithm retains a packed common Z between levels and pays one accepted scalar inverse for its final affine result. Intermediate coordinate get/new conversions remain in the implementation and must be charged in measurements.

The single-pair level uses the ordinary equal-Z formula. Odd carried points rescale by the common square/cube. Initial identities, equal-x pairs at any level, and unsupported lengths decline before returning a speculative result. Complete original projective addition reduces the original immutable inputs. This standalone fallback is mathematically complete; it is **not yet integration with the accepted group_affine reducer**. It does not qualify weighted trees, G2 SIMD, portable targets, memory pressure or concurrent proof throughput.

One pinned Rust/xtask workflow builds and qualifies the prototype, both commands succeeding on their first executions. The actual final compile uses O3, native CPU, fat LTO and one codegen unit. The unchanged accepted field library is `1200e635…`; its existing arithmetic suites and the completed parent algebra screen are inherited, not replayed.

- 288 butterfly vectors check active counts0..8, inactive lanes equal to one, the broadcast full product and every cofactor against direct scalar products.
- 150 complete cases cover eight full-width-offset generator walks, every input length0..17, and initial/late equal, opposite and identity cases. Complete original addition and generator-weighted expected sums agree. Inputs remain unchanged.
- 22,050 further complete calls pass through all49 worker indices across1/16/32-worker Rayon pools requesting512KiB stacks. This checks execution on those pool workers; it does not establish CPU affinity, a measured stack peak or performance scaling.

The kernel uses a fixed sixteen-coordinate-pair array and existing public IFMA operations, with no unsafe code or heap scratch. The fixed array alone is1KiB; compiler temporaries and vector spills are additional. No key table, retained key copy, duplicate solver plan or production pool is introduced. The qualification executable hash is `362a89e7268bc1669dc40cbbf647f7869fe4611213708316c8ad02d47cbf8d1e`; the executable and libraries remain local.

The next required stage is a complete unweighted-reducer integration and prospective matched comparison with the original accepted reducer and a fresh control. It must include packing, exceptions, normalization, dispatch and allocations, retain unfavorable sizes and worker contexts, and profile the real caller. In particular, independently finishing many small groups could lose the accepted cross-group inversion sharing; standalone single-group gains would not justify that replacement. No threshold sweep or favorable-subset adoption is selected.

All-eight accepted CPU/M5 CPU/Metal comparisons and target limits are in the [parent report](../2026-11-07-common-z-product-tree176/README.md#accepted-comparisons-unchanged). Goal status remains active and unmet,0/8. No new proof, Gnark export, AWS/Aeglos result or proof-ledger change:31,387 distinct exports remain recorded.

[Design](artifacts/design.md), [kernel](artifacts/kernel.rs), [qualification source](artifacts/qualify.rs), [results](artifacts/qualification.json), [source/library pins](artifacts/pins.json), [workflow](artifacts/plan.json), [checksums](SHA256SUMS).
