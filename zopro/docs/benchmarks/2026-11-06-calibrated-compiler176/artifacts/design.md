# Calibrated compiler scheduling feasibility

Production176 and the accepted SDK remain unchanged. The previous split-REDC candidate is closed, including its 1/16-worker gains and 32-worker losses. The all-eight verified CPU10x goal remains active, with the supplied CPU and Metal comparisons retained; this experiment alone does not predict a twofold gain.

## Hypothesis

Compile the exact accepted Rust consumer with a local LLVM backend whose znver5 scheduling entries correct only measured register instruction costs. The Rust1.98.1 source commit 48a229ceaefd4985c50990b14116b6d856af0985 pins rust-lang/llvm-project submodule52ed14fcd56afc30f9cccd8ca8ce237c2eef7e04. Its X86.td aliases znver5 to Znver4Model. The actual fork has a four-cycle resource reservation for register IFMA and one-cycle ZMM integer add/shift latency; our completed hardware calibration measured approximately0.5 cycles/IFMA throughput,4-cycle recurrence,2-cycle add/sub/52-bit-shift latency and0.25/0.5 add/shift throughput. No new calibration is required.

This tests compiler scheduling, not a new field arithmetic recipe. Do not reuse the rejected split-REDC implementation. No target-width/layout/annotation/threshold sweep, key table, point copy, production pool, runtime dispatch or C/assembly prover rewrite is selected. LLVM itself is an external compiler written in C++; prover source stays Rust.

## Fixed scope before native work

Use a separately named copy of the exact inherited Znver4 model, mapped ONLY to znver5 in the owned ON backend. Keep all unrelated model entries byte-identical modulo the model/resource namespace. Change only unmasked ZMM register low/high IFMA (latency4, one cycle on the inherited two-member FP resource group) and unmasked register packed-qword add/sub (latency2, one cycle on the inherited four-member group). Leave masked and memory forms, all shifts, other widths and other processors unchanged. This is deliberately a partial correction: the known52-bit-shift model mismatch remains; applying a generic immediate-shift override would extrapolate beyond the calibration. Shared physical port mappings remain inherited and are not independently validated.

Build frozen OFF/ON llc and llvm-mca from the same pinned Rust LLVM tree, flags and host compiler. Build common dependencies once; the second build changes only the scheduling source and affected generated tables/objects. Verify the generated schedule for selected and untouched forms, and separately test exact52 applicability before consumer execution. Static model agreement is a prerequisite, not a speed prediction. Preserve setup failures; never repeat successful capture/decode/test stages for bookkeeping.

Generate one new accepted-Rust actual-caller bitcode with native/O3/fatLTO/oneCGU flags and coherent accepted libraries, preserving the optimized bitcode and original linker command. Compile the SAME final optimized IR through both frozen backends with identical code-generation options. First prove the re-link recipe and exact target/ABI/code-generation provenance; inspect complete G1/G2/inverse consumers and check that no source arithmetic or API changed. If model selection or consumer provenance cannot be established, close this feasibility stage without a timing trial.

Only a credible new final caller advances to separately frozen correctness and matched timing/profile protocols: both curves, original full/tail sizes,1/16/32workers, both controls, unchanged capacity/zero allocations and independent point outputs. Do not invent timing evidence or promote a favorable subset. A qualified caller win still requires complete SDK all-eight plus merge54, independent Gnark/freshness, portable and resource checks before adoption. Compiler-only change may not improve every caller or the full proof.

## Resources and stop rule

Compiler builds are local research work, not prover memory use. No owned performance recording may overlap a build, hash, model analysis or archive. Do not install/replace a system compiler. Use a private pinned source archive because the connected GitHub source reader cannot download binary archives. Original and patched source deltas and build recipes must be reviewable and reproducible. A source-audit finding, model match or static count does not establish a speedup, minimum RAM, phone/TEE qualification, AWS result or Aeglos comparison.

If this fixed correction cannot affect the actual scheduling path, or final code is unchanged, close it using those receipts. Otherwise freeze the next decisive caller protocol before measuring. No parameter search or unchanged retries.
