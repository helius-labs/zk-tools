# Linear-operation count correction, before the first SDK capture

The frozen protocol's “eight fewer field additions/doublings” is an arithmetic
counting error. EFD gives old9add+3double+one quadruple. In the actual source,
quadrupling is two doubles, so the old method has14 linear field operations.
HMV has6add+one double=7. The source-level saving is **seven**, not eight.

The implemented formulas, binary artifacts, tests, codegen results, controls,
sample counts, order, timing boundaries, resource/statistical criteria and
decision are unchanged. Both still have eleven product-backend calls for Fq.
This note is written after plan review but before any SDK preflight or timing;
it does not require rerunning a successful build/test/analysis. Preserve the
original protocol as captured. Static linked counts already show the actual
implementation:2479→2295 instruction sites,352MULX in both. Those remain static
counts, not dynamic savings or a speedup estimate.
