# zopro's Xark tools

What zopro needs from [Xark](https://github.com/ananas-block/xark) (the rev
the DvP circuits pin, 38c0de6), offline and in clients that solve with
Xark: the rows a circuit's proving key was made for, and the witness. The
keys themselves are read by `zopro_core::xark`, without Xark or arkworks;
`zopro-bench pack-xark` packs them with the rows into a zopro key.

```sh
tools/xark/pack.sh target/xark/my_circuit my_circuit my_circuit.zopro [metadata]
```

The directory is `xark build`'s and `xark setup`'s: `circuit.xbc`,
`pk.bin`, `vk.bin`, and `r1cs.stream` beside a minimized key.

The key's inputs are Xark's assignment z without its constant one: the
public inputs, then every other variable, in Xark's order. Xark solves the
witness, zopro checks every row and proves, and the proof verifies with
the circuit's `vk.bin`:

```rust
let circuit = zopro_xark::Circuit::load(&xbc, &pk, None)?;
let mut prover = zopro_prover::Prover::load(&std::fs::read("my_circuit.zopro")?)?;
let proved = prover.prove(&circuit.witness(inputs)?)?; // inputs: leaf name, value
let proof_bin = zopro_xark::xark_proof(&proved.proof)?; // Xark's proof.bin
```

`zopro-xark` (this crate's binary) has the commands behind `pack.sh`, and a
bench:

```sh
cargo run --release -- rows <circuit.xbc> <pk.bin> <out r1cs.stream>
cargo run --release -- witness <circuit.xbc> <pk.bin> <inputs.json> <out dir>
RAYON_NUM_THREADS=1 cargo run --release -- bench <circuit dir> <key.zopro> <inputs.json> [rounds]
```

`inputs.json` is one object of leaf names and values
(`ProgramProofInputs::to_json`). `witness` writes `<out dir>/witness`, which
`zopro-bench key-file <key.zopro> <out dir>` proves. `bench` proves the
inputs with Xark's prover and with the zopro key, every proof verified.

Keys of a smooth domain (`3·2^k` or `9·2^k` points) do not convert: zopro's
FFT is radix 2.

`cargo test --release` proves a small circuit with a key from Xark's setup,
compressed, uncompressed and without Xark's header, and checks each proof
with arkworks' verifier. With `ZOPRO_XARK_KEYS=<dir>` (circuits as
`<dir>/<circuit>/{circuit.xbc,pk.bin,vk.bin}`, e.g. a DvP program's
`target/zk/dvp-program`) and `ZOPRO_XARK_INPUTS=<dir>` (`<circuit>.json`),
it does the same for every circuit with inputs.
