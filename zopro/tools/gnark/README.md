# zopro's gnark tools

What zopro needs from gnark, offline, with gnark alone (v0.16.3, the version
that wrote the keys): a Groth16 BN254 constraint system's and its keys' parts
as zopro's kernel takes them (a dump), which `zopro-bench pack` packs into a
zopro key.

```sh
tools/gnark/pack.sh circuit.r1cs circuit.pk circuit.vk my_circuit my_circuit.zopro [metadata]
```

The files are gnark's `WriteTo` (or `WriteRawTo`) of the `R1CS`, the proving
key and the verifying key. `metadata`, when given, is kept in the key as it
is, for the application (`Prover::metadata`). Go callers that hold the
objects call `zoprognark.Dump(dir, r1cs, pk, vk, metadata)` and pack the
directory with `zopro-bench pack <dir> <name> <out>`.

A circuit the kernel cannot prove alone fails here: commitments (gnark's
range-check API commits), or hints other than gnark's `InvZeroHint` and
`bits.nBits`. `go test .` dumps a small circuit; with `ZOPRO_CUBIC=<dir>` it
writes zopro-prover's test fixture there.
