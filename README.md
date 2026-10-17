# ringil

A Groth16 BN254 prover in Rust that proves from key images: the kernel's
point vectors and solver plan as a host keeps them, with nothing decoded or
rebuilt at load. One crate for the browser (WebAssembly, SIMD lanes) and for
phones and laptops (native), with GPU backends to come.

`crates/ringil-core` is the arithmetic and the witness solver, `crates/ringil-bench`
proves a dumped shape natively and times its stages
(`RAYON_NUM_THREADS=4 ringil-bench <dump dir>`; `--features ringil-core/trace`
prints each stage). The dumps and the proof referee are `TestRingilDump` in
the web SDK's `wasm/prover-wasm`.
