# Single-pass fingerprint capacity on real scalar streams

Replay a strictly bounded, single-pass certificate in the debugger on the ordinary 1×2 reference and two independent witnesses. In the space of the release bitmap, each bank stores up to 16 distinct nonzero byte fingerprints. A fixed prefix of the existing multiplicative hash selects a bank and the next byte selects its fingerprint. Zero fingerprints map to one. Full banks ignore new values. This count can only underestimate the number of distinct nonzero input scalars.

Compare the number certified by this layout with the release bitmap, the two-pass bitmap, the exact count, and the original grouping threshold on the 21,006-scalar canonical query and 30,089-scalar G2 query. Capture the first stream at each query site and count all four site invocations. Verify each of the three exported proofs independently. The debugger computes counts on live input; these executions supply no performance comparison. Do not save the scalar values.

[Release grouping census](../2026-11-19-canonical-index629/README.md) · [Two-pass certificate](../2026-11-19-split-certificate636/protocol.md)
