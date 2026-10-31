# Aggregate exception test and mutation order

For a batch with distinct destination indices, each destination is independent.
The feeder admits only finite incoming and current affine points; the original
identity shortcut and busy-bit rule are retained. Define d_i = bx_i - ax_i in
Fq. A product of d_i is nonzero exactly when all d_i are nonzero, since Fq is a
field. The original eight per-lane chains, scalar tail factors and central
product tree already compute that product. Canonical export before its scalar
inverse gives the ordinary field-zero semantics, including lazy zero residues.

If the inverse exists, every original slope and reverse formula is valid. If
it does not exist, the checked helper returns before both the scalar-tail
reverse loop and the vector reverse loop. The speculative forward pass has
only written scratch, not destination buckets. Complete addition can therefore
use the original bucket and signed input for every distinct destination. Those
outputs represent the same mathematical sum as the original feeder's deferred
projective doubling and immediate cancellation cases. Subsequent queue/finish
operations preserve that sum, although their intermediate representation and
exception routing can differ.

This does not remove the original need for valid curve points or unique pending
indices. It does not assume exceptional cases are rare for correctness. Rarity
would affect performance: any exception discards that batch's speculative
forward work and invokes complete scalar additions for the whole batch.

The checked inverse preserves the accepted product-tree arithmetic and adds no
field multiplication. This is a source-operation statement; Option handling,
register allocation, code size and actual CPU latency must be measured.
