# Source and history audit

The idea is window-major, proof-local signed digits, not native bucket columns, a sorting/tree fill, a transformed key table, or a retained witness cache. Current original Window/feed/batch/kernel/finish stay exact. No external code is copied; the new decoder calls Zopro's original Booth reader.

Primary source inspiration: TACEO ark-algebra0.1.0, source VCS9236cfeae662456b6db4def818303e3783722e6a, ark-algebra/src/msm.rs. Published crate downloaded from https://static.crates.io/crates/taceo-ark-algebra/taceo-ark-algebra-0.1.0.crate and kept local at ../msm-source-study176; its code/license stay local with hashes in fixture-bindings. It materializes window-major u32 digits and implements generic arkworks/gnark-derived batch-affine MSM. This experiment uses i16 Zopro Booth digits, its own existing memory policy and original optimized IFMA kernels. No comparative performance claim about TACEO is made.

Closed prior work reviewed: shared-wire classification saves40%hashvisits but full SDKregresses~16%; exact earlygroupingreject smallpositive/failedgate; ordinaryindexedG1tree/G2prefix/indexgather/routedrounds/nativecolumns/tiles allfailed; reduction scratch reuse is already adopted. Source/resultsearch for window-major/predecoded/digitmatrix finds no earlier local candidate, but absence of search matches is not an exhaustive novelty proof. Prior ring/GLS/SRE/affineclosure/field/queue failures remain closed.

Do not extrapolate input-byte reduction to totalDRAMtraffic or2xprooflatency. Point loads, fieldwork, extra decoder stage/matrix initialization/allocation/admission and scheduling may dominate. Rawpre-groupfixtures are not guaranteed finalMSMpopulations. Only completecaller evidence can select a later actualquery/SDK trial.
