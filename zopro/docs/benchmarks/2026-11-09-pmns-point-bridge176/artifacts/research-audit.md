# Source and prior-work audit

Current accepted176/SDK6d83 is unchanged. The new screen follows the pushed
2026-11-08-pmns-toeplitz6-feasibility176 result5cf885ba68: its coefficient
identity is inherited, not rerun. The old local six-coefficient PMNS
native experiments and dense truncation remain closed. Before selecting
this root, completed README paths matching PMNS/Toeplitz/conversion/bridge
and actual current G1/G2 source were inspected.

Current production source contexts:
- msm.rs stream_g1_impl, both write_groups and push constructors, full-group
  reverse path, scalar tail and inverse bridge.
- msm/g2_borrowed.rs signed prefix, tail and reverse path.
- msm.rs Fq2 lanes_scaled_numerator/mul/square/norm and fq2_pack/unpack.
- simd.rs/FqBatch and bn254/fp.rs compact Montgomery coordinates.

Primary reference inspected via connected GitHub fetch:
https://github.com/arithPMNS/low_memory_efficient_PMNS/blob/main/C_codes_generator/gen_file_useful_functs.sage
Git blob SHA1e4c4933bf78c43563eb80b995c1f58c1f32a9750.
It emits a binary-digit/constant-polynomial linear combination followed by
internal reduction. The archived AVX example's useful_functs.c also has
that pattern, for a different prime. No C/Sage/Python code is executed or
imported into production. Full third-party source stays private and hash-bound.
Our bridge adapts the value scale to existing R256 host coordinates and
tests literal Montgomery export separately. No external runtime is transferred.

The primary PDF https://eprint.iacr.org/2025/467.pdf returnedHTTP403. Its
full conversion algorithm was not read from that PDF. The source above,
not the inaccessible PDF, is used to ground the conversion family. Initial
GitHub fetches tried master and an extracted AVX directory path; those
returned404. The repository uses main and ships the AVX example in a tar.xz.
The current generator file was then inspected successfully. These read-only
lookups are not workflow/scientific failures or performance observations.

No new paper-level mathematical novelty or speedup claim. Counts describe
the explicit local bridge recipe. Faster conversion, different invariants,
formats or algorithms are not excluded by this fixed study.
