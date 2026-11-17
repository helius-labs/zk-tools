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

[clients/spr-prover](clients/spr-prover/) is the web and Node client,
`@heliuslabs/spr-web-prover`, with the wallet and benchmark examples beside
the package. `clients/spr-prover/runtime.lock.json` pins the zopro revision
that client builds. The mobile SDK takes `zopro-spr` as a git dependency of
this repository.

The Go bridges this repository first held (`cgo/`, `prover-wasm/`, gnark's
prover for the SDKs' earlier native and wasm paths) are in the history
before zopro arrived; nothing builds them any more.
