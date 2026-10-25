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

The solver program is streamed to its dump file. Importers that write the
five point-vector files first can release those arrays, load the constraint
system, then call `DumpSolver` to write the remaining files. This function
needs the proving key's fixed points, domain and infinity masks, but not its
point vectors. The caller must provide vectors from the same key.

`Prover::write_dump_image` accepts mapped dump parts and writes a raw image
directly, avoiding an intermediate compressed point image and a second
serialized solver allocation. New packed keys use format 3 and raw images
use `zopro-raw-4`, with 64-bit blob/header/solver byte lengths. Older packed
keys and `zopro-raw-3` images remain readable. Wire and constraint indices
retain their existing bounds; wider byte lengths do not guarantee a circuit
fits available memory.

New raw images omit the deflated source program and coefficient table from
their headers: both are already represented by the serialized solver plan.
Keep the original key or dump to rebuild a cache if a future solver-image
version becomes incompatible. Packed keys still contain the source program.

A circuit the kernel cannot prove alone fails here: commitments (gnark's
range-check API commits), or hints other than gnark's `InvZeroHint` and
`bits.nBits`. `go test .` dumps a small circuit; with `ZOPRO_CUBIC=<dir>` it
writes zopro-prover's test fixture there.
