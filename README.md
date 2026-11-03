# zk-tools

[zopro](zopro/): a Groth16 BN254 prover in Rust for gnark circuits, the
kernel the Zolana web and mobile SDKs prove with. One kernel for browsers
(WebAssembly, SIMD lanes, WebGPU), phones and laptops (native, AArch64
assembly, Metal or wgpu); gnark stays offline: a zopro key is built once
from a gnark constraint system and its keys (`zopro/tools/gnark`), and
gnark's verifier accepts zopro's proofs. See `zopro/README.md` for the
layout, the tests and the benchmarks.

**Experimental.** New and unaudited: do not rely on it where a wrong or
leaked proof would cost anything.

The SDKs pin a commit of this repository: `zolana-web-sdk`'s
`runtime.lock.json` (repository, path `zopro`, revision) builds the browser
runtime from it, and `zolana-mobile-sdk`'s native crate takes `zopro-zolana`
as a git dependency of it.

The Go bridges this repository first held (`cgo/`, `prover-wasm/`, gnark's
prover for the SDKs' earlier native and wasm paths) are in the history
before zopro arrived; nothing builds them any more.
