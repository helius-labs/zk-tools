# Proposed next experiment: consume owned G2 batches in two finishing phases

This is a design proposal, not implemented or measured. The counter evidence describes whole warm SDK execution; it does not identify register spills as an exclusive bottleneck or predict a gain.

The accepted `msm/g2_prefix.rs::apply` computes norm/prefix records, then one reverse loop computes the inverse recurrence, slope, x and y together. The current `Window::execute` owns each input batch and clears it immediately after applying it. For a full batch, its incoming x coordinates are dead after slope and x are formed. The two existing prefix-record slots are also dead once their inverse-recurrence step is consumed.

Use these existing, now-dead storage locations rather than allocate another record buffer:

1. Keep the forward norm/prefix pass unchanged.
2. In the reverse pass, compute the same inverse recurrence and slope, then the new x coordinate. Replace that group's consumed `[prefix,norm]` record with the two FqBatch slope components. Before storing new bucket x, replace each owned incoming point's x with the original bucket x. Preserve original bucket y.
3. In a separate forward pass, load the retained slope, the saved original x from the consumed input batch, the new bucket x, and original bucket y. Compute the unchanged `y = slope*(old_x-new_x)-old_y`, then store y.

The two record slots are reused and the batch already exists. Persistent key data, record capacity, point count, field formulas and number of multiplications are unchanged. There are extra owned-batch x stores and another indexed pass. They may outweigh any reduction in live vector operands or caller complexity.

Expose an explicitly destructive private batch method. The default trait path retains ordinary immutable application. Select the new G2 path only when native IFMA8, prefix records enabled, direct mode active and the batch is a nonempty multiple of eight. Ragged batches, disabled/portable paths and immutable tree consumers use the existing implementation. Window clears the consumed batch after application and must not inspect its point coordinates. Bucket indices remain unchanged and are pairwise distinct, with nonzero x differences, as required by the existing batch contract.

This is distinct from the rejected G1 finish-phases experiment, which overwrote existing rich numerator/xsum records, and from the rejected G1 scratch-store specialization. G2 prefix records retain only two Fq batches, so the proposal additionally consumes the input batch's dead x coordinates to avoid introducing a third scratch record. Neither previous negative is rerun or relabeled as a gain.

Before timings: compare complete G2 additions with independent generator-weight or projective oracles, including nontrivial index permutations, positive/negative inputs, all inverse/canonical modes, reused scratch and ragged fallbacks. Verify the consumed-batch x postcondition and untouched indices, and audit every affected caller for post-use reads. Existing core/MSM/raw/cubic suites remain required. No new field primitive needs a standalone microbenchmark.

Inspect actual linked G2 caller code and run a separately frozen accepted/OFF/ON whole-SDK comparison across all nine shapes and physical16 primary, with independent exported proofs and warm profiles. Keep every result. A positive result still needs concurrency, small-stack, memory-pressure and large-circuit qualification; no adoption based on source lifetime or static code counts.
