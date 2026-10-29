# History and changed mechanism

3587a185 packed factorization computes many discarded lane values:14vector
linear ops and15shuffles for two transforms. It lost12% despite1ratherthan2
vector products. That exactvectorfactorization remains closed. The new tile
changes SIMD axes:8independentDFTs instead of4adjacententries perhalf. Its
4M/26linear/32transpose-shuffle schedule over eightDFTs differs from oldpacked
8M/24linear/32within-shuffle, and from rejected packed4M/56linear/60shuffle.
No causal cost is inferred from these static counts. Full native DIFleaf/SDK
will charge transpose, setup, stack and memory effects, not just4products.

Old fixed/serial FFTleaves, rootprecompute/cache, passgrain, Eulercompact,
Euleroutputfusion and FFTcolumnprefetch studies change other work/geometry
and remain closed. Current accepted dif_serial source has no cross-sequence
transpose/factored8tile. No newexternalcodeported. Independent algebra from
previousalgebra.md reused with provenance; noGPLimplementation transplanted.
PaperHLS performance/counts notCPU predictions. No field/kernel/MSM change.
