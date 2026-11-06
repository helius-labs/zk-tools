# LLVM scheduling-model applicability audit

The scheduling tool parses the tested IFMA forms, but **no calibrated Ryzen9950X performance model or new runtime candidate is selected**. The `znver5` CPU name in upstream LLVM20.1.2 and 22.1.8 resolves to `Znver4Model`. This audit changes the next action: obtain independent hardware/resource evidence before relying on simulated costs to choose another kernel.

## Evidence

The benchmark Rust toolchain reports 1.98.1 with LLVM22.1.8. The [matching upstream LLVM tag](https://github.com/llvm/llvm-project/blob/llvmorg-22.1.8/llvm/lib/Target/X86/X86.td#L1994) explicitly assigns `znver5` to `Znver4Model`, with separate Zen5 feature/tuning sets. The custom Rust LLVM patchset was not inspected, so this is an upstream-source finding rather than proof of exact compiler-fork contents. Shared scheduling classes do not imply identical Zen4/Zen5 generated code.

No llvm-mca binary was installed initially. Two Ubuntu packages, LLVM20.1.2 and its shared library, were downloaded into a private research directory, verified against apt SHA-256 metadata and extracted locally. No system package or production dependency changed. The running tool recognizes `znver5` and exposes only `Zn4` resource names in its model output. Downloaded packages, executable and library remain local with hashes.

One fixed model-only input contains eight instructions: four IFMA register/memory forms, vector addition, logical/arithmetic shifts and a vector stack store. All eight parse and receive scheduling data without unsupported-instruction skipping or stderr. These instructions are never assembled or executed as a benchmark. A100-iteration simulation describes 800 modeled instructions, not measured CPU work.

|IFMA model form|Reported latency cycles|Reported reciprocal throughput|Reported model micro-ops|
|---|---:|---:|---:|
|Register low|4|2|1|
|Register high|4|2|1|
|Folded-memory low|10|1|1|
|Folded-memory high|10|1|1|

The faster modeled folded-memory throughput is not evidence that adding loads is faster on hardware. These are uncalibrated scheduling entries; no missing resource or actual timing error is proved. No model parameters were patched, tuned or swept, and no full-consumer simulation was run using this inherited model.

## Limits and next step

The [LLVM documentation](https://llvm.org/docs/CommandGuide/llvm-mca.html#instruction-flow) states that the model omits frontend fetch/decode and branch prediction. Its [memory-model limitations](https://llvm.org/docs/CommandGuide/llvm-mca.html#load-store-unit-and-memory-consistency-model) also exclude cache hierarchy and store-to-load forwarding. Those omissions matter for the large, stack-using G2 consumers. Recognizing an instruction or CPU name is insufficient to qualify a bottleneck diagnosis.

Next, use actual saved consumers and independently checked dependency/work accounting, with explicit hardware calibration where needed. Do not infer a compiler defect, an AWS/Aeglos explanation, a removable wall-time fraction or a new field algorithm from this audit. Earlier successful models/captures/decodes and rejected arithmetic/scheduling implementations remain closed.

Preparation failures are retained in the research notes: the initial process launch named a not-yet-created working directory and performed no work; the first package-path check omitted apt's encoded epoch and stopped before extraction. Correct filenames were then verified before one extraction. CPU-help intentionally exits1 after listing targets, which stopped that command sequence after successful extraction/version/help output; those successful steps were not repeated. An incorrectly named scheduling source and a nonexistent separate Zen5 source/processor file returned404; the actual model mapping was read from X86.td. No benchmark, proof, build or successful model run was repeated for these repairs.

## Accepted baseline

Source176/SDK6d83 remain unchanged. Accepted local 32-worker1×2 is 7.928660 ms and merge54 is 161.883899 ms. Inherited 132.75 MiB RSS is neither a new peak nor minimum RAM. Ledger30846 is unchanged. The all-eight CPU 10× goal remains active and unmet (0/8); inherited [M5 CPU and Metal comparisons](../2026-11-06-g2-validated-capture176/README.md#accepted-proof-baseline-unchanged) remain current. No new proof timing, AWS/Aeglos, device, huge-circuit, pressure/swap or concurrency measurement is added.

[Analysis](artifacts/analysis.json), [model output](artifacts/coverage.json), [model input](artifacts/ifma-coverage.s), [audit source](artifacts/audit.rs), [local input hashes](artifacts/pins.json), [tool version](artifacts/version.txt) and [checksums](SHA256SUMS) preserve the evidence. Third-party LLVM source and binary packages are not redistributed; upstream URLs/tags and local hashes identify them.
