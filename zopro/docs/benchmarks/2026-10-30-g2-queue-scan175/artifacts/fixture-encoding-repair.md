# Fixture encoding correction before timing

The first independent oracle rejected the first point before completing either MSM. The fixture preparer correctly wrote the output of public decompress_g2 but incorrectly described it as little-endian Montgomery coordinates. That API returns canonical big-endian x.c1,x.c0,y.c1,y.c0, exactly the format consumed by decode_g2 in the timing driver. Scalar words remain canonical little-endian Fr.

Retain both successful fixture preparations and their two solver calls. Do not modify fixture bytes or repeat preparation. Correct only independent oracle decoding and add a corrected encoding receipt for the existing manifest. Original source, binary, build log and failed workflow are retained. A separate two-step plan runs the corrected oracles. This does not change candidate arithmetic, workload, timing protocol or output comparison.

A source review also found decode_g2 consumes the kernel's Montgomery format, not Gnark raw. Before any native driver execution, replace that call with explicit canonical big-endian Fq parsing and c1/c0 reordering. Preserve all three first-link binaries and their receipts as pre-encoding-repair artifacts. Relink all variants with the same corrected driver. No successful runtime preflight or timing is repeated; none ran before this repair.
