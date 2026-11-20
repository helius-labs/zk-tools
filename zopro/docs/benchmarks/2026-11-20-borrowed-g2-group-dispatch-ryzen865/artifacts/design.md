# Borrowed first layer through both grouped G2 dispatch branches

- The preceding route probes reached the reducer on 2×2. The grouped dispatcher instead called generic MSM outside its narrow Horner interval, which bypassed the trial on 4×2 and 8×2. Extend the opt-in reducer to that actual grouped call site and keep the original raw MSM.
- Reserve the first pass's left-point, right-reference and intermediate-point vectors according to their tile counts. Retain the fixed input cap and the original oversized-tile path.
- Compare the released SDK, ordinary same-build path and borrowed first layer on two inputs each for 2×2, 4×2 and 8×2. Separately clock grouping. Route probes must confirm that both shapes use the trial and fall back under zero optional workspace.
- Check that both geometry branches preserve point weights and output order. Use independent proof verification and the native generator oracle. Record the distribution of class sizes and tile sizes in a separate diagnostic.
- Allow three minutes for build and functional verification and two minutes for full-proof measurement. Use C7a only after verifying actual route selection and a repeatable timing effect.
